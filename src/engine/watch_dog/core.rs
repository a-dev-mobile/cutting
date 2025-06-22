//! Core WatchDog implementation

use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use crate::logging::{error, info, warn};

use super::config::WatchDogConfig;
use super::monitoring::TaskMonitor;
use super::cleanup::TaskCleanup;
use super::statistics::WatchDogStatistics;
use super::super::running_tasks::RunningTasks;

/// WatchDog monitors running tasks and handles cleanup of stale tasks
pub struct WatchDog {
    monitor: TaskMonitor,
    cleanup: TaskCleanup,
    statistics: WatchDogStatistics,
    config: WatchDogConfig,
    is_running: std::sync::atomic::AtomicBool,
}

impl WatchDog {
    /// Creates a new WatchDog instance with default configuration
    pub fn new(running_tasks: Arc<RunningTasks>) -> Self {
        let config = WatchDogConfig::default();
        Self::with_config(running_tasks, config)
    }

    /// Creates a new WatchDog with custom configuration
    pub fn with_config(running_tasks: Arc<RunningTasks>, config: WatchDogConfig) -> Self {
        let monitor = TaskMonitor::new(running_tasks.clone(), config.clone());
        let cleanup = TaskCleanup::new(running_tasks.clone());
        let statistics = WatchDogStatistics::new(running_tasks);

        Self {
            monitor,
            cleanup,
            statistics,
            config,
            is_running: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Creates a new WatchDog with custom intervals (backward compatibility)
    pub fn with_intervals(
        running_tasks: Arc<RunningTasks>,
        check_interval: Duration,
        task_timeout: Duration,
    ) -> Self {
        let config = WatchDogConfig::new(
            check_interval,
            task_timeout,
            Duration::from_secs(300), // Default grace period
        );
        Self::with_config(running_tasks, config)
    }

    /// Starts the WatchDog monitoring loop
    pub async fn start(&self) {
        if self.is_running.swap(true, std::sync::atomic::Ordering::SeqCst) {
            warn!("WatchDog is already running");
            return;
        }

        // Validate configuration before starting
        if let Err(e) = self.config.validate() {
            error!("Invalid WatchDog configuration: {}", e);
            self.is_running.store(false, std::sync::atomic::Ordering::SeqCst);
            return;
        }

        info!("Starting WatchDog with check interval: {:?}", self.config.check_interval);
        
        let mut interval = interval(self.config.check_interval);
        
        while self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            interval.tick().await;
            
            if let Err(e) = self.check_and_cleanup_tasks().await {
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

    /// Performs a single check and cleanup cycle
    async fn check_and_cleanup_tasks(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Check for tasks that need cleanup
        let tasks_to_cleanup = self.monitor.check_tasks().await?;
        
        // Cleanup identified tasks
        if !tasks_to_cleanup.is_empty() {
            let cleanup_result = self.cleanup.cleanup_tasks(tasks_to_cleanup).await;
            
            if cleanup_result.has_failures() {
                warn!("Some tasks failed to cleanup: {} failures", cleanup_result.failed_cleanups.len());
            }
            
            info!(
                "Cleanup completed: {} successful, {} not found, {} failed",
                cleanup_result.successful_cleanups.len(),
                cleanup_result.not_found_tasks.len(),
                cleanup_result.failed_cleanups.len()
            );
        }
        
        // Log statistics
        self.statistics.log_statistics();
        
        Ok(())
    }

    /// Forces cleanup of a specific task
    pub fn force_cleanup_task(&self, task_id: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        info!("WatchDog force cleaning up task: {}", task_id);
        self.cleanup.force_cleanup_task(task_id)
    }

    /// Gets the current status of the WatchDog
    pub fn is_running(&self) -> bool {
        self.is_running.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Sets the task timeout duration
    pub fn set_task_timeout(&mut self, timeout: Duration) {
        self.config.task_timeout = timeout;
        self.monitor.update_config(self.config.clone());
        info!("WatchDog task timeout set to: {:?}", timeout);
    }

    /// Sets the check interval duration
    pub fn set_check_interval(&mut self, interval: Duration) {
        self.config.check_interval = interval;
        self.monitor.update_config(self.config.clone());
        info!("WatchDog check interval set to: {:?}", interval);
    }

    /// Sets the grace period duration
    pub fn set_grace_period(&mut self, grace_period: Duration) {
        self.config.grace_period = grace_period;
        self.monitor.update_config(self.config.clone());
        info!("WatchDog grace period set to: {:?}", grace_period);
    }

    /// Updates the entire configuration
    pub fn update_config(&mut self, config: WatchDogConfig) -> Result<(), String> {
        config.validate()?;
        self.config = config.clone();
        self.monitor.update_config(config);
        info!("WatchDog configuration updated");
        Ok(())
    }

    /// Gets the current configuration
    pub fn get_config(&self) -> &WatchDogConfig {
        &self.config
    }

    /// Gets task statistics
    pub fn get_statistics(&self) -> super::statistics::WatchDogDetailedStats {
        self.statistics.get_detailed_statistics()
    }

    /// Gets a summary of task statistics
    pub fn get_summary(&self) -> super::statistics::WatchDogSummary {
        self.statistics.get_summary()
    }

    /// Checks if the system is healthy based on error rates
    pub fn is_system_healthy(&self, max_error_rate: f64) -> bool {
        self.statistics.is_system_healthy(max_error_rate)
    }

    /// Gets tasks that might need attention (long-running tasks)
    pub fn get_attention_needed_tasks(&self, max_running_time_hours: f64) -> Vec<String> {
        self.statistics.get_attention_needed_tasks(max_running_time_hours)
    }

    /// Performs manual cleanup of all completed tasks
    pub async fn cleanup_all_completed_tasks(&self) -> super::cleanup::CleanupResult {
        info!("Manual cleanup of all completed tasks requested");
        self.cleanup.cleanup_all_completed_tasks().await
    }

    /// Performs a one-time check without starting the monitoring loop
    pub async fn check_once(&self) -> Result<super::cleanup::CleanupResult, Box<dyn std::error::Error + Send + Sync>> {
        info!("Performing one-time WatchDog check");
        
        let tasks_to_cleanup = self.monitor.check_tasks().await?;
        let cleanup_result = if !tasks_to_cleanup.is_empty() {
            self.cleanup.cleanup_tasks(tasks_to_cleanup).await
        } else {
            super::cleanup::CleanupResult::new()
        };
        
        self.statistics.log_statistics();
        
        Ok(cleanup_result)
    }
}

impl Drop for WatchDog {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

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
}
