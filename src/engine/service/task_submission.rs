//! Task submission operations
//!
//! This module handles task submission and all related validation logic.

use chrono::Utc;

use crate::{
    errors::Result,
    models::{
        CalculationRequest, CalculationSubmissionResult,
        enums::StatusCode,
        tile_dimensions::structs::TileDimensions,
        configuration::structs::Configuration,
        task::structs::Task,
        panel::structs::Panel,
        enums::Orientation,
    },
    logging::macros::{info, error, debug},
};

use super::{
    core::{CutListOptimizerServiceImpl, MAX_PANELS_LIMIT, MAX_STOCK_PANELS_LIMIT, MAX_ALLOWED_DIGITS},
    computation::grouping::CollectionUtils,
};

/// Task submission operations implementation
impl CutListOptimizerServiceImpl {
    /// Submit a new optimization task for processing
    pub async fn submit_task_impl(&self, request: CalculationRequest) -> Result<CalculationSubmissionResult> {
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // Validate request
        if let Some(error_code) = RequestValidator::validate_request(&request).await {
            return Ok(CalculationSubmissionResult {
                status_code: error_code,
                task_id: None,
            });
        }

        // Check if multiple tasks per client are allowed
        if !self.get_allow_multiple_tasks_per_client() {
            // TODO: Add client_info field to CalculationRequest or remove this check
            // For now, skip client task limit check
        }

        // Generate task ID using date format + counter (like Java implementation)
        let task_id = self.generate_task_id();

        // Start computation in background
        let request_clone = request.clone();
        let task_id_clone = task_id.clone();
        
        tokio::spawn(async move {
            if let Err(e) = Self::compute_task(request_clone, task_id_clone).await {
                error!("Task computation failed: {}", e);
            }
        });

        Ok(CalculationSubmissionResult {
            status_code: StatusCode::Ok,
            task_id: Some(task_id),
        })
    }

    /// Check client task limit
    async fn check_client_task_limit(&self, client_id: &str) -> Result<Option<StatusCode>> {
        // TODO: Implement with running tasks manager
        // For now, allow all tasks
        let _ = client_id;
        Ok(None)
    }

    /// Main computation method (migrated from Java)
    async fn compute_task(request: CalculationRequest, task_id: String) -> Result<()> {
        info!("Starting computation for task: {}", task_id);

        // Convert panels to tile dimensions with scaling factor
        let (tiles, stock_tiles, _factor) = Self::convert_panels_to_tiles(&request)?;

        // Create task
        let task = Task::new(task_id.clone());
        
        // Group tiles by material
        let tiles_per_material = CollectionUtils::get_tile_dimensions_per_material(&tiles)?;
        let stock_per_material = CollectionUtils::get_tile_dimensions_per_material(&stock_tiles)?;

        // Process each material
        for (material, material_tiles) in tiles_per_material {
            if let Some(material_stock) = stock_per_material.get(&material) {
                Self::compute_material(
                    material_tiles,
                    material_stock.clone(),
                    request.configuration.as_ref(),
                    &task,
                    &material,
                ).await?;
            }
        }

        info!("Completed computation for task: {}", task_id);
        Ok(())
    }

    /// Convert panels to tile dimensions with proper scaling
    fn convert_panels_to_tiles(request: &CalculationRequest) -> Result<(Vec<TileDimensions>, Vec<TileDimensions>, f64)> {
        let mut tiles = Vec::new();
        let mut stock_tiles = Vec::new();

        // Calculate scaling factor based on decimal places (like Java)
        let max_decimal_places = Self::get_max_decimal_places(&request.panels, &request.stock_panels);
        let factor = 10.0_f64.powi(max_decimal_places as i32);

        // Convert regular panels
        for panel in &request.panels {
            if panel.is_valid()? {
                for _ in 0..panel.count {
                    let width_str = panel.width.as_ref().ok_or_else(|| crate::errors::AppError::invalid_input("Panel width is None"))?;
                    let height_str = panel.height.as_ref().ok_or_else(|| crate::errors::AppError::invalid_input("Panel height is None"))?;
                    
                    let width = (width_str.parse::<f64>().map_err(|e| crate::errors::AppError::Core(crate::errors::CoreError::ParseFloat(e)))? * factor).round() as i32;
                    let height = (height_str.parse::<f64>().map_err(|e| crate::errors::AppError::Core(crate::errors::CoreError::ParseFloat(e)))? * factor).round() as i32;
                    
                    let mut tile = TileDimensions::new(panel.id, width, height);
                    tile.material = panel.material.clone();
                    tile.orientation = Self::convert_orientation(panel.orientation);
                    tile.label = panel.label.clone();
                    
                    tiles.push(tile);
                }
            }
        }

        // Convert stock panels
        for panel in &request.stock_panels {
            if panel.is_valid()? {
                for _ in 0..panel.count {
                    let width_str = panel.width.as_ref().ok_or_else(|| crate::errors::AppError::invalid_input("Panel width is None"))?;
                    let height_str = panel.height.as_ref().ok_or_else(|| crate::errors::AppError::invalid_input("Panel height is None"))?;
                    
                    let width = (width_str.parse::<f64>().map_err(|e| crate::errors::AppError::Core(crate::errors::CoreError::ParseFloat(e)))? * factor).round() as i32;
                    let height = (height_str.parse::<f64>().map_err(|e| crate::errors::AppError::Core(crate::errors::CoreError::ParseFloat(e)))? * factor).round() as i32;
                    
                    let mut tile = TileDimensions::new(panel.id, width, height);
                    tile.material = panel.material.clone();
                    tile.orientation = Self::convert_orientation(panel.orientation);
                    tile.label = panel.label.clone();
                    
                    stock_tiles.push(tile);
                }
            }
        }

        Ok((tiles, stock_tiles, factor))
    }

    /// Get maximum decimal places from panels
    fn get_max_decimal_places(panels: &[Panel], stock_panels: &[Panel]) -> usize {
        let mut max_decimal = 0;

        for panel in panels.iter().chain(stock_panels.iter()) {
            if panel.is_valid().unwrap_or(false) {
                if let Some(width_str) = &panel.width {
                    max_decimal = max_decimal.max(Self::count_decimal_places(width_str));
                }
                if let Some(height_str) = &panel.height {
                    max_decimal = max_decimal.max(Self::count_decimal_places(height_str));
                }
            }
        }

        max_decimal.min(MAX_ALLOWED_DIGITS)
    }

    /// Count decimal places in a string number
    fn count_decimal_places(value: &str) -> usize {
        if let Some(dot_pos) = value.find('.') {
            value.len() - dot_pos - 1
        } else {
            0
        }
    }

    /// Convert integer orientation to Orientation enum
    fn convert_orientation(orientation: i32) -> Orientation {
        match orientation {
            0 => Orientation::Any,
            1 => Orientation::Horizontal,
            2 => Orientation::Vertical,
            _ => Orientation::Any,
        }
    }

    /// Compute optimization for a specific material
    async fn compute_material(
        tiles: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
        _configuration: Option<&Configuration>,
        task: &Task,
        material: &str,
    ) -> Result<()> {
        debug!("Computing material: {} with {} tiles and {} stock tiles", 
               material, tiles.len(), stock_tiles.len());

        // Generate groups (like Java implementation)
        let groups = CollectionUtils::generate_groups(&tiles, &stock_tiles, task)?;
        
        // TODO: Implement full permutation processing logic
        // This would include:
        // 1. Generate permutations
        // 2. Stock solution generation
        // 3. Cut list thread spawning
        // 4. Solution comparison and ranking
        
        // For now, just log the progress
        debug!("Generated {} groups for material {}", groups.len(), material);

        Ok(())
    }
}

/// Request validation utilities
mod validation {
    use crate::models::{CalculationRequest, enums::StatusCode};
    use super::{MAX_PANELS_LIMIT, MAX_STOCK_PANELS_LIMIT};

    pub struct RequestValidator;

    impl RequestValidator {
        /// Validate a calculation request (migrated from Java)
        pub async fn validate_request(request: &CalculationRequest) -> Option<StatusCode> {
            // Count valid panels
            let panel_count: usize = request.panels.iter()
                .filter(|p| p.is_valid().unwrap_or(false))
                .map(|p| p.count as usize)
                .sum();

            if panel_count == 0 {
                return Some(StatusCode::InvalidTiles);
            }

            if panel_count > MAX_PANELS_LIMIT {
                return Some(StatusCode::TooManyPanels);
            }

            // Count valid stock panels
            let stock_count: usize = request.stock_panels.iter()
                .filter(|p| p.is_valid().unwrap_or(false))
                .map(|p| p.count as usize)
                .sum();

            if stock_count == 0 {
                return Some(StatusCode::InvalidStockTiles);
            }

            if stock_count > MAX_STOCK_PANELS_LIMIT {
                return Some(StatusCode::TooManyStockPanels);
            }

            None // Request is valid
        }
    }
}

// Re-export for use in other modules
pub use validation::RequestValidator;
