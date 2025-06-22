//! Task submission operations
//!
//! This module handles task submission and all related validation logic.

use crate::{
    errors::Result,
    models::{
        CalculationRequest, CalculationSubmissionResult,
        enums::StatusCode,
    },
    logging::macros::{error},
};

use super::{
    core::{CutListOptimizerServiceImpl, MAX_PANELS_LIMIT, MAX_STOCK_PANELS_LIMIT},
    computation::main_compute,
};

/// Task submission operations implementation
impl CutListOptimizerServiceImpl {
    /// Submit a new optimization task for processing
    pub async fn submit_task_impl(&self, request: CalculationRequest) -> Result<CalculationSubmissionResult> {
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // Validate request
        if let Some(error_code) = RequestValidator::validate_request(&request).await {
            return Ok(CalculationSubmissionResult {
                status_code: error_code,
                task_id: None,
            });
        }

        // Check if multiple tasks per client are allowed
        if !self.get_allow_multiple_tasks_per_client() {
            // TODO: Add client_info field to CalculationRequest or remove this check
            // For now, skip client task limit check
        }

        // Generate task ID using date format + counter (like Java implementation)
        let task_id = self.generate_task_id();

        // Start computation in background
        let request_clone = request.clone();
        let task_id_clone = task_id.clone();
        
        tokio::spawn(async move {
            if let Err(e) = main_compute::compute_task(request_clone, task_id_clone).await {
                error!("Task computation failed: {}", e);
            }
        });

        Ok(CalculationSubmissionResult {
            status_code: StatusCode::Ok,
            task_id: Some(task_id),
        })
    }

    /// Check client task limit
    async fn check_client_task_limit(&self, client_id: &str) -> Result<Option<StatusCode>> {
        // TODO: Implement with running tasks manager
        // For now, allow all tasks
        let _ = client_id;
        Ok(None)
    }


}

/// Request validation utilities
mod validation {
    use crate::models::{CalculationRequest, enums::StatusCode};
    use super::{MAX_PANELS_LIMIT, MAX_STOCK_PANELS_LIMIT};

    pub struct RequestValidator;

    impl RequestValidator {
        /// Validate a calculation request (migrated from Java)
        pub async fn validate_request(request: &CalculationRequest) -> Option<StatusCode> {
            // Count valid panels
            let panel_count: usize = request.panels.iter()
                .filter(|p| p.is_valid().unwrap_or(false))
                .map(|p| p.count as usize)
                .sum();

            if panel_count == 0 {
                return Some(StatusCode::InvalidTiles);
            }

            if panel_count > MAX_PANELS_LIMIT {
                return Some(StatusCode::TooManyPanels);
            }

            // Count valid stock panels
            let stock_count: usize = request.stock_panels.iter()
                .filter(|p| p.is_valid().unwrap_or(false))
                .map(|p| p.count as usize)
                .sum();

            if stock_count == 0 {
                return Some(StatusCode::InvalidStockTiles);
            }

            if stock_count > MAX_STOCK_PANELS_LIMIT {
                return Some(StatusCode::TooManyStockPanels);
            }

            None // Request is valid
        }
    }
}

// Re-export for use in other modules
pub use validation::RequestValidator;
