//! Comprehensive tests for the Task struct migration from Java
//! 
//! This test suite verifies that the Rust implementation maintains
//! the same behavior as the original Java Task class.

use std::{sync::Arc, thread, time::Duration};
use cutlist_optimizer_cli::{
    models::{
        task::Task,
        enums::{Status, Orientation},
        CalculationRequest, TileDimensions,
    },
    error::TaskError,
};

#[test]
fn test_task_creation() {
    let task = Task::new("test-task-001".to_string());
    
    assert_eq!(task.id(), "test-task-001");
    assert_eq!(task.status(), Status::Queued);
    assert_eq!(task.factor(), 1.0);
    assert!(!task.is_min_trim_dimension_influenced());
    assert!(task.no_material_tiles().is_empty());
    assert!(task.calculation_request().is_none());
    assert!(task.solution().is_none());
    assert!(!task.has_solution());
    assert!(!task.has_solution_all_fit());
}

#[test]
fn test_status_transitions() {
    let task = Task::new("status-test".to_string());
    
    // Initial status should be Queued
    assert_eq!(task.status(), Status::Queued);
    assert!(!task.is_running());
    
    // Should be able to start from Queued
    assert!(task.set_running_status().is_ok());
    assert_eq!(task.status(), Status::Running);
    assert!(task.is_running());
    
    // Should not be able to start again when already running
    assert!(matches!(
        task.set_running_status(),
        Err(TaskError::InvalidStatusTransition { .. })
    ));
    
    // Should be able to stop when running
    assert!(task.stop().is_ok());
    assert_eq!(task.status(), Status::Finished);
    assert!(!task.is_running());
}

#[test]
fn test_status_transitions_terminate() {
    let task = Task::new("terminate-test".to_string());
    
    // Start the task
    task.set_running_status().unwrap();
    assert_eq!(task.status(), Status::Running);
    
    // Should be able to terminate when running
    assert!(task.terminate().is_ok());
    assert_eq!(task.status(), Status::Terminated);
    
    // Should not be able to stop when terminated
    assert!(matches!(
        task.stop(),
        Err(TaskError::InvalidStatusTransition { .. })
    ));
}

#[test]
fn test_error_status() {
    let task = Task::new("error-test".to_string());
    
    // Start the task
    task.set_running_status().unwrap();
    assert_eq!(task.status(), Status::Running);
    
    // Terminate with error
    task.terminate_error();
    assert_eq!(task.status(), Status::Error);
}

#[test]
fn test_material_management() {
    let task = Task::new("material-test".to_string());
    
    // Add materials
    task.add_material_to_compute("wood".to_string());
    task.add_material_to_compute("metal".to_string());
    
    // Check initial state
    assert_eq!(task.percentage_done(), 0);
    assert!(task.solutions("wood").unwrap().is_empty());
    assert!(task.solutions("metal").unwrap().is_empty());
    assert!(task.solutions("plastic").is_none());
    
    // Set progress for materials
    task.set_material_percentage_done("wood".to_string(), 50);
    task.set_material_percentage_done("metal".to_string(), 75);
    
    // Check overall progress (average)
    assert_eq!(task.percentage_done(), 62); // (50 + 75) / 2 = 62.5 -> 62
    
    // Complete one material
    task.set_material_percentage_done("wood".to_string(), 100);
    assert_eq!(task.percentage_done(), 87); // (100 + 75) / 2 = 87.5 -> 87
    
    // Complete all materials - should trigger finished status
    task.set_running_status().unwrap(); // Need to be running first
    task.set_material_percentage_done("metal".to_string(), 100);
    
    // Give it a moment for the status change to propagate
    thread::sleep(Duration::from_millis(10));
    assert_eq!(task.status(), Status::Finished);
    assert_eq!(task.percentage_done(), 100);
}

#[test]
fn test_thread_group_rankings() {
    let task = Task::new("ranking-test".to_string());
    
    // Add material
    task.add_material_to_compute("wood".to_string());
    
    // Initially should be empty
    let rankings = task.thread_group_rankings("wood").unwrap();
    assert!(rankings.is_empty());
    
    // Increment rankings
    task.increment_thread_group_rankings("wood", "group1");
    task.increment_thread_group_rankings("wood", "group1");
    task.increment_thread_group_rankings("wood", "group2");
    
    let rankings = task.thread_group_rankings("wood").unwrap();
    assert_eq!(rankings.get("group1"), Some(&2));
    assert_eq!(rankings.get("group2"), Some(&1));
    assert_eq!(rankings.get("group3"), None);
    
    // Non-existent material should return None
    assert!(task.thread_group_rankings("metal").is_none());
}

#[test]
fn test_logging() {
    let task = Task::new("log-test".to_string());
    
    // Initially empty
    assert!(task.log().is_empty());
    
    // Set log content
    task.set_log("Initial log entry".to_string());
    assert_eq!(task.log(), "Initial log entry");
    
    // Append lines
    task.append_line_to_log("Second line");
    task.append_line_to_log("Third line");
    
    let expected = "Initial log entry\nSecond line\nThird line";
    assert_eq!(task.log(), expected);
    
    // Test appending to empty log
    let task2 = Task::new("log-test-2".to_string());
    task2.append_line_to_log("First line");
    assert_eq!(task2.log(), "First line");
}

#[test]
fn test_time_tracking() {
    let task = Task::new("time-test".to_string());
    
    // Should have start time
    let start_time = task.start_time();
    assert!(start_time > 0);
    
    // End time should be 0 initially
    assert_eq!(task.end_time(), 0);
    
    // Elapsed time should be reasonable
    thread::sleep(Duration::from_millis(10));
    let elapsed = task.elapsed_time();
    assert!(elapsed >= 10);
    
    // Set running and then stop to set end time
    task.set_running_status().unwrap();
    thread::sleep(Duration::from_millis(10));
    task.stop().unwrap();
    
    // End time should now be set
    assert!(task.end_time() > 0);
    
    // Elapsed time should be reasonable
    let final_elapsed = task.elapsed_time();
    assert!(final_elapsed >= elapsed);
}

    #[test]
    fn test_solution_building_with_request() {
        let mut task = Task::new("test_task".to_string());
        
        // Set a calculation request first (required for building solution)
        let request = CalculationRequest {
            configuration: None,
            panels: vec![],
            stock_panels: vec![],
        };
        task.set_calculation_request(request);
        
        // Initially should have no solution
        assert!(!task.has_solution());
        
        // Build and set solution
        task.build_and_set_solution();
        
        // Should now have a solution (even if empty)
        assert!(task.has_solution());
    }

#[test]
fn test_getters_and_setters() {
    let mut task = Task::new("getset-test".to_string());
    
    // Test ID
    task.set_id("new-id".to_string());
    assert_eq!(task.id(), "new-id");
    
    // Test factor
    task.set_factor(2.5);
    assert_eq!(task.factor(), 2.5);
    
    // Test min trim dimension influenced
    task.set_min_trim_dimension_influenced(true);
    assert!(task.is_min_trim_dimension_influenced());
    
    // Test no material tiles
    let tiles = vec![
        TileDimensions {
            id: 1,
            width: 100,
            height: 200,
            label: None,
            material: "wood".to_string(),
            orientation: Orientation::Vertical,
            is_rotated: false,
        },
        TileDimensions {
            id: 2,
            width: 150,
            height: 250,
            label: None,
            material: "wood".to_string(),
            orientation: Orientation::Vertical,
            is_rotated: false,
        },
    ];
    task.set_no_material_tiles(tiles.clone());
    assert_eq!(task.no_material_tiles().len(), 2);
    assert_eq!(task.no_material_tiles()[0].width, 100);
    assert_eq!(task.no_material_tiles()[1].height, 250);
}

#[test]
fn test_thread_safe_operations() {
    let task = Arc::new(Task::new("thread-safe-test".to_string()));
    
    // Add material
    task.add_material_to_compute("wood".to_string());
    
    let task_clone = Arc::clone(&task);
    let handle = thread::spawn(move || {
        // Simulate concurrent operations
        for i in 0..10 {
            task_clone.increment_thread_group_rankings("wood", &format!("group{}", i % 3));
            task_clone.append_line_to_log(&format!("Log entry {}", i));
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // Concurrent operations from main thread
    for i in 10..20 {
        task.increment_thread_group_rankings("wood", &format!("group{}", i % 3));
        task.append_line_to_log(&format!("Main entry {}", i));
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
    
    // Verify operations completed without data races
    let rankings = task.thread_group_rankings("wood").unwrap();
    let total_rankings: i32 = rankings.values().sum();
    assert_eq!(total_rankings, 20); // 20 total increments
    
    let log = task.log();
    assert!(log.contains("Log entry"));
    assert!(log.contains("Main entry"));
}

#[test]
fn test_clone_behavior() {
    let mut task = Task::new("clone-test".to_string());
    
    // Set up some state
    task.set_factor(3.14);
    task.set_min_trim_dimension_influenced(true);
    task.add_material_to_compute("wood".to_string());
    task.set_material_percentage_done("wood".to_string(), 50);
    task.append_line_to_log("Original log");
    
    // Clone the task
    let cloned_task = task.clone();
    
    // Verify cloned state
    assert_eq!(cloned_task.id(), task.id());
    assert_eq!(cloned_task.factor(), task.factor());
    assert_eq!(cloned_task.is_min_trim_dimension_influenced(), task.is_min_trim_dimension_influenced());
    assert_eq!(cloned_task.percentage_done(), task.percentage_done());
    assert_eq!(cloned_task.log(), task.log());
    
    // Verify independence for non-shared fields - changes to original don't affect clone
    task.set_factor(2.71);
    assert_eq!(cloned_task.factor(), 3.14); // Should still be original value
    
    // Note: Arc-wrapped fields (like log, solutions, etc.) are shared between clones
    // This is the correct behavior for thread-safe sharing
    task.append_line_to_log("Modified log");
    assert_eq!(cloned_task.log(), "Original log\nModified log"); // Shared Arc means both see the change
    
    // Verify that both tasks see the same shared state
    cloned_task.append_line_to_log("Clone log");
    assert_eq!(task.log(), "Original log\nModified log\nClone log");
    assert_eq!(cloned_task.log(), "Original log\nModified log\nClone log");
}

#[test]
fn test_solution_building() {
    let mut task = Task::new("solution-test".to_string());
    
    // Initially no solution
    assert!(!task.has_solution());
    assert!(!task.has_solution_all_fit());
    assert!(task.build_solution().is_none());
    
    // Create a minimal calculation request
    let request = CalculationRequest {
        configuration: None,
        panels: vec![],
        stock_panels: vec![],
    };
    task.set_calculation_request(request);
    
    // Now should be able to build solution
    let solution = task.build_solution();
    assert!(solution.is_some());
    
    // Set the solution and test
    task.build_and_set_solution();
    assert!(task.has_solution());
    // Note: has_solution_all_fit() will be false because panels is empty
}

#[test]
fn test_thread_counting_placeholders() {
    let task = Task::new("thread-count-test".to_string());
    
    // These methods exist but will return 0 since we don't have actual CutListThread instances
    assert_eq!(task.nbr_running_threads(), 0);
    assert_eq!(task.nbr_queued_threads(), 0);
    assert_eq!(task.nbr_finished_threads(), 0);
    assert_eq!(task.nbr_finished_threads_for_material("wood"), 0);
    assert_eq!(task.nbr_terminated_threads(), 0);
    assert_eq!(task.nbr_error_threads(), 0);
    assert_eq!(task.max_thread_progress_percentage(), 0);
    assert_eq!(task.nbr_total_threads(), 0);
}

#[test]
fn test_edge_cases() {
    let task = Task::new("edge-test".to_string());
    
    // Test percentage calculation with no materials
    assert_eq!(task.percentage_done(), 0);
    
    // Test solutions for non-existent material
    assert!(task.solutions("nonexistent").is_none());
    
    // Test thread group rankings for non-existent material
    assert!(task.thread_group_rankings("nonexistent").is_none());
    
    // Test check_if_finished when already finished
    task.set_running_status().unwrap();
    task.add_material_to_compute("wood".to_string());
    task.set_material_percentage_done("wood".to_string(), 100);
    
    // Should be finished now
    assert_eq!(task.status(), Status::Finished);
    
    // Calling check_if_finished again should be safe
    task.check_if_finished();
    assert_eq!(task.status(), Status::Finished);
}
