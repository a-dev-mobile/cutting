//! Cut list optimization engine module
//!
//! This module contains the core optimization logic for cutting list calculations,
//! including the main computation thread and related utilities.

pub mod cut_list_thread;


pub use cut_list_thread::{CutListThread, SolutionComparator};

