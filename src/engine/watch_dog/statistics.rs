//! Statistics functionality for WatchDog service

use std::sync::Arc;
use crate::logging::debug;
use crate::engine::running_tasks::{RunningTasks, StatisticsCollector, TaskManager};
use crate::models::stats::Stats;
use crate::models::enums::Status;

/// WatchDog statistics component
pub struct WatchDogStatistics {
    running_tasks: Arc<RunningTasks>,
}

impl WatchDogStatistics {
    /// Creates a new WatchDogStatistics
    pub fn new(running_tasks: Arc<RunningTasks>) -> Self {
        Self { running_tasks }
    }

    /// Logs current task statistics
    pub fn log_statistics(&self) {
        let stats = self.running_tasks.get_stats();
        let total_tasks = self.calculate_total_tasks(&stats);
        
        if total_tasks > 0 {
            debug!(
                "WatchDog stats - Total: {}, Running: {}, Finished: {}, Error: {}, Idle: {}, Terminated: {}",
                total_tasks,
                stats.nbr_running_tasks,
                stats.nbr_finished_tasks,
                stats.nbr_error_tasks,
                stats.nbr_idle_tasks,
                stats.nbr_terminated_tasks
            );
        }
    }

    /// Gets detailed statistics
    pub fn get_detailed_statistics(&self) -> WatchDogDetailedStats {
        let stats = self.running_tasks.get_stats();
        let total_tasks = self.calculate_total_tasks(&stats);
        
        WatchDogDetailedStats {
            total_tasks,
            running_tasks: stats.nbr_running_tasks as u32,
            finished_tasks: stats.nbr_finished_tasks as u32,
            error_tasks: stats.nbr_error_tasks as u32,
            idle_tasks: stats.nbr_idle_tasks as u32,
            terminated_tasks: stats.nbr_terminated_tasks as u32,
            active_tasks: (stats.nbr_running_tasks + stats.nbr_idle_tasks) as u32,
            completed_tasks: (stats.nbr_finished_tasks + stats.nbr_error_tasks + stats.nbr_terminated_tasks) as u32,
        }
    }

    /// Gets a summary of task statistics
    pub fn get_summary(&self) -> WatchDogSummary {
        let detailed = self.get_detailed_statistics();
        
        WatchDogSummary {
            total_tasks: detailed.total_tasks,
            active_tasks: detailed.active_tasks,
            completed_tasks: detailed.completed_tasks,
            success_rate: if detailed.total_tasks > 0 {
                (detailed.finished_tasks as f64 / detailed.total_tasks as f64) * 100.0
            } else {
                0.0
            },
            error_rate: if detailed.total_tasks > 0 {
                (detailed.error_tasks as f64 / detailed.total_tasks as f64) * 100.0
            } else {
                0.0
            },
        }
    }

    /// Calculates the total number of tasks
    fn calculate_total_tasks(&self, stats: &Stats) -> u32 {
        (stats.nbr_idle_tasks + 
        stats.nbr_running_tasks + 
        stats.nbr_finished_tasks + 
        stats.nbr_terminated_tasks + 
        stats.nbr_error_tasks) as u32
    }

    /// Checks if the system is healthy based on error rates
    pub fn is_system_healthy(&self, max_error_rate: f64) -> bool {
        let summary = self.get_summary();
        summary.error_rate <= max_error_rate
    }

    /// Gets tasks that might need attention (long-running tasks)
    pub fn get_attention_needed_tasks(&self, max_running_time_hours: f64) -> Vec<String> {
        let tasks = self.running_tasks.get_tasks();
        let mut attention_tasks = Vec::new();
        let now = std::time::SystemTime::now();
        let max_duration = std::time::Duration::from_secs_f64(max_running_time_hours * 3600.0);
        
        for task_arc in tasks {
            let task = task_arc.read();
            let status = *task.status.read().unwrap();
            
            // Only check running tasks
            if matches!(status, Status::Running) {
                if let Ok(elapsed) = now.duration_since(task.start_time) {
                    if elapsed > max_duration {
                        attention_tasks.push(task.id.clone());
                    }
                }
            }
        }
        
        attention_tasks
    }
}

/// Detailed statistics for WatchDog
#[derive(Debug, Clone)]
pub struct WatchDogDetailedStats {
    pub total_tasks: u32,
    pub running_tasks: u32,
    pub finished_tasks: u32,
    pub error_tasks: u32,
    pub idle_tasks: u32,
    pub terminated_tasks: u32,
    pub active_tasks: u32,
    pub completed_tasks: u32,
}

/// Summary statistics for WatchDog
#[derive(Debug, Clone)]
pub struct WatchDogSummary {
    pub total_tasks: u32,
    pub active_tasks: u32,
    pub completed_tasks: u32,
    pub success_rate: f64,
    pub error_rate: f64,
}

impl WatchDogSummary {
    /// Returns a human-readable string representation
    pub fn to_string(&self) -> String {
        format!(
            "Total: {}, Active: {}, Completed: {}, Success Rate: {:.1}%, Error Rate: {:.1}%",
            self.total_tasks,
            self.active_tasks,
            self.completed_tasks,
            self.success_rate,
            self.error_rate
        )
    }
}
