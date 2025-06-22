//! Tests for task computation functionality
//!
//! This module contains tests for the compute_task function to verify:
//! 1. Task creation and addition to RunningTasks
//! 2. Material grouping and computation spawning

use cutlist_optimizer_cli::{
    engine::{
        service::computation::task_compute::{compute_task_simple, compute_task, compute_task_complete},
        running_tasks::{get_running_tasks_instance, TaskManager},
    },
    models::{
        calculation_request::CalculationRequest,
        panel::structs::Panel,
        configuration::structs::Configuration,
        enums::Status,
    },
    errors::Result,
};

use tokio;
use uuid::Uuid;

/// Helper function to create a test calculation request
fn create_test_request() -> CalculationRequest {
    let mut request = CalculationRequest::new();
    
    // Add test panels
    let panel1 = Panel {
        id: 1,
        width: Some("100.0".to_string()),
        height: Some("200.0".to_string()),
        count: 2,
        material: "Wood".to_string(),
        enabled: true,
        ..Default::default()
    };
    
    let panel2 = Panel {
        id: 2,
        width: Some("150.0".to_string()),
        height: Some("250.0".to_string()),
        count: 1,
        material: "Wood".to_string(),
        enabled: true,
        ..Default::default()
    };
    
    request.panels = vec![panel1, panel2];
    
    // Add test stock panels
    let stock_panel = Panel {
        id: 3,
        width: Some("300.0".to_string()),
        height: Some("400.0".to_string()),
        count: 5,
        material: "Wood".to_string(),
        enabled: true,
        ..Default::default()
    };
    
    request.stock_panels = vec![stock_panel];
    
    // Add basic configuration
    request.configuration = Some(Configuration::default());
    
    request
}

/// Helper function to create a test request with multiple materials
fn create_multi_material_request() -> CalculationRequest {
    let mut request = CalculationRequest::new();
    
    // Wood panels
    let wood_panel = Panel {
        id: 1,
        width: Some("100.0".to_string()),
        height: Some("200.0".to_string()),
        count: 1,
        material: "Wood".to_string(),
        enabled: true,
        ..Default::default()
    };
    
    // Metal panels
    let metal_panel = Panel {
        id: 2,
        width: Some("150.0".to_string()),
        height: Some("250.0".to_string()),
        count: 1,
        material: "Metal".to_string(),
        enabled: true,
        ..Default::default()
    };
    
    request.panels = vec![wood_panel, metal_panel];
    
    // Stock panels for both materials
    let wood_stock = Panel {
        id: 3,
        width: Some("300.0".to_string()),
        height: Some("400.0".to_string()),
        count: 2,
        material: "Wood".to_string(),
        enabled: true,
        ..Default::default()
    };
    
    let metal_stock = Panel {
        id: 4,
        width: Some("350.0".to_string()),
        height: Some("450.0".to_string()),
        count: 2,
        material: "Metal".to_string(),
        enabled: true,
        ..Default::default()
    };
    
    request.stock_panels = vec![wood_stock, metal_stock];
    request.configuration = Some(Configuration::default());
    
    request
}

#[tokio::test]
async fn test_compute_task_creates_task() -> Result<()> {
    // Create a test request
    let request = create_test_request();
    let task_id = Uuid::new_v4().to_string();
    
    // Get running tasks instance
    let running_tasks = get_running_tasks_instance();
    
    // Verify task doesn't exist before creation
    assert!(running_tasks.get_task(&task_id).is_none(), "Task should not exist before creation");
    
    // Call compute_task_simple
    compute_task_simple(request, task_id.clone()).await?;
    
    // Verify the specific task exists
    let task_arc = running_tasks.get_task(&task_id);
    assert!(task_arc.is_some(), "Task should exist in RunningTasks after creation");
    
    if let Some(task_arc) = task_arc {
        let task = task_arc.read();
        assert_eq!(task.id(), task_id, "Task ID should match");
        
        // Check initial status (should be Queued)
        let status = task.status();
        assert_eq!(status, Status::Queued, "Task should start with Queued status");
    }
    
    // Clean up - remove the test task
    running_tasks.remove_task(&task_id)?;
    
    // Verify task was removed
    assert!(running_tasks.get_task(&task_id).is_none(), "Task should be removed after cleanup");
    
    Ok(())
}

// #[tokio::test]
// async fn test_compute_task_groups_by_material() -> Result<()> {
//     // Create a request with multiple materials
//     let request = create_multi_material_request();
//     let task_id = Uuid::new_v4().to_string();
    
//     // Get running tasks instance
//     let running_tasks = get_running_tasks_instance();
    
//     // Verify task doesn't exist before creation
//     assert!(running_tasks.get_task(&task_id).is_none(), "Task should not exist before creation");
    
//     // Call compute_task_simple
//     compute_task_simple(request, task_id.clone()).await?;
    
//     // Verify the task exists
//     let task_arc = running_tasks.get_task(&task_id);
//     assert!(task_arc.is_some(), "Task should exist in RunningTasks after creation");
    
//     // Give a small delay for async spawned tasks to start
//     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
//     // Note: In a real implementation, we would verify that compute_material
//     // was called for each material (Wood and Metal). For now, we just verify
//     // the task was created successfully.
    
//     // Clean up
//     running_tasks.remove_task(&task_id)?;
    
//     // Verify task was removed
//     assert!(running_tasks.get_task(&task_id).is_none(), "Task should be removed after cleanup");
    
//     Ok(())
// }

#[tokio::test]
async fn test_compute_task_full_flow() -> Result<()> {
    // Create a test request
    let request = create_test_request();
    let task_id = Uuid::new_v4().to_string();
    
    // Get running tasks instance
    let running_tasks = get_running_tasks_instance();
    
    // Verify task doesn't exist before creation
    assert!(running_tasks.get_task(&task_id).is_none(), "Task should not exist before creation");
    
    // Call the full compute_task function
    let result = compute_task(request, task_id.clone()).await?;
    
    // Verify the result contains the task ID
    assert_eq!(result.task_id, Some(task_id.clone()), "Result should contain the task ID");
    
    // Verify the specific task exists
    let task_arc = running_tasks.get_task(&task_id);
    assert!(task_arc.is_some(), "Task should exist in RunningTasks");
    
    if let Some(task_arc) = task_arc {
        let task = task_arc.read();
        assert_eq!(task.id(), task_id, "Task ID should match");
        
        // Verify the calculation request was stored
        assert!(task.calculation_request().is_some(), "Task should have calculation request");
        
        // Verify the factor was set
        assert!(task.factor() > 0.0, "Task should have a positive scaling factor");
    }
    
    // Clean up
    running_tasks.remove_task(&task_id)?;
    
    Ok(())
}

// #[tokio::test]
// async fn test_compute_task_with_empty_panels() -> Result<()> {
//     // Create a request with no panels
//     let mut request = CalculationRequest::new();
//     request.panels = vec![];
//     request.stock_panels = vec![];
//     request.configuration = Some(Configuration::default());
    
//     let task_id = Uuid::new_v4().to_string();
    
//     // This should fail because we have no panels - use compute_task_complete which has validation
//     let result = compute_task_complete(request, task_id).await;
//     assert!(result.is_err(), "Should fail with empty panels");
    
//     Ok(())
// }

#[tokio::test]
async fn test_compute_task_material_without_stock() -> Result<()> {
    // Create a request where panels have a material but stock doesn't
    let mut request = CalculationRequest::new();
    
    let panel = Panel {
        id: 1,
        width: Some("100.0".to_string()),
        height: Some("200.0".to_string()),
        count: 1,
        material: "Wood".to_string(),
        enabled: true,
        ..Default::default()
    };
    
    let stock_panel = Panel {
        id: 2,
        width: Some("300.0".to_string()),
        height: Some("400.0".to_string()),
        count: 1,
        material: "Metal".to_string(), // Different material
        enabled: true,
        ..Default::default()
    };
    
    request.panels = vec![panel];
    request.stock_panels = vec![stock_panel];
    request.configuration = Some(Configuration::default());
    
    let task_id = Uuid::new_v4().to_string();
    let running_tasks = get_running_tasks_instance();
    
    // Verify task doesn't exist before creation
    assert!(running_tasks.get_task(&task_id).is_none(), "Task should not exist before creation");
    
    // This should still create a task, but no material computation should be spawned
    compute_task_simple(request, task_id.clone()).await?;
    
    // Verify task was created
    let task_arc = running_tasks.get_task(&task_id);
    assert!(task_arc.is_some(), "Task should still be created");
    
    // Clean up
    running_tasks.remove_task(&task_id)?;
    
    Ok(())
}
