//! Task lifecycle management operations
//! 
//! This module handles task submission, status, control operations.
//! 
//! **Note**: Task monitoring utilities have been moved to `utilities::task_monitor`.
//! Use the following import instead:
//! ```rust
//! use crate::engine::service::utilities::TaskMonitor;
//! ```

/// Legacy task monitoring utilities - DEPRECATED
/// 
/// These utilities have been moved to `crate::engine::service::utilities::task_monitor`.
/// This module is kept for backward compatibility but will be removed in future versions.
pub mod monitoring {
    // Re-export from the new location
    pub use crate::engine::service::utilities::task_monitor::TaskMonitor;
}

// Re-export for backward compatibility
pub use monitoring::TaskMonitor;
