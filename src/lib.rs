//! CutList Optimizer - Core library for material cutting optimization
//! 
//! This library provides algorithms and data structures for optimizing
//! the layout of cuts when processing sheet materials like wood, metal, etc.

pub mod cli;
pub mod engine;
pub mod error;
// pub mod io;
pub mod models;
// pub mod utils;

pub use error::{OptimizerError, Result};
// pub use models::{Configuration, Solution, TileDimensions};
// pub use engine::CutListOptimizerService;