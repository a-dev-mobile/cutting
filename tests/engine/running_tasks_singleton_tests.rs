//! Tests for RunningTasks singleton functionality

use std::sync::Arc;
use cutlist_optimizer_cli::engine::running_tasks::{RunningTasks, TaskManagerSingleton, get_running_tasks_instance};

#[test]
fn test_singleton_same_instance() {
    let instance1 = RunningTasks::get_instance();
    let instance2 = RunningTasks::get_instance();
    
    // Should be the same instance
    assert!(Arc::ptr_eq(instance1, instance2));
}

#[test]
fn test_convenience_function() {
    let instance1 = get_running_tasks_instance();
    let instance2 = RunningTasks::get_instance();
    
    // Should be the same instance
    assert!(Arc::ptr_eq(instance1, instance2));
}
