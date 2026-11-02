use crate::{
    house_properties::house::House,
    statistics::{
        histogram_bin::{BinDetails, HistogramBin},
        house_statistics::HouseStatistics,
    },
};
use std::{error::Error, path::Path};

#[derive(serde::Serialize)]
pub(crate) struct StatisticsManager {
    pub(crate) house_statistics: Option<HouseStatistics>,
    pub(crate) bins_details: Option<BinDetails>,
}

impl StatisticsManager {
    pub(crate) fn new() -> Self {
        Self {
            house_statistics: None,
            bins_details: None,
        }
    }

    pub(crate) fn create_house_statistics(&mut self, houses: &mut [House]) {
        self.house_statistics = if houses.len() >= 3 {
            HouseStatistics::calculate_house_statistics(houses)
        } else {
            None
        };
    }

    pub(crate) fn calculate_bins(&mut self, houses: &[House], maximum_price: f64) {
        let (bins, end, max_count) = HistogramBin::calculate_bins(houses, maximum_price);
        self.bins_details = Some(BinDetails::new(bins, end, max_count));
    }

    pub(crate) fn draw_histograms(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let details = self
            .bins_details
            .as_ref()
            .ok_or("Calculate histogram bins before drawing")?;
        HistogramBin::draw_histogram(&details.bins, path, details.end, details.max_bin_count)
    }

    pub(crate) fn display_bins(&self) {
        if let Some(details) = &self.bins_details {
            println!("Price histogram bins (last upper boundary included):");
            for (i, bin) in details.bins.iter().enumerate() {
                println!(
                    "  [${:>9}, ${:>9}{}: {} houses",
                    bin.get_start(),
                    bin.get_end(),
                    if i + 1 == details.bins.len() {
                        ']'
                    } else {
                        ')'
                    },
                    bin.get_count()
                );
            }
        }
    }
}
