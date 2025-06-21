use serde::{Deserialize, Serialize};

/// Orientation of a tile (grain direction for wood, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Orientation {
    Horizontal,
    Vertical,
    Any,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Any
    }
}
