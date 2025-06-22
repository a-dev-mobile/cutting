//! CutList Optimizer Service trait definitions
//!
//! This module defines the main service interfaces for the cutting optimization engine,
//! providing async methods for task management and monitoring.

use crate::{
    errors::Result,
    models::{
        CalculationRequest, CalculationSubmissionResult, TaskStatusResponse, Stats,
        enums::Status,
    },
};

/// Main service trait for cut list optimization operations
/// 
/// This trait defines the core interface for submitting optimization tasks,
/// monitoring their progress, and managing the optimization service lifecycle.
/// 
/// # Examples
/// 
/// ```rust
/// use cutlist_optimizer_cli::engine::service::CutListOptimizerService;
/// use cutlist_optimizer_cli::models::{CalculationRequest, enums::Status};
/// 
/// async fn example_usage<T: CutListOptimizerService>(service: &T) -> Result<(), Box<dyn std::error::Error>> {
///     // Submit a new optimization task
///     let request = CalculationRequest::new();
///     let result = service.submit_task(request).await?;
///     
///     if let Some(task_id) = result.task_id {
///         // Monitor task progress
///         while let Some(status) = service.get_task_status(&task_id).await? {
///             if status.status == Status::Finished {
///                 break;
///             }
///             tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
///         }
///     }
///     
///     Ok(())
/// }
/// ```
#[async_trait::async_trait]
pub trait CutListOptimizerService: Send + Sync {
    /// Submit a new optimization task for processing
    /// 
    /// # Arguments
    /// * `request` - The calculation request containing panels, stock, and configuration
    /// 
    /// # Returns
    /// * `Ok(CalculationSubmissionResult)` - Contains task ID if successful, or error code
    /// * `Err(AppError)` - If submission failed due to system error
    async fn submit_task(&self, request: CalculationRequest) -> Result<CalculationSubmissionResult>;
    
    /// Get the current status of a specific task
    /// 
    /// # Arguments
    /// * `task_id` - The unique identifier of the task
    /// 
    /// # Returns
    /// * `Ok(Some(TaskStatusResponse))` - Current task status and progress
    /// * `Ok(None)` - Task not found
    /// * `Err(AppError)` - If status retrieval failed
    async fn get_task_status(&self, task_id: &str) -> Result<Option<TaskStatusResponse>>;
    
    /// Stop a running task gracefully
    /// 
    /// # Arguments
    /// * `task_id` - The unique identifier of the task to stop
    /// 
    /// # Returns
    /// * `Ok(Some(TaskStatusResponse))` - Final task status after stopping
    /// * `Ok(None)` - Task not found
    /// * `Err(AppError)` - If stop operation failed
    async fn stop_task(&self, task_id: &str) -> Result<Option<TaskStatusResponse>>;
    
    /// Terminate a task immediately (forceful stop)
    /// 
    /// # Arguments
    /// * `task_id` - The unique identifier of the task to terminate
    /// 
    /// # Returns
    /// * `Ok(0)` - Task terminated successfully
    /// * `Ok(-1)` - Task not found
    /// * `Ok(other)` - Error code indicating termination failure
    /// * `Err(AppError)` - If termination operation failed
    async fn terminate_task(&self, task_id: &str) -> Result<i32>;
    
    /// Get list of task IDs for a specific client and status
    /// 
    /// # Arguments
    /// * `client_id` - The client identifier
    /// * `status` - Filter tasks by this status
    /// 
    /// # Returns
    /// * `Ok(Vec<String>)` - List of task IDs matching the criteria
    /// * `Err(AppError)` - If retrieval failed
    async fn get_tasks(&self, client_id: &str, status: Status) -> Result<Vec<String>>;
    
    /// Get comprehensive statistics about the service
    /// 
    /// # Returns
    /// * `Ok(Stats)` - Current service statistics including task counts and performance metrics
    /// * `Err(AppError)` - If statistics retrieval failed
    async fn get_stats(&self) -> Result<Stats>;
    
    /// Configure whether multiple tasks per client are allowed
    /// 
    /// # Arguments
    /// * `allow` - True to allow multiple concurrent tasks per client
    fn set_allow_multiple_tasks_per_client(&mut self, allow: bool);
    
    /// Initialize the service with specified thread pool size
    /// 
    /// # Arguments
    /// * `thread_pool_size` - Maximum number of worker threads
    /// 
    /// # Returns
    /// * `Ok(())` - Service initialized successfully
    /// * `Err(AppError)` - If initialization failed
    async fn init(&mut self, thread_pool_size: usize) -> Result<()>;
    
    /// Shutdown the service gracefully
    /// 
    /// This method should stop accepting new tasks and wait for
    /// existing tasks to complete or timeout.
    /// 
    /// # Returns
    /// * `Ok(())` - Service shutdown successfully
    /// * `Err(AppError)` - If shutdown encountered errors
    async fn shutdown(&mut self) -> Result<()>;
}

/// Extension trait for additional service operations
#[async_trait::async_trait]
pub trait CutListOptimizerServiceExt: CutListOptimizerService {
    /// Get detailed task information including logs and performance metrics
    async fn get_task_details(&self, task_id: &str) -> Result<Option<TaskDetails>>;
    
    /// Cancel all tasks for a specific client
    async fn cancel_client_tasks(&self, client_id: &str) -> Result<Vec<String>>;
    
    /// Get service health status
    async fn health_check(&self) -> Result<HealthStatus>;
}

/// Detailed task information for monitoring and debugging
#[derive(Debug, Clone)]
pub struct TaskDetails {
    pub task_id: String,
    pub client_id: String,
    pub status: Status,
    pub progress_percentage: u8,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub thread_count: i32,
    pub memory_usage_mb: Option<f64>,
    pub log_entries: Vec<String>,
    pub error_message: Option<String>,
}

/// Service health status information
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub is_healthy: bool,
    pub uptime_seconds: u64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub active_tasks: i32,
    pub queue_size: i32,
    pub last_error: Option<String>,
}
