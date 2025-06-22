//! Main service implementation
//!
//! This module provides the complete implementation of the CutListOptimizerService trait,
//! delegating to the appropriate specialized modules for different operations.

use async_trait::async_trait;

use crate::{
    errors::Result,
    models::{
        CalculationRequest, CalculationSubmissionResult, TaskStatusResponse, Stats,
        enums::Status,
    },
};

use super::{
    core::CutListOptimizerServiceImpl,
    traits::{CutListOptimizerService, CutListOptimizerServiceExt, TaskDetails, HealthStatus},
};

#[async_trait]
impl CutListOptimizerService for CutListOptimizerServiceImpl {
    async fn submit_task(&self, request: CalculationRequest) -> Result<CalculationSubmissionResult> {
        self.submit_task_impl(request).await
    }

    async fn get_task_status(&self, task_id: &str) -> Result<Option<TaskStatusResponse>> {
        self.get_task_status_impl(task_id).await
    }

    async fn stop_task(&self, task_id: &str) -> Result<Option<TaskStatusResponse>> {
        self.stop_task_impl(task_id).await
    }

    async fn terminate_task(&self, task_id: &str) -> Result<i32> {
        self.terminate_task_impl(task_id).await
    }

    async fn get_tasks(&self, client_id: &str, status: Status) -> Result<Vec<String>> {
        self.get_tasks_impl(client_id, status).await
    }

    async fn get_stats(&self) -> Result<Stats> {
        self.get_stats_impl().await
    }

    fn set_allow_multiple_tasks_per_client(&mut self, allow: bool) {
        self.set_allow_multiple_tasks_per_client_impl(allow);
    }

    async fn init(&mut self, thread_pool_size: usize) -> Result<()> {
        self.init_impl(thread_pool_size).await
    }

    async fn shutdown(&mut self) -> Result<()> {
        self.shutdown_impl().await
    }
}

#[async_trait]
impl CutListOptimizerServiceExt for CutListOptimizerServiceImpl {
    async fn get_task_details(&self, task_id: &str) -> Result<Option<TaskDetails>> {
        self.get_task_details_impl(task_id).await
    }

    async fn cancel_client_tasks(&self, client_id: &str) -> Result<Vec<String>> {
        self.cancel_client_tasks_impl(client_id).await
    }

    async fn health_check(&self) -> Result<HealthStatus> {
        self.health_check_impl().await
    }
}
