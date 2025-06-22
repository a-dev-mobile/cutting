//! Full integration tests for the CutListOptimizerService
//! 
//! This module contains comprehensive tests that verify the entire pipeline
//! from task submission to result retrieval, testing the complete workflow.

use cutlist_optimizer_cli::engine::{
    running_tasks::{TaskCleanup, get_running_tasks_instance},
    service::{CutListOptimizerService, CutListOptimizerServiceImpl}
};
use cutlist_optimizer_cli::models::{
    CalculationRequest, Panel, Configuration,
    enums::{Status, StatusCode},
};
use serial_test::serial;
use std::time::Duration;
use tokio::time::timeout;

/// Helper function to create a simple test request
fn create_test_request() -> CalculationRequest {
    let panels = vec![
        Panel {
            id: 1,
            width: Some("100.0".to_string()),
            height: Some("50.0".to_string()),
            count: 2,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Test Panel 1".to_string()),
            edge: None,
        },
        Panel {
            id: 2,
            width: Some("75.0".to_string()),
            height: Some("25.0".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Test Panel 2".to_string()),
            edge: None,
        },
    ];

    let stock_panels = vec![
        Panel {
            id: 101,
            width: Some("200.0".to_string()),
            height: Some("100.0".to_string()),
            count: 5,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Stock Panel".to_string()),
            edge: None,
        },
    ];

    CalculationRequest {
        configuration: Some(Configuration::default()),
        panels,
        stock_panels,
    }
}

/// Helper function to create an invalid request (no valid panels)
fn create_invalid_request() -> CalculationRequest {
    let panels = vec![
        Panel {
            id: 1,
            width: None, // Invalid - no width
            height: Some("50.0".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: None,
            edge: None,
        },
    ];

    let stock_panels = vec![
        Panel {
            id: 101,
            width: Some("200.0".to_string()),
            height: Some("100.0".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: None,
            edge: None,
        },
    ];

    CalculationRequest {
        configuration: Some(Configuration::default()),
        panels,
        stock_panels,
    }
}

// /// Full pipeline integration test
// /// 
// /// This test verifies the complete workflow:
// /// 1. Service initialization
// /// 2. Task submission
// /// 3. Task creation and processing
// /// 4. Status monitoring
// /// 5. Result retrieval
// #[tokio::test]
// #[serial]
// async fn test_full_pipeline() {
//     // Clear any existing tasks from previous tests
//     let running_tasks = get_running_tasks_instance();
//     running_tasks.clear_all_tasks().unwrap();

//     // 1. Create service and initialize
//     let mut service = CutListOptimizerServiceImpl::new();
//     let init_result = service.init(4).await;
//     assert!(init_result.is_ok(), "Service initialization should succeed");
//     assert!(service.is_initialized(), "Service should be initialized");

//     // 2. Submit a simple task
//     let request = create_test_request();
//     let submission_result = service.submit_task(request).await;
//     assert!(submission_result.is_ok(), "Task submission should succeed");
    
//     let submission = submission_result.unwrap();
//     assert_eq!(submission.status_code, StatusCode::Ok, "Submission should return OK status");
//     assert!(submission.task_id.is_some(), "Submission should return a task ID");
    
//     let task_id = submission.task_id.unwrap();
//     println!("Submitted task with ID: {}", task_id);

//     // 3. Check that task was created
//     let tasks_result = service.get_tasks(None).await;
//     assert!(tasks_result.is_ok(), "Should be able to get task list");
    
//     let tasks = tasks_result.unwrap();
//     assert!(!tasks.is_empty(), "Task list should not be empty after submission");
//     assert!(tasks.contains(&task_id), "Task list should contain our submitted task");

//     // 4. Wait for processing (or timeout)
//     let mut attempts = 0;
//     let max_attempts = 30; // 30 seconds timeout
//     let mut final_status = None;

//     while attempts < max_attempts {
//         let status_result = timeout(
//             Duration::from_secs(2),
//             service.get_task_status(&task_id)
//         ).await;

//         if let Ok(Ok(Some(status_response))) = status_result {
//             println!("Task {} status: {:?}, progress: {}%", 
//                     task_id, status_response.status, status_response.percentage_done);
            
//             // Check if task has finished (either successfully or with error)
//             match status_response.status {
//                 Status::Finished | Status::Error | Status::Terminated | Status::Stopped => {
//                     final_status = Some(status_response);
//                     break;
//                 }
//                 Status::Running | Status::Queued => {
//                     // Task is still processing, continue waiting
//                     tokio::time::sleep(Duration::from_secs(1)).await;
//                 }
//             }
//         } else {
//             println!("Failed to get task status on attempt {}", attempts + 1);
//         }

//         attempts += 1;
//     }

//     // 5. Get final status
//     if final_status.is_none() {
//         // If we didn't get a final status, try one more time
//         let final_status_result = service.get_task_status(&task_id).await;
//         if let Ok(Some(status_response)) = final_status_result {
//             final_status = Some(status_response);
//         }
//     }

//     // 6. Check that we have a meaningful result
//     assert!(final_status.is_some(), "Should have received a final status for the task");
    
//     let final_status = final_status.unwrap();
//     println!("Final task status: {:?}", final_status.status);
    
//     // The task should have completed processing (not necessarily successfully, but it should have tried)
//     assert!(
//         matches!(final_status.status, Status::Finished | Status::Error | Status::Terminated),
//         "Task should have reached a final state (Finished, Error, or Terminated)"
//     );

//     // If the task finished successfully, check that we have a solution
//     if final_status.status == Status::Finished {
//         assert!(final_status.solution.is_some(), "Finished task should have a solution");
//         println!("Task completed successfully with solution");
//     } else {
//         println!("Task completed with status: {:?}", final_status.status);
//     }

//     // Verify that the task is still in the system
//     let final_tasks_result = service.get_tasks(None).await;
//     assert!(final_tasks_result.is_ok(), "Should be able to get final task list");
    
//     let final_tasks = final_tasks_result.unwrap();
//     assert!(final_tasks.contains(&task_id), "Task should still be in the system");

//     // Clean up
//     let shutdown_result = service.shutdown().await;
//     assert!(shutdown_result.is_ok(), "Service shutdown should succeed");
// }

/// Test pipeline with invalid request
#[tokio::test]
async fn test_pipeline_with_invalid_request() {
    // Clear any existing tasks
    let running_tasks = get_running_tasks_instance();
    running_tasks.clear_all_tasks().unwrap();

    // Initialize service
    let mut service = CutListOptimizerServiceImpl::new();
    service.init(4).await.unwrap();

    // Submit invalid request
    let invalid_request = create_invalid_request();
    let submission_result = service.submit_task(invalid_request).await;
    assert!(submission_result.is_ok(), "Submission should not fail even with invalid request");
    
    let submission = submission_result.unwrap();
    
    // Should get an error status code for invalid tiles
    assert_ne!(submission.status_code, StatusCode::Ok, "Should not return OK for invalid request");
    assert!(submission.task_id.is_none(), "Should not return task ID for invalid request");

    service.shutdown().await.unwrap();
}

// /// Test multiple tasks pipeline
// #[tokio::test]
// #[serial]
// async fn test_multiple_tasks_pipeline() {
//     // Clear any existing tasks
//     let running_tasks = get_running_tasks_instance();
//     running_tasks.clear_all_tasks().unwrap();

//     // Initialize service with multiple tasks allowed
//     let mut service = CutListOptimizerServiceImpl::new();
//     service.init(4).await.unwrap();
//     service.set_allow_multiple_tasks_per_client(true);

//     // Submit multiple tasks
//     let mut task_ids = Vec::new();
    
//     for i in 0..3 {
//         let mut request = create_test_request();
//         // Modify the request slightly for each task
//         request.panels[0].id = i + 1;
        
//         let submission_result = service.submit_task(request).await;
//         assert!(submission_result.is_ok(), "Task {} submission should succeed", i);
        
//         let submission = submission_result.unwrap();
//         assert_eq!(submission.status_code, StatusCode::Ok, "Task {} should return OK status", i);
//         assert!(submission.task_id.is_some(), "Task {} should return a task ID", i);
        
//         task_ids.push(submission.task_id.unwrap());
//     }

//     println!("Submitted {} tasks: {:?}", task_ids.len(), task_ids);

//     // Verify all tasks are in the system
//     let tasks_result = service.get_tasks(None).await;
//     assert!(tasks_result.is_ok(), "Should be able to get task list");
    
//     let tasks = tasks_result.unwrap();
//     assert!(tasks.len() >= task_ids.len(), "Should have at least {} tasks", task_ids.len());
    
//     for task_id in &task_ids {
//         assert!(tasks.contains(task_id), "Task list should contain task {}", task_id);
//     }

//     // Wait a bit for tasks to start processing
//     tokio::time::sleep(Duration::from_secs(2)).await;

//     // Check status of all tasks
//     for task_id in &task_ids {
//         let status_result = service.get_task_status(task_id).await;
//         assert!(status_result.is_ok(), "Should be able to get status for task {}", task_id);
        
//         if let Some(status_response) = status_result.unwrap() {
//             println!("Task {} status: {:?}", task_id, status_response.status);
//             // Task should be in some valid state
//             assert!(
//                 matches!(status_response.status, 
//                     Status::Queued | Status::Running | Status::Finished | Status::Error | Status::Terminated),
//                 "Task {} should be in a valid state", task_id
//             );
//         }
//     }

//     service.shutdown().await.unwrap();
// }

/// Test task stopping functionality
#[tokio::test]
#[serial]
async fn test_task_stop_pipeline() {
    // Clear any existing tasks
    let running_tasks = get_running_tasks_instance();
    running_tasks.clear_all_tasks().unwrap();

    // Initialize service
    let mut service = CutListOptimizerServiceImpl::new();
    service.init(4).await.unwrap();

    // Submit a task
    let request = create_test_request();
    let submission_result = service.submit_task(request).await;
    assert!(submission_result.is_ok(), "Task submission should succeed");
    
    let task_id = submission_result.unwrap().task_id.unwrap();
    println!("Submitted task for stopping test: {}", task_id);

    // Wait a moment for task to start
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Try to stop the task
    let stop_result = service.stop_task(&task_id).await;
    assert!(stop_result.is_ok(), "Stop task should not fail");
    
    if let Some(stop_response) = stop_result.unwrap() {
        println!("Task {} stopped with status: {:?}", task_id, stop_response.status);
        // Task should be in a stopped state or already finished
        assert!(
            matches!(stop_response.status, 
                Status::Finished | Status::Error | Status::Terminated),
            "Stopped task should be in a final state"
        );
    }

    // Test stopping non-existent task
    let stop_nonexistent_result = service.stop_task("non-existent-task").await;
    assert!(stop_nonexistent_result.is_ok(), "Stop non-existent task should not fail");
    assert!(stop_nonexistent_result.unwrap().is_none(), "Should return None for non-existent task");

    service.shutdown().await.unwrap();
}

// /// Test service statistics during pipeline execution
// #[tokio::test]
// #[serial]
// async fn test_stats_during_pipeline() {
//     // Clear any existing tasks
//     let running_tasks = get_running_tasks_instance();
//     running_tasks.clear_all_tasks().unwrap();

//     // Initialize service
//     let mut service = CutListOptimizerServiceImpl::new();
//     service.init(4).await.unwrap();

//     // Get initial stats
//     let initial_stats = service.get_stats().await.unwrap();
//     println!("Initial stats: running={}, finished={}, error={}", 
//              initial_stats.nbr_running_tasks, 
//              initial_stats.nbr_finished_tasks, 
//              initial_stats.nbr_error_tasks);

//     // Submit a task
//     let request = create_test_request();
//     let submission_result = service.submit_task(request).await;
//     let task_id = submission_result.unwrap().task_id.unwrap();

//     // Wait a moment and check stats again
//     tokio::time::sleep(Duration::from_millis(500)).await;
    
//     let after_submission_stats = service.get_stats().await.unwrap();
//     println!("After submission stats: running={}, finished={}, error={}", 
//              after_submission_stats.nbr_running_tasks, 
//              after_submission_stats.nbr_finished_tasks, 
//              after_submission_stats.nbr_error_tasks);

//     // Should have at least one task in the system now
//     let total_tasks = after_submission_stats.nbr_running_tasks + 
//                      after_submission_stats.nbr_finished_tasks + 
//                      after_submission_stats.nbr_error_tasks +
//                      after_submission_stats.nbr_idle_tasks +
//                      after_submission_stats.nbr_stopped_tasks +
//                      after_submission_stats.nbr_terminated_tasks;
    
//     assert!(total_tasks >= 1, "Should have at least one task in the system");

//     // Wait for task to complete or timeout
//     let mut attempts = 0;
//     while attempts < 10 {
//         let current_stats = service.get_stats().await.unwrap();
//         println!("Current stats (attempt {}): running={}, finished={}, error={}", 
//                  attempts + 1,
//                  current_stats.nbr_running_tasks, 
//                  current_stats.nbr_finished_tasks, 
//                  current_stats.nbr_error_tasks);
        
//         if current_stats.nbr_running_tasks == 0 {
//             // No more running tasks
//             break;
//         }
        
//         tokio::time::sleep(Duration::from_secs(1)).await;
//         attempts += 1;
//     }

//     // Get final stats
//     let final_stats = service.get_stats().await.unwrap();
//     println!("Final stats: running={}, finished={}, error={}", 
//              final_stats.nbr_running_tasks, 
//              final_stats.nbr_finished_tasks, 
//              final_stats.nbr_error_tasks);

//     service.shutdown().await.unwrap();
// }
