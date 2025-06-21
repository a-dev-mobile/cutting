//! CalculationSubmissionResult implementation

use super::CalculationSubmissionResult;
use crate::models::enums::StatusCode;

impl CalculationSubmissionResult {
    /// Create a new CalculationSubmissionResult with both status code and task ID
    /// 
    /// # Arguments
    /// * `status_code` - The status code of the submission
    /// * `task_id` - The task identifier for tracking
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};
    /// 
    /// let result = CalculationSubmissionResult::new(StatusCode::Ok, "task_123");
    /// assert_eq!(result.status_code, StatusCode::Ok);
    /// assert_eq!(result.task_id, Some("task_123".to_string()));
    /// ```
    pub fn new(status_code: StatusCode, task_id: impl Into<String>) -> Self {
        Self {
            status_code,
            task_id: Some(task_id.into()),
        }
    }
    
    /// Create a new CalculationSubmissionResult with only a status code
    /// 
    /// # Arguments
    /// * `status_code` - The status code of the submission
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};
    /// 
    /// let result = CalculationSubmissionResult::with_status(StatusCode::ServerUnavailable);
    /// assert_eq!(result.status_code, StatusCode::ServerUnavailable);
    /// assert_eq!(result.task_id, None);
    /// ```
    pub fn with_status(status_code: StatusCode) -> Self {
        Self {
            status_code,
            task_id: None,
        }
    }
    
    /// Create a successful result with a task ID
    /// 
    /// # Arguments
    /// * `task_id` - The task identifier for tracking
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};
    /// 
    /// let result = CalculationSubmissionResult::success("task_456");
    /// assert_eq!(result.status_code, StatusCode::Ok);
    /// assert_eq!(result.task_id, Some("task_456".to_string()));
    /// ```
    pub fn success(task_id: impl Into<String>) -> Self {
        Self::new(StatusCode::Ok, task_id)
    }
    
    /// Create a failed result with an error status code
    /// 
    /// # Arguments
    /// * `status_code` - The error status code
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};
    /// 
    /// let result = CalculationSubmissionResult::error(StatusCode::InvalidTiles);
    /// assert_eq!(result.status_code, StatusCode::InvalidTiles);
    /// assert_eq!(result.task_id, None);
    /// ```
    pub fn error(status_code: StatusCode) -> Self {
        Self::with_status(status_code)
    }
    
    /// Check if the submission was successful
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};
    /// 
    /// let success = CalculationSubmissionResult::success("task_123");
    /// assert!(success.is_success());
    /// 
    /// let error = CalculationSubmissionResult::error(StatusCode::InvalidTiles);
    /// assert!(!error.is_success());
    /// ```
    pub fn is_success(&self) -> bool {
        self.status_code.is_ok()
    }
    
    /// Check if the submission failed
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};
    /// 
    /// let success = CalculationSubmissionResult::success("task_123");
    /// assert!(!success.is_error());
    /// 
    /// let error = CalculationSubmissionResult::error(StatusCode::InvalidTiles);
    /// assert!(error.is_error());
    /// ```
    pub fn is_error(&self) -> bool {
        self.status_code.is_error()
    }
    
    /// Get the task ID if present
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};
    /// 
    /// let result = CalculationSubmissionResult::success("task_123");
    /// assert_eq!(result.get_task_id(), Some("task_123"));
    /// 
    /// let error = CalculationSubmissionResult::error(StatusCode::InvalidTiles);
    /// assert_eq!(error.get_task_id(), None);
    /// ```
    pub fn get_task_id(&self) -> Option<&str> {
        self.task_id.as_deref()
    }
    
    /// Get the status code
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};
    /// 
    /// let result = CalculationSubmissionResult::success("task_123");
    /// assert_eq!(result.get_status_code(), StatusCode::Ok);
    /// ```
    pub fn get_status_code(&self) -> StatusCode {
        self.status_code
    }
    
    /// Set the task ID
    /// 
    /// # Arguments
    /// * `task_id` - The new task identifier
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};
    /// 
    /// let mut result = CalculationSubmissionResult::with_status(StatusCode::Ok);
    /// result.set_task_id("new_task_id");
    /// assert_eq!(result.task_id, Some("new_task_id".to_string()));
    /// ```
    pub fn set_task_id(&mut self, task_id: impl Into<String>) {
        self.task_id = Some(task_id.into());
    }
    
    /// Clear the task ID
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};
    /// 
    /// let mut result = CalculationSubmissionResult::success("task_123");
    /// result.clear_task_id();
    /// assert_eq!(result.task_id, None);
    /// ```
    pub fn clear_task_id(&mut self) {
        self.task_id = None;
    }
    
    /// Set the status code
    /// 
    /// # Arguments
    /// * `status_code` - The new status code
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::models::{CalculationSubmissionResult, enums::StatusCode};
    /// 
    /// let mut result = CalculationSubmissionResult::success("task_123");
    /// result.set_status_code(StatusCode::TaskAlreadyRunning);
    /// assert_eq!(result.status_code, StatusCode::TaskAlreadyRunning);
    /// ```
    pub fn set_status_code(&mut self, status_code: StatusCode) {
        self.status_code = status_code;
    }
}

impl Default for CalculationSubmissionResult {
    /// Create a default CalculationSubmissionResult with Ok status and no task ID
    fn default() -> Self {
        Self::with_status(StatusCode::Ok)
    }
}

impl std::fmt::Display for CalculationSubmissionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.task_id {
            Some(task_id) => write!(f, "Status: {}, Task ID: {}", self.status_code, task_id),
            None => write!(f, "Status: {}", self.status_code),
        }
    }
}
