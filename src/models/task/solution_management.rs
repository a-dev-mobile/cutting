//! Solution management for Task struct
//! 
//! This module contains methods for managing task solutions and building final results.

use tracing::debug;
use crate::models::CalculationResponse;
use super::Task;

impl Task {
    // ===== Solution Management =====

    /// Check if the task has a solution
    pub fn has_solution(&self) -> bool {
        self.solution.read().unwrap().is_some()
    }

    /// Check if the solution has all panels fitting
    pub fn has_solution_all_fit(&self) -> bool {
        self.has_solution() && 
        self.solution
            .read()
            .unwrap()
            .as_ref()
            .map(|s| s.no_fit_panels.is_empty())
            .unwrap_or(false)
    }

    /// Build the final solution from all thread solutions
    /// Returns the built solution or None if no calculation request exists
    pub fn build_solution(&self) -> Option<CalculationResponse> {
        // This would typically use a CalculationResponseBuilder
        // For now, we'll create a placeholder implementation
        debug!("Building solution for task {}", self.id);
        
        // In a real implementation, this would:
        // 1. Collect all solutions from threads
        // 2. Apply optimization algorithms
        // 3. Build the final CalculationResponse
        
        if let Some(request) = &self.calculation_request {
            let response = CalculationResponse {
                version: "1.0.0".to_string(),
                edge_bands: None,
                elapsed_time: self.elapsed_time(),
                id: Some(self.id.clone()),
                panels: Some(Vec::new()), // Would be populated with actual results
                request: Some(request.clone()),
                solution_elapsed_time: Some(self.elapsed_time()),
                task_id: Some(self.id.clone()),
                total_cut_length: 0.0,
                total_nbr_cuts: 0,
                total_used_area: 0.0,
                total_used_area_ratio: 0.0,
                total_wasted_area: 0.0,
                used_stock_panels: None,
                no_fit_panels: Vec::new(),
                mosaics: Vec::new(),
            };
            
            Some(response)
        } else {
            None
        }
    }

    /// Build and set the solution for this task
    pub fn build_and_set_solution(&self) {
        if let Some(solution) = self.build_solution() {
            *self.solution.write().unwrap() = Some(solution);
        }
    }
}
