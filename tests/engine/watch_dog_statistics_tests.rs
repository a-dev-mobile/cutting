//! Tests for WatchDog statistics functionality

use cutlist_optimizer_cli::engine::watch_dog::statistics::WatchDogSummary;

#[test]
fn test_watchdog_summary_to_string() {
    let summary = WatchDogSummary {
        total_tasks: 100,
        active_tasks: 10,
        completed_tasks: 90,
        success_rate: 85.0,
        error_rate: 5.0,
    };
    
    let result = summary.to_string();
    assert!(result.contains("Total: 100"));
    assert!(result.contains("Active: 10"));
    assert!(result.contains("Completed: 90"));
    assert!(result.contains("Success Rate: 85.0%"));
    assert!(result.contains("Error Rate: 5.0%"));
}
