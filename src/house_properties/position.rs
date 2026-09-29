use std::{fs::File, io::BufReader, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub(crate) struct Position {
    pub(crate) y: usize,
    pub(crate) x: usize,
}

impl Position {
    pub(crate) fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    pub(crate) fn get_position(&self) -> (usize, usize) {
        (self.y, self.x)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub(crate) fn read_vector_of_positions(path: &Path) -> Option<Vec<Position>> {
    let file = File::open(path).ok()?;
    let buff_reader = BufReader::new(file);
    serde_json::from_reader(buff_reader).ok()
}

pub(crate) fn display_vec_of_positions(positions: &Vec<Position>) {
    for pos in positions {
        println!("y: {} x: {}", pos.y, pos.x);
    }
}
