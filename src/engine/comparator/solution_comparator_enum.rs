//! Solution comparator enum and related functionality
//! 
//! This module contains the `SolutionComparator` enum which provides a type-safe
//! way to select different comparison functions for sorting solutions.

use std::cmp::Ordering;
use crate::models::Solution;
use super::{
    compare_by_biggest_unused_tile_area,
    compare_by_least_nbr_cuts,
    compare_by_least_nbr_mosaics,
    compare_by_least_nbr_unused_tiles,
    compare_by_least_wasted_area,
    compare_by_hv_discrepancy,
    compare_by_most_nbr_tiles,
    compare_by_most_unused_panel_area,
    compare_by_smallest_center_of_mass_dist_to_origin,
};

/// Enum representing different solution comparison strategies
/// 
/// This enum provides a type-safe way to select different comparison functions
/// and can be used to dynamically choose sorting criteria at runtime.
/// 
/// # Performance
/// All variants are zero-cost abstractions that compile to direct function calls.
/// The enum itself is Copy and has no runtime overhead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)] // Optimize enum representation for better performance
pub enum SolutionComparator {
    /// Compare by biggest unused tile area (descending)
    BiggestUnusedTileArea,
    /// Compare by number of cuts (ascending)
    LeastNbrCuts,
    /// Compare by number of mosaics (ascending)
    LeastNbrMosaics,
    /// Compare by number of unused tiles (ascending)
    LeastNbrUnusedTiles,
    /// Compare by wasted area (ascending)
    LeastWastedArea,
    /// Compare by H/V discrepancy (ascending)
    HvDiscrepancy,
    /// Compare by number of final tiles (descending)
    MostNbrTiles,
    /// Compare by unused panel area (descending)
    MostUnusedPanelArea,
    /// Compare by center of mass distance to origin (ascending)
    SmallestCenterOfMassDistToOrigin,
}

impl SolutionComparator {
    /// Get the comparison function for this comparator
    /// 
    /// # Returns
    /// A function that can be used with `sort_by()` to sort solutions
    pub fn compare_fn(self) -> fn(&Solution, &Solution) -> Ordering {
        match self {
            Self::BiggestUnusedTileArea => compare_by_biggest_unused_tile_area,
            Self::LeastNbrCuts => compare_by_least_nbr_cuts,
            Self::LeastNbrMosaics => compare_by_least_nbr_mosaics,
            Self::LeastNbrUnusedTiles => compare_by_least_nbr_unused_tiles,
            Self::LeastWastedArea => compare_by_least_wasted_area,
            Self::HvDiscrepancy => compare_by_hv_discrepancy,
            Self::MostNbrTiles => compare_by_most_nbr_tiles,
            Self::MostUnusedPanelArea => compare_by_most_unused_panel_area,
            Self::SmallestCenterOfMassDistToOrigin => compare_by_smallest_center_of_mass_dist_to_origin,
        }
    }
    
    /// Compare two solutions using this comparator
    /// 
    /// # Arguments
    /// * `a` - First solution to compare
    /// * `b` - Second solution to compare
    /// 
    /// # Returns
    /// The ordering result based on this comparator's criteria
    pub fn compare(self, a: &Solution, b: &Solution) -> Ordering {
        self.compare_fn()(a, b)
    }
    
    /// Get a human-readable description of this comparator
    pub fn description(self) -> &'static str {
        match self {
            Self::BiggestUnusedTileArea => "Biggest unused tile area (descending)",
            Self::LeastNbrCuts => "Least number of cuts (ascending)",
            Self::LeastNbrMosaics => "Least number of mosaics (ascending)",
            Self::LeastNbrUnusedTiles => "Least number of unused tiles (ascending)",
            Self::LeastWastedArea => "Least wasted area (ascending)",
            Self::HvDiscrepancy => "H/V discrepancy (ascending)",
            Self::MostNbrTiles => "Most number of tiles (descending)",
            Self::MostUnusedPanelArea => "Most unused panel area (descending)",
            Self::SmallestCenterOfMassDistToOrigin => "Smallest center of mass distance to origin (ascending)",
        }
    }
    
    /// Get all available comparators
    pub fn all() -> &'static [Self] {
        &[
            Self::BiggestUnusedTileArea,
            Self::LeastNbrCuts,
            Self::LeastNbrMosaics,
            Self::LeastNbrUnusedTiles,
            Self::LeastWastedArea,
            Self::HvDiscrepancy,
            Self::MostNbrTiles,
            Self::MostUnusedPanelArea,
            Self::SmallestCenterOfMassDistToOrigin,
        ]
    }
}
