use serde::{Deserialize, Serialize};
use crate::engine::comparator::OptimizationPriority;

/// Configuration parameters for the optimization process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    /// Thickness of the cutting blade (kerf)
    pub cut_thickness: i32,
    
    /// Minimum trim dimension (waste edge)
    pub min_trim_dimension: i32,
    
    /// Whether to consider grain orientation
    pub consider_orientation: bool,
    
    /// Optimization accuracy factor (1-10, higher = more accurate but slower)
    pub optimization_factor: i32,
    
    /// Primary optimization goal
    pub optimization_priority: OptimizationPriority,
    
    /// Whether to use only single stock unit per solution
    pub use_single_stock_unit: bool,
    
    /// Measurement units
    pub units: String,
    
    /// Performance constraints
    pub performance_thresholds: PerformanceThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum number of simultaneous tasks
    pub max_simultaneous_tasks: usize,
    
    /// Maximum number of threads per task
    pub max_simultaneous_threads: usize,
    
    /// Interval between thread status checks (milliseconds)
    pub thread_check_interval: u64,
}
