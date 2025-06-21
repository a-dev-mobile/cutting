//! Tests for CutListOptimizerService trait and related functionality

use cutlist_optimizer_cli::{
    engine::CutListOptimizerService,
    errors::Result,
    models::{
        CalculationRequest, CalculationSubmissionResult, TaskStatusResponse, Stats,
        enums::{Status, StatusCode},
    },
};
use async_trait::async_trait;

// Mock implementation for testing
struct MockService {
    allow_multiple_tasks: bool,
}

#[async_trait]
impl CutListOptimizerService for MockService {
    async fn submit_task(&self, _request: CalculationRequest) -> Result<CalculationSubmissionResult> {
        Ok(CalculationSubmissionResult {
            status_code: StatusCode::Ok,
            task_id: Some("test-task-123".to_string()),
        })
    }

    async fn get_task_status(&self, _task_id: &str) -> Result<Option<TaskStatusResponse>> {
        Ok(None)
    }

    async fn stop_task(&self, _task_id: &str) -> Result<Option<TaskStatusResponse>> {
        Ok(None)
    }

    async fn terminate_task(&self, _task_id: &str) -> Result<i32> {
        Ok(-1)
    }

    async fn get_tasks(&self, _client_id: &str, _status: Status) -> Result<Vec<String>> {
        Ok(vec![])
    }

    async fn get_stats(&self) -> Result<Stats> {
        Ok(Stats::new())
    }

    fn set_allow_multiple_tasks_per_client(&mut self, allow: bool) {
        self.allow_multiple_tasks = allow;
    }

    async fn init(&mut self, _thread_pool_size: usize) -> Result<()> {
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

#[tokio::test]
async fn test_mock_service() {
    let mut service = MockService {
        allow_multiple_tasks: false,
    };

    // Test initialization
    assert!(service.init(4).await.is_ok());

    // Test configuration
    service.set_allow_multiple_tasks_per_client(true);
    assert!(service.allow_multiple_tasks);

    // Test task submission
    let request = CalculationRequest::new();
    let result = service.submit_task(request).await.unwrap();
    assert_eq!(result.status_code, StatusCode::Ok);
    assert!(result.task_id.is_some());

    // Test shutdown
    assert!(service.shutdown().await.is_ok());
}
