//! Task monitoring utilities
//! 
//! This module provides helper functions for monitoring task lifecycle,
//! status tracking, and progress reporting.

use crate::models::{TaskStatusResponse, enums::Status};

/// Task monitoring helper functions
pub struct TaskMonitor;

impl TaskMonitor {
    /// Create a task status response from task data
    pub fn create_status_response(
        status: Status,
        progress: u8,
    ) -> TaskStatusResponse {
        // TODO: Implement proper status response creation
        // This should include:
        // - Current status and progress
        // - Start/end times
        // - Performance metrics
        // - Error information if applicable
        
        TaskStatusResponse {
            status,
            percentage_done: progress,
            init_percentage: 0,
            solution: None,
        }
    }

    /// Check if a task status indicates completion
    pub fn is_task_completed(status: &Status) -> bool {
        matches!(status, Status::Finished | Status::Terminated | Status::Error)
    }

    /// Check if a task status indicates it's running
    pub fn is_task_running(status: &Status) -> bool {
        matches!(status, Status::Running | Status::Queued)
    }

    /// Calculate task progress percentage
    pub fn calculate_progress(completed_steps: u32, total_steps: u32) -> u8 {
        if total_steps == 0 {
            return 0;
        }
        
        let progress = (completed_steps as f64 / total_steps as f64 * 100.0) as u8;
        progress.min(100)
    }

    /// Check if task should be automatically cleaned up
    pub fn should_cleanup_task(status: &Status, elapsed_hours: u64) -> bool {
        // Clean up completed tasks after 24 hours
        Self::is_task_completed(status) && elapsed_hours >= 24
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_task_completed() {
        assert!(TaskMonitor::is_task_completed(&Status::Finished));
        assert!(TaskMonitor::is_task_completed(&Status::Terminated));
        assert!(TaskMonitor::is_task_completed(&Status::Error));
        assert!(!TaskMonitor::is_task_completed(&Status::Running));
        assert!(!TaskMonitor::is_task_completed(&Status::Queued));
    }

    #[test]
    fn test_is_task_running() {
        assert!(TaskMonitor::is_task_running(&Status::Running));
        assert!(TaskMonitor::is_task_running(&Status::Queued));
        assert!(!TaskMonitor::is_task_running(&Status::Finished));
        assert!(!TaskMonitor::is_task_running(&Status::Error));
    }

    #[test]
    fn test_calculate_progress() {
        assert_eq!(TaskMonitor::calculate_progress(0, 100), 0);
        assert_eq!(TaskMonitor::calculate_progress(50, 100), 50);
        assert_eq!(TaskMonitor::calculate_progress(100, 100), 100);
        assert_eq!(TaskMonitor::calculate_progress(150, 100), 100); // Capped at 100
        assert_eq!(TaskMonitor::calculate_progress(10, 0), 0); // Handle division by zero
    }

    #[test]
    fn test_should_cleanup_task() {
        assert!(TaskMonitor::should_cleanup_task(&Status::Finished, 25));
        assert!(!TaskMonitor::should_cleanup_task(&Status::Finished, 23));
        assert!(!TaskMonitor::should_cleanup_task(&Status::Running, 25));
    }
}
