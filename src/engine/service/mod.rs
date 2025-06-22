//! Service layer for the CutList Optimizer

pub mod trait_def;           // Trait definition
pub mod core;               // Only structure and utilities
pub mod trait_impl;         // Complete trait implementation
pub mod task_management;    // Task monitoring utilities
pub mod service_management; // Statistics collection utilities
pub mod computation;        // Computational logic
pub mod validation;         // Validation utilities

// Re-exports
pub use trait_def::CutListOptimizerService;
pub use core::CutListOptimizerServiceImpl;
pub use validation::RequestValidator;
pub use task_management::TaskMonitor;
pub use service_management::StatsCollector;
