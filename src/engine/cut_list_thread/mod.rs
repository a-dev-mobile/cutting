//! Cut List Thread Module
//! 
//! This module provides the core thread implementation for computing cutting solutions.
//! It's a direct migration from the Java CutListThread class, adapted to Rust patterns
//! and ownership model.

pub mod structs;
pub mod getters_setters;
pub mod validation;
pub mod core_computation;
pub mod tile_fitting;
pub mod cutting_strategies;
pub mod execution_impls;

// Re-export the main types
pub use structs::{CutListThread, SolutionComparator};
