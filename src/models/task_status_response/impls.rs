//! TaskStatusResponse implementation

use crate::models::{CalculationResponse, enums::Status};
use crate::errors::{AppError, Result};
use super::TaskStatusResponse;

impl TaskStatusResponse {
    /// Creates a new TaskStatusResponse with the specified status
    /// 
    /// # Arguments
    /// * `status` - The initial status of the task
    /// 
    /// # Returns
    /// A new TaskStatusResponse instance
    pub fn new(status: Status) -> Self {
        Self {
            status,
            percentage_done: 0,
            init_percentage: 0,
            solution: None,
        }
    }

    /// Creates a new TaskStatusResponse with all fields specified
    /// 
    /// # Arguments
    /// * `status` - The status of the task
    /// * `percentage_done` - Current completion percentage (0-100)
    /// * `init_percentage` - Initial percentage when task started
    /// * `solution` - Optional calculation result
    /// 
    /// # Returns
    /// A new TaskStatusResponse instance
    /// 
    /// # Errors
    /// Returns `AppError::invalid_input` if percentage values are invalid
    pub fn with_details(
        status: Status,
        percentage_done: u8,
        init_percentage: u8,
        solution: Option<CalculationResponse>,
    ) -> Result<Self> {
        if percentage_done > 100 {
            return Err(AppError::invalid_input(format!("percentage_done must be <= 100, got {}", percentage_done)));
        }
        
        if init_percentage > 100 {
            return Err(AppError::invalid_input(format!("init_percentage must be <= 100, got {}", init_percentage)));
        }

        Ok(Self {
            status,
            percentage_done,
            init_percentage,
            solution,
        })
    }

    /// Gets the current status of the task
    pub fn status(&self) -> Status {
        self.status
    }

    /// Sets the status of the task
    /// 
    /// # Arguments
    /// * `status` - The new status to set
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    /// Gets the current completion percentage
    pub fn percentage_done(&self) -> u8 {
        self.percentage_done
    }

    /// Sets the completion percentage
    /// 
    /// # Arguments
    /// * `percentage` - The completion percentage (0-100)
    /// 
    /// # Errors
    /// Returns `AppError::invalid_input` if percentage is > 100
    pub fn set_percentage_done(&mut self, percentage: u8) -> Result<()> {
        if percentage > 100 {
            return Err(AppError::invalid_input(format!("percentage_done must be <= 100, got {}", percentage)));
        }
        self.percentage_done = percentage;
        Ok(())
    }

    /// Gets the initial percentage
    pub fn init_percentage(&self) -> u8 {
        self.init_percentage
    }

    /// Sets the initial percentage
    /// 
    /// # Arguments
    /// * `percentage` - The initial percentage (0-100)
    /// 
    /// # Errors
    /// Returns `AppError::invalid_input` if percentage is > 100
    pub fn set_init_percentage(&mut self, percentage: u8) -> Result<()> {
        if percentage > 100 {
            return Err(AppError::invalid_input(format!("init_percentage must be <= 100, got {}", percentage)));
        }
        self.init_percentage = percentage;
        Ok(())
    }

    /// Gets a reference to the solution, if available
    pub fn solution(&self) -> Option<&CalculationResponse> {
        self.solution.as_ref()
    }

    /// Takes ownership of the solution, leaving None in its place
    pub fn take_solution(&mut self) -> Option<CalculationResponse> {
        self.solution.take()
    }

    /// Sets the solution
    /// 
    /// # Arguments
    /// * `solution` - The calculation response to set
    pub fn set_solution(&mut self, solution: CalculationResponse) {
        self.solution = Some(solution);
    }

    /// Clears the solution
    pub fn clear_solution(&mut self) {
        self.solution = None;
    }

    /// Checks if the task is completed (finished, terminated, or error)
    pub fn is_completed(&self) -> bool {
        matches!(self.status, Status::Finished | Status::Terminated | Status::Error)
    }

    /// Checks if the task is currently running
    pub fn is_running(&self) -> bool {
        self.status == Status::Running
    }

    /// Checks if the task is queued
    pub fn is_queued(&self) -> bool {
        self.status == Status::Queued
    }

    /// Checks if the task finished successfully
    pub fn is_successful(&self) -> bool {
        self.status == Status::Finished
    }

    /// Checks if the task has an error
    pub fn has_error(&self) -> bool {
        self.status == Status::Error
    }

    /// Updates the progress and optionally the status
    /// 
    /// # Arguments
    /// * `percentage` - New completion percentage
    /// * `status` - Optional new status
    /// 
    /// # Errors
    /// Returns `AppError::invalid_input` if percentage is > 100
    pub fn update_progress(&mut self, percentage: u8, status: Option<Status>) -> Result<()> {
        self.set_percentage_done(percentage)?;
        if let Some(new_status) = status {
            self.set_status(new_status);
        }
        Ok(())
    }

    /// Marks the task as completed with a solution
    /// 
    /// # Arguments
    /// * `solution` - The final calculation result
    pub fn complete_with_solution(&mut self, solution: CalculationResponse) {
        self.status = Status::Finished;
        self.percentage_done = 100;
        self.solution = Some(solution);
    }

    /// Marks the task as failed with error status
    pub fn mark_as_error(&mut self) {
        self.status = Status::Error;
        self.solution = None;
    }

    /// Marks the task as terminated
    pub fn mark_as_terminated(&mut self) {
        self.status = Status::Terminated;
        self.solution = None;
    }
}
