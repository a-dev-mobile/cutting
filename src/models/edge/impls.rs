use super::structs::Edge;

impl Edge {
    /// Create a new edge with all sides set to None
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an edge with all sides set to the same value
    pub fn uniform(value: String) -> Self {
        Self {
            top: Some(value.clone()),
            left: Some(value.clone()),
            bottom: Some(value.clone()),
            right: Some(value),
        }
    }

    /// Check if any edge is defined
    pub fn has_any_edge(&self) -> bool {
        self.top.is_some() || self.left.is_some() || self.bottom.is_some() || self.right.is_some()
    }

    /// Check if all edges are defined
    pub fn has_all_edges(&self) -> bool {
        self.top.is_some() && self.left.is_some() && self.bottom.is_some() && self.right.is_some()
    }
}
