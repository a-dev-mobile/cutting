//! Utility modules for the cutting optimization system
//! 
//! This module provides a collection of utility functions and types that are used
//! throughout the cutting optimization system. The utilities are organized into
//! logical modules for better maintainability and discoverability.
//! 
//! # Available Modules
//! 
//! - [`arrangement`] - Permutation generation utilities for optimization algorithms
//! - [`edge_banding`] - Edge banding calculation utilities for panel processing
//! - [`timing`] - Performance measurement and timing utilities
//! - [`math`] - Mathematical functions and calculations
//! 
//! # Quick Start
//! 
//! ```rust
//! use cutlist_optimizer_cli::utils::{
//!     arrangement::generate_permutations,
//!     edge_banding::calc_edge_bands,
//!     timing::{Timer, format_duration},
//!     math::percentage,
//! };
//! 
//! // Generate permutations for optimization
//! let items = vec![1, 2, 3];
//! let permutations = generate_permutations(items);
//! 
//! // Measure performance
//! let timer = Timer::new("My operation");
//! // ... do work ...
//! let elapsed = timer.finish();
//! 
//! // Calculate percentages
//! let used_area = 75.0;
//! let total_area = 100.0;
//! let efficiency = percentage(used_area, total_area);
//! ```

pub mod arrangement;
pub mod edge_banding;
pub mod timing;
pub mod math;

// Re-export commonly used items for convenience
pub use timing::{Timer, format_duration};
pub use math::percentage;
pub use arrangement::generate_permutations;
pub use edge_banding::{calc_edge_bands, calc_edge_bands_safe};

/// Utility result type for operations that can fail
pub type UtilResult<T> = Result<T, UtilError>;

/// Common error types for utility operations
#[derive(Debug, Clone, PartialEq)]
pub enum UtilError {
    /// Invalid input parameter
    InvalidInput(String),
    /// Calculation error
    CalculationError(String),
    /// Edge banding specific error
    EdgeBanding(edge_banding::EdgeBandingError),
}

impl std::fmt::Display for UtilError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UtilError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            UtilError::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
            UtilError::EdgeBanding(err) => write!(f, "Edge banding error: {}", err),
        }
    }
}

impl std::error::Error for UtilError {}

impl From<edge_banding::EdgeBandingError> for UtilError {
    fn from(err: edge_banding::EdgeBandingError) -> Self {
        UtilError::EdgeBanding(err)
    }
}

/// Validation utilities
pub mod validation {
    use super::UtilError;
    
    /// Validate that a value is positive
    pub fn validate_positive(value: f64, name: &str) -> Result<(), UtilError> {
        if value <= 0.0 {
            Err(UtilError::InvalidInput(format!("{} must be positive, got: {}", name, value)))
        } else {
            Ok(())
        }
    }
    
    /// Validate that a value is non-negative
    pub fn validate_non_negative(value: f64, name: &str) -> Result<(), UtilError> {
        if value < 0.0 {
            Err(UtilError::InvalidInput(format!("{} must be non-negative, got: {}", name, value)))
        } else {
            Ok(())
        }
    }
    
    /// Validate that a collection is not empty
    pub fn validate_not_empty<T>(collection: &[T], name: &str) -> Result<(), UtilError> {
        if collection.is_empty() {
            Err(UtilError::InvalidInput(format!("{} cannot be empty", name)))
        } else {
            Ok(())
        }
    }
    
    /// Validate that a value is within a range
    pub fn validate_range(value: f64, min: f64, max: f64, name: &str) -> Result<(), UtilError> {
        if value < min || value > max {
            Err(UtilError::InvalidInput(format!(
                "{} must be between {} and {}, got: {}", 
                name, min, max, value
            )))
        } else {
            Ok(())
        }
    }
}

/// Common constants used throughout the system
pub mod constants {
    /// Default epsilon for floating point comparisons
    pub const DEFAULT_EPSILON: f64 = 1e-10;
    
    /// Conversion factor from millimeters to meters
    pub const MM_TO_M: f64 = 0.001;
    
    /// Conversion factor from meters to millimeters
    pub const M_TO_MM: f64 = 1000.0;
    
    /// Conversion factor from inches to millimeters
    pub const INCHES_TO_MM: f64 = 25.4;
    
    /// Maximum reasonable number of permutations to generate
    pub const MAX_PERMUTATIONS: usize = 5040; // 7!
}

/// Helper macros for common operations
#[macro_export]
macro_rules! validate_positive {
    ($value:expr, $name:expr) => {
        $crate::utils::validation::validate_positive($value, $name)?
    };
}

#[macro_export]
macro_rules! validate_non_negative {
    ($value:expr, $name:expr) => {
        $crate::utils::validation::validate_non_negative($value, $name)?
    };
}

#[macro_export]
macro_rules! timed_operation {
    ($name:expr, $operation:expr) => {
        $crate::utils::timing::timed($name, || $operation)
    };
}

/// Utility functions that don't fit into specific modules
pub mod misc {
    /// Generate a unique identifier string
    pub fn generate_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("id_{}", timestamp)
    }
    
    /// Format a number with thousands separators
    pub fn format_number(num: f64) -> String {
        if num.abs() >= 1000.0 {
            format!("{:.2}", num)
        } else {
            format!("{:.2}", num)
        }
    }
    
    /// Truncate a string to a maximum length with ellipsis
    pub fn truncate_string(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else if max_len <= 3 {
            "...".to_string()
        } else {
            format!("{}...", &s[..max_len - 3])
        }
    }
}
