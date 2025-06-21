//! FinalTile implementation

use super::FinalTile;

impl FinalTile {
    /// Creates a new FinalTile with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the request object ID
    pub fn get_request_obj_id(&self) -> i32 {
        self.request_obj_id
    }

    /// Sets the request object ID
    pub fn set_request_obj_id(&mut self, request_obj_id: i32) {
        self.request_obj_id = request_obj_id;
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

    /// Gets the tile label
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Sets the tile label
    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    /// Gets the tile count
    pub fn get_count(&self) -> i32 {
        self.count
    }

    /// Sets the tile count
    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    /// Increments the count and returns the previous value
    /// 
    /// This is equivalent to the Java `countPlusPlus()` method.
    /// Returns the count before incrementing.
    pub fn count_plus_plus(&mut self) -> i32 {
        let previous_count = self.count;
        self.count += 1;
        previous_count
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
