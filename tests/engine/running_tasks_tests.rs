//! Tests for RunningTasks management system

use cutlist_optimizer_cli::{
    engine::RunningTasks,
    models::{task::Task, enums::Status},
};
use std::sync::Arc;

fn create_test_task(id: &str) -> Task {
    Task::new(id.to_string())
}

#[test]
fn test_running_tasks_basic_operations() {
    let running_tasks = RunningTasks::new();
    
    // Test adding task
    let task = create_test_task("task1");
    assert!(running_tasks.add_task(task).is_ok());
    
    // Test getting task
    let retrieved = running_tasks.get_task("task1");
    assert!(retrieved.is_some());
    
    // Test removing task
    let removed = running_tasks.remove_task("task1").unwrap();
    assert!(removed.is_some());
    
    // Test task no longer exists
    let not_found = running_tasks.get_task("task1");
    assert!(not_found.is_none());
}

#[test]
fn test_status_counters() {
    let running_tasks = RunningTasks::new();
    
    // Add task with Queued status
    let task = create_test_task("task1");
    running_tasks.add_task(task).unwrap();
    
    let stats = running_tasks.get_stats();
    assert_eq!(stats.nbr_idle_tasks, 1);
    assert_eq!(stats.nbr_running_tasks, 0);
    
    // Update status to Running
    running_tasks.update_task_status("task1", Status::Queued, Status::Running).unwrap();
    
    let stats = running_tasks.get_stats();
    assert_eq!(stats.nbr_idle_tasks, 0);
    assert_eq!(stats.nbr_running_tasks, 1);
}

#[test]
fn test_task_status_filtering() {
    let running_tasks = RunningTasks::new();
    
    // Add tasks with different statuses
    let task1 = create_test_task("task1");
    let task2 = create_test_task("task2");
    let task3 = create_test_task("task3");
    
    running_tasks.add_task(task1).unwrap();
    running_tasks.add_task(task2).unwrap();
    running_tasks.add_task(task3).unwrap();
    
    // Get tasks with Queued status
    let queued_tasks = running_tasks.get_tasks_with_status(Status::Queued);
    assert_eq!(queued_tasks.len(), 3);
    assert!(queued_tasks.contains(&"task1".to_string()));
    assert!(queued_tasks.contains(&"task2".to_string()));
    assert!(queued_tasks.contains(&"task3".to_string()));
}

#[test]
fn test_singleton_instance() {
    let instance1 = RunningTasks::get_instance();
    let instance2 = RunningTasks::get_instance();
    
    // Should be the same instance
    assert!(Arc::ptr_eq(instance1, instance2));
}
