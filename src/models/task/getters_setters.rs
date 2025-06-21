//! Basic getters and setters for Task struct
//! 
//! This module contains simple accessor methods for Task fields.

use std::collections::HashMap;
use crate::models::{CalculationRequest, CalculationResponse, TileDimensions};
use super::Task;

impl Task {
    // ===== Basic Getters and Setters =====

    /// Get the task ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set the task ID
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    /// Get the calculation request
    pub fn calculation_request(&self) -> &Option<CalculationRequest> {
        &self.calculation_request
    }

    /// Set the calculation request
    pub fn set_calculation_request(&mut self, request: CalculationRequest) {
        self.calculation_request = Some(request);
    }

    /// Get the solution (returns a clone due to RwLock)
    pub fn solution(&self) -> Option<CalculationResponse> {
        self.solution.read().unwrap().clone()
    }

    /// Set the solution
    pub fn set_solution(&self, solution: CalculationResponse) {
        *self.solution.write().unwrap() = Some(solution);
    }

    /// Get the factor
    pub fn factor(&self) -> f64 {
        self.factor
    }

    /// Set the factor
    pub fn set_factor(&mut self, factor: f64) {
        self.factor = factor;
    }

    /// Check if min trim dimension is influenced
    pub fn is_min_trim_dimension_influenced(&self) -> bool {
        self.is_min_trim_dimension_influenced
    }

    /// Set min trim dimension influenced flag
    pub fn set_min_trim_dimension_influenced(&mut self, influenced: bool) {
        self.is_min_trim_dimension_influenced = influenced;
    }

    /// Get no material tiles
    pub fn no_material_tiles(&self) -> &Vec<TileDimensions> {
        &self.no_material_tiles
    }

    /// Set no material tiles
    pub fn set_no_material_tiles(&mut self, tiles: Vec<TileDimensions>) {
        self.no_material_tiles = tiles;
    }

    /// Get tile dimensions per material
    pub fn tile_dimensions_per_material(&self) -> &Option<HashMap<String, Vec<TileDimensions>>> {
        &self.tile_dimensions_per_material
    }

    /// Set tile dimensions per material
    pub fn set_tile_dimensions_per_material(&mut self, dimensions: HashMap<String, Vec<TileDimensions>>) {
        self.tile_dimensions_per_material = Some(dimensions);
    }

    /// Get stock dimensions per material
    pub fn stock_dimensions_per_material(&self) -> &Option<HashMap<String, Vec<TileDimensions>>> {
        &self.stock_dimensions_per_material
    }

    /// Set stock dimensions per material
    pub fn set_stock_dimensions_per_material(&mut self, dimensions: HashMap<String, Vec<TileDimensions>>) {
        self.stock_dimensions_per_material = Some(dimensions);
    }
}
