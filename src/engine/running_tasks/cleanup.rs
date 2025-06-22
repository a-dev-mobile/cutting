//! Task cleanup operations
//!
//! This module provides functionality for cleaning up old and finished tasks
//! to prevent memory leaks and maintain system performance.

use crate::{
    errors::Result,
    models::enums::Status,
};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;

use super::{structs::RunningTasks, task_management::TaskManager};

/// Trait for task cleanup operations
pub trait TaskCleanup {
    /// Clean up old finished tasks (older than specified duration)
    fn cleanup_old_tasks(&self, max_age_seconds: u64) -> Result<usize>;
    
    /// Clean up tasks with specific status
    fn cleanup_tasks_with_status(&self, status: Status) -> Result<usize>;
    
    /// Clean up all finished tasks regardless of age
    fn cleanup_all_finished_tasks(&self) -> Result<usize>;
}

impl TaskCleanup for RunningTasks {
    /// Clean up old finished tasks (older than specified duration)
    fn cleanup_old_tasks(&self, max_age_seconds: u64) -> Result<usize> {
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .saturating_sub(max_age_seconds);
        
        let mut removed_count = 0;
        let mut tasks_to_remove = Vec::new();
        
        // Collect tasks to remove
        for entry in self.tasks.iter() {
            let task = entry.value().read();
            let status = *task.status.read().unwrap();
            
            // Only remove finished, terminated, or error tasks
            if matches!(status, Status::Finished | Status::Terminated | Status::Error) {
                let task_time = task.start_time
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                if task_time < cutoff_time {
                    tasks_to_remove.push(entry.key().clone());
                }
            }
        }
        
        // Remove old tasks
        for task_id in tasks_to_remove {
            if self.remove_task(&task_id)?.is_some() {
                removed_count += 1;
                debug!("Cleaned up old task: {}", task_id);
            }
        }
        
        if removed_count > 0 {
            debug!("Cleaned up {} old tasks", removed_count);
        }
        
        Ok(removed_count)
    }
    
    /// Clean up tasks with specific status
    fn cleanup_tasks_with_status(&self, status: Status) -> Result<usize> {
        let mut removed_count = 0;
        let mut tasks_to_remove = Vec::new();
        
        // Collect tasks to remove
        for entry in self.tasks.iter() {
            let task = entry.value().read();
            let task_status = *task.status.read().unwrap();
            
            if task_status == status {
                tasks_to_remove.push(entry.key().clone());
            }
        }
        
        // Remove tasks
        for task_id in tasks_to_remove {
            if self.remove_task(&task_id)?.is_some() {
                removed_count += 1;
                debug!("Cleaned up task with status {:?}: {}", status, task_id);
            }
        }
        
        if removed_count > 0 {
            debug!("Cleaned up {} tasks with status {:?}", removed_count, status);
        }
        
        Ok(removed_count)
    }
    
    /// Clean up all finished tasks regardless of age
    fn cleanup_all_finished_tasks(&self) -> Result<usize> {
        let mut removed_count = 0;
        let mut tasks_to_remove = Vec::new();
        
        // Collect finished tasks to remove
        for entry in self.tasks.iter() {
            let task = entry.value().read();
            let status = *task.status.read().unwrap();
            
            if matches!(status, Status::Finished | Status::Terminated | Status::Error) {
                tasks_to_remove.push(entry.key().clone());
            }
        }
        
        // Remove finished tasks
        for task_id in tasks_to_remove {
            if self.remove_task(&task_id)?.is_some() {
                removed_count += 1;
                debug!("Cleaned up finished task: {}", task_id);
            }
        }
        
        if removed_count > 0 {
            debug!("Cleaned up {} finished tasks", removed_count);
        }
        
        Ok(removed_count)
    }
}
