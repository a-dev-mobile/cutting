use thiserror::Error;

pub type Result<T> = std::result::Result<T, OptimizerError>;

#[derive(Error, Debug)]
pub enum OptimizerError {
    #[error("Invalid configuration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Task not found: {id}")]
    TaskNotFound { id: String },

    #[error("Optimization failed: {reason}")]
    OptimizationFailed { reason: String },

    #[error("Invalid input data: {details}")]
    InvalidInput { details: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Task execution error: {0}")]
    TaskExecution(#[from] tokio::task::JoinError),

    #[error("Computation error: {message}")]
    Computation { message: String },
}