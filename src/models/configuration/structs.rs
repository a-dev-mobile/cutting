use serde::{Deserialize, Serialize};
use crate::engine::comparator::OptimizationPriority;
use crate::models::performance_thresholds::PerformanceThresholds;

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
