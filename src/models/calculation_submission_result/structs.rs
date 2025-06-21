//! CalculationSubmissionResult structure definition

use serde::{Deserialize, Serialize};
use crate::models::enums::StatusCode;

/// Result of a calculation submission request
/// 
/// This structure represents the response from submitting a calculation task,
/// containing the status of the submission and an optional task identifier
/// for tracking the submitted calculation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalculationSubmissionResult {
    /// Status code indicating the result of the submission
    pub status_code: StatusCode,
    
    /// Optional task identifier for tracking the submitted calculation
    /// None if the submission failed or no task was created
    pub task_id: Option<String>,
}
