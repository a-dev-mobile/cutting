//! Time management for Task struct
//! 
//! This module contains methods for managing task timing and duration tracking.

use std::time::{SystemTime, UNIX_EPOCH};
use super::Task;

impl Task {
    // ===== Time Management =====

    /// Get the start time as milliseconds since epoch
    pub fn start_time(&self) -> u64 {
        self.start_time
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Get the end time as milliseconds since epoch
    pub fn end_time(&self) -> u64 {
        self.end_time
            .lock()
            .unwrap()
            .map(|t| {
                t.duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64
            })
            .unwrap_or(0)
    }

    /// Set the end time to now
    pub(crate) fn set_end_time(&self) {
        *self.end_time.lock().unwrap() = Some(SystemTime::now());
    }

    /// Get elapsed time in milliseconds
    pub fn elapsed_time(&self) -> u64 {
        let end_time = self.end_time.lock().unwrap();
        let end = end_time.unwrap_or_else(SystemTime::now);
        end.duration_since(self.start_time)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Get last queried time as milliseconds since epoch
    pub fn last_queried(&self) -> u64 {
        self.last_queried
            .lock()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Set last queried time to now
    pub fn set_last_queried(&self) {
        *self.last_queried.lock().unwrap() = SystemTime::now();
    }
}
