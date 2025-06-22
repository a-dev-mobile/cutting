//! Task monitoring functionality for WatchDog service

use std::sync::Arc;
use std::time::{Duration, SystemTime};
use crate::logging::{debug, warn};

use crate::models::enums::Status;
use super::super::running_tasks::{RunningTasks, TaskManager};
use super::config::WatchDogConfig;

/// Task monitoring component
pub struct TaskMonitor {
    running_tasks: Arc<RunningTasks>,
    config: WatchDogConfig,
}

impl TaskMonitor {
    /// Creates a new TaskMonitor
    pub fn new(running_tasks: Arc<RunningTasks>, config: WatchDogConfig) -> Self {
        Self {
            running_tasks,
            config,
        }
    }

    /// Checks all running tasks and identifies those that need cleanup
    pub async fn check_tasks(&self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let now = SystemTime::now();
        let mut tasks_to_cleanup = Vec::new();
        
        // Get all tasks
        let tasks = self.running_tasks.get_tasks();
        
        debug!("TaskMonitor checking {} tasks", tasks.len());
        
        for task_arc in tasks {
            let task = task_arc.read();
            let task_id = task.id.clone();
            let status = *task.status.read().unwrap();
            let start_time = task.start_time;
            
            // Check if task has timed out
            if let Ok(elapsed) = now.duration_since(start_time) {
                if self.is_task_timed_out(elapsed) {
                    warn!("Task {} has timed out, marking for cleanup", task_id);
                    tasks_to_cleanup.push(task_id);
                    continue;
                }
                
                // Check if task is in a terminal state but still in running tasks
                if self.should_cleanup_completed_task(status, elapsed) {
                    debug!("Task {} is completed and past grace period, marking for cleanup", task_id);
                    tasks_to_cleanup.push(task_id);
                }
            }
        }
        
        Ok(tasks_to_cleanup)
    }

    /// Checks if a task has timed out
    fn is_task_timed_out(&self, elapsed: Duration) -> bool {
        elapsed > self.config.task_timeout
    }

    /// Checks if a completed task should be cleaned up
    fn should_cleanup_completed_task(&self, status: Status, elapsed: Duration) -> bool {
        match status {
            Status::Finished | Status::Error | Status::Terminated => {
                elapsed > self.config.grace_period
            }
            _ => false,
        }
    }

    /// Gets the current configuration
    pub fn get_config(&self) -> &WatchDogConfig {
        &self.config
    }

    /// Updates the configuration
    pub fn update_config(&mut self, config: WatchDogConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_is_task_timed_out() {
        let config = WatchDogConfig::default();
        let running_tasks = Arc::new(RunningTasks::new());
        let monitor = TaskMonitor::new(running_tasks, config);
        
        // Task that has not timed out
        let short_duration = Duration::from_secs(30);
        assert!(!monitor.is_task_timed_out(short_duration));
        
        // Task that has timed out
        let long_duration = Duration::from_secs(4000);
        assert!(monitor.is_task_timed_out(long_duration));
    }

    #[test]
    fn test_should_cleanup_completed_task() {
        let config = WatchDogConfig::default();
        let running_tasks = Arc::new(RunningTasks::new());
        let monitor = TaskMonitor::new(running_tasks, config);
        
        // Completed task within grace period
        let short_duration = Duration::from_secs(100);
        assert!(!monitor.should_cleanup_completed_task(Status::Finished, short_duration));
        
        // Completed task past grace period
        let long_duration = Duration::from_secs(400);
        assert!(monitor.should_cleanup_completed_task(Status::Finished, long_duration));
        
        // Running task should not be cleaned up regardless of duration
        assert!(!monitor.should_cleanup_completed_task(Status::Running, long_duration));
    }
}
