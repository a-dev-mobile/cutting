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
