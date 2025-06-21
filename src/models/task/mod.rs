//! Task module for tracking calculation execution state and metadata

pub mod structs;
pub mod getters_setters;
pub mod status_management;
pub mod time_management;
pub mod material_management;
pub mod thread_management;
pub mod solution_management;
pub mod logging;

pub use structs::Task;
