//! Tests for WatchDog functionality

use cutlist_optimizer_cli::engine::{RunningTasks, WatchDog};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_watchdog_creation() {
    let running_tasks = Arc::new(RunningTasks::new());
    let watchdog = WatchDog::new(running_tasks);
    
    assert!(!watchdog.is_running());
}

#[tokio::test]
async fn test_watchdog_with_custom_intervals() {
    let running_tasks = Arc::new(RunningTasks::new());
    let check_interval = Duration::from_secs(10);
    let task_timeout = Duration::from_secs(60);
    
    let watchdog = WatchDog::with_intervals(running_tasks, check_interval, task_timeout);
    
    // Note: We can't directly access the intervals as they are private fields
    // This test mainly ensures the constructor works with custom intervals
    assert!(!watchdog.is_running());
}

#[tokio::test]
async fn test_watchdog_start_stop() {
    let running_tasks = Arc::new(RunningTasks::new());
    let watchdog = Arc::new(WatchDog::with_intervals(
        running_tasks,
        Duration::from_millis(100),
        Duration::from_secs(1),
    ));
    
    let watchdog_clone = watchdog.clone();
    let handle = tokio::spawn(async move {
        watchdog_clone.start().await;
    });
    
    // Let it run for a bit
    sleep(Duration::from_millis(200)).await;
    assert!(watchdog.is_running());
    
    // Stop it
    watchdog.stop();
    sleep(Duration::from_millis(200)).await;
    
    // Wait for the task to complete
    let _ = handle.await;
    assert!(!watchdog.is_running());
}
