//! Tests for WatchDog core functionality

use std::sync::Arc;
use std::time::Duration;
use cutlist_optimizer_cli::engine::watch_dog::{WatchDog, WatchDogConfig};
use cutlist_optimizer_cli::engine::running_tasks::RunningTasks;

#[test]
fn test_watchdog_creation() {
    let running_tasks = Arc::new(RunningTasks::new());
    let watchdog = WatchDog::new(running_tasks);
    
    assert!(!watchdog.is_running());
    assert_eq!(watchdog.get_config().check_interval, Duration::from_secs(30));
    assert_eq!(watchdog.get_config().task_timeout, Duration::from_secs(3600));
}

#[test]
fn test_watchdog_with_custom_config() {
    let running_tasks = Arc::new(RunningTasks::new());
    let config = WatchDogConfig::new(
        Duration::from_secs(10),
        Duration::from_secs(1800),
        Duration::from_secs(120),
    );
    let watchdog = WatchDog::with_config(running_tasks, config);
    
    assert_eq!(watchdog.get_config().check_interval, Duration::from_secs(10));
    assert_eq!(watchdog.get_config().task_timeout, Duration::from_secs(1800));
    assert_eq!(watchdog.get_config().grace_period, Duration::from_secs(120));
}

#[test]
fn test_config_update() {
    let running_tasks = Arc::new(RunningTasks::new());
    let mut watchdog = WatchDog::new(running_tasks);
    
    let new_config = WatchDogConfig::new(
        Duration::from_secs(15),
        Duration::from_secs(2400),
        Duration::from_secs(180),
    );
    
    assert!(watchdog.update_config(new_config).is_ok());
    assert_eq!(watchdog.get_config().check_interval, Duration::from_secs(15));
}
