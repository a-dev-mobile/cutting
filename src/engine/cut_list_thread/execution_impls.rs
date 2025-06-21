//! Execution implementations for CutListThread
//! 
//! This module contains thread execution logic and runnable behavior.

use crate::{
    log_error, log_info,
    Status,
};

use super::structs::CutListThread;

// Implement Runnable-like behavior for threading
impl CutListThread {
    /// Run the thread computation (equivalent to Java's run() method)
    pub fn run(&mut self) {
        log_info!("Starting CutListThread execution for group: {:?}", self.group);
        
        // Validate configuration before starting
        if let Err(e) = self.validate_configuration() {
            self.status = Status::Error;
            log_error!("Configuration validation failed for group: {:?} - Error: {}", self.group, e);
            return;
        }

        match self.compute_solutions() {
            Ok(()) => {
                if self.status != Status::Terminated {
                    self.status = Status::Finished;
                }
                log_info!("Thread completed successfully for group: {:?}", self.group);
            }
            Err(e) => {
                self.status = Status::Error;
                log_error!("Thread failed for group: {:?} - Error: {}", self.group, e);
            }
        }
    }

    /// Terminate the thread execution
    pub fn terminate(&mut self) {
        self.status = Status::Terminated;
        log_info!("Thread terminated for group: {:?}", self.group);
    }

    /// Check if the thread is running
    pub fn is_running(&self) -> bool {
        matches!(self.status, Status::Running)
    }

    /// Check if the thread is finished
    pub fn is_finished(&self) -> bool {
        matches!(self.status, Status::Finished)
    }

    /// Check if the thread has an error
    pub fn has_error(&self) -> bool {
        matches!(self.status, Status::Error)
    }

    /// Check if the thread is terminated
    pub fn is_terminated(&self) -> bool {
        matches!(self.status, Status::Terminated)
    }
}
