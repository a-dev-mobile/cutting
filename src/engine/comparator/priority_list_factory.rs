//! Priority list factory for creating prioritized comparator lists
//! 
//! This module provides factory functions to create ordered lists of solution
//! comparators based on configuration settings, similar to the Java PriorityListFactory.

use crate::models::configuration::Configuration;
use crate::models::enums::OptimizationPriority;
use super::SolutionComparator;

/// Factory for creating prioritized lists of solution comparators
pub struct PriorityListFactory;

impl PriorityListFactory {
    /// Get a prioritized list of comparator strings for final solution selection
    /// 
    /// This function replicates the logic from the Java PriorityListFactory.
    /// The ordering depends on the optimization priority setting in the configuration.
    /// 
    /// # Arguments
    /// * `configuration` - The configuration containing optimization settings
    /// 
    /// # Returns
    /// Vector of optimization priority strings in prioritized order
    /// 
    /// # Logic
    /// - If optimization_priority is MostTiles (equivalent to Java's 0):
    ///   1. MOST_TILES
    ///   2. LEAST_WASTED_AREA  
    ///   3. LEAST_NBR_CUTS
    /// - Otherwise:
    ///   1. MOST_TILES
    ///   2. LEAST_NBR_CUTS
    ///   3. LEAST_WASTED_AREA
    /// - Common suffix for both cases:
    ///   4. LEAST_NBR_MOSAICS
    ///   5. BIGGEST_UNUSED_TILE_AREA
    ///   6. MOST_HV_DISCREPANCY
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::comparator::PriorityListFactory;
    /// use cutlist_optimizer_cli::models::configuration::Configuration;
    /// use cutlist_optimizer_cli::models::enums::OptimizationPriority;
    /// 
    /// let mut config = Configuration::default();
    /// config.optimization_priority = OptimizationPriority::MostTiles;
    /// 
    /// let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config);
    /// assert_eq!(priorities[0], "MOST_TILES");
    /// assert_eq!(priorities[1], "LEAST_WASTED_AREA");
    /// ```
    pub fn get_final_solution_prioritized_comparator_list(
        configuration: &Configuration
    ) -> Vec<String> {
        let mut priority_list = Vec::with_capacity(6);
        
        // First three priorities depend on optimization_priority setting
        if configuration.optimization_priority == OptimizationPriority::MostTiles {
            // Java equivalent: if (configuration.getOptimizationPriority() == 0)
            priority_list.push(OptimizationPriority::MostTiles.to_string());
            priority_list.push(OptimizationPriority::LeastWastedArea.to_string());
            priority_list.push(OptimizationPriority::LeastNbrCuts.to_string());
        } else {
            // All other optimization priorities
            priority_list.push(OptimizationPriority::MostTiles.to_string());
            priority_list.push(OptimizationPriority::LeastNbrCuts.to_string());
            priority_list.push(OptimizationPriority::LeastWastedArea.to_string());
        }
        
        // Common suffix for all cases
        priority_list.push(OptimizationPriority::LeastNbrMosaics.to_string());
        priority_list.push(OptimizationPriority::BiggestUnusedTileArea.to_string());
        priority_list.push(OptimizationPriority::MostHvDiscrepancy.to_string());
        
        priority_list
    }
    
    /// Get a prioritized list of SolutionComparator enums for final solution selection
    /// 
    /// This is a type-safe alternative to the string-based version that returns
    /// actual SolutionComparator enums instead of strings.
    /// 
    /// # Arguments
    /// * `configuration` - The configuration containing optimization settings
    /// 
    /// # Returns
    /// Vector of SolutionComparator enums in prioritized order
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::comparator::{PriorityListFactory, SolutionComparator};
    /// use cutlist_optimizer_cli::models::configuration::Configuration;
    /// use cutlist_optimizer_cli::models::enums::OptimizationPriority;
    /// 
    /// let mut config = Configuration::default();
    /// config.optimization_priority = OptimizationPriority::MostTiles;
    /// 
    /// let comparators = PriorityListFactory::get_final_solution_prioritized_comparator_enum_list(&config);
    /// assert_eq!(comparators[0], SolutionComparator::MostNbrTiles);
    /// ```
    pub fn get_final_solution_prioritized_comparator_enum_list(
        configuration: &Configuration
    ) -> Vec<SolutionComparator> {
        let mut comparator_list = Vec::with_capacity(6);
        
        // First three comparators depend on optimization_priority setting
        if configuration.optimization_priority == OptimizationPriority::MostTiles {
            comparator_list.push(SolutionComparator::MostNbrTiles);
            comparator_list.push(SolutionComparator::LeastWastedArea);
            comparator_list.push(SolutionComparator::LeastNbrCuts);
        } else {
            comparator_list.push(SolutionComparator::MostNbrTiles);
            comparator_list.push(SolutionComparator::LeastNbrCuts);
            comparator_list.push(SolutionComparator::LeastWastedArea);
        }
        
        // Common suffix for all cases
        comparator_list.push(SolutionComparator::LeastNbrMosaics);
        comparator_list.push(SolutionComparator::BiggestUnusedTileArea);
        comparator_list.push(SolutionComparator::HvDiscrepancy);
        
        comparator_list
    }
    
    /// Get a prioritized list of comparison functions for final solution selection
    /// 
    /// This returns actual function pointers that can be used directly for sorting.
    /// 
    /// # Arguments
    /// * `configuration` - The configuration containing optimization settings
    /// 
    /// # Returns
    /// Vector of comparison functions in prioritized order
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::comparator::PriorityListFactory;
    /// use cutlist_optimizer_cli::models::configuration::Configuration;
    /// 
    /// let config = Configuration::default();
    /// let functions = PriorityListFactory::get_final_solution_prioritized_comparator_functions(&config);
    /// 
    /// // Can be used directly for sorting
    /// // solutions.sort_by(functions[0]);
    /// ```
    pub fn get_final_solution_prioritized_comparator_functions(
        configuration: &Configuration
    ) -> Vec<fn(&crate::models::Solution, &crate::models::Solution) -> std::cmp::Ordering> {
        Self::get_final_solution_prioritized_comparator_enum_list(configuration)
            .into_iter()
            .map(|comparator| comparator.compare_fn())
            .collect()
    }
    
    /// Create a custom priority list based on a primary optimization goal
    /// 
    /// This is an extension that allows creating priority lists with any
    /// optimization priority as the primary goal, while maintaining the
    /// same fallback hierarchy.
    /// 
    /// # Arguments
    /// * `primary_priority` - The primary optimization goal
    /// 
    /// # Returns
    /// Vector of optimization priority strings with the primary goal first
    pub fn create_custom_priority_list(primary_priority: OptimizationPriority) -> Vec<String> {
        let mut priority_list = Vec::with_capacity(7);
        
        // Add the primary priority first
        priority_list.push(primary_priority.to_string());
        
        // Add standard fallback priorities (excluding the primary if it's already added)
        let standard_priorities = [
            OptimizationPriority::MostTiles,
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts,
            OptimizationPriority::LeastNbrMosaics,
            OptimizationPriority::BiggestUnusedTileArea,
            OptimizationPriority::MostHvDiscrepancy,
        ];
        
        for priority in &standard_priorities {
            if *priority != primary_priority {
                priority_list.push(priority.to_string());
            }
        }
        
        priority_list
    }
}
