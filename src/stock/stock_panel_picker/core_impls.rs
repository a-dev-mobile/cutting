//! Core implementation methods for StockPanelPicker

use std::sync::Arc;
use crate::{log_debug, errors::{AppError, Result}};
use super::{StockPanelPicker, StockPanelPickerStats, SolutionSortConfig};

impl StockPanelPicker {
    /// Get the required area from the stock solution generator
    /// 
    /// This corresponds to the Java method: `getRequiredArea()`
    pub fn get_required_area(&self) -> i64 {
        self.stock_solution_generator.get_required_area()
    }

    /// Get statistics about the current state of the stock panel picker
    pub fn get_stats(&self) -> Result<StockPanelPickerStats> {
        let solutions = self.stock_solutions.lock()
            .map_err(|e| AppError::thread_sync(
                format!("Failed to lock stock solutions: {}", e)
            ))?;
        
        let max_retrieved_idx = *self.max_retrieved_idx.lock()
            .map_err(|e| AppError::thread_sync(
                format!("Failed to lock max retrieved index: {}", e)
            ))?;

        let generation_thread = self.generation_thread.lock()
            .map_err(|e| AppError::thread_sync(
                format!("Failed to lock generation thread: {}", e)
            ))?;

        let is_generating = generation_thread.as_ref()
            .map(|handle| !handle.is_finished())
            .unwrap_or(false);

        Ok(StockPanelPickerStats {
            total_solutions: solutions.len(),
            max_retrieved_idx,
            is_generating,
            required_area: self.get_required_area(),
        })
    }

    /// Sort stock solutions according to the provided configuration
    /// 
    /// This corresponds to the Java method: `sortStockSolutions()`
    /// Note: The Java version silently ignores sorting exceptions, this version returns errors
    pub fn sort_stock_solutions(&self, config: &SolutionSortConfig) -> Result<()> {
        let mut solutions = self.stock_solutions.lock()
            .map_err(|e| AppError::thread_sync(
                format!("Failed to lock stock solutions for sorting: {}", e)
            ))?;

        if config.sort_by_area {
            // Use stable sort to maintain order for equal elements
            solutions.sort_by(|a, b| {
                let area_a = a.get_total_area();
                let area_b = b.get_total_area();
                
                if config.ascending {
                    area_a.cmp(&area_b)
                } else {
                    area_b.cmp(&area_a)
                }
            });
        }

        log_debug!("Sorted {} stock solutions by area (ascending: {})", 
                  solutions.len(), config.ascending);
        Ok(())
    }

    /// Sort stock solutions by total area in ascending order (default behavior)
    /// 
    /// This matches the Java implementation which sorts by smallest area first
    pub fn sort_stock_solutions_default(&self) -> Result<()> {
        self.sort_stock_solutions(&SolutionSortConfig::default())
    }

    /// Sort stock solutions with Java-compatible behavior
    /// 
    /// This method replicates the exact Java sorting behavior, including:
    /// - Potential integer overflow when casting long to int
    /// - Silent error handling (ignores sorting exceptions)
    /// 
    /// This corresponds exactly to the Java method: `sortStockSolutions()`
    pub fn sort_stock_solutions_java_compatible(&self) -> Result<()> {
        let mut solutions = self.stock_solutions.lock()
            .map_err(|e| AppError::thread_sync(
                format!("Failed to lock stock solutions for sorting: {}", e)
            ))?;

        // Java-compatible sorting with potential overflow behavior
        // Java code: return (int) (stockSolution.getTotalArea() - stockSolution2.getTotalArea());
        solutions.sort_by(|a, b| {
            let area_a = a.get_total_area();
            let area_b = b.get_total_area();
            
            // Mimic Java's (int) cast behavior which can overflow
            let diff = area_a.saturating_sub(area_b);
            
            // Java's int cast would overflow for values outside i32 range
            if diff > i32::MAX as i64 {
                // Large positive difference becomes negative after overflow
                std::cmp::Ordering::Less
            } else if diff < i32::MIN as i64 {
                // Large negative difference becomes positive after overflow  
                std::cmp::Ordering::Greater
            } else {
                // Normal case - no overflow
                (diff as i32).cmp(&0)
            }
        });

        log_debug!("Java-compatible sorted {} stock solutions by area", solutions.len());
        Ok(())
    }

    /// Sort stock solutions with Java-style silent error handling
    /// 
    /// This method exactly matches Java's behavior:
    /// ```java
    /// try {
    ///     Collections.sort(this.stockSolutions, comparator);
    /// } catch (Exception unused) {
    ///     // Silently ignore sorting exceptions
    /// }
    /// ```
    pub fn sort_stock_solutions_java_style(&self) {
        // Match Java's silent exception handling - ignore all errors
        let _ = self.sort_stock_solutions_java_compatible();
    }

    /// Check if the picker has been initialized (thread started)
    pub fn is_initialized(&self) -> Result<bool> {
        let generation_thread = self.generation_thread.lock()
            .map_err(|e| AppError::thread_sync(
                format!("Failed to lock generation thread: {}", e)
            ))?;

        Ok(generation_thread.is_some())
    }

    /// Get the number of currently generated solutions
    pub fn solution_count(&self) -> Result<usize> {
        let solutions = self.stock_solutions.lock()
            .map_err(|e| AppError::thread_sync(
                format!("Failed to lock stock solutions: {}", e)
            ))?;

        Ok(solutions.len())
    }

    /// Check if more solutions are being generated
    pub fn is_generating(&self) -> Result<bool> {
        let generation_thread = self.generation_thread.lock()
            .map_err(|e| AppError::thread_sync(
                format!("Failed to lock generation thread: {}", e)
            ))?;

        Ok(generation_thread.as_ref()
            .map(|handle| !handle.is_finished())
            .unwrap_or(false))
    }

    /// Get a reference to the task
    pub fn get_task(&self) -> &Arc<crate::models::task::Task> {
        &self.task
    }

    /// Get the maximum retrieved index
    pub fn get_max_retrieved_idx(&self) -> Result<usize> {
        let max_idx = *self.max_retrieved_idx.lock()
            .map_err(|e| AppError::thread_sync(
                format!("Failed to lock max retrieved index: {}", e)
            ))?;

        Ok(max_idx)
    }
}
