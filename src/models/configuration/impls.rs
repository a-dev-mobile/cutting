use super::structs::{Configuration, PerformanceThresholds};
use crate::engine::comparator::OptimizationPriority;
use crate::error::{OptimizerError, Result};

impl Default for Configuration {
    fn default() -> Self {
        Self {
            cut_thickness: 3,
            min_trim_dimension: 10,
            consider_orientation: true,
            optimization_factor: 5,
            optimization_priority: OptimizationPriority::LeastWastedArea,
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
    pub fn validate(&self) -> Result<()> {
        if self.cut_thickness < 0 {
            return Err(OptimizerError::InvalidConfiguration {
                message: "Cut thickness cannot be negative".to_string(),
            });
        }
        
        if self.min_trim_dimension < 0 {
            return Err(OptimizerError::InvalidConfiguration {
                message: "Min trim dimension cannot be negative".to_string(),
            });
        }
        
        if !(1..=10).contains(&self.optimization_factor) {
            return Err(OptimizerError::InvalidConfiguration {
                message: "Optimization factor must be between 1 and 10".to_string(),
            });
        }
        
        Ok(())
    }
}
