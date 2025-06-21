use serde::{Deserialize, Serialize};

/// Represents a cut operation on a tile, defining how a tile is divided into two child tiles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cut {
    /// X coordinate of the first corner
    pub x1: i32,
    /// Y coordinate of the first corner
    pub y1: i32,
    /// X coordinate of the second corner
    pub x2: i32,
    /// Y coordinate of the second corner
    pub y2: i32,
    /// Original width of the tile before cutting
    pub original_width: i32,
    /// Original height of the tile before cutting
    pub original_height: i32,
    /// Whether the cut is horizontal (true) or vertical (false)
    pub is_horizontal: bool,
    /// Coordinate where the cut is made
    pub cut_coord: i32,
    /// ID of the original tile being cut
    pub original_tile_id: i32,
    /// ID of the first child tile after cutting
    pub child1_tile_id: i32,
    /// ID of the second child tile after cutting
    pub child2_tile_id: i32,
}

/// Builder pattern for constructing Cut instances with fluent API
#[derive(Debug, Clone, Default)]
pub struct CutBuilder {
    pub(crate) x1: i32,
    pub(crate) y1: i32,
    pub(crate) x2: i32,
    pub(crate) y2: i32,
    pub(crate) original_width: i32,
    pub(crate) original_height: i32,
    pub(crate) is_horizontal: bool,
    pub(crate) cut_coord: i32,
    pub(crate) original_tile_id: i32,
    pub(crate) child1_tile_id: i32,
    pub(crate) child2_tile_id: i32,
}
