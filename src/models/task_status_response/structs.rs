//! TaskStatusResponse structure definition

use serde::{Deserialize, Serialize};
use crate::models::{CalculationResponse, enums::Status};

/// Response structure containing the status and progress of a cutting optimization task
/// 
/// This structure represents the current state of a task execution, including
/// progress percentages, status information, and the final solution when available.
/// 
/// # Key Differences from Java Version
/// - Uses `Status` enum instead of `String` for type safety
/// - Uses `Option<CalculationResponse>` instead of nullable reference
/// - Follows Rust naming conventions (snake_case)
/// - Implements common Rust traits for better integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatusResponse {
    /// Current status of the task (Queued, Running, Finished, Terminated, Error)
    pub status: Status,
    
    /// Percentage of work completed (0-100)
    pub percentage_done: u8,
    
    /// Initial percentage when task started (typically 0)
    pub init_percentage: u8,
    
    /// The final calculation result, available when task is finished
    pub solution: Option<CalculationResponse>,
}

impl Default for TaskStatusResponse {
    fn default() -> Self {
        Self {
            status: Status::default(),
            percentage_done: 0,
            init_percentage: 0,
            solution: None,
        }
    }
}
