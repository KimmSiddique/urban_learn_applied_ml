// This will contain the map, which I will design, initially will be an empty 2D Vector of types enums

use crate::house::Position;
use rand;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub(crate) enum BuildingType {
    Empty = 0,
    House = 1,
    School = 2,
    Shop = 3,
    Park = 4,
    Factory = 5,
}

pub(crate) fn create_map(
    rows: usize,
    cols: usize,
    building_positions: &mut [&mut Vec<Position>],
) -> Vec<Vec<BuildingType>> {
    let mut map = vec![vec![BuildingType::Empty; cols]; rows];

    let mut probability;

    for (y_pos, row) in map.iter_mut().enumerate() {
        for (x_pos, cell) in row.iter_mut().enumerate() {
            probability = rand::random_range(0..=100);

            if probability >= 0 && probability < 20 {
                *cell = BuildingType::House;
                building_positions[0].push(Position::new(y_pos, x_pos));
            } else if probability >= 20 && probability < 30 {
                *cell = BuildingType::School;
                building_positions[1].push(Position::new(y_pos, x_pos));
            } else if probability >= 30 && probability < 35 {
                *cell = BuildingType::Shop;
                building_positions[2].push(Position::new(y_pos, x_pos));
            } else if probability >= 35 && probability < 38 {
                *cell = BuildingType::Park;
                building_positions[3].push(Position::new(y_pos, x_pos));
            } else if probability >= 38 && probability < 45 {
                *cell = BuildingType::Factory;
                building_positions[4].push(Position::new(y_pos, x_pos));
            } else {
                *cell = BuildingType::Empty;
                building_positions[5].push(Position::new(y_pos, x_pos));
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

pub(crate) fn construct_map_from_file(path: &Path) -> Option<Vec<Vec<BuildingType>>> {
    let file = File::open(path).ok()?; // remmeber 
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).ok()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_construct_map_from_file() {
        let mut map_path = PathBuf::from("test_vectors");
        map_path.push("test_map1.json");

        let map = construct_map_from_file(&map_path).expect("failed to construct map from file");

        display_map(&map);

        assert_eq!(map[0][0], BuildingType::Empty);
        assert_eq!(map[3][3], BuildingType::Empty);
        assert_eq!(map[3][0], BuildingType::Factory);
    }
}
