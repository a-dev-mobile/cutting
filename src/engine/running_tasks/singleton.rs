//! Singleton pattern implementation for RunningTasks
//!
//! This module provides thread-safe singleton access to the RunningTasks instance
//! using lazy initialization.

use std::sync::{Arc, OnceLock};

use super::structs::RunningTasks;

/// Trait for singleton access to task manager
pub trait TaskManagerSingleton {
    /// Get singleton instance (thread-safe lazy initialization)
    fn get_instance() -> &'static Arc<Self>;
}

impl TaskManagerSingleton for RunningTasks {
    /// Get singleton instance (thread-safe lazy initialization)
    fn get_instance() -> &'static Arc<Self> {
        static INSTANCE: OnceLock<Arc<RunningTasks>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::new()))
    }
}

/// Convenience function to get the singleton instance
pub fn get_running_tasks_instance() -> &'static Arc<RunningTasks> {
    RunningTasks::get_instance()
}
