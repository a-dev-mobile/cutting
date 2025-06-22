//! Core data structures for the running tasks service
//!
//! This module contains the main RunningTasks structure and related types
//! for managing optimization tasks in a thread-safe manner.

use crate::models::{task::Task, Stats};
use dashmap::DashMap;
use parking_lot::RwLock;
use std::{
    sync::{
        atomic::{AtomicI32, AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::SystemTime,
};

/// Task execution statistics for monitoring
#[derive(Debug, Default)]
pub struct TaskStats {
    pub total_tasks: u64,
    pub queued_tasks: u64,
    pub running_tasks: u64,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
}

/// Thread-safe container for managing running optimization tasks
/// 
/// This structure provides concurrent access to task storage and maintains
/// real-time statistics about task execution states.
#[derive(Debug)]
pub struct RunningTasks {
    /// Map of task_id -> Task for fast lookup (using DashMap for better concurrency)
    pub(crate) tasks: DashMap<String, Arc<RwLock<Task>>>,
    
    /// Atomic counters for different task states (optimized for performance)
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
    
    /// Task execution statistics (protected by mutex for complex operations)
    pub(crate) stats: Mutex<TaskStats>,
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
            stats: Mutex::new(TaskStats::default()),
        }
    }

    /// Get current task statistics (optimized for frequent access)
    pub fn get_task_counts(&self) -> (i32, i32, i32, i32, i32, i32) {
        (
            self.nbr_idle_tasks.load(Ordering::Relaxed),
            self.nbr_running_tasks.load(Ordering::Relaxed),
            self.nbr_finished_tasks.load(Ordering::Relaxed),
            self.nbr_stopped_tasks.load(Ordering::Relaxed),
            self.nbr_terminated_tasks.load(Ordering::Relaxed),
            self.nbr_error_tasks.load(Ordering::Relaxed),
        )
    }

    /// Update task count atomically when status changes
    pub fn update_task_count(&self, old_status: Option<crate::models::enums::Status>, new_status: crate::models::enums::Status) {
        use crate::models::enums::Status;
        
        // Decrement old status count
        if let Some(old) = old_status {
            match old {
                Status::Queued => self.nbr_idle_tasks.fetch_sub(1, Ordering::Relaxed),
                Status::Running => self.nbr_running_tasks.fetch_sub(1, Ordering::Relaxed),
                Status::Finished => self.nbr_finished_tasks.fetch_sub(1, Ordering::Relaxed),
                Status::Stopped => self.nbr_stopped_tasks.fetch_sub(1, Ordering::Relaxed),
                Status::Terminated => self.nbr_terminated_tasks.fetch_sub(1, Ordering::Relaxed),
                Status::Error => self.nbr_error_tasks.fetch_sub(1, Ordering::Relaxed),
            };
        }
        
        // Increment new status count
        match new_status {
            Status::Queued => self.nbr_idle_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Running => self.nbr_running_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Finished => self.nbr_finished_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Stopped => self.nbr_stopped_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Terminated => self.nbr_terminated_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Error => self.nbr_error_tasks.fetch_add(1, Ordering::Relaxed),
        };
    }

    /// Get comprehensive statistics for monitoring
    pub fn get_comprehensive_stats(&self) -> Stats {
        let (idle, running, finished, stopped, terminated, error) = self.get_task_counts();
        
        Stats {
            nbr_idle_tasks: idle,
            nbr_running_tasks: running,
            nbr_finished_tasks: finished,
            nbr_stopped_tasks: stopped,
            nbr_terminated_tasks: terminated,
            nbr_error_tasks: error,
            nbr_running_threads: 0, // Will be updated by thread pool
            nbr_queued_threads: 0,  // Will be updated by thread pool
            nbr_finished_threads: self.nbr_finished_threads.load(Ordering::Relaxed) as i64,
            task_reports: Vec::new(), // Will be populated by watch dog
        }
    }
}

impl Default for RunningTasks {
    fn default() -> Self {
        Self::new()
    }
}
