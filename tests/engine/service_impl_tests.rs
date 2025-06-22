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
