use crate::house_properties::{
    building_type::BuildingType, environment_features::EnvironmentFeatures, house::House,
    house_details::HouseDetails, position::*,
};
use crate::simulation::agent::Agent;
use crate::simulation::map::create_map;
use crate::statistics::house_statistics::HouseStatistics;
use std::collections::VecDeque;

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
    agents: VecDeque<Agent>,
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
            agents: VecDeque::new(),
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
                                house_evaluation_price += 7_500.0;
                            }
                            3..=5 => {
                                house_evaluation_price += 5_000.0;
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
                                house_evaluation_price += 3_000.0;
                            }
                            4..=10 => {
                                house_evaluation_price += 5_000.0;
                            }
                            11..=15 => {
                                house_evaluation_price += 2_000.0;
                            }
                            _ => {
                                house_evaluation_price += 0.0;
                            }
                        }
                        environment_features.add_shop(Position::new(y, x), house_pos);
                    } else if building == BuildingType::Park && distance <= 20 {
                        match distance {
                            1..=5 => {
                                house_evaluation_price += 10_000.0;
                            }
                            6..=15 => {
                                house_evaluation_price += 6_000.0;
                            }
                            16..=20 => {
                                house_evaluation_price += 2_500.0;
                            }
                            _ => {
                                house_evaluation_price += 0.0;
                            }
                        }
                        environment_features.add_park(Position::new(y, x), house_pos);
                    } else if building == BuildingType::Factory && distance <= 30 {
                        match distance {
                            1..=10 => {
                                house_evaluation_price -= 20_000.0;
                            }
                            11..=20 => {
                                house_evaluation_price -= 10_000.0;
                            }
                            21..=30 => {
                                house_evaluation_price -= 4_000.0;
                            }
                            _ => {
                                house_evaluation_price -= 1_500.0;
                            }
                        }
                        environment_features.add_factory(Position::new(y, x), house_pos);
                    }
                }
            }

            let (house_y, house_x) = house_pos.get_position();
            let house_dets = HouseDetails::generate_random_house_details(
                Position::new(house_y, house_x),
                house_evaluation_price,
            );
            house_vec.push(House::new(house_dets, environment_features));
        }
        house_vec
    }

    pub(crate) fn display_house_details(&self) {
        for (index, house) in self.houses.iter().enumerate() {
            println!(
                "House #{} | Price: ${}",
                index + 1,
                house.house_details.get_house_sale_price()
            );
        }
    }

    // Use this when processing larger batches of house to distribute workload across multiple threads
    pub(crate) fn multithread_process_house(
        house_positions: &mut Vec<Position>,
        width: usize,
        height: usize,
        map: &Vec<Vec<BuildingType>>,
    ) -> Vec<House> {
        let mid = house_positions.len();
        let (first, second) = house_positions.split_at_mut(mid);

        let mut new_combined_house_vec = Vec::new();

        std::thread::scope(|scope| {
            let pt1 = scope.spawn(|| SimulationManager::process_house(width, height, first, map));
            let pt2 = scope.spawn(|| SimulationManager::process_house(width, height, second, map));

            let mut house1 = pt1
                .join()
                .expect("Failed to process house in multithread_process_house");
            let mut house2 = pt2
                .join()
                .expect("Failed to process house in multithread_process_house");

            new_combined_house_vec.append(&mut house1);
            new_combined_house_vec.append(&mut house2);
            new_combined_house_vec
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::map::construct_map_from_file;
    use std::path::PathBuf;

    #[test]
    fn test_generate_random_houses() {
        let house_evaluation_price = 0.0;
        for _ in 1..100 {
            let house_dets = HouseDetails::generate_random_house_details(
                Position::new(10, 10),
                house_evaluation_price,
            );

            if (2..=5).contains(&house_dets.get_bathrooms()) {
                assert!(true);
            }
            if (1..=10).contains(&house_dets.get_bedrooms()) {
                assert!(true);
            }
            if (75..=300).contains(&house_dets.get_land_size()) {
                assert!(true);
            }
            if (150..1000).contains(&house_dets.get_living_area_size()) {
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

        let combined_house_vec =
            SimulationManager::multithread_process_house(&mut house_positions, width, height, &map);

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

        let combined_house_vec =
            SimulationManager::multithread_process_house(&mut house_positions, width, height, &map);

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
    }
}
