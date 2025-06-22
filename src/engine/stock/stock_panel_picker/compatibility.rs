//! Compatibility layer for Java-style API usage

use std::time::Duration;
use crate::errors::{AppError, Result};
use super::super::StockSolution;
use crate::constants::StockConstants;
use super::StockPanelPicker;

impl StockPanelPicker {
    /// Java-style blocking get stock solution with interrupt handling
    /// 
    /// This provides a more Java-like API that blocks until a solution is available
    /// or returns None if no more solutions can be generated.
    /// 
    /// # Arguments
    /// * `index` - The index of the solution to retrieve
    /// * `timeout` - Optional timeout for the operation
    /// 
    /// # Returns
    /// * `Ok(Some(solution))` - If a solution exists at the given index
    /// * `Ok(None)` - If no more solutions can be generated
    /// * `Err(StockGenerationInterrupted)` - If the operation was interrupted
    /// * `Err(...)` - If an error occurs during retrieval
    pub fn get_stock_solution_blocking(&self, index: usize, timeout: Option<Duration>) -> Result<Option<StockSolution>> {
        // Check if thread is initialized
        if !self.is_initialized()? {
            return Err(AppError::stock_panel_picker_not_initialized());
        }

        let start_time = std::time::Instant::now();
        let mut iteration_count = 0;

        loop {
            iteration_count += 1;
            if iteration_count > StockConstants::MAX_ITERATIONS {
                return Err(AppError::stock_generation_interrupted("Maximum iteration count exceeded"));
            }

            // Check for interruption signal before each operation (Java-style)
            if self.is_interrupted()? {
                return Err(AppError::stock_generation_interrupted("Operation was interrupted"));
            }

            // Check timeout
            if let Some(timeout_duration) = timeout {
                if start_time.elapsed() >= timeout_duration {
                    return Err(AppError::stock_generation_interrupted("Operation timed out"));
                }
            }

            // Try to get the solution
            match self.try_get_solution_immediate(index)? {
                Some(solution) => return Ok(Some(solution)),
                None => {
                    // Check if generation is still active
                    if !self.is_generating()? {
                        return Ok(None);
                    }
                }
            }

            // Sleep before retrying (mimics Java's Thread.sleep)
            std::thread::sleep(Duration::from_millis(StockConstants::RETRY_SLEEP_MS));
        }
    }

    /// Try to get a solution immediately without waiting
    fn try_get_solution_immediate(&self, index: usize) -> Result<Option<StockSolution>> {
        let solutions = self.stock_solutions.lock()
            .map_err(|e| AppError::thread_sync(format!("Failed to lock stock solutions: {}", e)))?;

        if solutions.len() > index {
            // Update max retrieved index
            {
                let mut max_idx = self.max_retrieved_idx.lock()
                    .map_err(|e| AppError::thread_sync(format!("Failed to lock max retrieved index: {}", e)))?;
                *max_idx = (*max_idx).max(index);
            }
            Ok(Some(solutions[index].clone()))
        } else {
            Ok(None)
        }
    }

    /// Java-style initialization that returns a Result instead of being async
    /// 
    /// This provides synchronous initialization for compatibility with Java-style usage
    pub fn init_sync(&self) -> Result<()> {
        // Use a simple runtime for the async init
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| AppError::thread_error(format!("Failed to create async runtime: {}", e)))?;

        rt.block_on(async {
            self.init().await
        })
    }

    /// Java-style stop that returns a Result instead of being async
    pub fn stop_sync(&self) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| AppError::thread_error(format!("Failed to create async runtime: {}", e)))?;

        rt.block_on(async {
            self.stop_generation().await
        })
    }

    /// Check if the picker is in a valid state for operations
    pub fn is_ready(&self) -> bool {
        self.is_initialized().unwrap_or(false) && self.is_generation_active()
    }

    /// Get solution count without error handling (Java-style)
    pub fn get_solution_count_unchecked(&self) -> usize {
        self.solution_count().unwrap_or(0)
    }

    /// Get max retrieved index without error handling (Java-style)
    pub fn get_max_retrieved_idx_unchecked(&self) -> usize {
        self.get_max_retrieved_idx().unwrap_or(0)
    }

    /// Java-style method to check if more solutions are available
    /// 
    /// Returns true if either:
    /// - More solutions are currently available
    /// - Generation is still active and might produce more solutions
    pub fn has_more_solutions(&self) -> bool {
        let solution_count = self.get_solution_count_unchecked();
        let max_retrieved = self.get_max_retrieved_idx_unchecked();
        let is_generating = self.is_generation_active();

        solution_count > max_retrieved + 1 || is_generating
    }

    /// Check if the current thread/operation has been interrupted
    /// 
    /// This mimics Java's Thread.interrupted() behavior
    fn is_interrupted(&self) -> Result<bool> {
        // Check if shutdown signal was sent by trying to receive without blocking
        let sender_guard = self.shutdown_sender.lock()
            .map_err(|e| AppError::thread_sync(format!("Failed to lock shutdown sender: {}", e)))?;

        // If sender is None, we're not interrupted
        // If sender exists but channel is closed, we might be interrupted
        Ok(sender_guard.is_none())
    }

    /// Interrupt the current generation process
    /// 
    /// This provides a way to cancel ongoing operations, similar to Java's interrupt mechanism
    pub fn interrupt(&self) -> Result<()> {
        // Send shutdown signal if available
        let sender_guard = self.shutdown_sender.lock()
            .map_err(|e| AppError::thread_sync(format!("Failed to lock shutdown sender: {}", e)))?;

        if let Some(sender) = sender_guard.as_ref() {
            sender.send(()).map_err(|_| AppError::stock_generation_interrupted("Failed to send interrupt signal"))?;
        }

        Ok(())
    }

    /// Java-style get stock solution that matches the original method signature
    /// 
    /// This corresponds exactly to the Java method:
    /// `public StockSolution getStockSolution(int i) throws InterruptedException`
    pub fn get_stock_solution_java_style(&self, index: usize) -> Result<Option<StockSolution>> {
        // Check for interruption signal before starting (Java-style)
        if self.is_interrupted()? {
            return Err(AppError::stock_generation_interrupted("Thread was interrupted before operation"));
        }

        // Use the blocking method with no timeout (like Java)
        self.get_stock_solution_blocking(index, None)
    }
}

/// Builder for Java-style configuration
pub struct JavaStyleStockPanelPickerBuilder {
    inner: super::StockPanelPickerBuilder,
}

impl JavaStyleStockPanelPickerBuilder {
    pub fn new() -> Self {
        Self {
            inner: super::StockPanelPickerBuilder::new(),
        }
    }

    pub fn tiles_to_fit(mut self, tiles: Vec<crate::models::TileDimensions>) -> Self {
        self.inner = self.inner.tiles_to_fit(tiles);
        self
    }

    pub fn stock_tiles(mut self, tiles: Vec<crate::models::TileDimensions>) -> Self {
        self.inner = self.inner.stock_tiles(tiles);
        self
    }

    pub fn task(mut self, task: std::sync::Arc<crate::models::task::Task>) -> Self {
        self.inner = self.inner.task(task);
        self
    }

    pub fn max_stock_solution_length_hint(mut self, hint: usize) -> Self {
        self.inner = self.inner.max_stock_solution_length_hint(hint);
        self
    }

    /// Build and initialize the picker in one step (Java-style)
    pub fn build_and_init(self) -> Result<StockPanelPicker> {
        let picker = self.inner.build()?;
        picker.init_sync()?;
        Ok(picker)
    }
}

impl Default for JavaStyleStockPanelPickerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StockPanelPicker {
    /// Create a Java-style builder
    pub fn java_builder() -> JavaStyleStockPanelPickerBuilder {
        JavaStyleStockPanelPickerBuilder::new()
    }
}
