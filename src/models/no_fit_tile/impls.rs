//! NoFitTile implementation

use super::NoFitTile;

impl NoFitTile {
    /// Creates a new NoFitTile with the specified parameters
    /// 
    /// # Arguments
    /// * `id` - Unique identifier for the tile
    /// * `width` - Width of the tile
    /// * `height` - Height of the tile  
    /// * `count` - Number of tiles needed
    pub fn new(id: i32, width: f64, height: f64, count: i32) -> Self {
        Self {
            id,
            width,
            height,
            count,
            label: None,
            material: None,
        }
    }

    /// Gets the tile ID
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Sets the tile ID
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    /// Gets the tile width
    pub fn get_width(&self) -> f64 {
        self.width
    }

    /// Sets the tile width
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    /// Gets the tile height
    pub fn get_height(&self) -> f64 {
        self.height
    }

    /// Sets the tile height
    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    /// Gets the tile count
    pub fn get_count(&self) -> i32 {
        self.count
    }

    /// Sets the tile count
    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    /// Gets the tile label
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Sets the tile label
    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    /// Gets the tile material
    pub fn get_material(&self) -> Option<&str> {
        self.material.as_deref()
    }

    /// Sets the tile material
    pub fn set_material(&mut self, material: Option<String>) {
        self.material = material;
    }

    /// Calculates the area of the tile
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Calculates the total area for all tiles of this type
    pub fn total_area(&self) -> f64 {
        self.area() * self.count as f64
    }
}
