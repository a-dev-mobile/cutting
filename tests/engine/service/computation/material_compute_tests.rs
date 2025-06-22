//! Tests for material computation functionality
//!
//! This module contains tests for the compute_material function to verify:
//! 1. Groups are generated correctly
//! 2. Material computation completes without errors
//! 3. Task status is updated properly

use cutlist_optimizer_cli::{
    engine::service::computation::material_compute::compute_material,
    models::{
        tile_dimensions::structs::TileDimensions,
        configuration::structs::Configuration,
        task::structs::Task,
        enums::{orientation::Orientation, Status},
    },
    errors::Result,
};

use tokio;
use uuid::Uuid;

/// Helper function to create test tiles for a specific material
fn create_test_tiles(material: &str, count: usize) -> Vec<TileDimensions> {
    (0..count)
        .map(|i| TileDimensions {
            id: i as i32,
            width: 100 + (i * 10) as i32,
            height: 200 + (i * 5) as i32,
            material: material.to_string(),
            orientation: Orientation::Any,
            label: Some(format!("Tile_{}", i)),
            is_rotated: false,
        })
        .collect()
}

/// Helper function to create test stock tiles
fn create_test_stock_tiles(material: &str, count: usize) -> Vec<TileDimensions> {
    (0..count)
        .map(|i| TileDimensions {
            id: (100 + i) as i32,
            width: 300 + (i * 20) as i32,
            height: 400 + (i * 10) as i32,
            material: material.to_string(),
            orientation: Orientation::Any,
            label: Some(format!("Stock_{}", i)),
            is_rotated: false,
        })
        .collect()
}

#[tokio::test]
async fn test_compute_material_generates_groups() -> Result<()> {
    // Create tiles of the same material
    let tiles = create_test_tiles("Wood", 5);
    let stock_tiles = create_test_stock_tiles("Wood", 3);
    
    // Create test configuration
    let configuration = Configuration::default();
    
    // Create test task
    let task_id = Uuid::new_v4().to_string();
    let task = Task::new(task_id.clone());
    
    // Verify initial task status
    assert_eq!(task.status(), Status::Queued, "Task should start with Queued status");
    
    // Call compute_material
    let result = compute_material(
        tiles,
        stock_tiles,
        &configuration,
        &task,
        "Wood",
    ).await;
    
    // Verify the computation completed successfully
    assert!(result.is_ok(), "Material computation should complete successfully");
    
    // Verify that groups were generated (this is tested indirectly through successful completion)
    // The actual group generation is tested in the grouping module tests
    
    Ok(())
}

#[tokio::test]
async fn test_compute_material_updates_task_status() -> Result<()> {
    // Create test data
    let tiles = create_test_tiles("Metal", 3);
    let stock_tiles = create_test_stock_tiles("Metal", 2);
    let configuration = Configuration::default();
    
    // Create test task
    let task_id = Uuid::new_v4().to_string();
    let task = Task::new(task_id.clone());
    
    // Add the material to the task so we can track its progress
    task.add_material_to_compute("Metal".to_string());
    
    // Verify initial material percentage is 0
    assert_eq!(task.percentage_done(), 0, "Initial percentage should be 0");
    
    // Call compute_material
    let result = compute_material(
        tiles,
        stock_tiles,
        &configuration,
        &task,
        "Metal",
    ).await;
    
    // Verify the computation completed successfully
    assert!(result.is_ok(), "Material computation should complete successfully");
    
    // Verify that the material status was updated to completed (100%)
    // Note: The percentage_done() method returns the average across all materials
    // Since we only have one material, it should be 100%
    assert_eq!(task.percentage_done(), 100, "Material should be marked as 100% complete");
    
    Ok(())
}

#[tokio::test]
async fn test_compute_material_with_empty_tiles() -> Result<()> {
    // Test with empty tiles (should fail)
    let tiles = vec![];
    let stock_tiles = create_test_stock_tiles("Wood", 2);
    let configuration = Configuration::default();
    let task = Task::new(Uuid::new_v4().to_string());
    
    let result = compute_material(
        tiles,
        stock_tiles,
        &configuration,
        &task,
        "Wood",
    ).await;
    
    // Should fail because tiles array is empty
    assert!(result.is_err(), "Should fail with empty tiles");
    
    Ok(())
}

#[tokio::test]
async fn test_compute_material_with_empty_stock() -> Result<()> {
    // Test with empty stock tiles (should fail)
    let tiles = create_test_tiles("Wood", 3);
    let stock_tiles = vec![];
    let configuration = Configuration::default();
    let task = Task::new(Uuid::new_v4().to_string());
    
    let result = compute_material(
        tiles,
        stock_tiles,
        &configuration,
        &task,
        "Wood",
    ).await;
    
    // Should fail because stock tiles array is empty
    assert!(result.is_err(), "Should fail with empty stock tiles");
    
    Ok(())
}

#[tokio::test]
async fn test_compute_material_multiple_materials() -> Result<()> {
    // Test with different materials to ensure material-specific processing
    let wood_tiles = create_test_tiles("Wood", 2);
    let wood_stock = create_test_stock_tiles("Wood", 1);
    
    let metal_tiles = create_test_tiles("Metal", 3);
    let metal_stock = create_test_stock_tiles("Metal", 2);
    
    let configuration = Configuration::default();
    
    // Create separate tasks for each material
    let wood_task = Task::new(Uuid::new_v4().to_string());
    let metal_task = Task::new(Uuid::new_v4().to_string());
    
    wood_task.add_material_to_compute("Wood".to_string());
    metal_task.add_material_to_compute("Metal".to_string());
    
    // Process wood material
    let wood_result = compute_material(
        wood_tiles,
        wood_stock,
        &configuration,
        &wood_task,
        "Wood",
    ).await;
    
    // Process metal material
    let metal_result = compute_material(
        metal_tiles,
        metal_stock,
        &configuration,
        &metal_task,
        "Metal",
    ).await;
    
    // Both should complete successfully
    assert!(wood_result.is_ok(), "Wood material computation should succeed");
    assert!(metal_result.is_ok(), "Metal material computation should succeed");
    
    // Both tasks should show 100% completion for their respective materials
    assert_eq!(wood_task.percentage_done(), 100, "Wood task should be 100% complete");
    assert_eq!(metal_task.percentage_done(), 100, "Metal task should be 100% complete");
    
    Ok(())
}

#[tokio::test]
async fn test_compute_material_large_dataset() -> Result<()> {
    // Test with a larger dataset to verify performance and correctness
    let tiles = create_test_tiles("Plywood", 50);
    let stock_tiles = create_test_stock_tiles("Plywood", 10);
    let configuration = Configuration::default();
    
    let task_id = Uuid::new_v4().to_string();
    let task = Task::new(task_id.clone());
    task.add_material_to_compute("Plywood".to_string());
    
    // Measure execution time
    let start = std::time::Instant::now();
    
    let result = compute_material(
        tiles,
        stock_tiles,
        &configuration,
        &task,
        "Plywood",
    ).await;
    
    let duration = start.elapsed();
    
    // Verify successful completion
    assert!(result.is_ok(), "Large dataset computation should succeed");
    assert_eq!(task.percentage_done(), 100, "Large dataset should complete to 100%");
    
    // Verify reasonable execution time (should complete within 5 seconds for this test)
    assert!(duration.as_secs() < 5, "Computation should complete within reasonable time");
    
    Ok(())
}
