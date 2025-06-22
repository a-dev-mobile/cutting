//! Tests for CutListOptimizerServiceImpl and related functionality

use cutlist_optimizer_cli::engine::{
    running_tasks::{TaskCleanup, get_running_tasks_instance},
    service::{CutListOptimizerService, CutListOptimizerServiceImpl}
};
use cutlist_optimizer_cli::models::{
    enums::Status,
};

#[tokio::test]
async fn test_service_creation() {
    // Clear any existing tasks from previous tests
    let running_tasks = get_running_tasks_instance();
    running_tasks.clear_all_tasks().unwrap();
    
    let service = CutListOptimizerServiceImpl::new();
    
    // Test that service can be created
    // Service creation should always succeed
    assert!(true);
}

#[tokio::test]
async fn test_service_basic_operations() {
    // Clear any existing tasks from previous tests
    let running_tasks = get_running_tasks_instance();
    running_tasks.clear_all_tasks().unwrap();
    
    let mut service = CutListOptimizerServiceImpl::new();
    
    // Test initialization
    let init_result = service.init(4).await;
    assert!(init_result.is_ok());
    
    // Test configuration
    service.set_allow_multiple_tasks_per_client(true);
    
    // Test getting stats
    let stats_result = service.get_stats().await;
    assert!(stats_result.is_ok());
    
    // Test shutdown
    let shutdown_result = service.shutdown().await;
    assert!(shutdown_result.is_ok());
}

#[tokio::test]
async fn test_task_operations() {
    // Clear any existing tasks from previous tests
    let running_tasks = get_running_tasks_instance();
    let _ = running_tasks.clear_all_tasks();
    
    let mut service = CutListOptimizerServiceImpl::new();
    service.init(4).await.unwrap();
    
    // Test getting tasks with Running status filter
    let tasks_result = service.get_tasks(Some(Status::Running)).await;
    assert!(tasks_result.is_ok());
    let tasks = tasks_result.unwrap();
    assert!(tasks.is_empty());
    
    // Test getting status for non-existent task
    let status_result = service.get_task_status("non-existent-task").await;
    assert!(status_result.is_ok());
    assert!(status_result.unwrap().is_none());
    
    service.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_init_service() {
    // Clear any existing tasks from previous tests
    let running_tasks = get_running_tasks_instance();
    let _ = running_tasks.clear_all_tasks();
    
    let mut service = CutListOptimizerServiceImpl::new();
    
    // Test that service is not initialized initially
    assert!(!service.is_initialized());
    
    // Test init() method
    let init_result = service.init(4).await;
    assert!(init_result.is_ok(), "Service initialization should succeed");
    
    // Test that service is now initialized
    assert!(service.is_initialized(), "Service should be initialized after calling init()");
    
    // Test that service is ready to accept tasks
    let stats_result = service.get_stats().await;
    assert!(stats_result.is_ok(), "Service should be able to provide stats after initialization");
    
    // Test that we can get tasks (should be empty initially)
    let tasks_result = service.get_tasks(None).await;
    assert!(tasks_result.is_ok(), "Service should be able to list tasks after initialization");
    let tasks = tasks_result.unwrap();
    assert!(tasks.is_empty(), "Task list should be empty initially");
    
    // Test invalid thread pool size
    let mut service2 = CutListOptimizerServiceImpl::new();
    let invalid_init_result = service2.init(0).await;
    assert!(invalid_init_result.is_err(), "Initialization with 0 threads should fail");
    
    // Clean up
    service.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_service_destroy() {
    // Clear any existing tasks from previous tests
    let running_tasks = get_running_tasks_instance();
    let _ = running_tasks.clear_all_tasks();
    
    let mut service = CutListOptimizerServiceImpl::new();
    
    // Initialize service
    service.init(4).await.unwrap();
    assert!(service.is_initialized());
    assert!(!service.is_shutdown());
    
    // Test destroy method
    let destroy_result = service.destroy().await;
    assert!(destroy_result.is_ok(), "Service destroy should succeed");
    
    // Test that service is now shutdown
    assert!(service.is_shutdown(), "Service should be shutdown after calling destroy()");
    
    // Test that operations fail after destroy
    let stats_result = service.get_stats().await;
    assert!(stats_result.is_err(), "Operations should fail after service is destroyed");
}


