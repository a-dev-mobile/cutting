//! Mosaic module for representing cutting patterns and optimization results
//!
//! This module provides the Mosaic struct which represents a complete cutting solution
//! for a piece of material, including the root tile node and all cuts made.

pub mod structs;
pub mod impls;

pub use structs::Mosaic;
