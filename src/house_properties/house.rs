use crate::house_properties::{
    environment_features::EnvironmentFeatures, house_details::HouseDetails,
};
use std::cmp::Ordering;

pub(crate) struct House {
    pub(crate) house_details: HouseDetails,
    pub(crate) environment_features: EnvironmentFeatures,
}

impl House {
    pub(crate) fn new(
        house_details: HouseDetails,
        environment_features: EnvironmentFeatures,
    ) -> Self {
        Self {
            house_details,
            environment_features,
        }
    }

    pub(crate) fn get_true_price(&self) -> f64 {
        self.house_details.get_house_price()
    }
}

impl PartialEq for House {
    fn eq(&self, other: &Self) -> bool {
        self.house_details == other.house_details
    }
}

impl Eq for House {}

impl PartialOrd for House {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.house_details.partial_cmp(&other.house_details)
    }
}

impl Ord for House {
    fn cmp(&self, other: &Self) -> Ordering {
        self.house_details.cmp(&other.house_details)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::house_properties::position::*;
    use std::path::PathBuf;

    #[test]
    fn test_calculate_average_distance() {
        let building_positions = vec![
            Position::new(10, 10),
            Position::new(5, 5),
            Position::new(2, 4),
        ];
        let parent_position = Position::new(5, 5);

        assert_eq!(
            14.0 / 3.0,
            EnvironmentFeatures::calculate_average_distance(&building_positions, &parent_position)
        );
    }

    #[test]
    fn test_get_position() {
        let house_position = Position::new(10, 10);
        let (pos_x, pos_y) = house_position.get_position();

        assert_eq!((10, 10), (pos_x, pos_y));
    }

    #[test]
    fn test_add_shop() {
        let mut env_features = EnvironmentFeatures::new();
        let parent_position = Position::new(8, 8);
        env_features.add_shop(Position::new(10, 10), &parent_position);
        assert_eq!(4.0, env_features.get_avg_shop_dist());

        env_features.add_shop(Position::new(5, 6), &parent_position);
        env_features.add_shop(Position::new(15, 20), &parent_position);

        assert_eq!(28.0 / 3.0, env_features.get_avg_shop_dist());
    }

    #[test]
    fn test_add_factory() {
        let mut env_features = EnvironmentFeatures::new();
        let parent_position = Position::new(8, 8);
        env_features.add_factory(Position::new(10, 10), &parent_position);
        assert_eq!(4.0, env_features.get_avg_factory_dist());

        env_features.add_factory(Position::new(5, 6), &parent_position);
        env_features.add_factory(Position::new(15, 20), &parent_position);

        assert_eq!(28.0 / 3.0, env_features.get_avg_factory_dist());
    }

    #[test]
    fn test_add_school() {
        let mut env_features = EnvironmentFeatures::new();
        let parent_position = Position::new(8, 8);
        env_features.add_school(Position::new(10, 10), &parent_position);
        assert_eq!(4.0, env_features.get_avg_school_dist());

        env_features.add_school(Position::new(10, 10), &parent_position);
        env_features.add_school(Position::new(0, 8), &parent_position);

        assert_eq!(16.0 / 3.0, env_features.get_avg_school_dist());
    }

    #[test]
    fn add_park() {
        let mut env_features = EnvironmentFeatures::new();
        let parent_position = Position::new(10, 10);
        env_features.add_park(Position::new(5, 5), &parent_position);
        assert_eq!(10.0, env_features.get_avg_park_dist());

        env_features.add_park(Position::new(0, 18), &parent_position);
        env_features.add_park(Position::new(2, 25), &parent_position);

        assert_eq!(17.0, env_features.get_avg_park_dist());
    }

    #[test]
    fn test_read_vector_of_positions() {
        let mut path = PathBuf::from("test_vectors");
        path.push("vec_positions1.json");

        let vector = read_vector_of_positions(&path).expect("failed to read vector of positions");

        display_vec_of_positions(&vector);

        assert_eq!(vector[0].y, 0);
        assert_eq!(vector[0].x, 0);

        assert_eq!(vector[6].y, 5);
        assert_eq!(vector[6].x, 0);
    }
}
