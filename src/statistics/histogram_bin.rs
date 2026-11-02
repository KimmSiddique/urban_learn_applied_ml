use crate::house_properties::house::House;
use plotters::prelude::*;
use std::{error::Error, fs::create_dir_all, path::Path};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;

#[derive(serde::Serialize)]
pub(crate) struct BinDetails {
    pub(crate) bins: Vec<HistogramBin>,
    pub(crate) end: i32,
    pub(crate) max_bin_count: i32,
}

impl BinDetails {
    pub(crate) fn new(bins: Vec<HistogramBin>, end: i32, max_bin_count: i32) -> Self {
        Self {
            bins,
            end,
            max_bin_count,
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct HistogramBin {
    start: i32,
    end: i32,
    count: i32,
}

impl HistogramBin {
    pub(crate) fn new(start: i32, end: i32, count: i32) -> Self {
        Self { start, end, count }
    }

    pub(crate) fn get_start(&self) -> i32 {
        self.start
    }
    pub(crate) fn get_end(&self) -> i32 {
        self.end
    }
    pub(crate) fn get_count(&self) -> i32 {
        self.count
    }

    pub(crate) fn calculate_bins(
        house_vec: &[House],
        max_house_price: f64,
    ) -> (Vec<HistogramBin>, i32, i32) {
        let maximum = house_vec
            .iter()
            .map(House::get_true_price)
            .fold(max_house_price, f64::max);
        let end =
            ((maximum / 100_000.0).ceil() * 100_000.0).clamp(100_000.0, 2_000_000_000.0) as i32;
        let interval = end / 5;
        let mut bins: Vec<_> = (0..5)
            .map(|i| Self::new(i * interval, (i + 1) * interval, 0))
            .collect();
        for house in house_vec {
            let index = ((house.get_true_price() / interval as f64) as usize).min(4);
            bins[index].count += 1;
        }
        let max_bin_count = bins.iter().map(Self::get_count).max().unwrap_or(0);
        (bins, end, max_bin_count)
    }

    pub(crate) fn draw_histogram(
        histogram_bins: &[HistogramBin],
        path: impl AsRef<Path>,
        max_house_price: i32,
        max_bin_count: i32,
    ) -> Result<(), Box<dyn Error>> {
        let mut path_buf = path.as_ref().to_path_buf();
        path_buf.set_extension("png");
        if let Some(parent) = path_buf.parent().filter(|p| !p.as_os_str().is_empty()) {
            create_dir_all(parent)?;
        }

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
                0..(max_bin_count + 1).max(1),
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
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::simulation::{map::create_map, simulation_manager::SimulationManager};
    use crate::statistics::house_statistics::HouseStatistics;

    fn priced_house(price: f64) -> House {
        use crate::house_properties::{
            environment_features::EnvironmentFeatures, house_details::HouseDetails,
            house_type::HouseType, position::Position,
        };
        House::new(
            HouseDetails::new(
                Position::new(0, 0),
                2,
                1,
                80,
                300,
                1,
                5,
                HouseType::Detached,
                20,
                price,
            ),
            EnvironmentFeatures::new(),
        )
    }

    #[test]
    fn bins_count_unsorted_prices_and_include_highest_boundary() {
        let houses = vec![
            priced_house(10_000.0),
            priced_house(40_000.0),
            priced_house(20_000.0),
            priced_house(100_000.0),
        ];
        let (bins, end, maximum) = HistogramBin::calculate_bins(&houses, 100_000.0);
        assert_eq!(end, 100_000);
        assert_eq!(maximum, 1);
        assert_eq!(
            bins.iter().map(HistogramBin::get_count).collect::<Vec<_>>(),
            vec![1, 1, 1, 0, 1]
        );
        let (sample, shared_end, _) = HistogramBin::calculate_bins(&houses[..1], 200_000.0);
        assert_eq!(shared_end, 200_000);
        assert_eq!(sample[0].get_end(), 40_000);
    }

    #[test]
    fn empty_and_constant_samples_have_valid_bins() {
        let (empty, end, maximum) = HistogramBin::calculate_bins(&[], 0.0);
        assert_eq!(empty.len(), 5);
        assert_eq!(end, 100_000);
        assert_eq!(maximum, 0);
        HistogramBin::draw_histogram(&empty, "plots/empty_histogram", end, maximum).unwrap();
        let (constant, _, maximum) = HistogramBin::calculate_bins(
            &[priced_house(10_000.0), priced_house(10_000.0)],
            10_000.0,
        );
        assert_eq!(maximum, 2);
        assert_eq!(constant.iter().map(HistogramBin::get_count).sum::<i32>(), 2);
    }

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
        histogram_bins.push(HistogramBin::new(1_500_000, 1_650_000, 10));

        HistogramBin::draw_histogram(&histogram_bins, "plots/test_plot1", 1_650_000, 30)
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

        HistogramBin::draw_histogram(&bins, "plots/plot100times100", end, max_count)
            .expect("Could not draw histogram");
        let statistics = HouseStatistics::calculate_house_statistics(&mut houses).unwrap();
        statistics.display_statistics();
    }
}
