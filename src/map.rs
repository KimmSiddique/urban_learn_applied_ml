// This will contain the map, which I will design, initially will be an empty 2D Vector of types enums

use rand;


#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum BuildingType {
    Empty = 0,
    House = 1,
    School = 2,
    Shop = 3,
    Park = 4,
    Factory = 5
}

pub(crate) fn create_map(rows: usize, cols: usize) -> Vec<Vec<BuildingType>> {
    let mut map = vec![vec![BuildingType::Empty; cols]; rows];

    let mut probability;

    for row in map.iter_mut() {
        for cell in row {
            
            probability = rand::random_range(0..=100);

            if probability >= 0 && probability < 20 {
                *cell = BuildingType::House;
            }
            else if probability >= 20 && probability < 30 {
                *cell = BuildingType::School;
            }
            else if probability >= 30 && probability < 35 {
                *cell = BuildingType::Shop;
            }
            else if probability >= 35  && probability < 38 {
                *cell = BuildingType::Park;
            } 
            else if probability >= 38 && probability < 45 {
                *cell = BuildingType::Factory
            }
            else {
                *cell = BuildingType::Empty;
            }

        }
    }
    map
}

pub(crate) fn display_map(map: &Vec<Vec<BuildingType>>) {
    for row in map {
        for cell in row {
            print!("{} ", *cell as u8);
        }
        println!();
    }
}