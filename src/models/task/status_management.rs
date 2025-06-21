//! Status management for Task struct
//! 
//! This module contains methods for managing task status transitions and validation.

use crate::{log_info, log_warn, log_error};
use crate::models::enums::Status;
use crate::error::TaskError;
use super::Task;

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
    pub fn set_running_status(&self) -> Result<(), TaskError> {
        let mut status = self.status.write().unwrap();
        if *status != Status::Queued {
            return Err(TaskError::InvalidStatusTransition {
                from: *status,
                to: Status::Running,
            });
        }
        *status = Status::Running;
        log_info!("Task {} set to running status", self.id);
        Ok(())
    }

    /// Stop the task
    /// Returns Ok(()) if successful, Err if task is not running
    pub fn stop(&self) -> Result<(), TaskError> {
        let mut status = self.status.write().unwrap();
        if *status != Status::Running {
            return Err(TaskError::InvalidStatusTransition {
                from: *status,
                to: Status::Finished, // Assuming stop means finished
            });
        }
        *status = Status::Finished;
        self.set_end_time();
        log_info!("Task {} stopped", self.id);
        Ok(())
    }

    /// Terminate the task
    /// Returns Ok(()) if successful, Err if task is not running
    pub fn terminate(&self) -> Result<(), TaskError> {
        let mut status = self.status.write().unwrap();
        if *status != Status::Running {
            return Err(TaskError::InvalidStatusTransition {
                from: *status,
                to: Status::Terminated,
            });
        }
        *status = Status::Terminated;
        self.set_end_time();
        log_warn!("Task {} terminated", self.id);
        Ok(())
    }

    /// Set the task status to error
    pub fn terminate_error(&self) {
        let mut status = self.status.write().unwrap();
        *status = Status::Error;
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
            *status = Status::Finished;
            self.set_end_time();
            
            if self.solution.read().unwrap().is_none() {
                // Build solution if not already built
                drop(status); // Release the lock before calling build_solution
                if let Some(solution) = self.build_solution() {
                    *self.solution.write().unwrap() = Some(solution);
                }
            }
            
            log_info!("Task {} finished", self.id);
        }
    }
}
