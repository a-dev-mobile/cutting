//! Task management and execution error types

use thiserror::Error;

/// Task-related errors including lifecycle, execution, and state management
#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task not found: {id}")]
    NotFound { id: String },

    #[error("Invalid task ID: {task_id}")]
    InvalidId { task_id: String },

    #[error("Task execution error: {0}")]
    Execution(#[from] tokio::task::JoinError),

    #[error("Task was cancelled")]
    Cancelled,

    #[error("Task timeout exceeded")]
    Timeout,

    #[error("Task is in invalid state: {current_state}")]
    InvalidState { current_state: String },

    #[error("Invalid status transition from {from:?} to {to:?}")]
    InvalidStatusTransition {
        from: crate::models::enums::Status,
        to: crate::models::enums::Status,
    },

    #[error("Cannot start thread without user info")]
    MissingClientInfo,

    #[error("Thread was terminated during execution")]
    ThreadTerminated,

    #[error("Thread synchronization error: {message}")]
    ThreadSync { message: String },

    #[error("Thread error: {details}")]
    ThreadError { details: String },

    #[error("Material mismatch: tile[{tile_material}] mosaic[{mosaic_material}]")]
    MaterialMismatch {
        tile_material: String,
        mosaic_material: String,
    },
}

impl TaskError {
    /// Creates a new NotFound error
    pub fn not_found(id: impl Into<String>) -> Self {
        Self::NotFound { id: id.into() }
    }

    /// Creates a new InvalidId error
    pub fn invalid_id(task_id: impl Into<String>) -> Self {
        Self::InvalidId {
            task_id: task_id.into(),
        }
    }

    /// Creates a new InvalidState error
    pub fn invalid_state(current_state: impl Into<String>) -> Self {
        Self::InvalidState {
            current_state: current_state.into(),
        }
    }

    /// Creates a new InvalidStatusTransition error
    pub fn invalid_status_transition(
        from: crate::models::enums::Status,
        to: crate::models::enums::Status,
    ) -> Self {
        Self::InvalidStatusTransition { from, to }
    }

    /// Creates a new ThreadSync error
    pub fn thread_sync(message: impl Into<String>) -> Self {
        Self::ThreadSync {
            message: message.into(),
        }
    }

    /// Creates a new ThreadError
    pub fn thread_error(details: impl Into<String>) -> Self {
        Self::ThreadError {
            details: details.into(),
        }
    }

    /// Creates a new MaterialMismatch error
    pub fn material_mismatch(
        tile_material: impl Into<String>,
        mosaic_material: impl Into<String>,
    ) -> Self {
        Self::MaterialMismatch {
            tile_material: tile_material.into(),
            mosaic_material: mosaic_material.into(),
        }
    }

    /// Returns true if this error indicates a temporary condition that might be retried
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Timeout | Self::Execution(_) | Self::ThreadSync { .. }
        )
    }

    /// Returns true if this error indicates a client error (4xx equivalent)
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            Self::NotFound { .. }
                | Self::InvalidId { .. }
                | Self::InvalidState { .. }
                | Self::InvalidStatusTransition { .. }
                | Self::MissingClientInfo
                | Self::MaterialMismatch { .. }
        )
    }

    /// Returns true if this error indicates a server error (5xx equivalent)
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            Self::Execution(_)
                | Self::ThreadTerminated
                | Self::ThreadSync { .. }
                | Self::ThreadError { .. }
        )
    }
}
