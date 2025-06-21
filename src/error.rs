use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
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

    // Cut list specific errors
    #[error("Cannot start thread without user info")]
    MissingClientInfo,

    #[error("Thread was terminated during execution")]
    ThreadTerminated,

    #[error("Error during solution computation: {message}")]
    SolutionComputation { message: String },

    #[error("Error during solution comparison: {message}")]
    SolutionComparison { message: String },

    #[error("Material mismatch: tile[{tile_material}] mosaic[{mosaic_material}]")]
    MaterialMismatch {
        tile_material: String,
        mosaic_material: String,
    },

    #[error("Thread synchronization error: {message}")]
    ThreadSync { message: String },

    #[error("Node copying error: {message}")]
    NodeCopy { message: String },

    #[error("Candidate search error: {message}")]
    CandidateSearch { message: String },

    // Task management errors
    #[error("Invalid status transition from {from:?} to {to:?}")]
    InvalidStatusTransition {
        from: crate::models::enums::Status,
        to: crate::models::enums::Status,
    },

    // Stock solution generation errors
    #[error("No stock tiles provided")]
    NoStockTiles,

    #[error("No tiles to fit provided")]
    NoTilesToFit,

    #[error("Stock solution computation exceeded reasonable limits")]
    StockComputationLimitExceeded,

    // Stock panel picker errors
    #[error("Stock panel picker thread not initialized")]
    StockPanelPickerNotInitialized,

    #[error("Stock solution generation interrupted: {message}")]
    StockGenerationInterrupted { message: String },

    #[error("No more stock solutions available")]
    NoMoreStockSolutions,

    #[error("Stock panel picker thread error: {message}")]
    StockPanelPickerThread { message: String },

    #[error("Thread error: {details}")]
    ThreadError { details: String },
}

/// Task-specific error type
pub type TaskError = AppError;
