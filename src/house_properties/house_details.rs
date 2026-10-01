use crate::house_properties::{house_type::HouseType, position::Position};
use std::cmp::Ordering;

pub(crate) struct HouseDetails {
    position: Position,
    bedrooms: u8,
    bathrooms: u8,
    living_area_size: f64,
    land_size: f64,
    garage_spaces: u8,
    house_quality: u8, // Has to be between 1-10
    house_type: HouseType,
    age: u16,
    sale_price: f64,
}

impl PartialEq for HouseDetails {
    fn eq(&self, other: &Self) -> bool {
        self.sale_price == other.sale_price
    }
}

impl Eq for HouseDetails {}

impl PartialOrd for HouseDetails {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sale_price.partial_cmp(&other.sale_price)
    }
}

impl Ord for HouseDetails {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sale_price
            .partial_cmp(&other.sale_price)
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
    pub(crate) fn get_living_area_size(&self) -> f64 {
        self.living_area_size
    }
    pub(crate) fn get_land_size(&self) -> f64 {
        self.land_size
    }
    pub(crate) fn get_house_age(&self) -> u16 {
        self.age
    }
    pub(crate) fn get_house_sale_price(&self) -> f64 {
        self.sale_price
    }
    pub(crate) fn get_house_type(&self) -> HouseType {
        self.house_type
    }
    pub(crate) fn get_house_quality(&self) -> u8 {
        self.house_quality
    }
    pub(crate) fn get_house_position(&self) -> Position {
        self.position
    }
    pub(crate) fn get_garage_spaces(&self) -> u8 {
        self.garage_spaces
    }

    pub(crate) fn new(
        position: Position,
        bedrooms: u8,
        bathrooms: u8,
        living_area_size: f64,
        land_size: f64,
        garage_spaces: u8,
        house_quality: u8, // Has to be between 1-10
        house_type: HouseType,
        age: u16,
        sale_price: f64,
    ) -> Self {
        Self {
            position,
            bedrooms,
            bathrooms,
            living_area_size,
            land_size,
            garage_spaces,
            house_quality,
            house_type,
            age,
            sale_price,
        }
    }
}
