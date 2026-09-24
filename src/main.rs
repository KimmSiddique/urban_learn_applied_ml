mod map;
mod house;

use map::{create_map, display_map};

fn main() {
    let map = create_map(10, 10);
    display_map(&map);
}