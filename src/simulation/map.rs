// This will contain the map, which I will design, initially will be an empty 2D Vector of types enums

use rand;
use crate::house_properties::{building_type::BuildingType, position::Position};
use std::{
    fs::{File, create_dir_all},
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

pub(crate) fn create_map(
    rows: usize,
    cols: usize,
    building_positions: &mut [&mut Vec<Position>],
) -> Vec<Vec<BuildingType>> {
    let mut map = vec![vec![BuildingType::Empty; cols]; rows];
    let building_positions_len = building_positions.len();

    let mut probability;

    for (y_pos, row) in map.iter_mut().enumerate() {
        for (x_pos, cell) in row.iter_mut().enumerate() {
            probability = rand::random_range(0..=100);

            if probability >= 0 && probability < 20 {
                *cell = BuildingType::House;
                building_positions[0].push(Position::new(y_pos, x_pos));
            } else if probability >= 20 && probability < 30 && building_positions_len >= 2 {
                *cell = BuildingType::School;
                building_positions[1].push(Position::new(y_pos, x_pos));
            } else if probability >= 30 && probability < 35 && building_positions_len >= 3 {
                *cell = BuildingType::Shop;
                building_positions[2].push(Position::new(y_pos, x_pos));
            } else if probability >= 35 && probability < 38 && building_positions_len >= 4 {
                *cell = BuildingType::Park;
                building_positions[3].push(Position::new(y_pos, x_pos));
            } else if probability >= 38 && probability < 45 && building_positions_len >= 5 {
                *cell = BuildingType::Factory;
                building_positions[4].push(Position::new(y_pos, x_pos));
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

pub(crate) fn load_map_to_file(
    map: &Vec<Vec<BuildingType>>,
    path: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut map_path = PathBuf::from("saved_maps");

    create_dir_all(&map_path)?;

    let mut pathy = path.as_ref().to_path_buf();

    if !pathy.extension().is_some_and(|ext| ext == "json") {
        pathy.set_extension("json");
    }

    map_path.push(pathy);

    let user_path = File::create(map_path)?;
    let mut writer = BufWriter::new(user_path);

    serde_json::to_writer_pretty(&mut writer, &map)?;

    writer.flush()?;

    Ok(())
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

    #[test]
    fn test_load_map_to_file() {
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
        let map = create_map(5, 5, &mut slice);
        load_map_to_file(&map, "saved_map1.json").expect("Could not save map");
        load_map_to_file(&map, "saved_map2").expect("Could not save map");
    }

    #[test]
    fn test_construct_map_after_load_map_to_file() {
        // Assuming you ran the test_load_map_to_file() function, there should exist some saved maps...
        test_load_map_to_file();
        let mut map_path = PathBuf::from("saved_maps");
        map_path.push("saved_map1.json");
        let map = construct_map_from_file(&map_path).expect("Could not construct map");

        display_map(&map);
    }
}
