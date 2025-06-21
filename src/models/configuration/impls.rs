use super::structs::Configuration;
use crate::comparator::OptimizationPriority;
use crate::errors::{AppError, Result};
use crate::models::performance_thresholds::PerformanceThresholds;
use crate::constants::ConfigurationDefaults;

impl Default for Configuration {
    fn default() -> Self {
        Self {
            cut_thickness: ConfigurationDefaults::DEFAULT_CUT_THICKNESS,
            min_trim_dimension: ConfigurationDefaults::DEFAULT_MIN_TRIM_DIMENSION,
            consider_orientation: true,
            optimization_factor: ConfigurationDefaults::DEFAULT_OPTIMIZATION_FACTOR,
            optimization_priority: OptimizationPriority::LeastWastedArea,
            use_single_stock_unit: false,
            units: "mm".to_string(),
            performance_thresholds: PerformanceThresholds::default(),
        }
    }
}


impl Configuration {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<()> {
        if self.cut_thickness < 0 {
            return Err(AppError::invalid_configuration("Cut thickness cannot be negative"));
        }
        
        if self.min_trim_dimension < 0 {
            return Err(AppError::invalid_configuration("Min trim dimension cannot be negative"));
        }
        
        if !(ConfigurationDefaults::MIN_OPTIMIZATION_FACTOR..=ConfigurationDefaults::MAX_OPTIMIZATION_FACTOR).contains(&self.optimization_factor) {
            return Err(AppError::invalid_configuration(format!("Optimization factor must be between {} and {}", 
                    ConfigurationDefaults::MIN_OPTIMIZATION_FACTOR, 
                    ConfigurationDefaults::MAX_OPTIMIZATION_FACTOR)));
        }
        
        Ok(())
    }
}
