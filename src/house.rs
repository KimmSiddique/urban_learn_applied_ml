pub(crate) struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }

    pub(crate) fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}


pub(crate) struct House {
    pub(crate) house_details: HouseDetails,
    pub(crate) environment_features: EnvironmentFeatures,

}

pub(crate) struct HouseDetails {
    position: Position,
    bedrooms: u8,
    bathrooms: u8,
    size: f64,
    land_size: f64,
    age: u16,
    true_price: f64,
}

impl HouseDetails {
    pub(crate) fn get_bedrooms(&self) -> u8 {
        self.bedrooms
    }
    pub(crate) fn get_bathrooms(&self) -> u8 {
        self.bathrooms
    }
    pub(crate) fn get_house_size(&self) -> f64 {
        self.size
    }
    pub(crate) fn get_land_size(&self) -> f64 {
        self.land_size
    }
    pub(crate) fn get_house_age(&self) -> u16 {
        self.age
    }
    pub(crate) fn get_house_price(&self) -> f64 {
        self.true_price
    }

    pub(crate) fn new(position: Position, bedrooms: u8, bathrooms: u8, size: f64, land_size: f64, age: u16, true_price: f64) -> Self {
        Self {
            position,
            bedrooms,
            bathrooms,
            size,
            land_size,
            age,
            true_price,
        }
    }
} 

pub(crate) struct EnvironmentFeatures {
    schools_within_10km: u8,
    school_positions: Vec<Position>,

    // school distance will be capped to 10km, so even if we don't find any schools within a 10km radius it will still just say 10km
    average_school_distance_capped: f32,

    shops_within_15km: u8,
    shop_positions: Vec<Position>,
    average_shop_distance_capped: f32,

    parks_within_20km: u8,
    park_positions: Vec<Position>,
    average_park_distance_capped: f32,

    factories_within_30km: u8,
    factory_positions: Vec<Position>,
    average_factory_distance_capped: f32,

}

impl EnvironmentFeatures {
    pub(crate) fn new() -> Self {
        Self {
            schools_within_10km: 0,
            school_positions: Vec::new(),
            average_school_distance_capped: 10.0,

            shops_within_15km: 0,
            shop_positions: Vec::new(),
            average_shop_distance_capped: 15.0,

            parks_within_20km: 0,
            park_positions: Vec::new(),
            average_park_distance_capped: 20.0,

            factories_within_30km: 0,
            factory_positions: Vec::new(),
            average_factory_distance_capped: 30.0,
        
        }
    }

    pub(crate) fn add_school(&mut self, school_position: Position, house_position: &Position) {
        self.schools_within_10km += 1;
        self.school_positions.push(school_position);
        let new_avg = Self::calculate_average_distance(&self.school_positions, house_position);
        self.average_school_distance_capped = new_avg;
    }

    pub(crate) fn add_shop(&mut self, shop_position: Position, house_position: &Position) {
        self.shops_within_15km += 1;
        self.shop_positions.push(shop_position);
        let new_avg = Self::calculate_average_distance(&self.shop_positions, house_position);
        self.average_shop_distance_capped = new_avg;
    }

    pub(crate) fn add_park(&mut self, park_position: Position, house_position: &Position) {
        self.parks_within_20km += 1;
        self.park_positions.push(park_position);
        let new_avg = Self::calculate_average_distance(&self.park_positions, house_position);
        self.average_park_distance_capped = new_avg;
    }

    pub(crate) fn add_factory(&mut self, factory_position: Position, house_position: &Position) {
        self.factories_within_30km += 1;
        self.factory_positions.push(factory_position);
        let new_avg = Self::calculate_average_distance(&self.factory_positions, house_position);
        self.average_factory_distance_capped = new_avg;
    }

    pub(crate) fn calculate_average_distance(building_positions: &Vec<Position>, position: &Position) -> f32 {
        // Using manhattan distance
        let mut dx;
        let mut dy;
        let mut total_distance: f32 = 0.0;
        
        for build_position in building_positions {
            dx = position.x.abs_diff(build_position.x);
            dy = position.y.abs_diff(build_position.y);
            total_distance += dx as f32 + dy as f32;
        }

        let building_count: f32 = building_positions.len() as f32;
        let average_distance = total_distance / building_count;
        average_distance

    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_calculate_average_distance() {
        let building_positions = vec![Position::new(10, 10), Position::new(5, 5), Position::new(2, 4)];
        let parent_position = Position::new(5, 5);

        assert_eq!(14.0/3.0, EnvironmentFeatures::calculate_average_distance(&building_positions, &parent_position));
        
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
        assert_eq!(4.0, env_features.average_shop_distance_capped);

        env_features.add_shop(Position::new(5, 6), &parent_position);
        env_features.add_shop(Position::new(15, 20), &parent_position);

        assert_eq!(28.0/3.0, env_features.average_shop_distance_capped);

    }

    #[test]
     fn test_add_factory() {
        let mut env_features = EnvironmentFeatures::new();
        let parent_position = Position::new(8, 8);
        env_features.add_factory(Position::new(10, 10), &parent_position);
        assert_eq!(4.0, env_features.average_factory_distance_capped);

        env_features.add_factory(Position::new(5, 6), &parent_position);
        env_features.add_factory(Position::new(15, 20), &parent_position);

        assert_eq!(28.0/3.0, env_features.average_factory_distance_capped);

    }
    
    #[test]
      fn test_add_school() {
        let mut env_features = EnvironmentFeatures::new();
        let parent_position = Position::new(8, 8);
        env_features.add_school(Position::new(10, 10), &parent_position);
        assert_eq!(4.0, env_features.average_school_distance_capped);

        env_features.add_school(Position::new(10, 10), &parent_position);
        env_features.add_school(Position::new(0, 8), &parent_position);

        assert_eq!(16.0/3.0, env_features.average_school_distance_capped);

    }

    #[test]
    fn add_park() {
        let mut env_features = EnvironmentFeatures::new();
        let parent_position = Position::new(10, 10);
        env_features.add_park(Position::new(5, 5), &parent_position);
        assert_eq!(10.0, env_features.average_park_distance_capped);

        env_features.add_park(Position::new(0, 18), &parent_position);
        env_features.add_park(Position::new(2, 25), &parent_position);

        assert_eq!(17.0, env_features.average_park_distance_capped);
    }
}
