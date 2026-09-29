use crate::house_properties::position::Position;
use std::cmp::Ordering;

pub(crate) struct HouseDetails {
    position: Position,
    bedrooms: u8,
    bathrooms: u8,
    size: f64,
    land_size: f64,
    age: u16,
    true_price: f64,
}

impl PartialEq for HouseDetails {
    fn eq(&self, other: &Self) -> bool {
        self.true_price == other.true_price
    }
}

impl Eq for HouseDetails {}

impl PartialOrd for HouseDetails {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.true_price.partial_cmp(&other.true_price)
    }
}

impl Ord for HouseDetails {
    fn cmp(&self, other: &Self) -> Ordering {
        self.true_price
            .partial_cmp(&other.true_price)
            .expect("Could not compare house details")
    }
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

    pub(crate) fn new(
        position: Position,
        bedrooms: u8,
        bathrooms: u8,
        size: f64,
        land_size: f64,
        age: u16,
        true_price: f64,
    ) -> Self {
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
