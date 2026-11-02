use super::{
    agent::Agent,
    agent_attributes::AgentMode,
    agent_manager::AgentManager,
    map::load_map_to_file,
    simulation_manager::SimulationManager,
    visualization::{comparison_chart, export_map, terminal_map},
};
use crate::{
    house_properties::{house::House, position::Position},
    statistics::{
        histogram_bin::BinDetails, house_statistics::HouseStatistics,
        statistics_manager::StatisticsManager,
    },
};
use serde::Serialize;
use smartcore::linalg::basic::matrix::DenseMatrix;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};

const FEATURE_NAMES: [&str; 16] = [
    "schools",
    "average_school_distance",
    "shops",
    "average_shop_distance",
    "parks",
    "average_park_distance",
    "factories",
    "average_factory_distance",
    "house_type_encoded",
    "land_size",
    "living_area",
    "bedrooms",
    "garage_spaces",
    "bathrooms",
    "quality",
    "age",
];

#[derive(Debug, Serialize)]
pub(crate) struct Metrics {
    pub(crate) mae: f64,
    pub(crate) rmse: f64,
    pub(crate) r_squared: Option<f64>,
}

pub(crate) fn metrics(actual: &[f64], predicted: &[f64]) -> Option<Metrics> {
    if actual.is_empty() || actual.len() != predicted.len() {
        return None;
    }
    let mean = actual.iter().sum::<f64>() / actual.len() as f64;
    let absolute = actual
        .iter()
        .zip(predicted)
        .map(|(a, p)| (a - p).abs())
        .sum::<f64>();
    let squared = actual
        .iter()
        .zip(predicted)
        .map(|(a, p)| (a - p).powi(2))
        .sum::<f64>();
    let total = actual.iter().map(|a| (a - mean).powi(2)).sum::<f64>();
    Some(Metrics {
        mae: absolute / actual.len() as f64,
        rmse: (squared / actual.len() as f64).sqrt(),
        r_squared: (total > 0.0).then(|| 1.0 - squared / total),
    })
}

#[derive(Serialize)]
struct HouseRecord {
    id: usize,
    position: Position,
    features: Vec<f64>,
    actual_price: f64,
}

#[derive(Serialize)]
struct Prediction {
    house_id: usize,
    position: Position,
    actual: f64,
    predicted: f64,
    signed_error: f64,
    absolute_error: f64,
}

#[derive(Serialize)]
pub(crate) struct AgentReport {
    pub(crate) strategy: String,
    pub(crate) steps: usize,
    energy_remaining: u32,
    pub(crate) houses: usize,
    steps_per_house: Option<f64>,
    unique_tiles: usize,
    house_types: usize,
    sample_statistics: Option<HouseStatistics>,
    sample_histogram: Option<BinDetails>,
    pub(crate) metrics: Option<Metrics>,
    mean_price_baseline: Option<Metrics>,
    status: String,
    sample_ids: Vec<usize>,
    route: Vec<Position>,
    predictions: Vec<Prediction>,
}

#[derive(Serialize)]
pub(crate) struct MapReport {
    map_id: usize,
    width: usize,
    height: usize,
    movement_budget: u32,
    collection_limit: usize,
    feature_names: [&'static str; 16],
    population_statistics: Option<HouseStatistics>,
    population_histogram: Option<BinDetails>,
    houses: Vec<HouseRecord>,
    test_house_ids: Vec<usize>,
    pub(crate) agents: Vec<AgentReport>,
}

fn statistics(
    houses: &[House],
    maximum_price: f64,
    path: impl AsRef<Path>,
) -> Result<StatisticsManager, Box<dyn Error>> {
    let mut manager = StatisticsManager::new();
    manager.create_house_statistics(&mut houses.to_vec());
    manager.calculate_bins(houses, maximum_price);
    manager.draw_histograms(path)?;
    Ok(manager)
}

pub(crate) fn common_unseen(agents: &[Agent], house_count: usize) -> Vec<usize> {
    (0..house_count)
        .filter(|i| agents.iter().all(|a| !a.collected.contains(i)))
        .collect()
}

fn number(value: Option<f64>) -> String {
    value.map_or_else(|| "N/A".into(), |v| format!("{v:.2}"))
}

pub(crate) fn run(
    simulation: &SimulationManager,
    map_id: usize,
    budget: u32,
    limit: usize,
    directory: &Path,
    print_maps: bool,
) -> Result<MapReport, Box<dyn Error>> {
    fs::create_dir_all(directory)?;
    let houses = &simulation.houses;
    let (height, width) = (simulation.map.len(), simulation.map[0].len());
    let mut manager = AgentManager::new();
    for mode in [
        AgentMode::RandomExplorationMode,
        AgentMode::SystematicExplorationMode,
        AgentMode::SmartExplorationMode,
    ] {
        manager.agents.push_back(Agent::new(mode, budget));
    }
    let mut agents = Vec::new();
    while let Some(mut agent) = manager.get_agent() {
        agent.explore(width, height, houses, limit);
        agents.push(agent);
    }
    let test_ids = common_unseen(&agents, houses.len());
    let test_features: Vec<_> = test_ids.iter().map(|&i| houses[i].featurify().0).collect();
    let test_x = if test_features.is_empty() {
        None
    } else {
        Some(DenseMatrix::from_2d_vec(&test_features)?)
    };
    let actual: Vec<_> = test_ids
        .iter()
        .map(|&i| houses[i].get_true_price())
        .collect();
    println!(
        "\n================ Map {map_id} | {} houses | {} common unseen houses ================",
        houses.len(),
        test_ids.len()
    );
    let maximum_price = houses.iter().map(House::get_true_price).fold(0.0, f64::max);
    let population = statistics(
        houses,
        maximum_price,
        directory.join("histogram_neighbourhood.png"),
    )?;
    if houses.len() <= 10 {
        simulation.display_house_details();
    }
    println!("Neighbourhood price statistics:");
    if let Some(stats) = &population.house_statistics {
        stats.display_statistics();
    } else {
        println!("N/A: fewer than three houses");
    }
    population.display_bins();
    let text_map = terminal_map(&simulation.map, &[], &[], &[]);
    fs::write(directory.join("neighbourhood.txt"), &text_map)?;
    if print_maps {
        println!("Neighbourhood:");
        println!("{text_map}");
    }
    export_map(&simulation.map, directory.join("neighbourhood.png"), &[])?;
    load_map_to_file(&simulation.map, directory.join("layout.json"))?;
    let mut reports = Vec::new();
    for (agent, name) in agents.iter_mut().zip(["Random", "Systematic", "Smart"]) {
        let mut sample = statistics(
            &agent.attributes.houses,
            maximum_price,
            directory.join(format!("histogram_{}.png", name.to_lowercase())),
        )?;
        let mut report = AgentReport {
            strategy: name.into(),
            steps: agent.path.len() - 1,
            energy_remaining: agent.attributes.get_agent_energy(),
            houses: agent.collected.len(),
            steps_per_house: (!agent.collected.is_empty())
                .then(|| (agent.path.len() - 1) as f64 / agent.collected.len() as f64),
            unique_tiles: agent
                .path
                .iter()
                .map(|p| (p.y, p.x))
                .collect::<std::collections::HashSet<_>>()
                .len(),
            house_types: agent
                .attributes
                .houses
                .iter()
                .map(|h| h.featurify().0[8] as u8)
                .collect::<std::collections::HashSet<_>>()
                .len(),
            sample_statistics: sample.house_statistics.clone(),
            sample_histogram: None,
            metrics: None,
            mean_price_baseline: None,
            status: "Evaluated".into(),
            sample_ids: agent.collected.clone(),
            route: agent.path.clone(),
            predictions: Vec::new(),
        };
        if report.houses < 2 {
            report.status = "N/A: fewer than two training houses".into();
        } else if let Some(x) = &test_x {
            let (train_x, train_y) = agent.attributes.create_dense_matrix_and_target_values()?;
            let mean = train_y.iter().sum::<f64>() / train_y.len() as f64;
            agent.attributes.set_regression_model(train_x, train_y)?;
            let predicted = agent
                .attributes
                .predict(x)
                .ok_or("Model prediction failed")?;
            report.metrics = metrics(&actual, &predicted);
            report.mean_price_baseline = metrics(&actual, &vec![mean; actual.len()]);
            report.predictions = test_ids
                .iter()
                .zip(predicted)
                .map(|(&id, p)| Prediction {
                    house_id: id,
                    position: houses[id].house_details.get_house_position(),
                    actual: houses[id].get_true_price(),
                    predicted: p,
                    signed_error: p - houses[id].get_true_price(),
                    absolute_error: (p - houses[id].get_true_price()).abs(),
                })
                .collect();
        } else {
            report.status = "N/A: no house remained unseen by every agent".into();
        }
        println!(
            "\n{name}: {} samples, {}/{} energy used, {} unique tiles, {} house types, {} steps/house",
            report.houses,
            report.steps,
            budget,
            report.unique_tiles,
            report.house_types,
            number(report.steps_per_house)
        );
        println!("Sample price statistics:");
        if let Some(stats) = &report.sample_statistics {
            stats.display_statistics();
        } else {
            println!("N/A: fewer than three samples");
        }
        sample.display_bins();
        report.sample_histogram = sample.bins_details.take();
        println!("{}", report.status);
        {
            let collected: Vec<_> = agent
                .attributes
                .houses
                .iter()
                .map(|h| h.house_details.get_house_position())
                .collect();
            let unseen: Vec<_> = test_ids
                .iter()
                .map(|&i| houses[i].house_details.get_house_position())
                .collect();
            let text_map = terminal_map(&simulation.map, &agent.path, &collected, &unseen);
            fs::write(
                directory.join(format!("{}.txt", name.to_lowercase())),
                &text_map,
            )?;
            if print_maps {
                println!("{text_map}");
            }
        }
        export_map(
            &simulation.map,
            directory.join(format!("{}.png", name.to_lowercase())),
            &agent.path,
        )?;
        reports.push(report);
    }
    println!(
        "\n{:<12} {:>8} {:>8} {:>14} {:>14} {:>10} {:>14}",
        "Strategy", "Samples", "Energy", "MAE", "RMSE", "R2", "Baseline RMSE"
    );
    for r in &reports {
        println!(
            "{:<12} {:>8} {:>8} {:>14} {:>14} {:>10} {:>14}",
            r.strategy,
            r.houses,
            r.steps,
            number(r.metrics.as_ref().map(|m| m.mae)),
            number(r.metrics.as_ref().map(|m| m.rmse)),
            number(r.metrics.as_ref().and_then(|m| m.r_squared)),
            number(r.mean_price_baseline.as_ref().map(|m| m.rmse))
        );
    }
    println!(
        "\nSame unseen houses: predicted price (absolute error); first 10, all saved to CSV/JSON"
    );
    println!(
        "{:>7} {:>12} {:>25} {:>25} {:>25}",
        "ID", "Actual", "Random", "Systematic", "Smart"
    );
    for (row, &id) in test_ids.iter().take(10).enumerate() {
        let describe = |agent: &AgentReport| {
            agent.predictions.get(row).map_or_else(
                || "N/A".into(),
                |p| format!("{:.2} ({:.2})", p.predicted, p.absolute_error),
            )
        };
        println!(
            "{:>7} {:>12.2} {:>25} {:>25} {:>25}",
            id,
            houses[id].get_true_price(),
            describe(&reports[0]),
            describe(&reports[1]),
            describe(&reports[2])
        );
    }
    comparison_chart(&reports, directory.join("comparison.png"))?;
    let mut csv = File::create(directory.join("predictions.csv"))?;
    writeln!(
        csv,
        "strategy,house_id,x,y,actual,predicted,signed_error,absolute_error"
    )?;
    for r in &reports {
        for p in &r.predictions {
            writeln!(
                csv,
                "{},{},{},{},{},{},{},{}",
                r.strategy,
                p.house_id,
                p.position.x,
                p.position.y,
                p.actual,
                p.predicted,
                p.signed_error,
                p.absolute_error
            )?;
        }
    }
    let report = MapReport {
        map_id,
        width,
        height,
        movement_budget: budget,
        collection_limit: limit,
        feature_names: FEATURE_NAMES,
        population_statistics: population.house_statistics,
        population_histogram: population.bins_details,
        houses: houses
            .iter()
            .enumerate()
            .map(|(id, h)| HouseRecord {
                id,
                position: h.house_details.get_house_position(),
                features: h.featurify().0,
                actual_price: h.get_true_price(),
            })
            .collect(),
        test_house_ids: test_ids,
        agents: reports,
    };
    serde_json::to_writer_pretty(File::create(directory.join("report.json"))?, &report)?;
    Ok(report)
}

#[derive(Serialize)]
struct SummaryValue {
    count: usize,
    mean: f64,
    sample_sd: Option<f64>,
}

fn aggregate(values: Vec<f64>) -> Option<SummaryValue> {
    if values.is_empty() {
        return None;
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    Some(SummaryValue {
        count: values.len(),
        mean,
        sample_sd: (values.len() > 1).then(|| {
            (values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64)
                .sqrt()
        }),
    })
}

pub(crate) fn summarize(maps: &[MapReport], output: &Path) -> Result<(), Box<dyn Error>> {
    let mut csv = File::create(output.join("summary.csv"))?;
    writeln!(
        csv,
        "map,width,height,strategy,samples,energy,test_houses,mae,rmse,r_squared"
    )?;
    for map in maps {
        for a in &map.agents {
            writeln!(
                csv,
                "{},{},{},{},{},{},{},{},{},{}",
                map.map_id,
                map.width,
                map.height,
                a.strategy,
                a.houses,
                a.steps,
                map.test_house_ids.len(),
                a.metrics
                    .as_ref()
                    .map(|m| m.mae.to_string())
                    .unwrap_or_default(),
                a.metrics
                    .as_ref()
                    .map(|m| m.rmse.to_string())
                    .unwrap_or_default(),
                a.metrics
                    .as_ref()
                    .and_then(|m| m.r_squared)
                    .map(|v| v.to_string())
                    .unwrap_or_default()
            )?;
        }
    }
    let mut summary = Vec::new();
    println!(
        "\nAcross {} map(s): mean +/- sample SD (each map has equal weight)",
        maps.len()
    );
    for strategy in ["Random", "Systematic", "Smart"] {
        let agents: Vec<_> = maps
            .iter()
            .flat_map(|m| &m.agents)
            .filter(|a| a.strategy == strategy)
            .collect();
        let mae = aggregate(
            agents
                .iter()
                .filter_map(|a| a.metrics.as_ref().map(|m| m.mae))
                .collect(),
        );
        let rmse = aggregate(
            agents
                .iter()
                .filter_map(|a| a.metrics.as_ref().map(|m| m.rmse))
                .collect(),
        );
        let r2 = aggregate(
            agents
                .iter()
                .filter_map(|a| a.metrics.as_ref().and_then(|m| m.r_squared))
                .collect(),
        );
        let describe = |v: &Option<SummaryValue>| {
            v.as_ref().map_or_else(
                || "N/A (0 maps)".into(),
                |s| {
                    format!(
                        "{:.2} +/- {} ({} maps)",
                        s.mean,
                        number(s.sample_sd),
                        s.count
                    )
                },
            )
        };
        println!(
            "{strategy:<12} MAE {} | RMSE {} | R2 {}",
            describe(&mae),
            describe(&rmse),
            describe(&r2)
        );
        summary.push(serde_json::json!({"strategy":strategy,"mae":mae,"rmse":rmse,"r_squared":r2,"samples":aggregate(agents.iter().map(|a|a.houses as f64).collect()),"energy":aggregate(agents.iter().map(|a|a.steps as f64).collect())}));
    }
    serde_json::to_writer_pretty(File::create(output.join("summary.json"))?, &summary)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn known_prediction_metrics() {
        let m = metrics(&[1.0, 2.0, 3.0], &[1.0, 2.0, 4.0]).unwrap();
        assert!((m.mae - 1.0 / 3.0).abs() < 1e-10);
        assert!((m.rmse - (1.0f64 / 3.0).sqrt()).abs() < 1e-10);
        assert_eq!(m.r_squared, Some(0.5));
        assert!(metrics(&[], &[]).is_none());
        assert!(metrics(&[1.0], &[]).is_none());
        assert_eq!(metrics(&[2.0, 2.0], &[1.0, 1.0]).unwrap().r_squared, None);
    }
    #[test]
    fn unseen_excludes_every_agents_samples() {
        let mut a = Agent::new(AgentMode::RandomExplorationMode, 1);
        let mut b = Agent::new(AgentMode::SmartExplorationMode, 1);
        a.collected = vec![0, 2];
        b.collected = vec![1, 2];
        assert_eq!(common_unseen(&[a, b], 5), vec![3, 4]);
    }
    #[test]
    fn summary_sample_sd_and_missing_values() {
        assert!(aggregate(vec![]).is_none());
        assert!(aggregate(vec![1.0]).unwrap().sample_sd.is_none());
        let s = aggregate(vec![1.0, 2.0, 3.0]).unwrap();
        assert_eq!(s.mean, 2.0);
        assert_eq!(s.sample_sd, Some(1.0));
    }
}
