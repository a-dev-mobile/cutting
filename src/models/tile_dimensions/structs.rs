use serde::{Deserialize, Serialize};

use crate::Orientation;






/// Represents the dimensions and properties of a tile/panel to be cut
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TileDimensions {
    pub id: i32,
    pub width: i32,
    pub height: i32,
    pub label: Option<String>,
    pub material: String,
    pub orientation: Orientation,
    pub is_rotated: bool,
}
