//! Solution comparator functions
//! 
//! This module contains Rust implementations of Java comparators used for
//! ranking and sorting solutions in the cut list optimization engine.
//! 
//! Each comparator function returns a `std::cmp::Ordering` which can be used
//! directly with Rust's sorting functions like `sort_by()` and `sort_by_key()`.

use std::cmp::Ordering;
use crate::models::Solution;

/// Compare solutions by biggest unused tile area (descending order)
/// 
/// Solutions with larger biggest unused tile areas are considered "greater".
/// This is equivalent to Java's `SolutionBiggestUnusedTileAreaComparator`.
/// 
/// # Arguments
/// * `a` - First solution to compare
/// * `b` - Second solution to compare
/// 
/// # Returns
/// * `Ordering::Greater` if `a` has bigger unused tile area than `b`
/// * `Ordering::Less` if `a` has smaller unused tile area than `b`
/// * `Ordering::Equal` if both have the same biggest unused tile area
pub fn compare_by_biggest_unused_tile_area(a: &Solution, b: &Solution) -> Ordering {
    // Java: long biggestArea = solution2.getBiggestArea() - solution.getBiggestArea();
    // Java: if (biggestArea == 0) return 0;
    // Java: return biggestArea > 0 ? 1 : -1;
    // 
    // This means: if solution2 has bigger area than solution1, return 1 (Greater)
    // So we want solutions with bigger areas to be "greater" (sorted later in ascending order)
    b.get_biggest_area().cmp(&a.get_biggest_area())
}

/// Compare solutions by number of cuts (ascending order)
/// 
/// Solutions with fewer cuts are considered "less" (better).
/// This is equivalent to Java's `SolutionLeastNbrCutsComparator`.
/// 
/// # Arguments
/// * `a` - First solution to compare
/// * `b` - Second solution to compare
/// 
/// # Returns
/// * `Ordering::Less` if `a` has fewer cuts than `b`
/// * `Ordering::Greater` if `a` has more cuts than `b`
/// * `Ordering::Equal` if both have the same number of cuts
pub fn compare_by_least_nbr_cuts(a: &Solution, b: &Solution) -> Ordering {
    a.get_nbr_cuts().cmp(&b.get_nbr_cuts())
}

/// Compare solutions by number of mosaics (ascending order)
/// 
/// Solutions with fewer mosaics are considered "less" (better).
/// This is equivalent to Java's `SolutionLeastNbrMosaicsComparator`.
/// 
/// # Arguments
/// * `a` - First solution to compare
/// * `b` - Second solution to compare
/// 
/// # Returns
/// * `Ordering::Less` if `a` has fewer mosaics than `b`
/// * `Ordering::Greater` if `a` has more mosaics than `b`
/// * `Ordering::Equal` if both have the same number of mosaics
pub fn compare_by_least_nbr_mosaics(a: &Solution, b: &Solution) -> Ordering {
    a.get_mosaics().len().cmp(&b.get_mosaics().len())
}

/// Compare solutions by number of unused tiles (ascending order)
/// 
/// Solutions with fewer unused tiles are considered "less" (better).
/// This is equivalent to Java's `SolutionLeastNbrUnusedTilesComparator`.
/// 
/// # Arguments
/// * `a` - First solution to compare
/// * `b` - Second solution to compare
/// 
/// # Returns
/// * `Ordering::Less` if `a` has fewer unused tiles than `b`
/// * `Ordering::Greater` if `a` has more unused tiles than `b`
/// * `Ordering::Equal` if both have the same number of unused tiles
pub fn compare_by_least_nbr_unused_tiles(a: &Solution, b: &Solution) -> Ordering {
    a.get_nbr_unused_tiles().cmp(&b.get_nbr_unused_tiles())
}

/// Compare solutions by wasted area (ascending order)
/// 
/// Solutions with less wasted area are considered "less" (better).
/// This is equivalent to Java's `SolutionLeastWastedAreaComparator`.
/// 
/// # Arguments
/// * `a` - First solution to compare
/// * `b` - Second solution to compare
/// 
/// # Returns
/// * `Ordering::Less` if `a` has less wasted area than `b`
/// * `Ordering::Greater` if `a` has more wasted area than `b`
/// * `Ordering::Equal` if both have the same wasted area
pub fn compare_by_least_wasted_area(a: &Solution, b: &Solution) -> Ordering {
    // Java: long unusedArea = solution.getUnusedArea() - solution2.getUnusedArea();
    // Java: if (unusedArea == 0) return 0;
    // Java: return unusedArea > 0 ? 1 : -1;
    let unused_area_diff = a.get_unused_area() - b.get_unused_area();
    match unused_area_diff {
        0 => Ordering::Equal,
        diff if diff > 0 => Ordering::Greater,
        _ => Ordering::Less,
    }
}

/// Compare solutions by H/V discrepancy (ascending order)
/// 
/// Solutions with smaller distinct tile set are considered "less" (better).
/// This is equivalent to Java's `SolutionMostHVDiscrepancyComparator`.
/// Note: The Java method name suggests "most" but the implementation sorts ascending.
/// 
/// # Arguments
/// * `a` - First solution to compare
/// * `b` - Second solution to compare
/// 
/// # Returns
/// * `Ordering::Less` if `a` has smaller distinct tile set than `b`
/// * `Ordering::Greater` if `a` has larger distinct tile set than `b`
/// * `Ordering::Equal` if both have the same distinct tile set size
pub fn compare_by_hv_discrepancy(a: &Solution, b: &Solution) -> Ordering {
    a.get_distinct_tile_set().cmp(&b.get_distinct_tile_set())
}

/// Compare solutions by number of final tiles (descending order)
/// 
/// Solutions with more final tiles are considered "greater" (better).
/// This is equivalent to Java's `SolutionMostNbrTilesComparator`.
/// 
/// # Arguments
/// * `a` - First solution to compare
/// * `b` - Second solution to compare
/// 
/// # Returns
/// * `Ordering::Greater` if `a` has more final tiles than `b`
/// * `Ordering::Less` if `a` has fewer final tiles than `b`
/// * `Ordering::Equal` if both have the same number of final tiles
pub fn compare_by_most_nbr_tiles(a: &Solution, b: &Solution) -> Ordering {
    b.get_nbr_final_tiles().cmp(&a.get_nbr_final_tiles())
}

/// Compare solutions by most unused panel area (descending order)
/// 
/// Solutions with larger unused panel areas are considered "greater".
/// This is equivalent to Java's `SolutionMostUnusedPanelAreaComparator`.
/// 
/// # Arguments
/// * `a` - First solution to compare
/// * `b` - Second solution to compare
/// 
/// # Returns
/// * `Ordering::Greater` if `a` has more unused panel area than `b`
/// * `Ordering::Less` if `a` has less unused panel area than `b`
/// * `Ordering::Equal` if both have the same unused panel area
pub fn compare_by_most_unused_panel_area(a: &Solution, b: &Solution) -> Ordering {
    // Java: long mostUnusedPanelArea = solution2.getMostUnusedPanelArea() - solution.getMostUnusedPanelArea();
    // Java: if (mostUnusedPanelArea == 0) return 0;
    // Java: return mostUnusedPanelArea > 0 ? 1 : -1;
    // 
    // This means: if solution2 has more unused area than solution1, return 1 (Greater)
    // So we want solutions with more unused area to be "greater" (sorted later)
    b.get_most_unused_panel_area().cmp(&a.get_most_unused_panel_area())
}

/// Compare solutions by center of mass distance to origin (ascending order)
/// 
/// Solutions with smaller center of mass distance are considered "less" (better).
/// This is equivalent to Java's `SolutionSmallestCenterOfMassDistToOriginComparator`.
/// 
/// # Arguments
/// * `a` - First solution to compare
/// * `b` - Second solution to compare
/// 
/// # Returns
/// * `Ordering::Less` if `a` has smaller center of mass distance than `b`
/// * `Ordering::Greater` if `a` has larger center of mass distance than `b`
/// * `Ordering::Equal` if both have the same center of mass distance
pub fn compare_by_smallest_center_of_mass_dist_to_origin(a: &Solution, b: &Solution) -> Ordering {
    // Java: float centerOfMassDistanceToOrigin = solution.getCenterOfMassDistanceToOrigin() - solution2.getCenterOfMassDistanceToOrigin();
    // Java: if (centerOfMassDistanceToOrigin == 0.0f) return 0;
    // Java: return centerOfMassDistanceToOrigin > 0.0f ? 1 : -1;
    let center_of_mass_diff = a.get_center_of_mass_distance_to_origin() - b.get_center_of_mass_distance_to_origin();
    if center_of_mass_diff == 0.0 {
        Ordering::Equal
    } else if center_of_mass_diff > 0.0 {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}
