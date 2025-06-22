//! Status management for Task struct
//! 
//! This module contains methods for managing task status transitions and validation.

use crate::{log_info, log_warn, log_error};
use crate::models::enums::Status;
use crate::errors::AppError;
use super::Task;

/// Helper function to update running tasks counters when status changes
fn update_running_tasks_counters(task_id: &str, old_status: Status, new_status: Status) {
    use crate::engine::running_tasks::get_running_tasks_instance;
    
    if old_status != new_status {
        let running_tasks = get_running_tasks_instance();
        running_tasks.decrement_status_counter(old_status);
        running_tasks.increment_status_counter(new_status);
        log_info!("Updated counters for task {}: {:?} -> {:?}", task_id, old_status, new_status);
    }
}

impl Task {
    // ===== Status Management =====

    /// Get the current status
    pub fn status(&self) -> Status {
        *self.status.read().unwrap()
    }

    /// Check if the task is currently running
    pub fn is_running(&self) -> bool {
        matches!(self.status(), Status::Running)
    }

    /// Set the task status to running
    /// Returns Ok(()) if successful, Err if task is not in a valid state to start
    pub fn set_running_status(&self) -> Result<(), AppError> {
        let mut status = self.status.write().unwrap();
        let old_status = *status;
        if old_status != Status::Queued {
            return Err(AppError::Task(crate::errors::TaskError::InvalidStatusTransition {
                from: old_status,
                to: Status::Running,
            }));
        }
        *status = Status::Running;
        drop(status); // Release lock before calling update function
        update_running_tasks_counters(&self.id, old_status, Status::Running);
        log_info!("Task {} set to running status", self.id);
        Ok(())
    }

    /// Stop the task
    /// Returns Ok(()) if successful, Err if task is not running
    pub fn stop(&self) -> Result<(), AppError> {
        let mut status = self.status.write().unwrap();
        let old_status = *status;
        if old_status != Status::Running {
            return Err(AppError::Task(crate::errors::TaskError::InvalidStatusTransition {
                from: old_status,
                to: Status::Finished, // Assuming stop means finished
            }));
        }
        *status = Status::Finished;
        drop(status); // Release lock before calling update function
        update_running_tasks_counters(&self.id, old_status, Status::Finished);
        self.set_end_time();
        log_info!("Task {} stopped", self.id);
        Ok(())
    }

    /// Terminate the task
    /// Returns Ok(()) if successful, Err if task is not running
    pub fn terminate(&self) -> Result<(), AppError> {
        let mut status = self.status.write().unwrap();
        let old_status = *status;
        if old_status != Status::Running {
            return Err(AppError::Task(crate::errors::TaskError::InvalidStatusTransition {
                from: old_status,
                to: Status::Terminated,
            }));
        }
        *status = Status::Terminated;
        drop(status); // Release lock before calling update function
        update_running_tasks_counters(&self.id, old_status, Status::Terminated);
        self.set_end_time();
        log_warn!("Task {} terminated", self.id);
        Ok(())
    }

    /// Set the task status to error
    pub fn terminate_error(&self) {
        let mut status = self.status.write().unwrap();
        let old_status = *status;
        *status = Status::Error;
        drop(status); // Release lock before calling update function
        update_running_tasks_counters(&self.id, old_status, Status::Error);
        self.set_end_time();
        log_error!("Task {} terminated with error", self.id);
    }

    /// Check if all materials are finished and update status accordingly
    pub fn check_if_finished(&self) {
        if matches!(self.status(), Status::Finished) {
            return;
        }

        let percentages = self.per_material_percentage_done.lock().unwrap();
        let all_finished = percentages.values().all(|&p| p == 100);

        if all_finished {
            let mut status = self.status.write().unwrap();
            let old_status = *status;
            *status = Status::Finished;
            drop(status); // Release the lock before calling update function
            update_running_tasks_counters(&self.id, old_status, Status::Finished);
            self.set_end_time();
            
            if self.solution.read().unwrap().is_none() {
                // Build solution if not already built
                if let Some(solution) = self.build_solution() {
                    *self.solution.write().unwrap() = Some(solution);
                }
            }
            
            log_info!("Task {} finished", self.id);
        }
    }
}
