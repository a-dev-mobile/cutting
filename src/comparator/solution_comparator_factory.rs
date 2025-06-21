//! Solution comparator factory for creating comparators from string identifiers
//! 
//! This module provides factory functions to create solution comparators from
//! string representations, similar to the Java SolutionComparatorFactory.

use std::cmp::Ordering;
use crate::models::Solution;
use crate::models::enums::OptimizationPriority;
use super::SolutionComparator;

/// Error type for comparator factory operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparatorFactoryError {
    /// Unknown optimization priority string
    UnknownPriority(String),
    /// Invalid input (null/empty string)
    InvalidInput,
}

impl std::fmt::Display for ComparatorFactoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownPriority(priority) => {
                write!(f, "Unknown optimization priority: {}", priority)
            }
            Self::InvalidInput => {
                write!(f, "Invalid input: null or empty string")
            }
        }
    }
}

impl std::error::Error for ComparatorFactoryError {}

/// Factory for creating solution comparators from string identifiers
pub struct SolutionComparatorFactory;

impl SolutionComparatorFactory {
    /// Get a solution comparator function from a string identifier
    /// 
    /// # Arguments
    /// * `priority_str` - String representation of the optimization priority
    /// 
    /// # Returns
    /// * `Ok(Some(comparator))` - Valid comparator function
    /// * `Ok(None)` - Input was None (equivalent to Java's null return)
    /// * `Err(error)` - Invalid priority string
    /// 
    /// # Examples
    /// ```
    /// use cutting::comparator::SolutionComparatorFactory;
    /// 
    /// let comparator = SolutionComparatorFactory::get_solution_comparator(
    ///     Some("MOST_TILES")
    /// ).unwrap();
    /// assert!(comparator.is_some());
    /// 
    /// let none_result = SolutionComparatorFactory::get_solution_comparator(None).unwrap();
    /// assert!(none_result.is_none());
    /// ```
    pub fn get_solution_comparator(
        priority_str: Option<&str>
    ) -> Result<Option<fn(&Solution, &Solution) -> Ordering>, ComparatorFactoryError> {
        let priority_str = match priority_str {
            Some(s) if !s.trim().is_empty() => s.trim(),
            _ => return Ok(None), // Equivalent to Java's null return
        };

        let comparator = Self::parse_priority_string(priority_str)?;
        Ok(Some(comparator.compare_fn()))
    }

    /// Get a solution comparator enum from a string identifier
    /// 
    /// # Arguments
    /// * `priority_str` - String representation of the optimization priority
    /// 
    /// # Returns
    /// * `Ok(Some(comparator))` - Valid comparator enum
    /// * `Ok(None)` - Input was None
    /// * `Err(error)` - Invalid priority string
    pub fn get_solution_comparator_enum(
        priority_str: Option<&str>
    ) -> Result<Option<SolutionComparator>, ComparatorFactoryError> {
        let priority_str = match priority_str {
            Some(s) if !s.trim().is_empty() => s.trim(),
            _ => return Ok(None),
        };

        let comparator = Self::parse_priority_string(priority_str)?;
        Ok(Some(comparator))
    }

    /// Get a list of solution comparator functions from string identifiers
    /// 
    /// # Arguments
    /// * `priority_strings` - Vector of string representations
    /// 
    /// # Returns
    /// Vector of valid comparator functions (invalid ones are filtered out)
    /// 
    /// # Examples
    /// ```
    /// use cutting::comparator::SolutionComparatorFactory;
    /// 
    /// let priorities = vec!["MOST_TILES", "INVALID", "LEAST_WASTED_AREA"];
    /// let comparators = SolutionComparatorFactory::get_solution_comparator_list(&priorities);
    /// assert_eq!(comparators.len(), 2); // Only valid ones included
    /// ```
    pub fn get_solution_comparator_list(
        priority_strings: &[&str]
    ) -> Vec<fn(&Solution, &Solution) -> Ordering> {
        priority_strings
            .iter()
            .filter_map(|&priority_str| {
                Self::get_solution_comparator(Some(priority_str))
                    .ok()
                    .flatten()
            })
            .collect()
    }

    /// Get a list of solution comparator enums from string identifiers
    /// 
    /// # Arguments
    /// * `priority_strings` - Vector of string representations
    /// 
    /// # Returns
    /// Vector of valid comparator enums (invalid ones are filtered out)
    pub fn get_solution_comparator_enum_list(
        priority_strings: &[&str]
    ) -> Vec<SolutionComparator> {
        priority_strings
            .iter()
            .filter_map(|&priority_str| {
                Self::get_solution_comparator_enum(Some(priority_str))
                    .ok()
                    .flatten()
            })
            .collect()
    }

    /// Parse a priority string into a SolutionComparator enum
    /// 
    /// # Arguments
    /// * `priority_str` - String to parse (case-insensitive)
    /// 
    /// # Returns
    /// * `Ok(comparator)` - Valid comparator
    /// * `Err(error)` - Unknown priority string
    fn parse_priority_string(priority_str: &str) -> Result<SolutionComparator, ComparatorFactoryError> {
        let normalized = priority_str.to_uppercase();
        
        match normalized.as_str() {
            "MOST_TILES" => Ok(SolutionComparator::MostNbrTiles),
            "LEAST_WASTED_AREA" => Ok(SolutionComparator::LeastWastedArea),
            "LEAST_NBR_CUTS" => Ok(SolutionComparator::LeastNbrCuts),
            "MOST_HV_DISCREPANCY" => Ok(SolutionComparator::HvDiscrepancy),
            "BIGGEST_UNUSED_TILE_AREA" => Ok(SolutionComparator::BiggestUnusedTileArea),
            "SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN" => Ok(SolutionComparator::SmallestCenterOfMassDistToOrigin),
            "LEAST_NBR_MOSAICS" => Ok(SolutionComparator::LeastNbrMosaics),
            "LEAST_NBR_UNUSED_TILES" => Ok(SolutionComparator::LeastNbrUnusedTiles),
            "MOST_UNUSED_PANEL_AREA" => Ok(SolutionComparator::MostUnusedPanelArea),
            _ => Err(ComparatorFactoryError::UnknownPriority(priority_str.to_string())),
        }
    }
}

/// Convert OptimizationPriority enum to SolutionComparator enum
impl From<OptimizationPriority> for SolutionComparator {
    fn from(priority: OptimizationPriority) -> Self {
        match priority {
            OptimizationPriority::MostTiles => Self::MostNbrTiles,
            OptimizationPriority::LeastWastedArea => Self::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts => Self::LeastNbrCuts,
            OptimizationPriority::MostHvDiscrepancy => Self::HvDiscrepancy,
            OptimizationPriority::BiggestUnusedTileArea => Self::BiggestUnusedTileArea,
            OptimizationPriority::SmallestCenterOfMassDistToOrigin => Self::SmallestCenterOfMassDistToOrigin,
            OptimizationPriority::LeastNbrMosaics => Self::LeastNbrMosaics,
            OptimizationPriority::LeastNbrUnusedTiles => Self::LeastNbrUnusedTiles,
            OptimizationPriority::MostUnusedPanelArea => Self::MostUnusedPanelArea,
        }
    }
}
