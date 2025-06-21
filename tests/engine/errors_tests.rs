//! Tests for AppError and error handling

use cutlist_optimizer_cli::errors::AppError;



#[test]
fn test_error_creation() {
    let error = AppError::task_not_found("test-task-123");
    assert!(matches!(error, AppError::Task(_)));
    assert!(error.is_client_error());
    assert!(!error.is_server_error());
    assert!(!error.is_retryable());
}

#[test]
fn test_client_already_has_task_error() {
    let error = AppError::client_already_has_task("client-1", "existing-task-456");
    assert!(matches!(error, AppError::Service(_)));
    assert!(error.is_client_error());
}

#[test]
fn test_retryable_errors() {
    let timeout_error = AppError::Task(cutlist_optimizer_cli::errors::TaskError::Timeout);
    assert!(timeout_error.is_retryable());

    let resource_error = AppError::resource_unavailable("database");
    assert!(resource_error.is_retryable());

    let not_found_error = AppError::task_not_found("test");
    assert!(!not_found_error.is_retryable());
}

#[test]
fn test_server_errors() {
    let internal_error = AppError::internal("Something went wrong");
    assert!(internal_error.is_server_error());
    assert!(!internal_error.is_client_error());

    let shutdown_error = AppError::Service(cutlist_optimizer_cli::errors::ServiceError::ShuttingDown);
    assert!(shutdown_error.is_server_error());
}

#[test]
fn test_error_display() {
    let error = AppError::task_not_found("test-123");
    let error_string = format!("{}", error);
    assert!(error_string.contains("test-123"));
    assert!(error_string.contains("Task not found"));
}

#[test]
fn test_task_already_exists_error() {
    let error = AppError::task_already_exists("duplicate-task");
    assert!(matches!(error, AppError::Service(_)));
    assert!(error.is_client_error());
}

#[test]
fn test_invalid_task_id_error() {
    let error = AppError::invalid_task_id("invalid-id");
    assert!(matches!(error, AppError::Task(_)));
    assert!(error.is_client_error());
}

#[test]
fn test_invalid_client_id_error() {
    let error = AppError::invalid_client_id("invalid-client");
    assert!(matches!(error, AppError::Service(_)));
    assert!(error.is_client_error());
}

#[test]
fn test_lock_failed_error() {
    let error = AppError::lock_failed("mutex");
    assert!(matches!(error, AppError::Service(_)));
    assert!(error.is_retryable());
    assert!(error.is_server_error());
}

#[test]
fn test_invalid_task_state_error() {
    let error = AppError::invalid_task_state("running");
    assert!(matches!(error, AppError::Task(_)));
    assert!(error.is_client_error());
}

#[test]
fn test_permission_denied_error() {
    let error = AppError::permission_denied("delete_task");
    assert!(matches!(error, AppError::Service(_)));
    assert!(error.is_client_error());
}
