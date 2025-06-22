//! Cut list optimization engine module
//!
//! This module contains the core optimization logic for cutting list calculations,
//! including the main computation thread and related utilities.

pub mod cut_list_thread;
pub mod comparator;
pub mod service;
pub mod running_tasks;
pub mod watch_dog;
pub mod stock;


pub use cut_list_thread::CutListThread;
pub use comparator::SolutionComparator;
pub use service::CutListOptimizerServiceImpl;
pub use running_tasks::{
    RunningTasks, 
    TaskManager, 
    StatusManager, 
    StatisticsCollector, 
    TaskCleanup as RunningTasksCleanup, 
    TaskManagerSingleton,
    get_running_tasks_instance,
};
pub use watch_dog::{WatchDog, WatchDogConfig, TaskMonitor, TaskCleanup as WatchDogTaskCleanup, WatchDogStatistics};

// Re-export stock module for public API compatibility
pub use stock::*;
