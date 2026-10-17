use crate::statistics::histogram_bin::HistogramBin;
use plotters::prelude::*;
use std::{
    error::Error,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;

fn draw_histogram(
    histogram_bins: &[HistogramBin],
    path: impl AsRef<Path>,
    max_house_price: i32,
    max_bin_count: i32,
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

    let mut histogram_bin_positions: Vec<i32> =
        histogram_bins.iter().map(|x| x.get_start()).collect();
    histogram_bin_positions.push(max_house_price);

    let mut chart = ChartBuilder::on(&root)
        .caption("House price distribution", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(
            (0..max_house_price).with_key_points(histogram_bin_positions),
            0..max_bin_count,
        )?;

    chart
        .configure_mesh()
        .x_desc("House Price")
        .y_desc("Number of Houses")
        .max_light_lines(1)
        .x_label_formatter(&|x| format!("${}k", x / 1000)) // reference to a closure.. never saw that before
        .draw()?;

    let chart_style = CYAN.filled();

    chart.draw_series(histogram_bins.iter().map(|x| {
        Rectangle::new(
            [(x.get_start(), 0), (x.get_end(), x.get_count())],
            chart_style,
        )
    }))?;

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
        histogram_bins.push(HistogramBin::new(1_500_000, i32::MAX, 10));

        draw_histogram(&histogram_bins, "test_plot1", 100_000, 30)
            .expect("Could not create histogram image");
    }

    #[test]
    fn test_draw_histogram_for_larger_maps() {
        let mut house_positions = vec![];
        let mut slice = [&mut house_positions];
        let map = create_map(100, 100, &mut slice);

        let mut houses = SimulationManager::process_house(100, 100, &mut house_positions, &map);
        houses.sort_unstable();
        let house_statistics = HouseStatistics::calculate_house_statistics(&mut houses)
            .expect("Could not calculate house statistics");

        let (bins, end, max_count) =
            HistogramBin::calculate_bins(&houses, house_statistics.get_high());

        draw_histogram(&bins, "plot100times100", end, max_count).expect("Could not draw histogram");
        let statistics = HouseStatistics::calculate_house_statistics(&mut houses).unwrap();
        statistics.display_statistics();
    }
}
