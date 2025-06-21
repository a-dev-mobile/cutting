use super::structs::TileDimensions;

use crate::{constants::MaterialConstants, Orientation};

impl TileDimensions {
    /// Create a new tile with given dimensions
    pub fn new(id: i32, width: i32, height: i32) -> Self {
        Self {
            id,
            width,
            height,
            label: None,
            material: MaterialConstants::DEFAULT_MATERIAL.to_string(),
            orientation: Orientation::Any,
            is_rotated: false,
        }
    }

    /// Calculate the area of the tile
    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    /// Check if this tile can fit within a container
    pub fn fits(&self, container: &TileDimensions) -> bool {
        (self.width <= container.width && self.height <= container.height)
            || (self.can_rotate()
                && self.width <= container.height
                && self.height <= container.width)
    }

    /// Check if the tile can be rotated based on orientation constraints
    pub fn can_rotate(&self) -> bool {
        matches!(self.orientation, Orientation::Any)
    }

    /// Rotate the tile 90 degrees
    pub fn rotate_90(&mut self) {
        if self.can_rotate() {
            std::mem::swap(&mut self.width, &mut self.height);
            self.is_rotated = !self.is_rotated;
        }
    }

    /// Get the maximum dimension (width or height)
    pub fn max_dimension(&self) -> i32 {
        self.width.max(self.height)
    }

    /// Check if the tile is square
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Check if the tile is horizontally oriented
    pub fn is_horizontal(&self) -> bool {
        self.width >= self.height
    }

    /// Get a string representation of dimensions
    pub fn dimensions_string(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }
}

// Java equivalent methods
impl TileDimensions {
    // Equivalent to Java's hasSameDimensions
    pub fn has_same_dimensions(&self, other: &TileDimensions) -> bool {
        (self.width == other.width && self.height == other.height)
            || (self.width == other.height && self.height == other.width)
    }

    // Equivalent to Java's dimensionsBasedHashCode
    pub fn dimensions_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        let (min_dim, max_dim) = if self.width <= self.height {
            (self.width, self.height)
        } else {
            (self.height, self.width)
        };
        min_dim.hash(&mut hasher);
        max_dim.hash(&mut hasher);
        hasher.finish()
    }
}
