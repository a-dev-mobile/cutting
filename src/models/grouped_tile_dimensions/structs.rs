use serde::{Deserialize, Serialize};
use crate::models::tile_dimensions::TileDimensions;

/// Represents tile dimensions with an associated group identifier
/// This is the Rust equivalent of Java's GroupedTileDimensions class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupedTileDimensions {
    /// The base tile dimensions (composition instead of inheritance)
    pub tile_dimensions: TileDimensions,
    /// The group identifier for this tile
    pub group: i32,
}
