//! Main computation operations
//!
//! This module handles the first compute method - the main computation logic
//! that creates tasks, groups by materials, and spawns threads for materials.

use crate::{
    errors::Result,
    models::{
        CalculationRequest,
        task::structs::Task,
    },
    logging::macros::info,
};

use super::{
    grouping::CollectionUtils,
    dimension_utils::DimensionUtils,
    material_compute,
};

/// Main computation method (migrated from Java)
pub async fn compute_task(request: CalculationRequest, task_id: String) -> Result<()> {
    info!("Starting computation for task: {}", task_id);

    // Convert panels to tile dimensions with scaling factor
    let (tiles, stock_tiles, _factor) = DimensionUtils::convert_panels_to_tiles(
        &request.panels, 
        &request.stock_panels, 
        crate::engine::service::core::MAX_ALLOWED_DIGITS
    )?;

    // Create task
    let task = Task::new(task_id.clone());
    
    // Group tiles by material
    let tiles_per_material = CollectionUtils::get_tile_dimensions_per_material(&tiles)?;
    let stock_per_material = CollectionUtils::get_tile_dimensions_per_material(&stock_tiles)?;

    // Process each material
    for (material, material_tiles) in tiles_per_material {
        if let Some(material_stock) = stock_per_material.get(&material) {
            material_compute::compute_material(
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
