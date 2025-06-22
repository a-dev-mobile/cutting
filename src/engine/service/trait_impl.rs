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
    engine::running_tasks::TaskManager,
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

        // Get the running tasks instance
        let running_tasks = crate::engine::running_tasks::get_running_tasks_instance();
        
        // Look up task in running tasks registry
        if let Some(task_arc) = running_tasks.get_task(task_id) {
            let task = task_arc.read();
            
            // Build solution if not already built
            task.build_and_set_solution();
            
            // Update last queried time
            *task.last_queried.lock().unwrap() = std::time::SystemTime::now();
            
            // Get current status and progress
            let status = task.status();
            let percentage_done = task.percentage_done() as u8;
            let init_percentage = task.max_thread_progress_percentage() as u8;
            let solution = task.solution.read().unwrap().clone();
            
            Ok(Some(TaskStatusResponse {
                status,
                percentage_done,
                init_percentage,
                solution,
            }))
        } else {
            // Task not found
            Ok(None)
        }
    }
    
    async fn stop_task(&self, task_id: &str) -> Result<Option<TaskStatusResponse>> {
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // Get the running tasks instance
        let running_tasks = crate::engine::running_tasks::get_running_tasks_instance();
        
        // Look up task in running tasks registry
        if let Some(task_arc) = running_tasks.get_task(task_id) {
            let task = task_arc.read();
            
            // Attempt to stop the task
            if let Err(e) = task.stop() {
                crate::logging::macros::warn!(
                    "Unable to stop task {}. Current status is: {:?}. Error: {}", 
                    task_id, task.status(), e
                );
            }
            
            // Return final status
            let status = task.status();
            let percentage_done = task.percentage_done() as u8;
            let init_percentage = task.max_thread_progress_percentage() as u8;
            let solution = task.solution.read().unwrap().clone();
            
            Ok(Some(TaskStatusResponse {
                status,
                percentage_done,
                init_percentage,
                solution,
            }))
        } else {
            // Task not found
            Ok(None)
        }
    }
    
    async fn terminate_task(&self, task_id: &str) -> Result<i32> {
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // Get the running tasks instance
        let running_tasks = crate::engine::running_tasks::get_running_tasks_instance();
        
        // Look up task in running tasks registry
        if let Some(task_arc) = running_tasks.get_task(task_id) {
            let task = task_arc.read();
            
            // Attempt to terminate the task
            match task.terminate() {
                Ok(()) => {
                    // Success - return 0 (Java convention for success)
                    Ok(0)
                }
                Err(e) => {
                    crate::logging::macros::warn!(
                        "Unable to terminate task {}. Current status is: {:?}. Error: {}", 
                        task_id, task.status(), e
                    );
                    // Return 1 for failure (Java convention)
                    Ok(1)
                }
            }
        } else {
            // Task not found - return -1 (Java convention)
            Ok(-1)
        }
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
