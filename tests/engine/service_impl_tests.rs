//! Tests for CutListOptimizerServiceImpl

use cutlist_optimizer_cli::{
    engine::CutListOptimizerServiceImpl,
    models::{CalculationRequest, Panel, Configuration},
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
