use serde::{Deserialize, Serialize};
use crate::models::TileDimensions;

/// Represents a rectangular tile with coordinates defining its position and boundaries
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tile {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}
