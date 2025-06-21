//! Tests for TaskStatusResponse

use cutlist_optimizer_cli::models::{TaskStatusResponse, CalculationResponse, enums::Status};
use cutlist_optimizer_cli::errors::{AppError, CoreError};

#[test]
fn test_new_task_status_response() {
    let response = TaskStatusResponse::new(Status::Running);
    assert_eq!(response.status(), Status::Running);
    assert_eq!(response.percentage_done(), 0);
    assert_eq!(response.init_percentage(), 0);
    assert!(response.solution().is_none());
}

#[test]
fn test_default_task_status_response() {
    let response = TaskStatusResponse::default();
    assert_eq!(response.status(), Status::Queued);
    assert_eq!(response.percentage_done(), 0);
    assert_eq!(response.init_percentage(), 0);
    assert!(response.solution().is_none());
}

#[test]
fn test_with_details_valid() {
    let response = TaskStatusResponse::with_details(
        Status::Running,
        50,
        0,
        None,
    ).unwrap();
    
    assert_eq!(response.status(), Status::Running);
    assert_eq!(response.percentage_done(), 50);
    assert_eq!(response.init_percentage(), 0);
    assert!(response.solution().is_none());
}

#[test]
fn test_with_details_invalid_percentage_done() {
    let result = TaskStatusResponse::with_details(
        Status::Running,
        101,
        0,
        None,
    );
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Core(CoreError::InvalidInput { details }) => {
            assert!(details.contains("percentage_done must be <= 100"));
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_with_details_invalid_init_percentage() {
    let result = TaskStatusResponse::with_details(
        Status::Running,
        50,
        101,
        None,
    );
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Core(CoreError::InvalidInput { details }) => {
            assert!(details.contains("init_percentage must be <= 100"));
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_set_percentage_done_valid() {
    let mut response = TaskStatusResponse::new(Status::Running);
    assert!(response.set_percentage_done(75).is_ok());
    assert_eq!(response.percentage_done(), 75);
}

#[test]
fn test_set_percentage_done_invalid() {
    let mut response = TaskStatusResponse::new(Status::Running);
    let result = response.set_percentage_done(101);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Core(CoreError::InvalidInput { details }) => {
            assert!(details.contains("percentage_done must be <= 100"));
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_set_init_percentage_valid() {
    let mut response = TaskStatusResponse::new(Status::Running);
    assert!(response.set_init_percentage(10).is_ok());
    assert_eq!(response.init_percentage(), 10);
}

#[test]
fn test_set_init_percentage_invalid() {
    let mut response = TaskStatusResponse::new(Status::Running);
    let result = response.set_init_percentage(101);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Core(CoreError::InvalidInput { details }) => {
            assert!(details.contains("init_percentage must be <= 100"));
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_status_checks() {
    let mut response = TaskStatusResponse::new(Status::Running);
    
    // Test running status
    assert!(response.is_running());
    assert!(!response.is_completed());
    assert!(!response.is_successful());
    assert!(!response.has_error());
    assert!(!response.is_queued());

    // Test finished status
    response.set_status(Status::Finished);
    assert!(!response.is_running());
    assert!(response.is_completed());
    assert!(response.is_successful());
    assert!(!response.has_error());
    assert!(!response.is_queued());

    // Test error status
    response.set_status(Status::Error);
    assert!(!response.is_running());
    assert!(response.is_completed());
    assert!(!response.is_successful());
    assert!(response.has_error());
    assert!(!response.is_queued());

    // Test terminated status
    response.set_status(Status::Terminated);
    assert!(!response.is_running());
    assert!(response.is_completed());
    assert!(!response.is_successful());
    assert!(!response.has_error());
    assert!(!response.is_queued());

    // Test queued status
    response.set_status(Status::Queued);
    assert!(!response.is_running());
    assert!(!response.is_completed());
    assert!(!response.is_successful());
    assert!(!response.has_error());
    assert!(response.is_queued());
}

#[test]
fn test_update_progress() {
    let mut response = TaskStatusResponse::new(Status::Running);
    
    // Update progress without changing status
    assert!(response.update_progress(50, None).is_ok());
    assert_eq!(response.percentage_done(), 50);
    assert_eq!(response.status(), Status::Running);
    
    // Update progress with status change
    assert!(response.update_progress(75, Some(Status::Running)).is_ok());
    assert_eq!(response.percentage_done(), 75);
    assert_eq!(response.status(), Status::Running);
    
    // Invalid percentage
    assert!(response.update_progress(101, None).is_err());
}

#[test]
fn test_solution_management() {
    let mut response = TaskStatusResponse::new(Status::Running);
    
    // Initially no solution
    assert!(response.solution().is_none());
    
    // Create a mock solution
    let solution = create_mock_calculation_response();
    
    // Set solution
    response.set_solution(solution.clone());
    assert!(response.solution().is_some());
    assert_eq!(response.solution().unwrap().version, "1.0");
    
    // Clear solution
    response.clear_solution();
    assert!(response.solution().is_none());
    
    // Set solution again and take it
    response.set_solution(solution);
    let taken_solution = response.take_solution();
    assert!(taken_solution.is_some());
    assert!(response.solution().is_none());
    assert_eq!(taken_solution.unwrap().version, "1.0");
}

#[test]
fn test_complete_with_solution() {
    let mut response = TaskStatusResponse::new(Status::Running);
    response.set_percentage_done(50).unwrap();
    
    let solution = create_mock_calculation_response();
    response.complete_with_solution(solution);
    
    assert_eq!(response.status(), Status::Finished);
    assert_eq!(response.percentage_done(), 100);
    assert!(response.solution().is_some());
    assert!(response.is_successful());
    assert!(response.is_completed());
}

#[test]
fn test_mark_as_error() {
    let mut response = TaskStatusResponse::new(Status::Running);
    let solution = create_mock_calculation_response();
    response.set_solution(solution);
    
    response.mark_as_error();
    
    assert_eq!(response.status(), Status::Error);
    assert!(response.solution().is_none());
    assert!(response.has_error());
    assert!(response.is_completed());
}

#[test]
fn test_mark_as_terminated() {
    let mut response = TaskStatusResponse::new(Status::Running);
    let solution = create_mock_calculation_response();
    response.set_solution(solution);
    
    response.mark_as_terminated();
    
    assert_eq!(response.status(), Status::Terminated);
    assert!(response.solution().is_none());
    assert!(response.is_completed());
    assert!(!response.is_successful());
}

#[test]
fn test_serialization() {
    let response = TaskStatusResponse::with_details(
        Status::Running,
        75,
        0,
        Some(create_mock_calculation_response()),
    ).unwrap();
    
    // Test JSON serialization
    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("Running"));
    assert!(json.contains("75"));
    
    // Test JSON deserialization
    let deserialized: TaskStatusResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.status(), Status::Running);
    assert_eq!(deserialized.percentage_done(), 75);
    assert!(deserialized.solution().is_some());
}

#[test]
fn test_boundary_values() {
    // Test with 0% progress
    let mut response = TaskStatusResponse::new(Status::Running);
    assert!(response.set_percentage_done(0).is_ok());
    assert_eq!(response.percentage_done(), 0);
    
    // Test with 100% progress
    assert!(response.set_percentage_done(100).is_ok());
    assert_eq!(response.percentage_done(), 100);
    
    // Test with 0% init percentage
    assert!(response.set_init_percentage(0).is_ok());
    assert_eq!(response.init_percentage(), 0);
    
    // Test with 100% init percentage
    assert!(response.set_init_percentage(100).is_ok());
    assert_eq!(response.init_percentage(), 100);
}

#[test]
fn test_progress_workflow() {
    let mut response = TaskStatusResponse::new(Status::Queued);
    
    // Start task
    response.set_status(Status::Running);
    assert!(response.update_progress(10, None).is_ok());
    
    // Progress updates
    assert!(response.update_progress(25, None).is_ok());
    assert!(response.update_progress(50, None).is_ok());
    assert!(response.update_progress(75, None).is_ok());
    
    // Complete task
    let solution = create_mock_calculation_response();
    response.complete_with_solution(solution);
    
    assert_eq!(response.percentage_done(), 100);
    assert_eq!(response.status(), Status::Finished);
    assert!(response.is_successful());
}

/// Helper function to create a mock CalculationResponse for testing
fn create_mock_calculation_response() -> CalculationResponse {
    CalculationResponse {
        version: "1.0".to_string(),
        edge_bands: None,
        elapsed_time: 1000,
        id: Some("test-id".to_string()),
        panels: None,
        request: None,
        solution_elapsed_time: Some(800),
        task_id: Some("task-123".to_string()),
        total_cut_length: 100.0,
        total_nbr_cuts: 10,
        total_used_area: 500.0,
        total_used_area_ratio: 0.8,
        total_wasted_area: 100.0,
        used_stock_panels: None,
        no_fit_panels: vec![],
        mosaics: vec![],
    }
}
