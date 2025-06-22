//! Service trait definition
//! 
//! This module defines the main CutListOptimizerService trait
//! that corresponds to the Java interface.

use async_trait::async_trait;
use crate::{
    errors::Result,
    models::{
        CalculationRequest, CalculationSubmissionResult, TaskStatusResponse, Stats,
        enums::Status,
    },
};

#[async_trait]
pub trait CutListOptimizerService {
    /// Initialize service with thread pool size
    async fn init(&mut self, thread_pool_size: usize) -> Result<()>;
    
    /// Submit a task for processing
    async fn submit_task(&self, request: CalculationRequest) -> Result<CalculationSubmissionResult>;
    
    /// Get task status by ID
    async fn get_task_status(&self, task_id: &str) -> Result<Option<TaskStatusResponse>>;
    
    /// Get list of tasks for client with specific status
    async fn get_tasks(&self, client_id: &str, status: Status) -> Result<Vec<String>>;
    
    /// Stop a task gracefully
    async fn stop_task(&self, task_id: &str) -> Result<Option<TaskStatusResponse>>;
    
    /// Terminate a task forcefully
    async fn terminate_task(&self, task_id: &str) -> Result<i32>;
    
    /// Get service statistics
    async fn get_stats(&self) -> Result<Stats>;
    
    /// Set whether to allow multiple tasks per client
    fn set_allow_multiple_tasks_per_client(&mut self, allow: bool);
    
    // /// Set cut list logger (если нужно)
    // fn set_cut_list_logger(&mut self, logger: Arc<dyn CutListLogger>);
}
