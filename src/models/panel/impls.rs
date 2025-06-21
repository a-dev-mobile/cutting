use crate::models::edge::Edge;
use crate::errors::{AppError, Result};
use crate::models::panel::Panel;

impl Panel {
    /// Create a new panel with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new panel with specified id
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    /// Set the material, ignoring None values (equivalent to Java's null check)
    pub fn set_material(&mut self, material: Option<String>) {
        if let Some(mat) = material {
            self.material = mat;
        }
    }

    /// Validate the panel configuration
    /// Returns Ok(true) if valid, Ok(false) if invalid, or Err for parsing errors
    pub fn is_valid(&self) -> Result<bool> {
        // Check if panel is enabled
        if !self.enabled {
            return Ok(false);
        }

        // Check count
        if self.count <= 0 {
            return Ok(false);
        }

        // Check width
        let width_str = match &self.width {
            Some(w) => w,
            None => return Ok(false),
        };

        let width_value: f64 = width_str.parse()
            .map_err(|_| AppError::invalid_input(format!("Invalid width value: {}", width_str)))?;

        if width_value <= 0.0 {
            return Ok(false);
        }

        // Check height
        let height_str = match &self.height {
            Some(h) => h,
            None => return Ok(false),
        };

        let height_value: f64 = height_str.parse()
            .map_err(|_| AppError::invalid_input(format!("Invalid height value: {}", height_str)))?;

        Ok(height_value > 0.0)
    }

    /// Get width as a parsed f64 value
    pub fn width_as_f64(&self) -> Result<f64> {
        match &self.width {
            Some(w) => w.parse().map_err(|_| AppError::invalid_input(format!("Invalid width value: {}", w))),
            None => Err(AppError::invalid_input("Width is not set")),
        }
    }

    /// Get height as a parsed f64 value
    pub fn height_as_f64(&self) -> Result<f64> {
        match &self.height {
            Some(h) => h.parse().map_err(|_| AppError::invalid_input(format!("Invalid height value: {}", h))),
            None => Err(AppError::invalid_input("Height is not set")),
        }
    }

    /// Calculate the area of the panel
    pub fn area(&self) -> Result<f64> {
        let width = self.width_as_f64()?;
        let height = self.height_as_f64()?;
        Ok(width * height)
    }

    /// Check if the panel has valid dimensions
    pub fn has_valid_dimensions(&self) -> bool {
        self.width_as_f64().is_ok() && self.height_as_f64().is_ok()
    }
}

// Builder pattern implementation for Panel
impl Panel {
    /// Builder method to set width
    pub fn with_width(mut self, width: String) -> Self {
        self.width = Some(width);
        self
    }

    /// Builder method to set height
    pub fn with_height(mut self, height: String) -> Self {
        self.height = Some(height);
        self
    }

    /// Builder method to set count
    pub fn with_count(mut self, count: i32) -> Self {
        self.count = count;
        self
    }

    /// Builder method to set material
    pub fn with_material(mut self, material: String) -> Self {
        self.material = material;
        self
    }

    /// Builder method to set enabled status
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Builder method to set orientation
    pub fn with_orientation(mut self, orientation: i32) -> Self {
        self.orientation = orientation;
        self
    }

    /// Builder method to set label
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Builder method to set edge
    pub fn with_edge(mut self, edge: Edge) -> Self {
        self.edge = Some(edge);
        self
    }
}
