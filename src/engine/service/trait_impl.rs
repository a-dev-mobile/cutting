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
        use crate::engine::running_tasks::get_running_tasks_instance;
        use crate::logging::macros::info;
        
        // Validate thread pool size (corresponds to Java validation)
        if thread_pool_size == 0 {
            return Err(crate::errors::AppError::invalid_configuration(
                "Thread pool size must be greater than 0"
            ));
        }

        // Initialize RunningTasks singleton (corresponds to Java: this.runningTasks = RunningTasks.getInstance())
        let _running_tasks = get_running_tasks_instance();
        info!("RunningTasks singleton initialized");

        // TODO: Initialize ThreadPoolExecutor equivalent
        // In Java: this.taskExecutor = new ThreadPoolExecutor(i, i, 10L, TimeUnit.SECONDS, 
        //          new ArrayBlockingQueue(THREAD_QUEUE_SIZE), Executors.defaultThreadFactory(), rejectedExecutionHandlerImpl);
        // For now, we use the semaphore-based approach in core.rs
        
        // TODO: Initialize WatchDog
        // In Java: WatchDog watchDog = new WatchDog();
        //          this.watchDog = watchDog;
        //          watchDog.setCutListLogger(this.cutListLogger);
        //          this.watchDog.setRunningTasks(this.runningTasks);
        //          this.watchDog.setTaskExecutor(this.taskExecutor);
        //          this.watchDog.setCutListOptimizerService(this);
        //          new Thread(this.watchDog, "watchDog").start();

        // Set initialization status (corresponds to Java implicit initialization completion)
        self.set_initialized(true);
        info!("CutListOptimizerService initialized with thread_pool_size: {}", thread_pool_size);

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
        use crate::logging::macros::debug;
        use crate::engine::running_tasks::{get_running_tasks_instance, TaskManager};
        
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        debug!("Getting task status for task_id: {}", task_id);

        // Get the running tasks instance
        let running_tasks = get_running_tasks_instance();
        
        // Look up task in running tasks registry
        if let Some(task_arc) = running_tasks.get_task(task_id) {
            debug!("Task found, building solution for task_id: {}", task_id);
            
            let task = task_arc.read();
            
            // Build solution if task is still running (matches Java logic)
            if task.is_running() {
                task.build_and_set_solution();
            }
            
            // Update last queried time (matches Java setLastQueried)
            *task.last_queried.lock().unwrap() = std::time::SystemTime::now();
            
            // Get current status and progress (matches Java TaskStatusResponse creation)
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
            // Task not found - return None (matches Java returning null)
            debug!("Task {} not found", task_id);
            Ok(None)
        }
    }
    
    async fn stop_task(&self, task_id: &str) -> Result<Option<TaskStatusResponse>> {
        use crate::logging::macros::warn;
        use crate::engine::running_tasks::{get_running_tasks_instance, TaskManager};
        
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // Get the running tasks instance
        let running_tasks = get_running_tasks_instance();
        
        // Look up task in running tasks registry
        if let Some(task_arc) = running_tasks.get_task(task_id) {
            let task = task_arc.read();
            
            // Attempt to stop the task (matches Java task.stop() != 0 check)
            if let Err(e) = task.stop() {
                warn!(
                    "Unable to stop task {}. Current status is: {:?}. Error: {}", 
                    task_id, task.status(), e
                );
            }
            
            // Return final status (matches Java TaskStatusResponse creation)
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
            // Task not found - return None (matches Java returning null)
            Ok(None)
        }
    }
    
    async fn terminate_task(&self, task_id: &str) -> Result<i32> {
        use crate::logging::macros::warn;
        use crate::engine::running_tasks::{get_running_tasks_instance, TaskManager};
        
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // Get the running tasks instance
        let running_tasks = get_running_tasks_instance();
        
        // Look up task in running tasks registry
        if let Some(task_arc) = running_tasks.get_task(task_id) {
            let task = task_arc.read();
            
            // Attempt to terminate the task (matches Java task.terminate() logic)
            match task.terminate() {
                Ok(()) => {
                    // Success - return 0 (Java convention for success)
                    Ok(0)
                }
                Err(e) => {
                    warn!(
                        "Unable to terminate task {}. Current status is: {:?}. Error: {}", 
                        task_id, task.status(), e
                    );
                    // Return 1 for failure (Java convention - matches Java returning iTerminate != 0)
                    Ok(1)
                }
            }
        } else {
            // Task not found - return -1 (Java convention - matches Java returning -1)
            Ok(-1)
        }
    }
    
    async fn get_tasks(&self, status: Option<Status>) -> Result<Vec<String>> {
        use crate::engine::running_tasks::{get_running_tasks_instance, TaskManager};
        
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // Get the running tasks instance
        let running_tasks = get_running_tasks_instance();
        
        // Filter by status if provided, otherwise return all tasks
        let task_ids = match status {
            Some(filter_status) => running_tasks.get_tasks_with_status(filter_status),
            None => {
                // Get all tasks and extract their IDs
                running_tasks.get_tasks()
                    .iter()
                    .map(|task_arc| task_arc.read().id.clone())
                    .collect()
            }
        };
        
        Ok(task_ids)
    }
    
    async fn get_stats(&self) -> Result<Stats> {
        use crate::engine::running_tasks::{get_running_tasks_instance, StatisticsCollector};
        
        self.ensure_initialized()?;
        self.ensure_not_shutdown()?;

        // Get the running tasks instance
        let running_tasks = get_running_tasks_instance();
        
        // Get base stats from running tasks
        let mut stats = running_tasks.get_stats();
        
        // TODO: Add thread executor statistics when available
        // For now, we'll use placeholder values for thread statistics
        // In the Java version, these come from taskExecutor.getActiveCount(), etc.
        stats.nbr_running_threads = 0; // Would come from task executor
        stats.nbr_queued_threads = 0;  // Would come from task executor queue size
        
        // TODO: Add watch dog task reports when available
        // In the Java version, this comes from this.watchDog.getTaskReports()
        // For now, we use the task reports from running tasks
        
        Ok(stats)
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
        use crate::logging::macros::info;
        
        // Set shutdown status
        self.set_shutdown(true);
        info!("Service shutdown initiated");

        // TODO: Implement graceful shutdown:
        // 1. Stop accepting new tasks
        // 2. Wait for running tasks to complete (with timeout)
        // 3. Force terminate remaining tasks if needed
        // 4. Clean up resources and connections
        // 5. Save any persistent state

        Ok(())
    }

    /// Destroy the service and clean up all resources
    /// 
    /// This method corresponds to the Java `destroy()` method which calls
    /// `this.taskExecutor.shutdown()` to stop the thread pool executor.
    pub async fn destroy(&mut self) -> Result<()> {
        use crate::logging::macros::info;
        use crate::engine::running_tasks::{get_running_tasks_instance, TaskCleanup};
        use crate::models::enums::Status;
        
        info!("Service destroy initiated");
        
        // Stop all active tasks (corresponds to Java taskExecutor.shutdown())
        let running_tasks = get_running_tasks_instance();
        
        // Stop all running tasks
        let _ = running_tasks.cleanup_tasks_with_status(Status::Running);
        let _ = running_tasks.cleanup_tasks_with_status(Status::Queued);
        
        // Set shutdown status
        self.set_shutdown(true);
        
        // TODO: Additional cleanup:
        // 1. Stop WatchDog thread
        // 2. Clean up thread pool executor
        // 3. Release any held resources
        
        info!("Service destroyed successfully");
        Ok(())
    }
}
