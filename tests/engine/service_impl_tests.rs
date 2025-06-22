//! Tests for CutListOptimizerServiceImpl

use cutlist_optimizer_cli::{
    engine::service::{CutListOptimizerService, CutListOptimizerServiceImpl},
    models::{CalculationRequest},
    models::enums::StatusCode,
};

// #[tokio::test]
// async fn test_service_lifecycle() {
//     let mut service = CutListOptimizerServiceImpl::new();

//     // Test initialization
//     assert!(service.init(4).await.is_ok());

//     // Test configuration
//     service.set_allow_multiple_tasks_per_client(true);

//     // Test task submission
//     let request = CalculationRequest {
//         configuration: Some(Configuration::default()),
//         panels: vec![Panel::default()],
//         stock_panels: vec![],
//     };

//     let result = service.submit_task(request).await.unwrap();
//     assert_eq!(result.status_code, StatusCode::Ok);
//     assert!(result.task_id.is_some());

//     // Test stats
//     let stats = service.get_stats().await.unwrap();
//     assert_eq!(stats.nbr_running_tasks, 0);

//     // Test shutdown
//     assert!(service.shutdown().await.is_ok());
// }

#[tokio::test]
async fn test_uninitialized_service() {
    let service = CutListOptimizerServiceImpl::new();

    // Operations should fail on uninitialized service
    let request = CalculationRequest {
        configuration: None,
        panels: vec![],
        stock_panels: vec![],
    };

    assert!(service.submit_task(request).await.is_err());
    assert!(service.get_task_status("test").await.is_err());
    assert!(service.get_stats().await.is_err());
}

#[tokio::test]
async fn test_invalid_request() {
    let mut service = CutListOptimizerServiceImpl::new();
    assert!(service.init(4).await.is_ok());

    // Empty panels should result in invalid request
    let request = CalculationRequest {
        configuration: None,
        panels: vec![], // Empty panels
        stock_panels: vec![],
    };

    let result = service.submit_task(request).await.unwrap();
    assert_eq!(result.status_code, StatusCode::InvalidTiles);
    assert!(result.task_id.is_none());
}

#[tokio::test]
async fn test_submit_valid_request() {
    use cutlist_optimizer_cli::models::{Panel, Configuration};
    
    let mut service = CutListOptimizerServiceImpl::new();
    assert!(service.init(4).await.is_ok());

    // Create valid CalculationRequest
    let valid_panel = Panel {
        id: 1,
        width: Some("100.0".to_string()),
        height: Some("200.0".to_string()),
        count: 1,
        material: "wood".to_string(),
        enabled: true,
        orientation: 0,
        label: Some("Test Panel".to_string()),
        edge: None,
    };

    let valid_stock_panel = Panel {
        id: 2,
        width: Some("300.0".to_string()),
        height: Some("400.0".to_string()),
        count: 1,
        material: "wood".to_string(),
        enabled: true,
        orientation: 0,
        label: Some("Stock Panel".to_string()),
        edge: None,
    };

    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels: vec![valid_panel],
        stock_panels: vec![valid_stock_panel],
    };

    // Call submit_task()
    let result = service.submit_task(request).await.unwrap();
    
    // Check that returned StatusCode::Ok and task_id
    assert_eq!(result.status_code, StatusCode::Ok);
    assert!(result.task_id.is_some());
    assert!(!result.task_id.unwrap().is_empty());
}

#[tokio::test]
async fn test_submit_invalid_panels() {
    use cutlist_optimizer_cli::models::{Panel, Configuration};
    
    let mut service = CutListOptimizerServiceImpl::new();
    assert!(service.init(4).await.is_ok());

    // Create request with invalid panels (invalid dimensions)
    let invalid_panel = Panel {
        id: 1,
        width: Some("invalid".to_string()), // Invalid width
        height: Some("invalid".to_string()), // Invalid height
        count: 1,
        material: "wood".to_string(),
        enabled: true,
        orientation: 0,
        label: Some("Invalid Panel".to_string()),
        edge: None,
    };

    let valid_stock_panel = Panel {
        id: 2,
        width: Some("300.0".to_string()),
        height: Some("400.0".to_string()),
        count: 1,
        material: "wood".to_string(),
        enabled: true,
        orientation: 0,
        label: Some("Stock Panel".to_string()),
        edge: None,
    };

    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels: vec![invalid_panel],
        stock_panels: vec![valid_stock_panel],
    };

    // Call submit_task()
    let result = service.submit_task(request).await.unwrap();
    
    // Check that returned StatusCode::InvalidTiles
    assert_eq!(result.status_code, StatusCode::InvalidTiles);
    assert!(result.task_id.is_none());
}

#[tokio::test]
async fn test_get_task_status_existing() {
    use cutlist_optimizer_cli::{
        models::{Panel, Configuration, Task},
        engine::running_tasks::{TaskManager, get_running_tasks_instance},
        models::enums::Status,
    };
    
    let mut service = CutListOptimizerServiceImpl::new();
    assert!(service.init(4).await.is_ok());

    // Create and add a task to running tasks
    let task_id = "test_task_123".to_string();
    let task = Task::new(task_id.clone());
    
    let running_tasks = get_running_tasks_instance();
    running_tasks.add_task(task).unwrap();

    // Test getting task status
    let status_response = service.get_task_status(&task_id).await.unwrap();
    
    // Should return Some(TaskStatusResponse)
    assert!(status_response.is_some());
    let response = status_response.unwrap();
    
    // Check that status is returned (should be Queued initially)
    assert_eq!(response.status, Status::Queued);
    assert_eq!(response.percentage_done, 0);
    assert_eq!(response.init_percentage, 0);
    assert!(response.solution.is_none()); // No solution initially
}

#[tokio::test]
async fn test_get_task_status_missing() {
    let mut service = CutListOptimizerServiceImpl::new();
    assert!(service.init(4).await.is_ok());

    // Request status for non-existent task
    let status_response = service.get_task_status("non_existent_task").await.unwrap();
    
    // Should return None for missing task
    assert!(status_response.is_none());
}

#[tokio::test]
async fn test_stop_task_existing() {
    use cutlist_optimizer_cli::{
        models::{Task},
        engine::running_tasks::{TaskManager, get_running_tasks_instance},
        models::enums::Status,
    };
    
    let mut service = CutListOptimizerServiceImpl::new();
    assert!(service.init(4).await.is_ok());

    // Create and add a running task
    let task_id = "test_task_stop_123".to_string();
    let mut task = Task::new(task_id.clone());
    
    // Set task to running status first
    task.set_running_status().unwrap();
    
    let running_tasks = get_running_tasks_instance();
    running_tasks.add_task(task).unwrap();

    // Test stopping the task
    let status_response = service.stop_task(&task_id).await.unwrap();
    
    // Should return Some(TaskStatusResponse)
    assert!(status_response.is_some());
    let response = status_response.unwrap();
    
    // Check that task is now finished
    assert_eq!(response.status, Status::Finished);
}

#[tokio::test]
async fn test_stop_task_missing() {
    let mut service = CutListOptimizerServiceImpl::new();
    assert!(service.init(4).await.is_ok());

    // Try to stop non-existent task
    let status_response = service.stop_task("non_existent_task").await.unwrap();
    
    // Should return None for missing task
    assert!(status_response.is_none());
}

#[tokio::test]
async fn test_terminate_task_existing() {
    use cutlist_optimizer_cli::{
        models::{Task},
        engine::running_tasks::{TaskManager, get_running_tasks_instance},
        models::enums::Status,
    };
    
    let mut service = CutListOptimizerServiceImpl::new();
    assert!(service.init(4).await.is_ok());

    // Create and add a running task
    let task_id = "test_task_terminate_123".to_string();
    let mut task = Task::new(task_id.clone());
    
    // Set task to running status first
    task.set_running_status().unwrap();
    
    let running_tasks = get_running_tasks_instance();
    running_tasks.add_task(task).unwrap();

    // Test terminating the task
    let result = service.terminate_task(&task_id).await.unwrap();
    
    // Should return 0 for success
    assert_eq!(result, 0);
}

#[tokio::test]
async fn test_terminate_task_missing() {
    let mut service = CutListOptimizerServiceImpl::new();
    assert!(service.init(4).await.is_ok());

    // Try to terminate non-existent task
    let result = service.terminate_task("non_existent_task").await.unwrap();
    
    // Should return -1 for task not found
    assert_eq!(result, -1);
}

#[tokio::test]
async fn test_terminate_task_invalid_status() {
    use cutlist_optimizer_cli::{
        models::{Task},
        engine::running_tasks::{TaskManager, get_running_tasks_instance},
        models::enums::Status,
    };
    
    let mut service = CutListOptimizerServiceImpl::new();
    assert!(service.init(4).await.is_ok());

    // Create and add a task that's not running (should be Queued by default)
    let task_id = "test_task_invalid_terminate_123".to_string();
    let task = Task::new(task_id.clone());
    
    let running_tasks = get_running_tasks_instance();
    running_tasks.add_task(task).unwrap();

    // Test terminating the task (should fail because it's not running)
    let result = service.terminate_task(&task_id).await.unwrap();
    
    // Should return 1 for failure (task not in running state)
    assert_eq!(result, 1);
}
