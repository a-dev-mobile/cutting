//! Cut list optimization engine module
//!
//! This module contains the core optimization logic for cutting list calculations,
//! including the main computation thread and related utilities.

pub mod cut_list_thread;
pub mod service;
pub mod service_impl;
pub mod running_tasks;
pub mod watch_dog;
pub mod stock;


pub use cut_list_thread::{CutListThread, SolutionComparator};
pub use service::CutListOptimizerService;
pub use service_impl::CutListOptimizerServiceImpl;
pub use running_tasks::RunningTasks;
pub use watch_dog::WatchDog;

// Re-export stock module for public API compatibility
pub use stock::*;
