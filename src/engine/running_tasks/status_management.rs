//! Status management operations
//!
//! This module provides functionality for updating task statuses and managing
//! status-related counters in a thread-safe manner.

use crate::{
    errors::Result,
    models::enums::Status,
};
use std::sync::atomic::Ordering;
use crate::logging::debug;

use super::structs::RunningTasks;

/// Trait for status management operations
pub trait StatusManager {
    /// Update task status and adjust counters accordingly
    fn update_task_status(&self, task_id: &str, old_status: Status, new_status: Status) -> Result<()>;
    
    /// Increment finished thread counter
    fn increment_finished_threads(&self);
    
    /// Get service uptime in seconds
    fn get_uptime_seconds(&self) -> u64;
}

impl StatusManager for RunningTasks {
    /// Update task status and adjust counters accordingly
    fn update_task_status(&self, task_id: &str, old_status: Status, new_status: Status) -> Result<()> {
        if old_status != new_status {
            debug!("Updating task {} status from {:?} to {:?}", task_id, old_status, new_status);
            
            self.decrement_status_counter(old_status);
            self.increment_status_counter(new_status);
        }
        
        Ok(())
    }
    
    /// Increment finished thread counter
    fn increment_finished_threads(&self) {
        self.nbr_finished_threads.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Get service uptime in seconds
    fn get_uptime_seconds(&self) -> u64 {
        self.start_time
            .elapsed()
            .unwrap_or_default()
            .as_secs()
    }
}
