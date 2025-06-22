//! Statistics collection and reporting
//!
//! This module provides functionality for collecting and reporting statistics
//! about running tasks, including task reports and system metrics.

use crate::models::{
    stats::{Stats, TaskReport},
    enums::Status,
};
use std::sync::atomic::Ordering;

use super::structs::RunningTasks;

/// Trait for statistics collection operations
pub trait StatisticsCollector {
    /// Get current statistics
    fn get_stats(&self) -> Stats;
    
    /// Get task count by status
    fn get_task_count_by_status(&self, status: Status) -> i32;
    
    /// Get total task count
    fn get_total_task_count(&self) -> usize;
    
    /// Get finished threads count
    fn get_finished_threads_count(&self) -> u64;
}

impl StatisticsCollector for RunningTasks {
    /// Get current statistics
    fn get_stats(&self) -> Stats {
        let mut task_reports = Vec::new();
        
        // Collect task reports
        for entry in self.tasks.iter() {
            let task_id = entry.key().clone();
            let task = entry.value().read();
            
            let status = *task.status.read().unwrap();
            let start_time = task.start_time;
            let running_time_ms = start_time
                .elapsed()
                .unwrap_or_default()
                .as_millis() as i64;
            
            let client_id = "unknown".to_string(); // Client info removed as requested
            
            let progress_percentage = task.percentage_done();
            
            let report = TaskReport {
                task_id: task_id.clone(),
                client_id,
                status: format!("{:?}", status),
                progress_percentage,
                start_time: start_time
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64,
                duration_ms: Some(running_time_ms),
                error_message: None,
            };
            
            task_reports.push(report);
        }
        
        Stats {
            nbr_idle_tasks: self.nbr_idle_tasks.load(Ordering::Relaxed),
            nbr_running_tasks: self.nbr_running_tasks.load(Ordering::Relaxed),
            nbr_finished_tasks: self.nbr_finished_tasks.load(Ordering::Relaxed),
            nbr_stopped_tasks: self.nbr_stopped_tasks.load(Ordering::Relaxed),
            nbr_terminated_tasks: self.nbr_terminated_tasks.load(Ordering::Relaxed),
            nbr_error_tasks: self.nbr_error_tasks.load(Ordering::Relaxed),
            nbr_running_threads: 0, // Will be set by service implementation
            nbr_queued_threads: 0,  // Will be set by service implementation
            nbr_finished_threads: self.nbr_finished_threads.load(Ordering::Relaxed) as i64,
            task_reports,
        }
    }
    
    /// Get task count by status
    fn get_task_count_by_status(&self, status: Status) -> i32 {
        match status {
            Status::Queued => self.nbr_idle_tasks.load(Ordering::Relaxed),
            Status::Running => self.nbr_running_tasks.load(Ordering::Relaxed),
            Status::Finished => self.nbr_finished_tasks.load(Ordering::Relaxed),
            Status::Stopped => self.nbr_stopped_tasks.load(Ordering::Relaxed),
            Status::Terminated => self.nbr_terminated_tasks.load(Ordering::Relaxed),
            Status::Error => self.nbr_error_tasks.load(Ordering::Relaxed),
        }
    }
    
    /// Get total task count
    fn get_total_task_count(&self) -> usize {
        self.tasks.len()
    }
    
    /// Get finished threads count
    fn get_finished_threads_count(&self) -> u64 {
        self.nbr_finished_threads.load(Ordering::Relaxed)
    }
}
