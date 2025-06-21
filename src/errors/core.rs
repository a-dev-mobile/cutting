//! Core error types for basic application errors and external library errors

use thiserror::Error;

/// Core application errors including configuration, I/O, and external library errors
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Invalid configuration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Invalid input data: {details}")]
    InvalidInput { details: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl CoreError {
    /// Creates a new InvalidConfiguration error
    pub fn invalid_configuration(message: impl Into<String>) -> Self {
        Self::InvalidConfiguration {
            message: message.into(),
        }
    }

    /// Creates a new InvalidInput error
    pub fn invalid_input(details: impl Into<String>) -> Self {
        Self::InvalidInput {
            details: details.into(),
        }
    }

    /// Creates a new Internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Returns true if this error indicates a temporary condition that might be retried
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::Io(_))
    }

    /// Returns true if this error indicates a client error (4xx equivalent)
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidConfiguration { .. }
                | Self::InvalidInput { .. }
                | Self::Json(_)
                | Self::Csv(_)
        )
    }

    /// Returns true if this error indicates a server error (5xx equivalent)
    pub fn is_server_error(&self) -> bool {
        matches!(self, Self::Internal { .. } | Self::Io(_))
    }
}
