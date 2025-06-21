use super::structs::PerformanceThresholds;
use crate::errors::{AppError, Result};
use crate::constants::PerformanceConstants;

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_simultaneous_tasks: 1, // Matches Java default
            max_simultaneous_threads: num_cpus::get(),
            thread_check_interval: PerformanceConstants::THREAD_CHECK_INTERVAL_MS,
        }
    }
}

impl PerformanceThresholds {
    /// Create new PerformanceThresholds with specified threads and check interval
    /// Equivalent to Java's PerformanceThresholds(int, long) constructor
    pub fn new(max_simultaneous_threads: usize, thread_check_interval: u64) -> Self {
        Self {
            max_simultaneous_tasks: 1, // Default value from Java
            max_simultaneous_threads,
            thread_check_interval,
        }
    }

    /// Create new PerformanceThresholds with all parameters
    pub fn with_all_params(
        max_simultaneous_tasks: usize,
        max_simultaneous_threads: usize,
        thread_check_interval: u64,
    ) -> Self {
        Self {
            max_simultaneous_tasks,
            max_simultaneous_threads,
            thread_check_interval,
        }
    }

    /// Get thread check interval
    pub fn thread_check_interval(&self) -> u64 {
        self.thread_check_interval
    }

    /// Set thread check interval
    pub fn set_thread_check_interval(&mut self, interval: u64) {
        self.thread_check_interval = interval;
    }

    /// Get maximum simultaneous threads
    pub fn max_simultaneous_threads(&self) -> usize {
        self.max_simultaneous_threads
    }

    /// Set maximum simultaneous threads
    pub fn set_max_simultaneous_threads(&mut self, threads: usize) {
        self.max_simultaneous_threads = threads;
    }

    /// Get maximum simultaneous tasks
    pub fn max_simultaneous_tasks(&self) -> usize {
        self.max_simultaneous_tasks
    }

    /// Set maximum simultaneous tasks
    pub fn set_max_simultaneous_tasks(&mut self, tasks: usize) {
        self.max_simultaneous_tasks = tasks;
    }

    /// Validate performance thresholds
    pub fn validate(&self) -> Result<()> {
        if self.max_simultaneous_tasks == 0 {
            return Err(AppError::invalid_configuration("Maximum simultaneous tasks must be greater than 0"));
        }

        if self.max_simultaneous_threads == 0 {
            return Err(AppError::invalid_configuration("Maximum simultaneous threads must be greater than 0"));
        }

        if self.thread_check_interval == 0 {
            return Err(AppError::invalid_configuration("Thread check interval must be greater than 0"));
        }

        Ok(())
    }
}
