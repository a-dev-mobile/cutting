//! Stock solution and panel picker error types

use thiserror::Error;

/// Stock-related errors including solution generation and panel picker operations
#[derive(Error, Debug)]
pub enum StockError {
    #[error("No stock tiles provided")]
    NoStockTiles,

    #[error("No tiles to fit provided")]
    NoTilesToFit,

    #[error("Stock solution computation exceeded reasonable limits")]
    ComputationLimitExceeded,

    #[error("Stock panel picker thread not initialized")]
    PanelPickerNotInitialized,

    #[error("Stock solution generation interrupted: {message}")]
    GenerationInterrupted { message: String },

    #[error("No more stock solutions available")]
    NoMoreSolutions,

    #[error("Stock panel picker thread error: {message}")]
    PanelPickerThread { message: String },
}

impl StockError {
    /// Creates a new GenerationInterrupted error
    pub fn generation_interrupted(message: impl Into<String>) -> Self {
        Self::GenerationInterrupted {
            message: message.into(),
        }
    }

    /// Creates a new PanelPickerThread error
    pub fn panel_picker_thread(message: impl Into<String>) -> Self {
        Self::PanelPickerThread {
            message: message.into(),
        }
    }

    /// Returns true if this error indicates a temporary condition that might be retried
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::GenerationInterrupted { .. } | Self::PanelPickerThread { .. }
        )
    }

    /// Returns true if this error indicates a client error (4xx equivalent)
    pub fn is_client_error(&self) -> bool {
        matches!(self, Self::NoStockTiles | Self::NoTilesToFit)
    }

    /// Returns true if this error indicates a server error (5xx equivalent)
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            Self::ComputationLimitExceeded
                | Self::PanelPickerNotInitialized
                | Self::GenerationInterrupted { .. }
                | Self::NoMoreSolutions
                | Self::PanelPickerThread { .. }
        )
    }
}
