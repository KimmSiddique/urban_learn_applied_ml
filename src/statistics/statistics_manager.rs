use crate::{
    house_properties::house::House,
    statistics::{
        histogram_bin::{BinDetails, HistogramBin},
        house_statistics::HouseStatistics,
    },
};
use std::path::Path;

// This will be used to manage the histograms and sorts later...
pub(crate) struct StatisticsManager {
    pub(crate) house_statistics: Option<HouseStatistics>,
    pub(crate) histogram_bin: Option<HistogramBin>,
    pub(crate) bins_details: Option<BinDetails>,
}

impl StatisticsManager {
    pub(crate) fn new() -> Self {
        Self {
            house_statistics: None,
            histogram_bin: None,
            bins_details: None,
        }
    }

    pub(crate) fn create_house_statistics(&mut self, houses: &mut [House]) {
        if let Some(house_stats) = HouseStatistics::calculate_house_statistics(houses) {
            self.house_statistics = Some(house_stats);
        }
    }

    pub(crate) fn calculate_bins(&mut self, house_vec: &[House]) {
        if let Some(house_statistics) = &self.house_statistics {
            let (bins, end, max_bin_count) =
                HistogramBin::calculate_bins(house_vec, house_statistics.get_high());
            self.bins_details = Some(BinDetails::new(bins, end, max_bin_count))
        }
    }

    pub(crate) fn draw_histograms(&self, path: impl AsRef<Path>) -> Option<()> {
        HistogramBin::draw_histogram(
            &self.bins_details.as_ref()?.bins,
            path,
            self.house_statistics.as_ref()?.get_high() as i32,
            self.bins_details.as_ref()?.max_bin_count,
        )
        .ok()?;
        Some(())
    }
}
