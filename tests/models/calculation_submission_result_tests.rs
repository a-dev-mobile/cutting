//! Tests for CalculationSubmissionResult

use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};

#[test]
fn test_new_with_both_parameters() {
    let result = CalculationSubmissionResult::new(StatusCode::Ok, "task_123");
    assert_eq!(result.status_code, StatusCode::Ok);
    assert_eq!(result.task_id, Some("task_123".to_string()));
}

#[test]
fn test_with_status_only() {
    let result = CalculationSubmissionResult::with_status(StatusCode::InvalidTiles);
    assert_eq!(result.status_code, StatusCode::InvalidTiles);
    assert_eq!(result.task_id, None);
}

#[test]
fn test_success_constructor() {
    let result = CalculationSubmissionResult::success("task_456");
    assert_eq!(result.status_code, StatusCode::Ok);
    assert_eq!(result.task_id, Some("task_456".to_string()));
    assert!(result.is_success());
    assert!(!result.is_error());
}

#[test]
fn test_error_constructor() {
    let result = CalculationSubmissionResult::error(StatusCode::ServerUnavailable);
    assert_eq!(result.status_code, StatusCode::ServerUnavailable);
    assert_eq!(result.task_id, None);
    assert!(!result.is_success());
    assert!(result.is_error());
}

#[test]
fn test_getters() {
    let result = CalculationSubmissionResult::new(StatusCode::Ok, "test_task");
    assert_eq!(result.get_status_code(), StatusCode::Ok);
    assert_eq!(result.get_task_id(), Some("test_task"));
}

#[test]
fn test_setters() {
    let mut result = CalculationSubmissionResult::default();
    
    result.set_task_id("new_task");
    assert_eq!(result.task_id, Some("new_task".to_string()));
    
    result.set_status_code(StatusCode::TaskAlreadyRunning);
    assert_eq!(result.status_code, StatusCode::TaskAlreadyRunning);
    
    result.clear_task_id();
    assert_eq!(result.task_id, None);
}

#[test]
fn test_default() {
    let result = CalculationSubmissionResult::default();
    assert_eq!(result.status_code, StatusCode::Ok);
    assert_eq!(result.task_id, None);
}

#[test]
fn test_display() {
    let result_with_task = CalculationSubmissionResult::success("task_123");
    assert_eq!(format!("{}", result_with_task), "Status: 0: Operation completed successfully, Task ID: task_123");
    
    let result_without_task = CalculationSubmissionResult::error(StatusCode::InvalidTiles);
    assert_eq!(format!("{}", result_without_task), "Status: 1: Invalid tiles provided");
}

#[test]
fn test_equality() {
    let result1 = CalculationSubmissionResult::new(StatusCode::Ok, "task_123");
    let result2 = CalculationSubmissionResult::new(StatusCode::Ok, "task_123");
    let result3 = CalculationSubmissionResult::new(StatusCode::Ok, "task_456");
    
    assert_eq!(result1, result2);
    assert_ne!(result1, result3);
}

#[test]
fn test_clone() {
    let original = CalculationSubmissionResult::success("task_123");
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_serialization() {
    let result = CalculationSubmissionResult::success("task_123");
    let serialized = serde_json::to_string(&result).unwrap();
    let deserialized: CalculationSubmissionResult = serde_json::from_str(&serialized).unwrap();
    assert_eq!(result, deserialized);
}

#[test]
fn test_java_constructor_equivalence() {
    // Test equivalent to Java constructor with both parameters
    let result1 = CalculationSubmissionResult::new(StatusCode::Ok, "task_123");
    assert_eq!(result1.status_code, StatusCode::Ok);
    assert_eq!(result1.task_id, Some("task_123".to_string()));
    
    // Test equivalent to Java constructor with status only
    let result2 = CalculationSubmissionResult::with_status(StatusCode::InvalidTiles);
    assert_eq!(result2.status_code, StatusCode::InvalidTiles);
    assert_eq!(result2.task_id, None);
}

#[test]
fn test_java_getter_setter_equivalence() {
    let mut result = CalculationSubmissionResult::with_status(StatusCode::Ok);
    
    // Test getter equivalence
    assert_eq!(result.get_status_code(), StatusCode::Ok);
    assert_eq!(result.get_task_id(), None);
    
    // Test setter equivalence
    result.set_task_id("new_task_id");
    assert_eq!(result.get_task_id(), Some("new_task_id"));
    
    result.set_status_code(StatusCode::TaskAlreadyRunning);
    assert_eq!(result.get_status_code(), StatusCode::TaskAlreadyRunning);
}

#[test]
fn test_rust_improvements() {
    // Test Option<String> instead of nullable String
    let result_without_task = CalculationSubmissionResult::with_status(StatusCode::Ok);
    assert!(result_without_task.task_id.is_none());
    
    let result_with_task = CalculationSubmissionResult::success("task_123");
    assert!(result_with_task.task_id.is_some());
    
    // Test enum instead of String for status code
    let result = CalculationSubmissionResult::error(StatusCode::InvalidTiles);
    assert!(result.status_code.is_error());
    assert!(!result.status_code.is_ok());
    
    // Test borrowing with get_task_id returning &str
    let result = CalculationSubmissionResult::success("task_123");
    let task_id_ref = result.get_task_id();
    assert_eq!(task_id_ref, Some("task_123"));
}

#[test]
fn test_convenience_methods() {
    // Test success convenience method
    let success = CalculationSubmissionResult::success("task_123");
    assert!(success.is_success());
    assert!(!success.is_error());
    assert_eq!(success.status_code, StatusCode::Ok);
    assert_eq!(success.task_id, Some("task_123".to_string()));
    
    // Test error convenience method
    let error = CalculationSubmissionResult::error(StatusCode::InvalidTiles);
    assert!(!error.is_success());
    assert!(error.is_error());
    assert_eq!(error.status_code, StatusCode::InvalidTiles);
    assert_eq!(error.task_id, None);
}
