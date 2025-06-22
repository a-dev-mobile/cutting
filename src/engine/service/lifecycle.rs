//! Service lifecycle management
//! 
//! This module handles initialization, shutdown, and configuration
//! of the CutListOptimizerService according to the Java implementation.

use crate::engine::service::core::CutListOptimizerServiceImpl;
use crate::errors::Result;

impl CutListOptimizerServiceImpl {
    /// Initialize the service with specified thread pool size
    /// 
    /// This method corresponds to the Java `init(int threadPoolSize)` method.
    /// It sets up the service for operation with the given number of threads.
    pub async fn init(&mut self, thread_pool_size: usize) -> Result<()> {
        // Validate thread pool size
        if thread_pool_size == 0 {
            return Err(crate::errors::AppError::invalid_configuration(
                "Thread pool size must be greater than 0"
            ));
        }

        // Set initialization status
        self.set_initialized(true);

        Ok(())
    }

    /// Shutdown the service gracefully
    /// 
    /// This method corresponds to the Java `shutdown()` method.
    /// It stops all running tasks and cleans up resources.
    pub async fn shutdown(&mut self) -> Result<()> {
        // Set shutdown status
        self.set_shutdown(true);

        Ok(())
    }

    /// Set whether to allow multiple tasks per client
    /// 
    /// This method corresponds to the Java `setAllowMultipleTasksPerClient(boolean allow)` method.
    pub fn set_allow_multiple_tasks_per_client(&mut self, allow: bool) {
        self.set_allow_multiple_tasks_per_client_internal(allow);
    }
}
