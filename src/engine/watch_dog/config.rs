//! Configuration module for WatchDog service

use std::time::Duration;

/// Configuration for WatchDog service
#[derive(Debug, Clone)]
pub struct WatchDogConfig {
    /// Interval between task checks
    pub check_interval: Duration,
    /// Timeout for tasks before they are considered stale
    pub task_timeout: Duration,
    /// Grace period for completed tasks before cleanup
    pub grace_period: Duration,
}

impl Default for WatchDogConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30), // Check every 30 seconds
            task_timeout: Duration::from_secs(3600), // 1 hour timeout
            grace_period: Duration::from_secs(300),  // 5 minutes grace period
        }
    }
}

impl WatchDogConfig {
    /// Creates a new WatchDogConfig with custom values
    pub fn new(check_interval: Duration, task_timeout: Duration, grace_period: Duration) -> Self {
        Self {
            check_interval,
            task_timeout,
            grace_period,
        }
    }

    /// Creates a config with custom check interval
    pub fn with_check_interval(mut self, interval: Duration) -> Self {
        self.check_interval = interval;
        self
    }

    /// Creates a config with custom task timeout
    pub fn with_task_timeout(mut self, timeout: Duration) -> Self {
        self.task_timeout = timeout;
        self
    }

    /// Creates a config with custom grace period
    pub fn with_grace_period(mut self, grace_period: Duration) -> Self {
        self.grace_period = grace_period;
        self
    }

    /// Validates the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.check_interval.is_zero() {
            return Err("Check interval cannot be zero".to_string());
        }
        
        if self.task_timeout.is_zero() {
            return Err("Task timeout cannot be zero".to_string());
        }
        
        if self.grace_period.is_zero() {
            return Err("Grace period cannot be zero".to_string());
        }
        
        if self.check_interval > self.task_timeout {
            return Err("Check interval should not be greater than task timeout".to_string());
        }
        
        Ok(())
    }
}
