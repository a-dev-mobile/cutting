//! Thread management for Task struct
//! 
//! This module contains methods for managing CutListThread instances and their status.

use std::sync::{Arc, Mutex};
use crate::models::enums::Status;
use crate::engine::cut_list_thread::CutListThread;
use super::Task;

impl Task {
    // ===== Thread Management =====

    /// Add a thread to the task
    pub fn add_thread(&self, thread: Arc<Mutex<CutListThread>>) {
        let mut threads = self.threads.lock().unwrap();
        threads.push(thread);
    }

    /// Get number of running threads
    pub fn nbr_running_threads(&self) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Running)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get number of queued threads
    pub fn nbr_queued_threads(&self) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Queued)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get number of finished threads
    pub fn nbr_finished_threads(&self) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Finished)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get number of finished threads for a specific material
    pub fn nbr_finished_threads_for_material(&self, material: &str) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Finished) && 
                    t.material().map_or(false, |m| m == material)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get number of terminated threads
    pub fn nbr_terminated_threads(&self) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Terminated)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get number of error threads
    pub fn nbr_error_threads(&self) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Error)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get maximum thread progress percentage
    pub fn max_thread_progress_percentage(&self) -> i32 {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter_map(|thread| {
                thread.lock().ok().map(|t| t.percentage_done())
            })
            .max()
            .unwrap_or(0)
    }

    /// Get total number of threads
    pub fn nbr_total_threads(&self) -> usize {
        self.threads.lock().unwrap().len()
    }
}
