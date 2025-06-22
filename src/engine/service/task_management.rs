//! Task lifecycle management operations
//! Handles task submission, status, control operations

/// Task monitoring utilities
pub mod monitoring {
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
    }
}

// Re-export for use in other modules
pub use monitoring::TaskMonitor;
