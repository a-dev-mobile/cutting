//! Solution retrieval methods for StockPanelPicker

use std::time::Duration;
use crate::errors::{AppError, Result};
use super::super::StockSolution;
use crate::constants::StockConstants;
use super::StockPanelPicker;

impl StockPanelPicker {
    /// Get a stock solution by index, waiting if necessary for generation to complete
    /// 
    /// This corresponds to the Java method: `getStockSolution(int i)`
    /// 
    /// # Arguments
    /// * `index` - The index of the solution to retrieve
    /// 
    /// # Returns
    /// * `Ok(Some(solution))` - If a solution exists at the given index
    /// * `Ok(None)` - If no more solutions can be generated
    /// * `Err(...)` - If an error occurs during retrieval
    pub fn get_stock_solution(&self, index: usize) -> Result<Option<StockSolution>> {
        // Check if thread is initialized
        if !self.is_initialized()? {
            return Err(AppError::stock_panel_picker_not_initialized());
        }

        // Wait for solution to be available or generation to complete
        loop {
            // Atomically check state and retrieve solution if available
            let (solution_available, solution) = {
                let solutions = self.stock_solutions.lock()
                    .map_err(|e| AppError::thread_sync(
                        format!("Failed to lock stock solutions: {}", e)
                    ))?;

                if solutions.len() > index {
                    (true, Some(solutions[index].clone()))
                } else {
                    (false, None)
                }
            };

            if solution_available {
                if let Some(solution) = solution {
                    // Update max retrieved index atomically
                    {
                        let mut max_idx = self.max_retrieved_idx.lock()
                            .map_err(|e| AppError::thread_sync(
                                format!("Failed to lock max retrieved index: {}", e)
                            ))?;
                        *max_idx = (*max_idx).max(index);
                    }
                    return Ok(Some(solution));
                }
            }

            // Check if generation is still active
            let is_generating = self.is_generating()?;
            if !is_generating {
                crate::log_debug!("No more possible stock solutions");
                return Ok(None);
            }

            // Wait before checking again
            crate::log_debug!("Waiting for stock solution generation: idx[{}]", index);
            std::thread::sleep(Duration::from_millis(StockConstants::SOLUTION_WAIT_SLEEP_MS));
        }
    }

    /// Get multiple stock solutions starting from the given index
    /// 
    /// # Arguments
    /// * `start_index` - The starting index
    /// * `count` - Maximum number of solutions to retrieve
    /// 
    /// # Returns
    /// A vector of solutions, which may be shorter than `count` if fewer solutions are available
    pub fn get_stock_solutions(&self, start_index: usize, count: usize) -> Result<Vec<StockSolution>> {
        let mut solutions = Vec::new();
        
        for i in start_index..start_index + count {
            match self.get_stock_solution(i)? {
                Some(solution) => solutions.push(solution),
                None => break, // No more solutions available
            }
        }

        Ok(solutions)
    }

    /// Get all currently available solutions without waiting
    /// 
    /// This method returns immediately with whatever solutions are currently generated
    pub fn get_available_solutions(&self) -> Result<Vec<StockSolution>> {
        let solutions = self.stock_solutions.lock()
            .map_err(|e| AppError::thread_sync(
                format!("Failed to lock stock solutions: {}", e)
            ))?;

        Ok(solutions.clone())
    }

    /// Get the next available solution (convenience method)
    /// 
    /// This retrieves the solution at the current max_retrieved_idx + 1
    pub fn get_next_solution(&self) -> Result<Option<StockSolution>> {
        let max_idx = self.get_max_retrieved_idx()?;
        self.get_stock_solution(max_idx + 1)
    }

    /// Check if a solution is available at the given index without waiting
    /// 
    /// # Arguments
    /// * `index` - The index to check
    /// 
    /// # Returns
    /// * `true` if a solution is available at the index
    /// * `false` if no solution is available yet
    pub fn is_solution_available(&self, index: usize) -> Result<bool> {
        let solution_count = self.solution_count()?;
        Ok(solution_count > index)
    }

    /// Wait for at least the specified number of solutions to be generated
    /// 
    /// # Arguments
    /// * `min_solutions` - Minimum number of solutions to wait for
    /// * `timeout` - Maximum time to wait (None for no timeout)
    /// 
    /// # Returns
    /// * `Ok(true)` if the minimum number of solutions was reached
    /// * `Ok(false)` if timeout was reached before minimum solutions were generated
    pub fn wait_for_solutions(&self, min_solutions: usize, timeout: Option<Duration>) -> Result<bool> {
        let start_time = std::time::Instant::now();

        loop {
            let solution_count = self.solution_count()?;
            let is_generating = self.is_generating()?;

            // Check if we have enough solutions
            if solution_count >= min_solutions {
                return Ok(true);
            }

            // Check if generation is complete but we don't have enough solutions
            if !is_generating {
                return Ok(false);
            }

            // Check timeout
            if let Some(timeout_duration) = timeout {
                if start_time.elapsed() >= timeout_duration {
                    return Ok(false);
                }
            }

            // Wait before checking again
            std::thread::sleep(Duration::from_millis(StockConstants::SOLUTION_WAIT_SLEEP_MS));
        }
    }
}
