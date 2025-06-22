//! Tests for WatchDog cleanup functionality

use std::sync::Arc;
use cutlist_optimizer_cli::engine::watch_dog::cleanup::{CleanupResult, FailedCleanup};

#[test]
fn test_cleanup_result_new() {
    let result = CleanupResult::new();
    assert_eq!(result.total_processed(), 0);
    assert!(result.all_successful());
    assert!(!result.has_failures());
}

#[test]
fn test_cleanup_result_with_data() {
    let mut result = CleanupResult::new();
    result.successful_cleanups.push("task1".to_string());
    result.not_found_tasks.push("task2".to_string());
    result.failed_cleanups.push(FailedCleanup {
        task_id: "task3".to_string(),
        error: "Test error".to_string(),
    });

    assert_eq!(result.total_processed(), 3);
    assert!(!result.all_successful());
    assert!(result.has_failures());
}
