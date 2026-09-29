use rand;
use crate::house_properties::{house::House, building_type::BuildingType, environment_features::EnvironmentFeatures, house_details::HouseDetails, position::*};
use crate::statistics::house_statistics::HouseStatistics;
use crate::simulation::map::create_map;

const RADIUS: usize = 30;

pub(crate) struct SimulationManager {
    map: Vec<Vec<BuildingType>>,

    house_positions: Vec<Position>,
    school_positions: Vec<Position>,
    shop_positions: Vec<Position>,
    park_positions: Vec<Position>,
    factory_positions: Vec<Position>,

    width: usize,
    height: usize,

    houses: Vec<House>,

    house_statistics: Option<HouseStatistics>,
}

impl SimulationManager {
    fn new(width: usize, height: usize) -> Self {
        let mut new_house_positions = Vec::new();
        let mut new_school_positions = Vec::new();
        let mut new_shop_positions = Vec::new();
        let mut new_park_positions = Vec::new();
        let mut new_factory_positions = Vec::new();

        let mut slice = [
            &mut new_house_positions,
            &mut new_school_positions,
            &mut new_shop_positions,
            &mut new_park_positions,
            &mut new_factory_positions,
        ];

        let new_map = create_map(height, width, &mut slice);

        Self {
            map: new_map,
            house_positions: new_house_positions,
            school_positions: new_school_positions,
            shop_positions: new_shop_positions,
            park_positions: new_park_positions,
            factory_positions: new_factory_positions,
            width: width,
            height: height,
            house_statistics: None,

            houses: Vec::new(),
        }
    }

    pub(crate) fn process_house(
        width: usize,
        height: usize,
        house_positions: &mut [Position],
        map: &Vec<Vec<BuildingType>>,
    ) -> Vec<House> {
        let mut house_vec: Vec<House> = Vec::new();
        for house_pos in house_positions {
            let mut environment_features = EnvironmentFeatures::new();

            let (house_y, house_x) = house_pos.get_position();

            let start_x = house_x.saturating_sub(RADIUS);
            let end_x = (house_x + RADIUS).min(width - 1);

            let start_y = house_y.saturating_sub(RADIUS);
            let end_y = (house_y + RADIUS).min(height - 1);

            let mut house_evaluation_price: f64 = 0.0;

            for y in start_y..=end_y {
                for x in start_x..=end_x {
                    let distance = house_x.abs_diff(x) + house_y.abs_diff(y);

                    let building = map[y][x];

                    if building == BuildingType::Empty {
                        continue;
                    }

                    if building == BuildingType::School && distance <= 10 {
                        // Remember you later have to increment the school type inside this dont forget to do this
                        match distance {
                            1..=2 => {
                                house_evaluation_price += 15_000.0;
                            }
                            3..=5 => {
                                house_evaluation_price += 7_500.0;
                            }
                            6..=10 => {
                                house_evaluation_price += 2_500.0;
                            }
                            _ => {
                                house_evaluation_price += 0.0;
                            }
                        }
                        environment_features.add_school(Position::new(y, x), house_pos);
                    } else if building == BuildingType::Shop && distance <= 15 {
                        match distance {
                            1..=3 => {
                                house_evaluation_price += 25_000.0;
                            }
                            4..=10 => {
                                house_evaluation_price += 10_000.0;
                            }
                            11..=15 => {
                                house_evaluation_price += 5_000.0;
                            }
                            _ => {
                                house_evaluation_price += 0.0;
                            }
                        }
                        environment_features.add_shop(Position::new(y, x), house_pos);
                    } else if building == BuildingType::Park && distance <= 20 {
                        match distance {
                            1..=5 => {
                                house_evaluation_price += 35_000.0;
                            }
                            6..=15 => {
                                house_evaluation_price += 15_000.0;
                            }
                            16..=20 => {
                                house_evaluation_price += 8_750.0;
                            }
                            _ => {
                                house_evaluation_price += 0.0;
                            }
                        }
                        environment_features.add_park(Position::new(y, x), house_pos);
                    } else if building == BuildingType::Factory && distance <= 30 {
                        match distance {
                            1..=10 => {
                                house_evaluation_price -= 50_000.0;
                            }
                            11..=20 => {
                                house_evaluation_price -= 35_000.0;
                            }
                            21..=30 => {
                                house_evaluation_price -= 27_550.0;
                            }
                            _ => {
                                house_evaluation_price -= 15_000.0;
                            }
                        }
                        environment_features.add_factory(Position::new(y, x), house_pos);
                    }
                }
            }

            let (house_y, house_x) = house_pos.get_position();
            let house_dets = Self::generate_random_house_details(
                Position::new(house_y, house_x),
                house_evaluation_price,
            );
            house_vec.push(House::new(house_dets, environment_features));
        }
        house_vec
    }

    // This will return a house, which will then later be pushed inside the vector.
    fn generate_random_house_details(
        position: Position,
        mut house_evaluation_price: f64,
    ) -> HouseDetails {
        let random_bedrooms: u64 = rand::random_range(1..=10);
        let random_bathrooms: u64 = rand::random_range(2..=5);
        let random_size = rand::random_range(75.0..=300.0);
        let random_land_size = rand::random_range(150.0..=1000.0);
        let random_age = rand::random_range(0..=150);

        let age_scaler = {
            match random_age {
                0..=5 => 500.0,
                6..=20 => 0.0,
                21..=50 => -500.0,
                _ => 650.0,
            }
        };

        match random_bedrooms {
            1..=2 => house_evaluation_price += 40_000.0,
            3..=4 => house_evaluation_price += 75_000.0,
            mut no_of_rooms @ _ => {
                no_of_rooms -= 4;
                house_evaluation_price += (75_000.0) + (no_of_rooms * 50_000) as f64;
            }
        }

        match random_bathrooms {
            1 => house_evaluation_price += 0.0,
            2 => house_evaluation_price += 40_000.0,
            mut no_of_bathrooms @ _ => {
                no_of_bathrooms -= 2;
                house_evaluation_price += (40_000.0) + (15_000 * no_of_bathrooms) as f64;
            }
        }

        house_evaluation_price += random_size * (1_250.0 + age_scaler);
        house_evaluation_price += random_land_size * 950.0;

        HouseDetails::new(
            position,
            random_bedrooms as u8,
            random_bathrooms as u8,
            random_size,
            random_land_size,
            random_age,
            house_evaluation_price,
        )
    }

    pub(crate) fn display_house_details(&self) {
        for (index, house) in self.houses.iter().enumerate() {
            println!(
                "House #{} | Price: ${}",
                index + 1,
                house.house_details.get_house_price()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::simulation::map::construct_map_from_file;
    use super::*;

    #[test]
    fn test_generate_random_houses() {
        let house_evaluation_price = 0.0;
        for _ in 1..100 {
            let house_dets = SimulationManager::generate_random_house_details(
                Position::new(10, 10),
                house_evaluation_price,
            );

            if (2..=5).contains(&house_dets.get_bathrooms()) {
                assert!(true);
            }
            if (1..=10).contains(&house_dets.get_bedrooms()) {
                assert!(true);
            }
            if (75.0..=300.0).contains(&house_dets.get_house_size()) {
                assert!(true);
            }
            if (150.0..1000.0).contains(&house_dets.get_land_size()) {
                assert!(true);
            }
            if (0..=150).contains(&house_dets.get_house_age()) {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_process_house_8x8() {
        let width = 8;
        let height = 8;

        let mut map_path = PathBuf::from("test_vectors");
        map_path.push("test_map2.json");
        let mut house_pos_path = PathBuf::from("test_vectors");
        house_pos_path.push("test_map2_house_pos.json");

        let map = construct_map_from_file(&map_path).expect("Could not load map");
        let mut house_positions =
            read_vector_of_positions(&house_pos_path).expect("Could not load house positions");

        let house_count_mid = house_positions.len() / 2;

        std::thread::scope(|scope| {
            let (first, rest) = house_positions.split_at_mut(house_count_mid);

            let mut combined_house_vec = Vec::new();

            let house1 =
                scope.spawn(|| SimulationManager::process_house(width, height, first, &map));

            let house2 =
                scope.spawn(|| SimulationManager::process_house(width, height, rest, &map));

            let mut house1 = house1.join().unwrap();
            let mut house2 = house2.join().unwrap();

            combined_house_vec.append(&mut house1);
            combined_house_vec.append(&mut house2);

            assert_eq!(
                combined_house_vec[0].environment_features.get_num_shops(),
                7
            );
            assert_eq!(
                combined_house_vec[2]
                    .environment_features
                    .get_num_factories(),
                4
            );
            assert_eq!(
                combined_house_vec[8].environment_features.get_num_schools(),
                3
            );
            assert_eq!(
                combined_house_vec[15].environment_features.get_num_parks(),
                7
            );
        });
    }

    #[test]
    fn test_process_house_20x20() {
        let width = 20;
        let height = 20;

        let mut map_path = PathBuf::from("test_vectors");
        map_path.push("test_map3.json");
        let mut house_pos_path = PathBuf::from("test_vectors");
        house_pos_path.push("test_map3_house_pos.json");

        let map = construct_map_from_file(&map_path).expect("Could not load map");
        let mut house_positions =
            read_vector_of_positions(&house_pos_path).expect("Could not load house positions");

        let house_count_mid = house_positions.len() / 2;

        std::thread::scope(|scope| {
            let (first, rest) = house_positions.split_at_mut(house_count_mid);

            let mut combined_house_vec = Vec::new();

            let house1 =
                scope.spawn(|| SimulationManager::process_house(width, height, first, &map));

            let house2 =
                scope.spawn(|| SimulationManager::process_house(width, height, rest, &map));

            let mut house1 = house1.join().unwrap();
            let mut house2 = house2.join().unwrap();

            combined_house_vec.append(&mut house1);
            combined_house_vec.append(&mut house2);

            assert_eq!(
                combined_house_vec[0].environment_features.get_num_shops(),
                16
            );
            println!("House price: {:.2}", combined_house_vec[0].get_true_price());
            assert_eq!(
                combined_house_vec[0]
                    .environment_features
                    .get_num_factories(),
                29
            );
            assert_eq!(
                combined_house_vec[0].environment_features.get_num_schools(),
                4
            );
            assert_eq!(
                combined_house_vec[0].environment_features.get_num_parks(),
                25
            );

            assert_eq!(
                combined_house_vec[0]
                    .environment_features
                    .get_avg_shop_dist(),
                10.0
            );
        });
    }

}
