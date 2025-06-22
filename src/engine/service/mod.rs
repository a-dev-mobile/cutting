//! Service layer for the CutList Optimizer

pub mod trait_def;           // Trait definition
pub mod core;               // Core service structure
pub mod trait_impl;         // Complete trait implementation  
pub mod validation;         // Request validation utilities
pub mod computation;        // Computational logic
pub mod utilities;          // Helper utilities

// Legacy modules - kept for backward compatibility but not re-exported
// to avoid namespace pollution. Use full paths to access:
// use crate::engine::service::task_management::monitoring::TaskMonitor;
// use crate::engine::service::service_management::stats_collector::StatsCollector;
pub mod task_management;
pub mod service_management;

// Main re-exports - only essential types
pub use trait_def::CutListOptimizerService;
pub use core::CutListOptimizerServiceImpl;
pub use validation::RequestValidator;

// For utilities, use full paths to avoid namespace pollution:
// use crate::engine::service::utilities::{TaskMonitor, StatsCollector};
