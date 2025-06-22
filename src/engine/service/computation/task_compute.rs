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
    },
    engine::running_tasks::{TaskManager, get_running_tasks_instance},
    logging::macros::{info, debug, warn},
};

use super::{
    grouping::CollectionUtils,
    dimension_utils::DimensionUtils,
    material_compute,
};

use std::collections::HashSet;
use tokio::task::JoinHandle;

/// Main computation method (migrated from Java compute() method around lines 200-250)
/// 
/// This function implements the core logic from the Java CutListOptimizerServiceImpl.compute() method:
/// 1. Converts panels to tiles using DimensionUtils
/// 2. Creates a new Task with the given task_id
/// 3. Adds the task to RunningTasks
/// 4. Groups tiles by material using CollectionUtils
/// 5. Spawns async computation for each material
pub async fn compute_task(request: CalculationRequest, task_id: String) -> Result<CalculationSubmissionResult> {
    info!("Starting computation for task: {}", task_id);

    // Convert panels to tile dimensions with scaling factor (Java lines ~210-240)
    let (tiles, stock_tiles, factor) = DimensionUtils::convert_panels_to_tiles(
        &request.panels, 
        &request.stock_panels, 
        6 // MAX_ALLOWED_DIGITS from Java
    )?;

    // Create task (Java line ~241)
    let mut task = Task::new(task_id.clone());
    task.calculation_request = Some(request.clone());
    task.factor = factor;
    
    // Build initial solution structure
    // task.build_solution(); // This would be implemented in task methods
    
    // Add task to running tasks (Java line ~244)
    let running_tasks = get_running_tasks_instance();
    running_tasks.add_task(task.clone())?;
    
    debug!("Task {} added to running tasks", task_id);

    // Group tiles by material (Java lines ~245-246)
    let tiles_per_material = CollectionUtils::get_tile_dimensions_per_material(&tiles)?;
    let stock_per_material = CollectionUtils::get_tile_dimensions_per_material(&stock_tiles)?;

    // Set material data on task (Java lines ~247-248)
    // Note: In a real implementation, we'd need to update the task in RunningTasks
    // For now, we'll work with the local copy
    
    // Find materials that have both tiles and stock (Java lines ~249-260)
    let mut all_materials = HashSet::new();
    all_materials.extend(tiles_per_material.keys().cloned());
    all_materials.extend(stock_per_material.keys().cloned());
    
    let mut materials_to_compute = Vec::new();
    let mut no_material_tiles = Vec::new();
    
    for material in &all_materials {
        if let Some(material_tiles) = tiles_per_material.get(material) {
            if stock_per_material.contains_key(material) {
                // Material has both tiles and stock - can be computed
                materials_to_compute.push(material.clone());
                debug!("Material '{}' added for computation with {} tiles", material, material_tiles.len());
            } else {
                // Material has tiles but no stock - add to no_material_tiles
                no_material_tiles.extend(material_tiles.clone());
                warn!("Material '{}' has tiles but no stock panels", material);
            }
        }
    }

    // Spawn computation tasks for each material (Java lines ~261-270)
    let mut computation_handles: Vec<JoinHandle<Result<()>>> = Vec::new();
    
    for material in materials_to_compute {
        if let (Some(material_tiles), Some(material_stock)) = (
            tiles_per_material.get(&material),
            stock_per_material.get(&material)
        ) {
            let material_tiles_clone = material_tiles.clone();
            let material_stock_clone = material_stock.clone();
            let configuration_clone = request.configuration.clone();
            let task_clone = task.clone();
            let material_clone = material.clone();
            
            debug!("Spawning computation for material: {}", material);
            
            let handle = tokio::spawn(async move {
                material_compute::compute_material(
                    material_tiles_clone,
                    material_stock_clone,
                    configuration_clone.as_ref(),
                    &task_clone,
                    &material_clone,
                ).await
            });
            
            computation_handles.push(handle);
        }
    }
    
    // Note: In the Java version, the method doesn't wait for completion here
    // The threads run independently and update the task status
    // For now, we'll return the submission result immediately
    
    info!("Spawned {} material computation tasks for task: {}", computation_handles.len(), task_id);
    
    // Return submission result with task ID (like Java)
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
                material_compute::compute_material(
                    material_tiles,
                    material_stock_clone,
                    configuration_clone.as_ref(),
                    &Task::new(task_id_clone), // Simplified for testing
                    &material_clone,
                ).await
            });
        }
    }

    Ok(())
}
