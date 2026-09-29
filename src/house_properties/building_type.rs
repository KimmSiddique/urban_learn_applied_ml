use serde::{Deserialize, Serialize};

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
