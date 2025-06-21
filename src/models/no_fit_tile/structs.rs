//! NoFitTile structure definition

/// Represents a tile that doesn't fit in the current cutting solution
/// 
/// This structure tracks tiles that cannot be placed in the current layout,
/// maintaining their dimensions, count, and optional metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct NoFitTile {
    /// Unique identifier for the tile
    pub id: i32,
    /// Width of the tile in units
    pub width: f64,
    /// Height of the tile in units  
    pub height: f64,
    /// Number of tiles of this type needed
    pub count: i32,
    /// Optional label for the tile
    pub label: Option<String>,
    /// Optional material specification
    pub material: Option<String>,
}

impl Default for NoFitTile {
    fn default() -> Self {
        Self {
            id: 0,
            width: 0.0,
            height: 0.0,
            count: 0,
            label: None,
            material: None,
        }
    }
}
