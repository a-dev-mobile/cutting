pub mod optimization;
pub mod solution_comparators;
pub mod solution_comparator_enum;
pub mod solution_sorting_trait;

pub use optimization::OptimizationPriority;
pub use solution_comparator_enum::SolutionComparator;
pub use solution_sorting_trait::SolutionSorting;
pub use solution_comparators::{
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
