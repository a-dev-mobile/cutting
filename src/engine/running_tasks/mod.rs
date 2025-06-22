//! Running tasks management service
//!
//! This module provides a comprehensive task management system with thread-safe
//! operations, status tracking, and statistics collection.

pub mod structs;
pub mod task_management;
pub mod status_management;
pub mod statistics;
pub mod cleanup;
pub mod singleton;

// Re-export the main struct and key types
pub use structs::RunningTasks;
pub use task_management::TaskManager;
pub use status_management::StatusManager;
pub use statistics::StatisticsCollector;
pub use cleanup::TaskCleanup;
pub use singleton::{TaskManagerSingleton, get_running_tasks_instance};
