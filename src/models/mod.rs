
// pub mod calculation;
pub mod configuration;
pub mod cut;
// pub mod geometry;

pub mod performance_thresholds;
// pub mod solution;
// pub mod task;
pub mod tile;
pub mod edge;
pub mod panel;
pub mod tile_dimensions;
pub mod tile_node;

/// Default material name used across the application
pub const DEFAULT_MATERIAL: &str = "DEFAULT";

// pub use calculation::{CalculationRequest, CalculationResponse};
pub use configuration::Configuration;
pub use cut::{Cut, CutBuilder};
// pub use geometry::{Cut, Mosaic, TileNode};

pub use performance_thresholds::PerformanceThresholds;
// pub use solution::Solution;
// pub use task::{Task, TaskStatus, TaskStatusResponse};
pub use tile::Tile;
pub use tile_dimensions::{Orientation, TileDimensions};
pub use tile_node::TileNode;
