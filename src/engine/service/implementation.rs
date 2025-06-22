//! Main service implementation
//!
//! This module provides the complete implementation of the CutListOptimizerService,
//! delegating to the appropriate specialized modules for different operations.

use crate::{
    errors::Result,
    models::{
        CalculationRequest, CalculationSubmissionResult, TaskStatusResponse, Stats,
        enums::Status,
    },
};

use super::core::CutListOptimizerServiceImpl;

impl CutListOptimizerServiceImpl {
    /// Public API methods that delegate to implementation modules
    
    pub async fn submit_task(&self, request: CalculationRequest) -> Result<CalculationSubmissionResult> {
        self.submit_task_impl(request).await
    }

    pub async fn get_task_status(&self, task_id: &str) -> Result<Option<TaskStatusResponse>> {
        self.get_task_status_impl(task_id).await
    }

    pub async fn stop_task(&self, task_id: &str) -> Result<Option<TaskStatusResponse>> {
        self.stop_task_impl(task_id).await
    }

    pub async fn terminate_task(&self, task_id: &str) -> Result<i32> {
        self.terminate_task_impl(task_id).await
    }

    pub async fn get_tasks(&self, client_id: &str, status: Status) -> Result<Vec<String>> {
        self.get_tasks_impl(client_id, status).await
    }

    pub async fn get_stats(&self) -> Result<Stats> {
        self.get_stats_impl().await
    }
}
