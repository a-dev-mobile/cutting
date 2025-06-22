//! Computation modules for the CutList Optimizer Service
//!
//! This module contains all computation-related functionality including
//! dimension utilities, grouping, material computation, and permutation utilities.

pub mod dimension_utils;
pub mod grouping;
pub mod task_compute;
pub mod material_compute;
pub mod permutation_utils;
pub mod debug_single_thread;

// Re-export main utilities for easier access
pub use dimension_utils::DimensionUtils;
pub use grouping::CollectionUtils;
pub use permutation_utils::PermutationUtils;
pub use debug_single_thread::{DebugConfig, DebugResult, debug_compute_complete, create_debug_test_case};
