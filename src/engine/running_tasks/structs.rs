//! Core data structures for the running tasks service
//!
//! This module contains the main RunningTasks structure and related types
//! for managing optimization tasks in a thread-safe manner.

use crate::models::task::Task;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::{
    sync::{
        atomic::{AtomicI32, AtomicU64},
        Arc,
    },
    time::SystemTime,
};

/// Thread-safe container for managing running optimization tasks
/// 
/// This structure provides concurrent access to task storage and maintains
/// real-time statistics about task execution states.
#[derive(Debug)]
pub struct RunningTasks {
    /// Map of task_id -> Task for fast lookup
    pub(crate) tasks: DashMap<String, Arc<RwLock<Task>>>,
    
    /// Atomic counters for different task states
    pub(crate) nbr_idle_tasks: AtomicI32,
    pub(crate) nbr_running_tasks: AtomicI32,
    pub(crate) nbr_finished_tasks: AtomicI32,
    pub(crate) nbr_stopped_tasks: AtomicI32,
    pub(crate) nbr_terminated_tasks: AtomicI32,
    pub(crate) nbr_error_tasks: AtomicI32,
    
    /// Thread execution statistics
    pub(crate) nbr_finished_threads: AtomicU64,
    
    /// Service start time for uptime calculation
    pub(crate) start_time: SystemTime,
}

impl RunningTasks {
    /// Create a new RunningTasks instance
    pub fn new() -> Self {
        Self {
            tasks: DashMap::new(),
            nbr_idle_tasks: AtomicI32::new(0),
            nbr_running_tasks: AtomicI32::new(0),
            nbr_finished_tasks: AtomicI32::new(0),
            nbr_stopped_tasks: AtomicI32::new(0),
            nbr_terminated_tasks: AtomicI32::new(0),
            nbr_error_tasks: AtomicI32::new(0),
            nbr_finished_threads: AtomicU64::new(0),
            start_time: SystemTime::now(),
        }
    }
}

impl Default for RunningTasks {
    fn default() -> Self {
        Self::new()
    }
}
