//! FinalTile structure definition

/// Represents a tile in the final cutting solution
/// 
/// This structure represents tiles that have been successfully placed
/// in the cutting layout, with tracking for the original request.
#[derive(Debug, Clone, PartialEq)]
pub struct FinalTile {
    /// Reference to the original request object ID
    pub request_obj_id: i32,
    /// Width of the tile in units
    pub width: f64,
    /// Height of the tile in units
    pub height: f64,
    /// Optional label for the tile
    pub label: Option<String>,
    /// Number of tiles of this type in the solution
    pub count: i32,
}

impl Default for FinalTile {
    fn default() -> Self {
        Self {
            request_obj_id: 0,
            width: 0.0,
            height: 0.0,
            label: None,
            count: 0,
        }
    }
}
