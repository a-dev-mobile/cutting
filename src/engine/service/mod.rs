//! CutList Optimizer Service Module
//!
//! This module provides a modular service implementation for the cutting optimization engine.
//! The service is organized into logical components for better maintainability and testing.
//!
//! # Module Structure
//!
//! - `traits` - Service trait definitions and extension traits
//! - `core` - Core service structure and basic functionality
//! - `task_lifecycle` - Task submission, stopping, and termination operations
//! - `status_monitoring` - Task status retrieval and monitoring operations
//! - `statistics` - Statistics gathering and health monitoring operations
//! - `lifecycle` - Service initialization, configuration, and shutdown operations
//! - `implementation` - Main trait implementation that ties everything together
//!
//! # Usage
//!
//! ```rust
//! use cutlist_optimizer_cli::engine::service::{CutListOptimizerService, CutListOptimizerServiceImpl};
//! use cutlist_optimizer_cli::models::CalculationRequest;
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut service = CutListOptimizerServiceImpl::new();
//!     
//!     // Initialize the service
//!     service.init(4).await?;
//!     
//!     // Submit a task
//!     let request = CalculationRequest::new();
//!     let result = service.submit_task(request).await?;
//!     
//!     // Monitor task progress
//!     if let Some(task_id) = result.task_id {
//!         while let Some(status) = service.get_task_status(&task_id).await? {
//!             if status.status == cutlist_optimizer_cli::models::enums::Status::Finished {
//!                 break;
//!             }
//!             tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
//!         }
//!     }
//!     
//!     // Shutdown the service
//!     service.shutdown().await?;
//!     
//!     Ok(())
//! }
//! ```

// Core modules
pub mod traits;
pub mod core;
pub mod decimal_places;
pub mod collection_utils;

// Operation modules
pub mod task_lifecycle;
pub mod status_monitoring;
pub mod statistics;
pub mod lifecycle;

// Implementation module
pub mod implementation;

// Re-export main types for convenience
pub use traits::{
    CutListOptimizerService, 
    CutListOptimizerServiceExt, 
    TaskDetails, 
    HealthStatus
};
pub use core::CutListOptimizerServiceImpl;

// Re-export utility types
pub use task_lifecycle::RequestValidator;
pub use status_monitoring::TaskMonitor;
pub use statistics::{StatsCollector, HealthMonitor};
pub use lifecycle::{ServiceInitializer, ServiceShutdown};
