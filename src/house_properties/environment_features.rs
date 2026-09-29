use crate::house_properties::position::Position;

pub(crate) struct EnvironmentFeatures {
    schools_within_10km: u16,
    school_positions: Vec<Position>,

    // school distance will be capped to 10km, so even if we don't find any schools within a 10km radius it will still just say 10km
    average_school_distance_capped: f32,

    shops_within_15km: u16,
    shop_positions: Vec<Position>,
    average_shop_distance_capped: f32,

    parks_within_20km: u16,
    park_positions: Vec<Position>,
    average_park_distance_capped: f32,

    factories_within_30km: u16,
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

    pub(crate) fn calculate_average_distance(
        building_positions: &Vec<Position>,
        position: &Position,
    ) -> f32 {
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

    pub(crate) fn get_num_schools(&self) -> u16 {
        self.schools_within_10km
    }

    pub(crate) fn get_num_shops(&self) -> u16 {
        self.shops_within_15km
    }

    pub(crate) fn get_num_parks(&self) -> u16 {
        self.parks_within_20km
    }

    pub(crate) fn get_num_factories(&self) -> u16 {
        self.factories_within_30km
    }

    pub(crate) fn get_avg_shop_dist(&self) -> f32 {
        self.average_shop_distance_capped
    }

    pub(crate) fn get_avg_school_dist(&self) -> f32 {
        self.average_school_distance_capped
    }

    pub(crate) fn get_avg_factory_dist(&self) -> f32 {
        self.average_factory_distance_capped
    }

    pub(crate) fn get_avg_park_dist(&self) -> f32 {
        self.average_park_distance_capped
    }
}
