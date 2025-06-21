//! Computation and optimization error types

use thiserror::Error;

/// Computation-related errors including optimization, solution computation, and algorithm errors
#[derive(Error, Debug)]
pub enum ComputationError {
    #[error("Optimization failed: {reason}")]
    OptimizationFailed { reason: String },

    #[error("Computation error: {message}")]
    General { message: String },

    #[error("Error during solution computation: {message}")]
    SolutionComputation { message: String },

    #[error("Error during solution comparison: {message}")]
    SolutionComparison { message: String },

    #[error("Node copying error: {message}")]
    NodeCopy { message: String },

    #[error("Candidate search error: {message}")]
    CandidateSearch { message: String },
}

impl ComputationError {
    /// Creates a new OptimizationFailed error
    pub fn optimization_failed(reason: impl Into<String>) -> Self {
        Self::OptimizationFailed {
            reason: reason.into(),
        }
    }

    /// Creates a new General computation error
    pub fn general(message: impl Into<String>) -> Self {
        Self::General {
            message: message.into(),
        }
    }

    /// Creates a new SolutionComputation error
    pub fn solution_computation(message: impl Into<String>) -> Self {
        Self::SolutionComputation {
            message: message.into(),
        }
    }

    /// Creates a new SolutionComparison error
    pub fn solution_comparison(message: impl Into<String>) -> Self {
        Self::SolutionComparison {
            message: message.into(),
        }
    }

    /// Creates a new NodeCopy error
    pub fn node_copy(message: impl Into<String>) -> Self {
        Self::NodeCopy {
            message: message.into(),
        }
    }

    /// Creates a new CandidateSearch error
    pub fn candidate_search(message: impl Into<String>) -> Self {
        Self::CandidateSearch {
            message: message.into(),
        }
    }

    /// Returns true if this error indicates a temporary condition that might be retried
    pub fn is_retryable(&self) -> bool {
        // Most computation errors are not retryable as they indicate algorithmic issues
        false
    }

    /// Returns true if this error indicates a client error (4xx equivalent)
    pub fn is_client_error(&self) -> bool {
        // Optimization failures might be due to invalid input parameters
        matches!(self, Self::OptimizationFailed { .. })
    }

    /// Returns true if this error indicates a server error (5xx equivalent)
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            Self::General { .. }
                | Self::SolutionComputation { .. }
                | Self::SolutionComparison { .. }
                | Self::NodeCopy { .. }
                | Self::CandidateSearch { .. }
        )
    }
}
