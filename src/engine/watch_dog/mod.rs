//! WatchDog service for monitoring and managing running tasks
//! 
//! This module provides functionality to monitor running tasks, handle timeouts,
//! and perform cleanup operations for stale or completed tasks.

pub mod core;
pub mod config;
pub mod monitoring;
pub mod cleanup;
pub mod statistics;

pub use core::WatchDog;
pub use config::WatchDogConfig;
pub use monitoring::TaskMonitor;
pub use cleanup::TaskCleanup;
pub use statistics::WatchDogStatistics;
