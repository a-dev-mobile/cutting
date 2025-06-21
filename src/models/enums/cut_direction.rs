use serde::{Deserialize, Serialize};

/// Direction in which a cut can be made
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CutDirection {
    Horizontal,
    Vertical,
    Both,
}

impl Default for CutDirection {
    fn default() -> Self {
        Self::Both
    }
}
