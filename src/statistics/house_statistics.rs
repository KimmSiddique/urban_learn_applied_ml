use smartcore::numbers::floatnum::FloatNumber;
use crate::house_properties::house::House;

#[derive(Debug, Default)]
pub(crate) struct HouseStatistics {
    range: f64,
    mean: f64,
    median: f64,
    low: f64,
    high: f64,
    sd: f64,
    count: u32,
}

impl HouseStatistics {
    pub(crate) fn new(
        range: f64,
        mean: f64,
        median: f64,
        low: f64,
        high: f64,
        sd: f64,
        count: u32,
    ) -> Self {
        Self {
            range,
            mean,
            median,
            low,
            high,
            sd,
            count,
        }
    }

    pub(crate) fn calculate_house_statistics(houses: &mut Vec<House>) -> Option<HouseStatistics> {
        let house_count = houses.len();

        if house_count < 3 {
            println!("Not enough houses!");
            return None;
        }

        houses.sort(); // Sort the houses first so that we can get what we need in which order

        // get house count first as that is the easiest

        let high = houses.last().unwrap().get_true_price();
        let low = houses.first().unwrap().get_true_price();
        let range = high - low;

        let total: f64 = houses.iter().map(|house| house.get_true_price()).sum();

        let mean = total / house_count as f64;

        // Sum of squared differences
        let sosd = houses.iter().fold(0.0, |acc, house| {
            let difference = house.get_true_price() - mean;
            acc + difference.square()
        });

        let median = {
            if house_count % 2 == 1 {
                houses[house_count / 2].get_true_price()
            } else {
                let middle = house_count / 2;
                (houses[middle - 1].get_true_price() + houses[middle].get_true_price())
                    / 2.0
            }
        };

        let sd = f64::sqrt(sosd / (house_count - 1) as f64);
        Some(HouseStatistics::new(
            range,
            mean,
            median,
            low,
            high,
            sd,
            house_count as u32,
        ))
    }

    pub(crate) fn display_statistics(&self) {
        println!("Mean: {:.2}", self.mean);
        println!("Median: {:.2}", self.median);
        println!("Range: {:.2}", self.range);
        println!("Low: {:.2}", self.low);
        println!("High: {:.2}", self.high);
        println!("Count: {:.2}", self.count);
    }
}
