use serde::{Deserialize, Serialize};

/// Represents an edge configuration for a panel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Edge {
    pub top: Option<String>,
    pub left: Option<String>,
    pub bottom: Option<String>,
    pub right: Option<String>,
}
