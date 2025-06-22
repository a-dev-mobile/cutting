//! Main computation operations
//!
//! This module handles the first compute method - the main computation logic
//! that creates tasks, groups by materials, and spawns threads for materials.

use crate::{
    errors::{Result, AppError},
    models::{
        calculation_request::CalculationRequest,
        calculation_submission_result::CalculationSubmissionResult,
        task::Task,
        tile_dimensions::TileDimensions,
        panel::Panel,
        enums::Status,
    },
    engine::running_tasks::{TaskManager, get_running_tasks_instance},
    logging::macros::{info, debug, warn, error},
};

use super::{
    grouping::CollectionUtils,
    dimension_utils::DimensionUtils,
    material_compute,
};

use std::{collections::{HashSet, HashMap}, sync::Arc};
use parking_lot::RwLock;
use tokio::task::JoinHandle;

/// Group tiles by material (Java equivalent of getTileDimensionsPerMaterial)
fn get_tile_dimensions_per_material(tiles: &[TileDimensions]) -> HashMap<String, Vec<TileDimensions>> {
    let mut material_map: HashMap<String, Vec<TileDimensions>> = HashMap::new();
    
    for tile in tiles {
        material_map
            .entry(tile.material.clone())
            .or_insert_with(Vec::new)
            .push(tile.clone());
    }
    
    material_map
}

/// Complete compute method implementation based on Java CutListOptimizerServiceImpl.compute()
/// 
/// Java reference: void compute(CalculationRequest calculationRequest, String str)
/// Lines 200-250 in the Java implementation
/// 
/// This function implements the complete logic:
/// 1. Input validation
/// 2. Decimal places calculation and scaling factor
/// 3. Panel to TileDimensions conversion with scaling
/// 4. Task creation and setup in RunningTasks
/// 5. Material grouping
/// 6. Spawning computation for each material
pub async fn compute_task_complete(request: CalculationRequest, task_id: String) -> Result<()> {
    info!("Starting complete computation for task: {}", task_id);

    // 1. Input validation
    if request.panels.is_empty() {
        return Err(AppError::invalid_input("No panels provided"));
    }
    if request.stock_panels.is_empty() {
        return Err(AppError::invalid_input("No stock panels provided"));
    }

    // 2. Calculate scaling factor (Java lines ~205-215)
    let panels = &request.panels;
    let stock_panels = &request.stock_panels;
    
    // Get maximum decimal places from panels and stock panels
    let max_decimal_panels = DimensionUtils::get_max_nbr_decimal_places(panels);
    let max_decimal_stock = DimensionUtils::get_max_nbr_decimal_places(stock_panels);
    
    // Include configuration values in decimal calculation
    let mut max_decimal_places = std::cmp::max(max_decimal_panels, max_decimal_stock);
    
    if let Some(config) = &request.configuration {
        let cut_thickness_str = config.cut_thickness.to_string();
        max_decimal_places = std::cmp::max(max_decimal_places, DimensionUtils::get_nbr_decimal_places(&cut_thickness_str));
        
        let min_trim_str = config.min_trim_dimension.to_string();
        max_decimal_places = std::cmp::max(max_decimal_places, DimensionUtils::get_nbr_decimal_places(&min_trim_str));
    }
    
    // Get maximum integer places
    let max_integer_panels = DimensionUtils::get_max_nbr_integer_places(panels);
    let max_integer_stock = DimensionUtils::get_max_nbr_integer_places(stock_panels);
    let max_integer_places = std::cmp::max(max_integer_panels, max_integer_stock);
    
    // Check digit limits (Java MAX_ALLOWED_DIGITS = 6)
    const MAX_ALLOWED_DIGITS: usize = 6;
    if max_decimal_places + max_integer_places > MAX_ALLOWED_DIGITS {
        warn!("Maximum allowed digits exceeded: decimal[{}] + integer[{}] = {} > max[{}]", 
              max_decimal_places, max_integer_places, 
              max_decimal_places + max_integer_places, MAX_ALLOWED_DIGITS);
        max_decimal_places = MAX_ALLOWED_DIGITS.saturating_sub(max_integer_places);
    }
    
    // Calculate scaling factor: double dPow = Math.pow(10.0d, iMax);
    let scaling_factor = 10.0_f64.powi(max_decimal_places as i32);
    
    debug!("Scaling factor: {} (decimal places: {}, integer places: {})", 
           scaling_factor, max_decimal_places, max_integer_places);

    // 3. Convert panels to TileDimensions with scaling (Java lines ~216-235)
    let mut tiles = Vec::new();
    let mut stock_tiles = Vec::new();
    
    // Convert regular panels
    for panel in panels {
        if panel.is_valid()? {
            for _ in 0..panel.count {
                let width_str = panel.width.as_ref().ok_or_else(|| AppError::invalid_input("Panel width is None"))?;
                let height_str = panel.height.as_ref().ok_or_else(|| AppError::invalid_input("Panel height is None"))?;
                
                let width_f64 = width_str.parse::<f64>().map_err(|e| AppError::Core(crate::errors::CoreError::ParseFloat(e)))?;
                let height_f64 = height_str.parse::<f64>().map_err(|e| AppError::Core(crate::errors::CoreError::ParseFloat(e)))?;
                
                // Apply scaling: (int) Math.round(Double.parseDouble(panel.getWidth()) * dPow)
                let scaled_width = (width_f64 * scaling_factor).round() as i32;
                let scaled_height = (height_f64 * scaling_factor).round() as i32;
                
                let mut tile = TileDimensions::new(panel.id, scaled_width, scaled_height);
                tile.material = panel.material.clone();
                tile.orientation = DimensionUtils::convert_orientation(panel.orientation);
                tile.label = panel.label.clone();
                
                tiles.push(tile);
            }
        }
    }
    
    // Convert stock panels
    for panel in stock_panels {
        if panel.is_valid()? {
            for _ in 0..panel.count {
                let width_str = panel.width.as_ref().ok_or_else(|| AppError::invalid_input("Stock panel width is None"))?;
                let height_str = panel.height.as_ref().ok_or_else(|| AppError::invalid_input("Stock panel height is None"))?;
                
                let width_f64 = width_str.parse::<f64>().map_err(|e| AppError::Core(crate::errors::CoreError::ParseFloat(e)))?;
                let height_f64 = height_str.parse::<f64>().map_err(|e| AppError::Core(crate::errors::CoreError::ParseFloat(e)))?;
                
                let scaled_width = (width_f64 * scaling_factor).round() as i32;
                let scaled_height = (height_f64 * scaling_factor).round() as i32;
                
                let mut tile = TileDimensions::new(panel.id, scaled_width, scaled_height);
                tile.material = panel.material.clone();
                tile.orientation = DimensionUtils::convert_orientation(panel.orientation);
                tile.label = panel.label.clone();
                
                stock_tiles.push(tile);
            }
        }
    }
    
    info!("Converted {} panels to {} tiles, {} stock panels to {} stock tiles", 
          panels.len(), tiles.len(), stock_panels.len(), stock_tiles.len());

    // 4. Create and setup task (Java lines ~236-242)
    let mut task = Task::new(task_id.clone());
    task.calculation_request = Some(request.clone());
    task.factor = scaling_factor;
    
    // Add task to running tasks: this.runningTasks.addTask(task);
    let running_tasks = get_running_tasks_instance();
    running_tasks.add_task(task)?;
    
    let task_arc = running_tasks.get_task(&task_id)
        .ok_or_else(|| AppError::invalid_input(&format!("Failed to retrieve task {}", task_id)))?;

    // 5. Group by materials (Java lines ~243-246)
    let tiles_per_material = get_tile_dimensions_per_material(&tiles);
    let stock_per_material = get_tile_dimensions_per_material(&stock_tiles);
    
    // Update task with material data (Java equivalent)
    {
        let mut task = task_arc.write();
        task.tile_dimensions_per_material = Some(tiles_per_material.clone());
        task.stock_dimensions_per_material = Some(stock_per_material.clone());
    }
    
    // Find all materials and determine which can be computed (Java lines ~247-260)
    let mut all_materials = HashSet::new();
    all_materials.extend(tiles_per_material.keys().cloned());
    all_materials.extend(stock_per_material.keys().cloned());
    
    let mut materials_to_compute = Vec::new();
    
    for material in &all_materials {
        if let Some(material_tiles) = tiles_per_material.get(material) {
            if stock_per_material.contains_key(material) {
                // Material has both tiles and stock - add to computation
                materials_to_compute.push(material.clone());
                
                // Add material to task: task.addMaterialToCompute(str2);
                {
                    let task = task_arc.read();
                    task.add_material_to_compute(material.clone());
                }
                
                debug!("Material '{}' added for computation with {} tiles", material, material_tiles.len());
            } else {
                // Material has tiles but no stock - add to no_material_tiles
                {
                    let mut task = task_arc.write();
                    task.no_material_tiles.extend(material_tiles.clone());
                }
                warn!("Material '{}' has tiles but no stock panels", material);
            }
        }
    }

    // 6. Spawn computation for each material (Java lines ~261-270)
    for material in materials_to_compute {
        if let (Some(material_tiles), Some(material_stock)) = (
            tiles_per_material.get(&material),
            stock_per_material.get(&material)
        ) {
            let material_tiles_clone = material_tiles.clone();
            let material_stock_clone = material_stock.clone();
            let configuration_clone = request.configuration.clone();
            let material_clone = material.clone();
            let task_id_clone = task_id.clone();
            
            debug!("Spawning computation thread for material: {}", material);
            
            // Spawn async computation (Java equivalent of new Thread().start())
            tokio::spawn(async move {
                if let Some(config) = configuration_clone.as_ref() {
                    let running_tasks = get_running_tasks_instance();
                    if let Some(task_arc) = running_tasks.get_task(&task_id_clone) {
                        if let Err(e) = material_compute::compute_material(
                            material_tiles_clone,
                            material_stock_clone,
                            config,
                            task_arc,
                            &material_clone,
                        ).await {
                            error!("Material computation failed for {}: {}", material_clone, e);
                        }
                    }
                }
            });
        }
    }
    
    // Check if task should finish (Java: task.checkIfFinished())
    {
        let task = task_arc.read();
        task.check_if_finished();
    }
    
    info!("Task {} computation setup complete", task_id);
    Ok(())
}

/// Legacy compute method for backward compatibility
pub async fn compute_task(request: CalculationRequest, task_id: String) -> Result<CalculationSubmissionResult> {
    compute_task_complete(request, task_id.clone()).await?;
    
    Ok(CalculationSubmissionResult::new(
        crate::models::enums::status_code::StatusCode::Ok,
        task_id
    ))
}

/// Helper function to check if task should finish (Java equivalent)
/// This would be called periodically to check if all materials are done
pub async fn check_task_completion(task_id: &str) -> Result<bool> {
    let running_tasks = get_running_tasks_instance();
    
    if let Some(task_arc) = running_tasks.get_task(task_id) {
        let _task = task_arc.read();
        
        // Check if all materials have completed
        // This would involve checking the per_material_percentage_done
        // and determining if the task should be marked as finished
        
        // For now, return false (task not complete)
        Ok(false)
    } else {
        Err(AppError::invalid_input(&format!("Task {} not found", task_id)))
    }
}

/// Simplified version for testing - creates task and groups by material
pub async fn compute_task_simple(request: CalculationRequest, task_id: String) -> Result<()> {
    // Convert panels to tiles
    let (tiles, stock_tiles, _factor) = DimensionUtils::convert_panels_to_tiles(
        &request.panels, 
        &request.stock_panels, 
        6
    )?;

    // Create task
    let task = Task::new(task_id.clone());
    
    // Add task to running tasks
    let running_tasks = get_running_tasks_instance();
    running_tasks.add_task(task)?;
    
    // Group tiles by material
    let tiles_per_material = CollectionUtils::get_tile_dimensions_per_material(&tiles)?;
    let stock_per_material = CollectionUtils::get_tile_dimensions_per_material(&stock_tiles)?;

    // Spawn computation for each material that has both tiles and stock
    for (material, material_tiles) in tiles_per_material {
        if let Some(material_stock) = stock_per_material.get(&material) {
            let material_stock_clone = material_stock.clone();
            let configuration_clone = request.configuration.clone();
            let task_id_clone = task_id.clone();
            let material_clone = material.clone();
            
            tokio::spawn(async move {
                if let Some(config) = configuration_clone.as_ref() {
                    // Get task from running tasks for the computation
                    let running_tasks = get_running_tasks_instance();
                    if let Some(task_arc) = running_tasks.get_task(&task_id_clone) {
                        // Clone the task data we need without holding the lock
                        let task_data = {
                            let task = task_arc.read();
                            task.clone()
                        };
                        let _ = material_compute::compute_material(
                            material_tiles,
                            material_stock_clone,
                            config,
                            task_arc,
                            &material_clone,
                        ).await;
                    }
                }
            });
        }
    }

    Ok(())
}
