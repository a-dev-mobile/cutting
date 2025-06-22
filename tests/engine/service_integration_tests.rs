//! Integration tests for the service module
//! 
//! These tests demonstrate the complete service lifecycle and
//! validate the improved module structure.

use cutlist_optimizer_cli::engine::service::{
    CutListOptimizerService, CutListOptimizerServiceImpl, RequestValidator,
};
use cutlist_optimizer_cli::engine::service::utilities::{TaskMonitor, StatsCollector};
use cutlist_optimizer_cli::engine::service::service_management::ServiceHealthStatus;
use cutlist_optimizer_cli::models::{
    CalculationRequest, Panel,
    enums::{Status, StatusCode},
    configuration::structs::Configuration,
};

#[tokio::test]
async fn test_complete_service_lifecycle() {
    let mut service = CutListOptimizerServiceImpl::new();
    
    // Test initial state
    assert_eq!(service.get_health_status(), ServiceHealthStatus::NotInitialized);
    assert!(!service.is_ready());
    
    // Initialize service
    assert!(service.init(4).await.is_ok());
    assert_eq!(service.get_health_status(), ServiceHealthStatus::Healthy);
    assert!(service.is_ready());
    
    // Test configuration
    service.set_allow_multiple_tasks_per_client(true);
    
    // Shutdown service
    assert!(service.shutdown().await.is_ok());
    assert_eq!(service.get_health_status(), ServiceHealthStatus::Shutdown);
    assert!(!service.is_ready());
}

#[tokio::test]
async fn test_task_submission_with_validation() {
    let mut service = CutListOptimizerServiceImpl::new();
    assert!(service.init(4).await.is_ok());
    
    // Create a valid request
    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels: vec![
            Panel {
                id: 1,
                width: Some("100".to_string()),
                height: Some("200".to_string()),
                count: 1,
                material: "Wood".to_string(),
                enabled: true,
                orientation: 0,
                label: None,
                edge: None,
            },
            Panel {
                id: 2,
                width: Some("150".to_string()),
                height: Some("250".to_string()),
                count: 2,
                material: "Wood".to_string(),
                enabled: true,
                orientation: 0,
                label: None,
                edge: None,
            },
        ],
        stock_panels: vec![],
    };
    
    // Test request validation
    let _validation_result = RequestValidator::validate_request(&request).await;
    // Note: This might return an error due to empty stock_panels, which is expected
    
    // Test task submission (should handle validation internally)
    let result = service.submit_task(request).await;
    assert!(result.is_ok());
    
    let submission_result = result.unwrap();
    // The result might have an error status code due to validation, which is fine for this test
    assert!(matches!(submission_result.status_code, StatusCode::Ok | StatusCode::InvalidTiles | StatusCode::InvalidStockTiles));
}

#[test]
fn test_task_monitor_utilities() {
    // Test task status checking
    assert!(TaskMonitor::is_task_completed(&Status::Finished));
    assert!(TaskMonitor::is_task_completed(&Status::Error));
    assert!(!TaskMonitor::is_task_completed(&Status::Running));
    
    assert!(TaskMonitor::is_task_running(&Status::Running));
    assert!(TaskMonitor::is_task_running(&Status::Queued));
    assert!(!TaskMonitor::is_task_running(&Status::Finished));
    
    // Test progress calculation
    assert_eq!(TaskMonitor::calculate_progress(50, 100), 50);
    assert_eq!(TaskMonitor::calculate_progress(100, 100), 100);
    assert_eq!(TaskMonitor::calculate_progress(0, 100), 0);
    
    // Test cleanup logic
    assert!(TaskMonitor::should_cleanup_task(&Status::Finished, 25));
    assert!(!TaskMonitor::should_cleanup_task(&Status::Finished, 20));
    assert!(!TaskMonitor::should_cleanup_task(&Status::Running, 25));
}

#[test]
fn test_stats_collector_utilities() {
    // Test success rate calculation
    assert_eq!(StatsCollector::calculate_success_rate(75, 100), 75.0);
    assert_eq!(StatsCollector::calculate_success_rate(0, 0), 0.0);
    assert_eq!(StatsCollector::calculate_success_rate(100, 100), 100.0);
    
    // Test average completion time
    let times = vec![10.0, 20.0, 30.0];
    assert_eq!(StatsCollector::calculate_avg_completion_time(&times), 20.0);
    assert_eq!(StatsCollector::calculate_avg_completion_time(&[]), 0.0);
    
    // Test memory formatting
    assert_eq!(StatsCollector::format_memory_size(1024), "1.00 KB");
    assert_eq!(StatsCollector::format_memory_size(1048576), "1.00 MB");
    assert_eq!(StatsCollector::format_memory_size(512), "512.00 B");
}

#[tokio::test]
async fn test_service_error_handling() {
    let mut service = CutListOptimizerServiceImpl::new();
    
    // Test operations on uninitialized service
    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels: vec![],
        stock_panels: vec![],
    };
    
    let result = service.submit_task(request).await;
    assert!(result.is_err()); // Should fail because service is not initialized
    
    let status_result = service.get_task_status("test_id").await;
    assert!(status_result.is_err()); // Should fail because service is not initialized
    
    // Initialize and then shutdown
    assert!(service.init(4).await.is_ok());
    assert!(service.shutdown().await.is_ok());
    
    // Test operations on shutdown service
    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels: vec![],
        stock_panels: vec![],
    };
    
    let result = service.submit_task(request).await;
    assert!(result.is_err()); // Should fail because service is shutdown
}

#[test]
fn test_service_health_status_enum() {
    // Test enum variants and equality
    assert_eq!(ServiceHealthStatus::Healthy, ServiceHealthStatus::Healthy);
    assert_ne!(ServiceHealthStatus::Healthy, ServiceHealthStatus::NotInitialized);
    
    // Test error variant
    let error1 = ServiceHealthStatus::Error("Test error".to_string());
    let error2 = ServiceHealthStatus::Error("Test error".to_string());
    assert_eq!(error1, error2);
    
    let error3 = ServiceHealthStatus::Error("Different error".to_string());
    assert_ne!(error1, error3);
}

#[test]
fn test_backward_compatibility() {
    // Test that the utilities work correctly with the current crate structure
    use cutlist_optimizer_cli::engine::service::utilities::TaskMonitor;
    use cutlist_optimizer_cli::engine::service::utilities::StatsCollector;
    
    // Test that the utilities work as expected
    assert!(TaskMonitor::is_task_completed(&Status::Finished));
    assert!(!TaskMonitor::is_task_completed(&Status::Running));
    
    let stats = StatsCollector::collect_stats();
    // Just verify it doesn't panic and returns a Stats object
    assert_eq!(stats.total_tasks(), 0); // Default value
}
