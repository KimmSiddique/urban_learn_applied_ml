use rand;

const DETACHED_PROB: i32 = 60;
const SEMIDETACHED_PROB: i32 = 20;
const TOWNHOUSE_PROB: i32 = 20;

const DETACHED_LOWER_BOUND: i32 = 0;
const DETACHED_UPPER_BOUND: i32 = DETACHED_LOWER_BOUND + DETACHED_PROB;
const SEMIDETACHED_LOWER_BOUND: i32 = DETACHED_UPPER_BOUND;
const SEMIDETACHED_UPPER_BOUND: i32 = SEMIDETACHED_LOWER_BOUND + SEMIDETACHED_PROB;
const TOWNHOUSE_LOWER_BOUND: i32 = SEMIDETACHED_UPPER_BOUND;
const TOWNHOUSE_UPPER_BOUND: i32 = TOWNHOUSE_LOWER_BOUND + TOWNHOUSE_PROB;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum HouseType {
    Detached,
    SemiDetached,
    TownHouse,
}

impl HouseType {
    pub(crate) fn get_random_housetype() -> HouseType {
        if DETACHED_PROB + SEMIDETACHED_PROB + TOWNHOUSE_PROB != 100 {
            panic!("House constant probabilities do not add up to 100");
        }

        let prob = rand::random_range(0..100);
        let housetype;
        if (DETACHED_LOWER_BOUND..DETACHED_UPPER_BOUND).contains(&prob) {
            housetype = HouseType::Detached
        } else if (SEMIDETACHED_LOWER_BOUND..SEMIDETACHED_UPPER_BOUND).contains(&prob) {
            housetype = HouseType::SemiDetached
        } else {
            housetype = HouseType::TownHouse
        }
        housetype
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_house_type() {
        // Generate random houses and then test their probabilities approximately
        const LOOP_SIZE: u32 = 100_000;
        let mut detached_count: f64 = 0.0;
        let mut semidetached_count: f64 = 0.0;
        let mut townhouse_count: f64 = 0.0;

        for _ in 0..LOOP_SIZE {
            let house = HouseType::get_random_housetype();

            match house {
                HouseType::Detached => detached_count += 1.0,
                HouseType::SemiDetached => semidetached_count += 1.0,
                HouseType::TownHouse => townhouse_count += 1.0,
            }
        }

        let detached_prob = detached_count / LOOP_SIZE as f64 * 100.0;
        let semidetached_prob = semidetached_count / LOOP_SIZE as f64 * 100.0;
        let townhouse_prob = townhouse_count / LOOP_SIZE as f64 * 100.0;

        println!("Detached prob: {:.2}", detached_prob);
        println!("Semi-Detached prob: {:.2}", semidetached_prob);
        println!("TownHouse prob: {:.2}", townhouse_prob);

        assert!(detached_prob > 30.0 && detached_prob < 65.0);
        assert!(semidetached_prob > 15.0 && semidetached_prob < 30.0);
        assert!(townhouse_prob > 15.0 && townhouse_prob < 30.0);

        assert_eq!(
            detached_count + semidetached_count + townhouse_count,
            LOOP_SIZE as f64
        );
    }
}
