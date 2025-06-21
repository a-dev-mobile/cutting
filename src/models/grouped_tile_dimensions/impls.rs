use super::structs::GroupedTileDimensions;
use crate::models::tile_dimensions::TileDimensions;
use std::fmt;
use std::hash::{Hash, Hasher};

impl GroupedTileDimensions {
    /// Create a new GroupedTileDimensions from another GroupedTileDimensions (copy constructor equivalent)
    pub fn from_grouped(other: &GroupedTileDimensions) -> Self {
        Self {
            tile_dimensions: other.tile_dimensions.clone(),
            group: other.group,
        }
    }

    /// Create a new GroupedTileDimensions from TileDimensions and group
    pub fn from_tile_dimensions(tile_dimensions: TileDimensions, group: i32) -> Self {
        Self {
            tile_dimensions,
            group,
        }
    }

    /// Create a new GroupedTileDimensions with width, height, and group
    pub fn new(width: i32, height: i32, group: i32) -> Self {
        Self {
            tile_dimensions: TileDimensions::new(0, width, height), // id will be 0 by default
            group,
        }
    }

    /// Create a new GroupedTileDimensions with id, width, height, and group
    pub fn with_id(id: i32, width: i32, height: i32, group: i32) -> Self {
        Self {
            tile_dimensions: TileDimensions::new(id, width, height),
            group,
        }
    }

    /// Get the group identifier
    pub fn get_group(&self) -> i32 {
        self.group
    }

    /// Get a reference to the underlying tile dimensions
    pub fn tile_dimensions(&self) -> &TileDimensions {
        &self.tile_dimensions
    }

    /// Get a mutable reference to the underlying tile dimensions
    pub fn tile_dimensions_mut(&mut self) -> &mut TileDimensions {
        &mut self.tile_dimensions
    }

    /// Delegate methods to access tile dimension properties directly
    pub fn id(&self) -> i32 {
        self.tile_dimensions.id
    }

    pub fn width(&self) -> i32 {
        self.tile_dimensions.width
    }

    pub fn height(&self) -> i32 {
        self.tile_dimensions.height
    }

    pub fn area(&self) -> i32 {
        self.tile_dimensions.area()
    }

    pub fn fits(&self, container: &TileDimensions) -> bool {
        self.tile_dimensions.fits(container)
    }

    pub fn can_rotate(&self) -> bool {
        self.tile_dimensions.can_rotate()
    }

    pub fn rotate_90(&mut self) {
        self.tile_dimensions.rotate_90();
    }
}

// Display trait implementation (equivalent to Java's toString)
impl fmt::Display for GroupedTileDimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Note: The original Java had a typo "gropup" instead of "group"
        // I'm keeping it for exact functional equivalence
        write!(
            f,
            "id={}, gropup={}[{}x{}]",
            self.tile_dimensions.id,
            self.group,
            self.tile_dimensions.width,
            self.tile_dimensions.height
        )
    }
}

// Custom Hash implementation to match Java's hashCode behavior
impl Hash for GroupedTileDimensions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Equivalent to: (super.hashCode() * 31) + this.group
        // Hash the tile dimensions components manually since TileDimensions doesn't implement Hash
        self.tile_dimensions.id.hash(state);
        self.tile_dimensions.width.hash(state);
        self.tile_dimensions.height.hash(state);
        self.tile_dimensions.label.hash(state);
        self.tile_dimensions.material.hash(state);
        // Note: orientation and is_rotated would need to be hashed too if they implement Hash

        // Then incorporate the group with the Java-style multiplication by 31
        let base_hash = {
            let mut temp_hasher = std::collections::hash_map::DefaultHasher::new();
            self.tile_dimensions.id.hash(&mut temp_hasher);
            self.tile_dimensions.width.hash(&mut temp_hasher);
            self.tile_dimensions.height.hash(&mut temp_hasher);
            self.tile_dimensions.label.hash(&mut temp_hasher);
            self.tile_dimensions.material.hash(&mut temp_hasher);
            temp_hasher.finish()
        };
        let combined_hash = base_hash.wrapping_mul(31).wrapping_add(self.group as u64);
        combined_hash.hash(state);
    }
}

// Custom PartialEq implementation to match Java's equals method
impl PartialEq for GroupedTileDimensions {
    fn eq(&self, other: &Self) -> bool {
        // Equivalent to Java's equals method:
        // super.equals(obj) && this.group == ((GroupedTileDimensions) obj).group
        self.tile_dimensions == other.tile_dimensions && self.group == other.group
    }
}

// Eq trait implementation (required for HashMap keys)
impl Eq for GroupedTileDimensions {}
