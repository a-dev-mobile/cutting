//! Error handling module for the cutting optimizer
//!
//! This module provides a comprehensive error handling system organized by domain:
//! - Core errors: Basic application errors and external library errors
//! - Task errors: Task management and execution errors
//! - Computation errors: Optimization and calculation errors
//! - Service errors: Service-level errors and resource management
//! - Stock errors: Stock solution and panel picker errors

pub mod core;
pub mod task;
pub mod computation;
pub mod service;
pub mod stock;

// Re-export all error types
pub use core::*;
pub use task::*;
pub use computation::*;
pub use service::*;
pub use stock::*;

use thiserror::Error;

/// Main application error type that encompasses all possible errors
#[derive(Error, Debug)]
pub enum AppError {
    // Core errors
    #[error(transparent)]
    Core(#[from] CoreError),

    // Task management errors
    #[error(transparent)]
    Task(#[from] TaskError),

    // Computation errors
    #[error(transparent)]
    Computation(#[from] ComputationError),

    // Service errors
    #[error(transparent)]
    Service(#[from] ServiceError),

    // Stock errors
    #[error(transparent)]
    Stock(#[from] StockError),
}

/// Result type alias for the application
pub type Result<T> = std::result::Result<T, AppError>;

impl AppError {
    /// Returns true if this error indicates a temporary condition that might be retried
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Core(e) => e.is_retryable(),
            Self::Task(e) => e.is_retryable(),
            Self::Computation(e) => e.is_retryable(),
            Self::Service(e) => e.is_retryable(),
            Self::Stock(e) => e.is_retryable(),
        }
    }

    /// Returns true if this error indicates a client error (4xx equivalent)
    pub fn is_client_error(&self) -> bool {
        match self {
            Self::Core(e) => e.is_client_error(),
            Self::Task(e) => e.is_client_error(),
            Self::Computation(e) => e.is_client_error(),
            Self::Service(e) => e.is_client_error(),
            Self::Stock(e) => e.is_client_error(),
        }
    }

    /// Returns true if this error indicates a server error (5xx equivalent)
    pub fn is_server_error(&self) -> bool {
        match self {
            Self::Core(e) => e.is_server_error(),
            Self::Task(e) => e.is_server_error(),
            Self::Computation(e) => e.is_server_error(),
            Self::Service(e) => e.is_server_error(),
            Self::Stock(e) => e.is_server_error(),
        }
    }
}

// Convenience constructors for common errors
impl AppError {
    /// Creates a new TaskNotFound error
    pub fn task_not_found(id: impl Into<String>) -> Self {
        Self::Task(TaskError::NotFound { id: id.into() })
    }

    /// Creates a new TaskAlreadyExists error
    pub fn task_already_exists(task_id: impl Into<String>) -> Self {
        Self::Service(ServiceError::TaskAlreadyExists {
            task_id: task_id.into(),
        })
    }

    /// Creates a new ClientAlreadyHasTask error
    pub fn client_already_has_task(
        client_id: impl Into<String>,
        existing_task_id: impl Into<String>,
    ) -> Self {
        Self::Service(ServiceError::ClientAlreadyHasTask {
            client_id: client_id.into(),
            existing_task_id: existing_task_id.into(),
        })
    }

    /// Creates a new InvalidConfiguration error
    pub fn invalid_configuration(message: impl Into<String>) -> Self {
        Self::Core(CoreError::InvalidConfiguration {
            message: message.into(),
        })
    }

    /// Creates a new InvalidTaskId error
    pub fn invalid_task_id(task_id: impl Into<String>) -> Self {
        Self::Task(TaskError::InvalidId {
            task_id: task_id.into(),
        })
    }

    /// Creates a new InvalidClientId error
    pub fn invalid_client_id(client_id: impl Into<String>) -> Self {
        Self::Service(ServiceError::InvalidClientId {
            client_id: client_id.into(),
        })
    }

    /// Creates a new OptimizationFailed error
    pub fn optimization_failed(reason: impl Into<String>) -> Self {
        Self::Computation(ComputationError::OptimizationFailed {
            reason: reason.into(),
        })
    }

    /// Creates a new Internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Core(CoreError::Internal {
            message: message.into(),
        })
    }

    /// Creates a new LockFailed error
    pub fn lock_failed(resource: impl Into<String>) -> Self {
        Self::Service(ServiceError::LockFailed {
            resource: resource.into(),
        })
    }

    /// Creates a new InvalidTaskState error
    pub fn invalid_task_state(current_state: impl Into<String>) -> Self {
        Self::Task(TaskError::InvalidState {
            current_state: current_state.into(),
        })
    }

    /// Creates a new ResourceUnavailable error
    pub fn resource_unavailable(resource: impl Into<String>) -> Self {
        Self::Service(ServiceError::ResourceUnavailable {
            resource: resource.into(),
        })
    }

    /// Creates a new PermissionDenied error
    pub fn permission_denied(operation: impl Into<String>) -> Self {
        Self::Service(ServiceError::PermissionDenied {
            operation: operation.into(),
        })
    }

    /// Creates a new InvalidInput error
    pub fn invalid_input(details: impl Into<String>) -> Self {
        Self::Core(CoreError::InvalidInput {
            details: details.into(),
        })
    }

    // Stock-related convenience constructors
    /// Creates a new NoStockTiles error
    pub fn no_stock_tiles() -> Self {
        Self::Stock(StockError::NoStockTiles)
    }

    /// Creates a new NoTilesToFit error
    pub fn no_tiles_to_fit() -> Self {
        Self::Stock(StockError::NoTilesToFit)
    }

    /// Creates a new StockPanelPickerNotInitialized error
    pub fn stock_panel_picker_not_initialized() -> Self {
        Self::Stock(StockError::PanelPickerNotInitialized)
    }

    /// Creates a new StockGenerationInterrupted error
    pub fn stock_generation_interrupted(message: impl Into<String>) -> Self {
        Self::Stock(StockError::GenerationInterrupted {
            message: message.into(),
        })
    }

    /// Creates a new ThreadSync error
    pub fn thread_sync(message: impl Into<String>) -> Self {
        Self::Service(ServiceError::ThreadSync {
            message: message.into(),
        })
    }

    /// Creates a new ThreadError
    pub fn thread_error(details: impl Into<String>) -> Self {
        Self::Service(ServiceError::ThreadError {
            details: details.into(),
        })
    }
}
