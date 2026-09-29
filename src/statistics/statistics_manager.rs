use crate::{house_properties::house::House, statistics::histogram_bin::HistogramBin};
use plotters::prelude::*;
use std::{
    error::Error,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;

pub(crate) fn calculate_bins(house_vec: &Vec<House>) -> Vec<HistogramBin> {
    let mut histogram_bins = Vec::new();

    let mut bin_1: usize = 0;
    let mut bin_2: usize = 0;
    let mut bin_3: usize = 0;
    let mut bin_4: usize = 0;
    let mut bin_5: usize = 0;
    let mut bin_6: usize = 0;
    let mut bin_7: usize = 0;
    let mut bin_8: usize = 0;
    let mut bin_9: usize = 0;
    let mut bin_10: usize = 0;
    let mut overflow_bin: usize = 0;

    for house in house_vec {
        if (0.0..150_000.0).contains(&house.get_true_price()) {
            bin_1 += 1;
        } else if (150_000.0..300_000.0).contains(&house.get_true_price()) {
            bin_2 += 1;
        } else if (300_000.0..450_000.0).contains(&house.get_true_price()) {
            bin_3 += 1;
        } else if (450_000.0..600_000.0).contains(&house.get_true_price()) {
            bin_4 += 1;
        } else if (600_000.0..750_000.0).contains(&house.get_true_price()) {
            bin_5 += 1;
        } else if (750_000.0..900_000.0).contains(&house.get_true_price()) {
            bin_6 += 1;
        } else if (900_000.0..1_050_000.0).contains(&house.get_true_price()) {
            bin_7 += 1;
        } else if (1_050_000.0..1_200_000.0).contains(&house.get_true_price()) {
            bin_8 += 1;
        } else if (1_200_000.0..1_350_000.0).contains(&house.get_true_price()) {
            bin_9 += 1;
        } else if (1_350_000.0..1_500_000.0).contains(&house.get_true_price()) {
            bin_10 += 1;
        } else {
            overflow_bin += 1;
        }
    }

    histogram_bins.push(HistogramBin::new(0, 150_000, bin_1));
    histogram_bins.push(HistogramBin::new(150_000, 300_000, bin_2));
    histogram_bins.push(HistogramBin::new(300_000, 450_000, bin_3));
    histogram_bins.push(HistogramBin::new(450_000, 600_000, bin_4));
    histogram_bins.push(HistogramBin::new(600_000, 750_000, bin_5));
    histogram_bins.push(HistogramBin::new(750_000, 900_000, bin_6));
    histogram_bins.push(HistogramBin::new(900_000, 1_050_000, bin_7));
    histogram_bins.push(HistogramBin::new(1_050_000, 1_200_000, bin_8));
    histogram_bins.push(HistogramBin::new(1_200_000, 1_350_000, bin_9));
    histogram_bins.push(HistogramBin::new(1_350_000, 1_500_000, bin_10));
    histogram_bins.push(HistogramBin::new(1_500_000, usize::MAX, overflow_bin));

    histogram_bins
}

fn draw_histogram(
    histogram_bins: &Vec<HistogramBin>,
    path: impl AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    // Plotters uses a drawing backend...
    let mut pathy = path.as_ref().to_path_buf();
    create_dir_all("plots")?;

    let mut path_buf = PathBuf::from("plots");

    if !pathy.extension().is_some_and(|ext| ext == "png") {
        pathy.set_extension("png");
    }
    path_buf.push(pathy);

    let root = BitMapBackend::new(&path_buf, (WIDTH, HEIGHT)).into_drawing_area();

    root.fill(&WHITE)?;

    let key_points = vec![
        0, 150_000, 300_000, 450_000, 600_000, 750_000, 900_000, 1_050_000, 1_200_000, 1_350_000,
        1_500_000,
    ];

    let mut chart = ChartBuilder::on(&root)
        .caption("House price distribution", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d((0..1_500_000).with_key_points(key_points), 0..600)?;

    chart
        .configure_mesh()
        .x_desc("House Price")
        .y_desc("Number of Houses")
        .x_label_formatter(&|x| format!("${}k", x / 1000)) // reference to a closure.. never saw that before
        .draw()?;

    for bin in histogram_bins {
        let start = bin.get_start() as i32;
        let end = bin.get_end() as i32;
        let count = bin.get_count() as i32;

        let chart_style = CYAN.filled();

        chart.draw_series(std::iter::once(Rectangle::new(
            [(start, 0), (end, count)],
            chart_style,
        )))?;
    }

    root.present()?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::simulation::{map::create_map, simulation_manager::SimulationManager};
    use crate::statistics::house_statistics::HouseStatistics;

    #[test]
    fn test_draw_histogram() {
        let mut histogram_bins = Vec::new();

        histogram_bins.push(HistogramBin::new(0, 150_000, 5));
        histogram_bins.push(HistogramBin::new(150_000, 300_000, 10));
        histogram_bins.push(HistogramBin::new(300_000, 450_000, 15));
        histogram_bins.push(HistogramBin::new(450_000, 600_000, 10));
        histogram_bins.push(HistogramBin::new(600_000, 750_000, 23));
        histogram_bins.push(HistogramBin::new(750_000, 900_000, 30));
        histogram_bins.push(HistogramBin::new(900_000, 1_050_000, 11));
        histogram_bins.push(HistogramBin::new(1_050_000, 1_200_000, 12));
        histogram_bins.push(HistogramBin::new(1_200_000, 1_350_000, 15));
        histogram_bins.push(HistogramBin::new(1_350_000, 1_500_000, 23));
        histogram_bins.push(HistogramBin::new(1_500_000, usize::MAX, 10));

        draw_histogram(&histogram_bins, "test_plot1").expect("Could not create histogram image");
    }

    #[test]
    fn test_draw_histogram_for_larger_maps() {
        let mut house_positions = vec![];
        let mut slice = [&mut house_positions];
        let map = create_map(100, 100, &mut slice);

        let mut houses = SimulationManager::process_house(100, 100, &mut house_positions, &map);
        let bins = calculate_bins(&houses);

        draw_histogram(&bins, "plot100times100").expect("Could not draw histogram");
        let statistics = HouseStatistics::calculate_house_statistics(&mut houses).unwrap();
        statistics.display_statistics();
    }
}
