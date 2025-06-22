//! Task cleanup functionality for WatchDog service

use std::sync::Arc;
use crate::logging::{error, info};
use crate::engine::running_tasks::{RunningTasks, TaskManager};
use crate::models::enums::Status;

/// Task cleanup component
#[derive(Debug)]
pub struct TaskCleanup {
    running_tasks: Arc<RunningTasks>,
}

impl TaskCleanup {
    /// Creates a new TaskCleanup
    pub fn new(running_tasks: Arc<RunningTasks>) -> Self {
        Self { running_tasks }
    }

    /// Cleans up a list of task IDs
    pub async fn cleanup_tasks(&self, task_ids: Vec<String>) -> CleanupResult {
        let mut result = CleanupResult::new();
        
        for task_id in task_ids {
            match self.cleanup_single_task(&task_id).await {
                Ok(success) => {
                    if success {
                        result.successful_cleanups.push(task_id);
                    } else {
                        result.not_found_tasks.push(task_id);
                    }
                }
                Err(e) => {
                    error!("Failed to cleanup task {}: {}", task_id, e);
                    result.failed_cleanups.push(FailedCleanup {
                        task_id,
                        error: e.to_string(),
                    });
                }
            }
        }
        
        result
    }

    /// Cleans up a single task
    pub async fn cleanup_single_task(&self, task_id: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        info!("Cleaning up task: {}", task_id);
        
        match self.running_tasks.remove_task(task_id) {
            Ok(Some(_)) => {
                info!("Successfully cleaned up task: {}", task_id);
                Ok(true)
            }
            Ok(None) => {
                info!("Task {} not found for cleanup", task_id);
                Ok(false)
            }
            Err(e) => {
                error!("Error cleaning up task {}: {}", task_id, e);
                Err(Box::new(e))
            }
        }
    }

    /// Forces cleanup of a specific task (used for manual cleanup)
    pub fn force_cleanup_task(&self, task_id: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        info!("Force cleaning up task: {}", task_id);
        
        match self.running_tasks.remove_task(task_id) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Cleans up all completed tasks (finished, error, terminated)
    pub async fn cleanup_all_completed_tasks(&self) -> CleanupResult {
        let tasks = self.running_tasks.get_tasks();
        let mut completed_task_ids = Vec::new();
        
        for task_arc in tasks {
            let task = task_arc.read();
            let status = *task.status.read().unwrap();
            
            match status {
                Status::Finished 
                | Status::Error 
                | Status::Terminated => {
                    completed_task_ids.push(task.id.clone());
                }
                _ => {}
            }
        }
        
        info!("Found {} completed tasks for cleanup", completed_task_ids.len());
        self.cleanup_tasks(completed_task_ids).await
    }
}

/// Result of cleanup operations
#[derive(Debug, Default)]
pub struct CleanupResult {
    /// Successfully cleaned up tasks
    pub successful_cleanups: Vec<String>,
    /// Tasks that were not found
    pub not_found_tasks: Vec<String>,
    /// Tasks that failed to cleanup
    pub failed_cleanups: Vec<FailedCleanup>,
}

impl CleanupResult {
    /// Creates a new empty CleanupResult
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the total number of tasks processed
    pub fn total_processed(&self) -> usize {
        self.successful_cleanups.len() + self.not_found_tasks.len() + self.failed_cleanups.len()
    }

    /// Returns true if all cleanup operations were successful
    pub fn all_successful(&self) -> bool {
        self.failed_cleanups.is_empty()
    }

    /// Returns true if any cleanup operations failed
    pub fn has_failures(&self) -> bool {
        !self.failed_cleanups.is_empty()
    }
}

/// Information about a failed cleanup operation
#[derive(Debug, Clone)]
pub struct FailedCleanup {
    /// ID of the task that failed to cleanup
    pub task_id: String,
    /// Error message
    pub error: String,
}
