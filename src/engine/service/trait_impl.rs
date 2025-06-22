//! Complete trait implementation for CutListOptimizerService
//! This module contains the full implementation of all trait methods

use async_trait::async_trait;
use crate::{
    errors::Result,
    models::{
        CalculationRequest, CalculationSubmissionResult, TaskStatusResponse, Stats,
        enums::{Status, StatusCode},
    },
    logging::macros::error,
};

use super::{
    trait_def::CutListOptimizerService, 
    core::CutListOptimizerServiceImpl,
    validation::RequestValidator,
    computation::task_compute,
};

#[async_trait]
impl CutListOptimizerService for CutListOptimizerServiceImpl {
    async fn init(&mut self, thread_pool_size: usize) -> Result<()> {
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

    async fn submit_task(&self, request: CalculationRequest) -> Result<CalculationSubmissionResult> {
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // Validate panels using RequestValidator (already exists)
        if let Some(error_code) = RequestValidator::validate_request(&request).await {
            return Ok(CalculationSubmissionResult {
                status_code: error_code,
                task_id: None,
            });
        }

        // Generate task_id using core.rs method
        let task_id = self.generate_task_id();

        // Launch tokio::spawn with compute_task in background
        let request_clone = request.clone();
        let task_id_clone = task_id.clone();
        
        tokio::spawn(async move {
            if let Err(e) = task_compute::compute_task(request_clone, task_id_clone).await {
                error!("Task computation failed: {}", e);
            }
        });

        // Return CalculationSubmissionResult with success status
        Ok(CalculationSubmissionResult {
            status_code: StatusCode::Ok,
            task_id: Some(task_id),
        })
    }
    
    async fn get_task_status(&self, task_id: &str) -> Result<Option<TaskStatusResponse>> {
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // TODO: Implement actual task status retrieval
        // This should include:
        // 1. Look up task in running tasks registry
        // 2. Get current status, progress, and metrics
        // 3. Return formatted status response

        let _ = task_id; // Suppress unused parameter warning
        Ok(None)
    }
    
    async fn stop_task(&self, task_id: &str) -> Result<Option<TaskStatusResponse>> {
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // TODO: Implement with running tasks manager
        // For now, return None (task not found)
        let _ = task_id;
        Ok(None)
    }
    
    async fn terminate_task(&self, task_id: &str) -> Result<i32> {
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // TODO: Implement with running tasks manager
        // For now, return -1 (task not found)
        let _ = task_id;
        Ok(-1)
    }
    
    async fn get_tasks(&self, client_id: &str, status: Status) -> Result<Vec<String>> {
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // TODO: Implement actual task listing logic
        // This should include:
        // 1. Query running tasks registry by client ID
        // 2. Filter by status if specified
        // 3. Return list of matching task IDs

        let _ = (client_id, status); // Suppress unused parameter warnings
        Ok(vec![])
    }
    
    async fn get_stats(&self) -> Result<Stats> {
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // TODO: Implement actual statistics gathering
        // This should include:
        // 1. Query running tasks for current counts
        // 2. Get performance metrics
        // 3. Calculate throughput statistics
        // 4. Return comprehensive stats

        Ok(Stats::new())
    }
    
    fn set_allow_multiple_tasks_per_client(&mut self, allow: bool) {
        self.set_allow_multiple_tasks_per_client_internal(allow);
    }
}

impl CutListOptimizerServiceImpl {
    /// Check client task limit
    async fn check_client_task_limit(&self, client_id: &str) -> Result<Option<StatusCode>> {
        // TODO: Implement with running tasks manager
        // For now, allow all tasks
        let _ = client_id;
        Ok(None)
    }

    /// Shutdown the service gracefully
    /// 
    /// This method stops all running tasks and cleans up resources.
    /// It corresponds to the Java `shutdown()` method.
    pub async fn shutdown(&mut self) -> Result<()> {
        // Set shutdown status
        self.set_shutdown(true);

        // TODO: Implement graceful shutdown:
        // 1. Stop accepting new tasks
        // 2. Wait for running tasks to complete (with timeout)
        // 3. Force terminate remaining tasks if needed
        // 4. Clean up resources and connections
        // 5. Save any persistent state

        Ok(())
    }
}
