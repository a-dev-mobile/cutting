use serde::{Deserialize, Serialize};
use crate::error::{OptimizerError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationPriority {
    MostTiles,
    LeastWastedArea,
    LeastNbrCuts,
    MostHvDiscrepancy,
    BiggestUnusedTileArea,
    SmallestCenterOfMassDistToOrigin,
    LeastNbrMosaics,
    LeastNbrUnusedTiles,
    MostUnusedPanelArea,
}


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

impl Default for Configuration {
    fn default() -> Self {
        Self {
            cut_thickness: 3,
            min_trim_dimension: 10,
            consider_orientation: true,
            optimization_factor: 5,
            optimization_priority: OptimizationPriority::MinimizeWaste,
            use_single_stock_unit: false,
            units: "mm".to_string(),
            performance_thresholds: PerformanceThresholds::default(),
        }
    }
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_simultaneous_tasks: 10,
            max_simultaneous_threads: num_cpus::get(),
            thread_check_interval: 1000,
        }
    }
}

impl Configuration {
    /// Validate configuration parameters
    pub fn validate(&self) -> crate::Result<()> {
        if self.cut_thickness < 0 {
            return Err(crate::OptimizerError::InvalidConfiguration {
                message: "Cut thickness cannot be negative".to_string(),
            });
        }
        
        if self.min_trim_dimension < 0 {
            return Err(crate::OptimizerError::InvalidConfiguration {
                message: "Min trim dimension cannot be negative".to_string(),
            });
        }
        
        if !(1..=10).contains(&self.optimization_factor) {
            return Err(crate::OptimizerError::InvalidConfiguration {
                message: "Optimization factor must be between 1 and 10".to_string(),
            });
        }
        
        Ok(())
    }
}
