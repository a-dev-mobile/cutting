use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{debug, error, info, warn};

use crate::models::enums::Status;
use super::running_tasks::RunningTasks;

/// WatchDog monitors running tasks and handles cleanup of stale tasks
pub struct WatchDog {
    running_tasks: Arc<RunningTasks>,
    check_interval: Duration,
    task_timeout: Duration,
    is_running: std::sync::atomic::AtomicBool,
}

impl WatchDog {
    /// Creates a new WatchDog instance
    pub fn new(running_tasks: Arc<RunningTasks>) -> Self {
        Self {
            running_tasks,
            check_interval: Duration::from_secs(30), // Check every 30 seconds
            task_timeout: Duration::from_secs(3600), // 1 hour timeout
            is_running: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Creates a new WatchDog with custom intervals
    pub fn with_intervals(
        running_tasks: Arc<RunningTasks>,
        check_interval: Duration,
        task_timeout: Duration,
    ) -> Self {
        Self {
            running_tasks,
            check_interval,
            task_timeout,
            is_running: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Starts the WatchDog monitoring loop
    pub async fn start(&self) {
        if self.is_running.swap(true, std::sync::atomic::Ordering::SeqCst) {
            warn!("WatchDog is already running");
            return;
        }

        info!("Starting WatchDog with check interval: {:?}", self.check_interval);
        
        let mut interval = interval(self.check_interval);
        
        while self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            interval.tick().await;
            
            if let Err(e) = self.check_tasks().await {
                error!("Error during WatchDog task check: {}", e);
            }
        }
        
        info!("WatchDog stopped");
    }

    /// Stops the WatchDog
    pub fn stop(&self) {
        info!("Stopping WatchDog");
        self.is_running.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    /// Checks all running tasks for timeouts and cleanup
    async fn check_tasks(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now = std::time::SystemTime::now();
        let mut tasks_to_cleanup = Vec::new();
        
        // Get all tasks
        let tasks = self.running_tasks.get_tasks();
        
        debug!("WatchDog checking {} tasks", tasks.len());
        
        for task_arc in tasks {
            let task = task_arc.read();
            let task_id = task.id.clone();
            let status = *task.status.read().unwrap();
            let start_time = task.start_time;
            
            // Check if task has timed out
            if let Ok(elapsed) = now.duration_since(start_time) {
                if elapsed > self.task_timeout {
                    warn!("Task {} has timed out, marking for cleanup", task_id);
                    tasks_to_cleanup.push(task_id);
                    continue;
                }
                
                // Check if task is in a terminal state but still in running tasks
                match status {
                    Status::Finished | Status::Error | Status::Terminated => {
                        // Allow some grace period before cleanup
                        let grace_period = Duration::from_secs(300); // 5 minutes
                        if elapsed > grace_period {
                            debug!("Task {} is completed and past grace period, marking for cleanup", task_id);
                            tasks_to_cleanup.push(task_id);
                        }
                    }
                    _ => {
                        // Task is still active - no additional checks needed for now
                    }
                }
            }
        }
        
        // Cleanup identified tasks
        for task_id in tasks_to_cleanup {
            info!("WatchDog cleaning up task: {}", task_id);
            if let Err(e) = self.running_tasks.remove_task(&task_id) {
                error!("Failed to cleanup task {}: {}", task_id, e);
            }
        }
        
        // Log statistics
        let stats = self.running_tasks.get_stats();
        let total_tasks = stats.nbr_idle_tasks + stats.nbr_running_tasks + stats.nbr_finished_tasks + 
                         stats.nbr_terminated_tasks + stats.nbr_error_tasks;
        if total_tasks > 0 {
            debug!(
                "WatchDog stats - Total: {}, Running: {}, Finished: {}, Error: {}",
                total_tasks, stats.nbr_running_tasks, stats.nbr_finished_tasks, stats.nbr_error_tasks
            );
        }
        
        Ok(())
    }

    /// Forces cleanup of a specific task
    pub fn force_cleanup_task(&self, task_id: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        info!("WatchDog force cleaning up task: {}", task_id);
        match self.running_tasks.remove_task(task_id) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Gets the current status of the WatchDog
    pub fn is_running(&self) -> bool {
        self.is_running.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Sets the task timeout duration
    pub fn set_task_timeout(&mut self, timeout: Duration) {
        self.task_timeout = timeout;
        info!("WatchDog task timeout set to: {:?}", timeout);
    }

    /// Sets the check interval duration
    pub fn set_check_interval(&mut self, interval: Duration) {
        self.check_interval = interval;
        info!("WatchDog check interval set to: {:?}", interval);
    }
}

impl Drop for WatchDog {
    fn drop(&mut self) {
        self.stop();
    }
}
