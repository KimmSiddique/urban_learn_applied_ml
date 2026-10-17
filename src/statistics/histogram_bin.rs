use crate::house_properties::house::House;

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
        // MAKE SURE HOUSE VEC IS SORTED OR WE ARE COOKED
        let mut index = 0;

        let mut max_bin_count = 0;
        let start = 0;
        let end = ((max_house_price / 100_000.0).ceil() * 100_000.0) as i32;

        // Interval is set to be about 5
        let interval = end / 5;

        let mut bins = Vec::new();

        for i in (start..end).step_by(interval as usize) {
            let bin_end = i + interval;
            let mut bin_count = 0;

            while index < house_vec.len() && house_vec[index].get_true_price() < bin_end as f64 {
                if house_vec[index].get_true_price() >= i as f64 {
                    bin_count += 1;
                }

                index += 1;
            }
            if bin_count > max_bin_count {
                max_bin_count = bin_count;
            }

            bins.push(HistogramBin::new(i as i32, bin_end as i32, bin_count));
        }
        (bins, end, max_bin_count)
    }
}
