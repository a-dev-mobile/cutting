//! Statistics and monitoring structures for the cutting optimization service
//!
//! This module provides comprehensive statistics tracking for tasks, threads,
//! and overall system performance monitoring.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Comprehensive statistics for the cutting optimization service
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Stats {
    // Task statistics
    pub nbr_idle_tasks: i32,
    pub nbr_running_tasks: i32,
    pub nbr_finished_tasks: i32,
    pub nbr_stopped_tasks: i32,
    pub nbr_terminated_tasks: i32,
    pub nbr_error_tasks: i32,
    
    // Thread statistics
    pub nbr_running_threads: i32,
    pub nbr_queued_threads: i32,
    pub nbr_finished_threads: i64,
    
    // Task reports for detailed monitoring
    pub task_reports: Vec<TaskReport>,
}

/// Individual task report for monitoring and debugging
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskReport {
    pub task_id: String,
    pub client_id: String,
    pub status: String,
    pub progress_percentage: i32,
    pub start_time: i64,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TaskReport {
    fn default() -> Self {
        Self::new()
    }
}

impl Stats {
    /// Create a new empty Stats instance
    pub fn new() -> Self {
        Self {
            nbr_idle_tasks: 0,
            nbr_running_tasks: 0,
            nbr_finished_tasks: 0,
            nbr_stopped_tasks: 0,
            nbr_terminated_tasks: 0,
            nbr_error_tasks: 0,
            nbr_running_threads: 0,
            nbr_queued_threads: 0,
            nbr_finished_threads: 0,
            task_reports: Vec::new(),
        }
    }

    /// Calculate total number of tasks across all states
    pub fn total_tasks(&self) -> i32 {
        self.nbr_idle_tasks
            + self.nbr_running_tasks
            + self.nbr_finished_tasks
            + self.nbr_stopped_tasks
            + self.nbr_terminated_tasks
            + self.nbr_error_tasks
    }

    /// Calculate total number of threads across all states
    pub fn total_threads(&self) -> i32 {
        self.nbr_running_threads + self.nbr_queued_threads
    }

    /// Check if the system is currently busy (has running tasks or threads)
    pub fn is_busy(&self) -> bool {
        self.nbr_running_tasks > 0 || self.nbr_running_threads > 0
    }

    /// Calculate success rate as percentage of finished vs total completed tasks
    pub fn success_rate(&self) -> f64 {
        let completed_tasks = self.nbr_finished_tasks + self.nbr_stopped_tasks + self.nbr_error_tasks;
        if completed_tasks == 0 {
            return 100.0; // No completed tasks yet, assume 100%
        }
        (self.nbr_finished_tasks as f64 / completed_tasks as f64) * 100.0
    }
}

impl TaskReport {
    /// Create a new empty TaskReport instance
    pub fn new() -> Self {
        Self {
            task_id: String::new(),
            client_id: String::new(),
            status: String::new(),
            progress_percentage: 0,
            start_time: 0,
            duration_ms: None,
            error_message: None,
        }
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Stats {{ tasks: {}/{} running, threads: {}/{} active, success_rate: {:.1}% }}",
            self.nbr_running_tasks,
            self.total_tasks(),
            self.nbr_running_threads,
            self.total_threads(),
            self.success_rate()
        )
    }
}

impl fmt::Display for TaskReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TaskReport {{ id: {}, client: {}, status: {}, progress: {}% }}",
            self.task_id, self.client_id, self.status, self.progress_percentage
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_default() {
        let stats = Stats::default();
        assert_eq!(stats.total_tasks(), 0);
        assert_eq!(stats.total_threads(), 0);
        assert!(!stats.is_busy());
        assert_eq!(stats.success_rate(), 100.0);
    }

    #[test]
    fn test_stats_calculations() {
        let stats = Stats {
            nbr_idle_tasks: 1,
            nbr_running_tasks: 2,
            nbr_finished_tasks: 3,
            nbr_stopped_tasks: 1,
            nbr_terminated_tasks: 0,
            nbr_error_tasks: 1,
            nbr_running_threads: 5,
            nbr_queued_threads: 3,
            nbr_finished_threads: 100,
            task_reports: vec![],
        };

        assert_eq!(stats.total_tasks(), 8);
        assert_eq!(stats.total_threads(), 8);
        assert!(stats.is_busy());
        assert_eq!(stats.success_rate(), 60.0); // 3 finished out of 5 completed (3+1+1)
    }

    #[test]
    fn test_stats_display() {
        let stats = Stats {
            nbr_running_tasks: 2,
            nbr_finished_tasks: 5,
            nbr_running_threads: 3,
            nbr_queued_threads: 1,
            ..Default::default()
        };

        let display = format!("{}", stats);
        assert!(display.contains("2/7 running")); // 2 running out of 7 total (2 running + 5 finished)
        assert!(display.contains("3/4 active"));   // 3 running out of 4 total threads (3 running + 1 queued)
    }
}
