//! Service-level error types for resource management and service operations

use thiserror::Error;

/// Service-level errors including resource management, concurrency, and service lifecycle
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Task already exists: {task_id}")]
    TaskAlreadyExists { task_id: String },

    #[error("Client {client_id} already has a running task: {existing_task_id}")]
    ClientAlreadyHasTask {
        client_id: String,
        existing_task_id: String,
    },

    #[error("Invalid client ID: {client_id}")]
    InvalidClientId { client_id: String },

    #[error("Service is shutting down")]
    ShuttingDown,

    #[error("Maximum number of concurrent tasks reached")]
    MaxTasksReached,

    #[error("Lock acquisition failed: {resource}")]
    LockFailed { resource: String },

    #[error("Resource not available: {resource}")]
    ResourceUnavailable { resource: String },

    #[error("Permission denied for operation: {operation}")]
    PermissionDenied { operation: String },

    #[error("Thread synchronization error: {message}")]
    ThreadSync { message: String },

    #[error("Thread error: {details}")]
    ThreadError { details: String },
}

impl ServiceError {
    /// Creates a new TaskAlreadyExists error
    pub fn task_already_exists(task_id: impl Into<String>) -> Self {
        Self::TaskAlreadyExists {
            task_id: task_id.into(),
        }
    }

    /// Creates a new ClientAlreadyHasTask error
    pub fn client_already_has_task(
        client_id: impl Into<String>,
        existing_task_id: impl Into<String>,
    ) -> Self {
        Self::ClientAlreadyHasTask {
            client_id: client_id.into(),
            existing_task_id: existing_task_id.into(),
        }
    }

    /// Creates a new InvalidClientId error
    pub fn invalid_client_id(client_id: impl Into<String>) -> Self {
        Self::InvalidClientId {
            client_id: client_id.into(),
        }
    }

    /// Creates a new LockFailed error
    pub fn lock_failed(resource: impl Into<String>) -> Self {
        Self::LockFailed {
            resource: resource.into(),
        }
    }

    /// Creates a new ResourceUnavailable error
    pub fn resource_unavailable(resource: impl Into<String>) -> Self {
        Self::ResourceUnavailable {
            resource: resource.into(),
        }
    }

    /// Creates a new PermissionDenied error
    pub fn permission_denied(operation: impl Into<String>) -> Self {
        Self::PermissionDenied {
            operation: operation.into(),
        }
    }

    /// Returns true if this error indicates a temporary condition that might be retried
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::ResourceUnavailable { .. }
                | Self::LockFailed { .. }
                | Self::MaxTasksReached
        )
    }

    /// Returns true if this error indicates a client error (4xx equivalent)
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            Self::TaskAlreadyExists { .. }
                | Self::ClientAlreadyHasTask { .. }
                | Self::InvalidClientId { .. }
                | Self::PermissionDenied { .. }
        )
    }

    /// Returns true if this error indicates a server error (5xx equivalent)
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            Self::ShuttingDown
                | Self::LockFailed { .. }
                | Self::ResourceUnavailable { .. }
                | Self::MaxTasksReached
        )
    }
}
