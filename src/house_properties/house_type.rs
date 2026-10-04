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
