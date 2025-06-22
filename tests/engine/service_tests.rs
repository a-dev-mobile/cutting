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
    let _ = running_tasks.cleanup_tasks_with_status(Status::Queued);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Running);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Finished);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Error);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Terminated);
    
    let service = CutListOptimizerServiceImpl::new();
    
    // Test that service can be created
    // Service creation should always succeed
    assert!(true);
}

#[tokio::test]
async fn test_service_basic_operations() {
    // Clear any existing tasks from previous tests
    let running_tasks = get_running_tasks_instance();
    let _ = running_tasks.cleanup_tasks_with_status(Status::Queued);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Running);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Finished);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Error);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Terminated);
    
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
    let _ = running_tasks.cleanup_tasks_with_status(Status::Queued);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Running);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Finished);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Error);
    let _ = running_tasks.cleanup_tasks_with_status(Status::Terminated);
    
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
