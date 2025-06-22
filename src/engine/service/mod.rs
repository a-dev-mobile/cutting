//! Service layer for the CutList Optimizer
//!
//! This module provides the main service implementation for cut list optimization,
//! including task management, computation, and status monitoring.

pub mod core;
pub mod task_submission;
pub mod task_control;
pub mod statistics;
pub mod lifecycle;
pub mod implementation;
pub mod computation;

// Re-export main service implementation
pub use core::CutListOptimizerServiceImpl;
pub use implementation::*;
pub use task_submission::*;
pub use task_control::*;
pub use statistics::*;
pub use lifecycle::*;
pub use computation::*;
