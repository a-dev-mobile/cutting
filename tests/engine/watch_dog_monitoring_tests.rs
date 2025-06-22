//! Tests for WatchDog monitoring functionality

use std::sync::Arc;
use std::time::Duration;
use cutlist_optimizer_cli::engine::watch_dog::{TaskMonitor, WatchDogConfig};
use cutlist_optimizer_cli::engine::running_tasks::RunningTasks;

#[test]
fn test_task_monitor_creation() {
    let config = WatchDogConfig::default();
    let running_tasks = Arc::new(RunningTasks::new());
    let monitor = TaskMonitor::new(running_tasks, config.clone());
    
    assert_eq!(monitor.get_config().check_interval, config.check_interval);
    assert_eq!(monitor.get_config().task_timeout, config.task_timeout);
}

#[test]
fn test_task_monitor_config_update() {
    let config = WatchDogConfig::default();
    let running_tasks = Arc::new(RunningTasks::new());
    let mut monitor = TaskMonitor::new(running_tasks, config);
    
    let new_config = WatchDogConfig::new(
        Duration::from_secs(15),
        Duration::from_secs(2400),
        Duration::from_secs(180),
    );
    
    monitor.update_config(new_config.clone());
    assert_eq!(monitor.get_config().check_interval, new_config.check_interval);
    assert_eq!(monitor.get_config().task_timeout, new_config.task_timeout);
}
