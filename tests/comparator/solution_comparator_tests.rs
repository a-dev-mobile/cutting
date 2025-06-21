use cutlist_optimizer_cli::models::solution::Solution;
use cutlist_optimizer_cli::models::tile_dimensions::TileDimensions;
use cutlist_optimizer_cli::models::mosaic::Mosaic;
use cutlist_optimizer_cli::comparator::solution_comparators::*;
use cutlist_optimizer_cli::comparator::solution_comparator_enum::SolutionComparator;
use cutlist_optimizer_cli::comparator::solution_sorting_trait::SolutionSorting;

/// Create a test solution with specific metrics for testing comparators
/// 
/// This creates a solution with controlled metrics that can be used to test
/// the comparator functions. The solution structure is simplified but provides
/// the necessary data for comparator testing.
fn create_test_solution_with_different_areas(
    base_width: i32,
    base_height: i32,
    nbr_mosaics: usize,
) -> Solution {
    let mut solution = Solution::new();
    
    // Create mosaics with different sizes to generate different metrics
    for i in 0..nbr_mosaics {
        let width = base_width + (i * 100) as i32; // Vary width to create different areas
        let height = base_height + (i * 50) as i32;  // Vary height
        
        let tile_dimensions = TileDimensions {
            id: i as i32,
            width,
            height,
            material: "Test Material".to_string(),
            orientation: cutlist_optimizer_cli::models::enums::orientation::Orientation::Any,
            label: Some(format!("Test Panel {}", i)),
            is_rotated: false,
        };
        
        let mosaic = Mosaic::from_tile_dimensions(&tile_dimensions);
        solution.add_mosaic(mosaic);
    }
    
    solution
}

/// Create a test solution with specific number of no-fit panels
fn create_test_solution_with_no_fit_panels(nbr_no_fit: usize) -> Solution {
    let mut solution = Solution::new();
    
    // Add one mosaic
    let tile_dimensions = TileDimensions {
        id: 0,
        width: 1000,
        height: 1000,
        material: "Test Material".to_string(),
        orientation: cutlist_optimizer_cli::models::enums::orientation::Orientation::Any,
        label: Some("Test Panel".to_string()),
        is_rotated: false,
    };
    let mosaic = Mosaic::from_tile_dimensions(&tile_dimensions);
    solution.add_mosaic(mosaic);
    
    // Add no-fit panels
    for i in 0..nbr_no_fit {
        let tile_dimensions = TileDimensions {
            id: (1000 + i) as i32,
            width: 100,
            height: 100,
            material: "Test Material".to_string(),
            orientation: cutlist_optimizer_cli::models::enums::orientation::Orientation::Any,
            label: Some(format!("No-fit Panel {}", i)),
            is_rotated: false,
        };
        solution.add_no_fit_panel(tile_dimensions);
    }
    
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_by_biggest_unused_tile_area() {
        // Create solutions with different panel sizes to get different biggest areas
        let solution1 = create_test_solution_with_different_areas(1000, 1000, 1); // 1000x1000 = 1,000,000
        let solution2 = create_test_solution_with_different_areas(1100, 1050, 1); // 1100x1050 = 1,155,000 (bigger)
        
        println!("Solution1 biggest area: {}", solution1.get_biggest_area());
        println!("Solution2 biggest area: {}", solution2.get_biggest_area());
        
        let result = compare_by_biggest_unused_tile_area(&solution1, &solution2);
        println!("Comparison result: {:?}", result);
        
        // According to Java: solution2.getBiggestArea() - solution.getBiggestArea()
        // If solution2 has bigger area, this should return positive (Greater)
        // So when comparing solution1 vs solution2 (where solution2 has bigger area), result should be Greater
        assert_eq!(compare_by_biggest_unused_tile_area(&solution1, &solution2), std::cmp::Ordering::Greater);
        assert_eq!(compare_by_biggest_unused_tile_area(&solution2, &solution1), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_compare_by_least_nbr_cuts() {
        let solution1 = Solution::new();
        let solution2 = Solution::new();
        
        // Both solutions have 0 cuts, so they should be equal
        assert_eq!(compare_by_least_nbr_cuts(&solution1, &solution2), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_compare_by_least_nbr_mosaics() {
        let solution1 = create_test_solution_with_different_areas(1000, 1000, 2);
        let solution2 = create_test_solution_with_different_areas(1000, 1000, 5);
        
        // Solution with fewer mosaics should be "less" (better)
        assert_eq!(compare_by_least_nbr_mosaics(&solution1, &solution2), std::cmp::Ordering::Less);
        assert_eq!(compare_by_least_nbr_mosaics(&solution2, &solution1), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_compare_by_least_nbr_unused_tiles() {
        let solution1 = create_test_solution_with_no_fit_panels(1); // 1 no-fit panel
        let solution2 = create_test_solution_with_no_fit_panels(2); // 2 no-fit panels
        
        // Both solutions should have 0 unused tiles from mosaics, so they should be equal
        assert_eq!(compare_by_least_nbr_unused_tiles(&solution1, &solution2), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_compare_by_least_wasted_area() {
        // Create solutions with different mosaic sizes to get different unused areas
        let solution1 = create_test_solution_with_different_areas(1000, 1000, 1); // Smaller unused area
        let solution2 = create_test_solution_with_different_areas(1000, 1000, 2); // Larger unused area
        
        // Solution with less wasted area should be "less" (better)
        assert_eq!(compare_by_least_wasted_area(&solution1, &solution2), std::cmp::Ordering::Less);
        assert_eq!(compare_by_least_wasted_area(&solution2, &solution1), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_compare_by_hv_discrepancy() {
        let solution1 = create_test_solution_with_different_areas(1000, 1000, 1);
        let solution2 = create_test_solution_with_different_areas(1000, 1000, 2);
        
        // Both solutions should have the same distinct tile set size (1), so they should be equal
        assert_eq!(compare_by_hv_discrepancy(&solution1, &solution2), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_compare_by_most_nbr_tiles() {
        let solution1 = create_test_solution_with_different_areas(1000, 1000, 1); // 1 mosaic
        let solution2 = create_test_solution_with_different_areas(1000, 1000, 2); // 2 mosaics
        
        println!("Solution1 nbr final tiles: {}", solution1.get_nbr_final_tiles());
        println!("Solution2 nbr final tiles: {}", solution2.get_nbr_final_tiles());
        
        // Both solutions have 0 final tiles (because mosaics created from single tiles don't have final tiles)
        // So the comparison should return Equal
        assert_eq!(compare_by_most_nbr_tiles(&solution1, &solution2), std::cmp::Ordering::Equal);
        assert_eq!(compare_by_most_nbr_tiles(&solution2, &solution1), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_compare_by_most_unused_panel_area() {
        let solution1 = create_test_solution_with_different_areas(1000, 1000, 1); // Smaller unused area
        let solution2 = create_test_solution_with_different_areas(1000, 1000, 2); // Larger unused area
        
        println!("Solution1 most unused panel area: {}", solution1.get_most_unused_panel_area());
        println!("Solution2 most unused panel area: {}", solution2.get_most_unused_panel_area());
        
        // Java: solution2.getMostUnusedPanelArea() - solution.getMostUnusedPanelArea()
        // If solution2 has more unused area, this should return positive (Greater)
        // So when comparing solution1 vs solution2 (where solution2 has more unused area), result should be Greater
        assert_eq!(compare_by_most_unused_panel_area(&solution1, &solution2), std::cmp::Ordering::Greater);
        assert_eq!(compare_by_most_unused_panel_area(&solution2, &solution1), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_compare_by_smallest_center_of_mass_dist_to_origin() {
        let solution1 = create_test_solution_with_different_areas(1000, 1000, 1);
        let solution2 = create_test_solution_with_different_areas(1000, 1000, 1);
        
        // Both solutions should have the same center of mass distance, so they should be equal
        assert_eq!(compare_by_smallest_center_of_mass_dist_to_origin(&solution1, &solution2), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_solution_comparator_enum() {
        let solution1 = create_test_solution_with_different_areas(1000, 1000, 2);
        let solution2 = create_test_solution_with_different_areas(1000, 1000, 5);
        
        // Test each comparator variant
        // For "Least" comparators: solution1 (fewer mosaics) should be "less" than solution2 (more mosaics)
        assert_eq!(SolutionComparator::LeastNbrMosaics.compare(&solution1, &solution2), std::cmp::Ordering::Less);
        assert_eq!(SolutionComparator::LeastNbrUnusedTiles.compare(&solution1, &solution2), std::cmp::Ordering::Less);
        
        // For "Most" comparators: Both solutions have 0 final tiles, so they should be equal
        assert_eq!(SolutionComparator::MostNbrTiles.compare(&solution1, &solution2), std::cmp::Ordering::Equal);
        
        // Test that the enum methods work
        assert!(!SolutionComparator::all().is_empty());
        assert!(SolutionComparator::LeastNbrMosaics.description().contains("mosaics"));
    }

    #[test]
    fn test_solution_sorting_trait() {
        let mut solutions = vec![
            create_test_solution_with_different_areas(1000, 1000, 5),
            create_test_solution_with_different_areas(1000, 1000, 2),
            create_test_solution_with_different_areas(1000, 1000, 3),
        ];

        // Test sorting by number of mosaics (ascending - best first)
        solutions.sort_by_comparator(SolutionComparator::LeastNbrMosaics);
        assert!(solutions[0].get_nbr_mosaics() <= solutions[1].get_nbr_mosaics());
        assert!(solutions[1].get_nbr_mosaics() <= solutions[2].get_nbr_mosaics());

        // Test finding best solution
        let best = solutions.best_by_comparator(SolutionComparator::LeastNbrMosaics);
        assert!(best.is_some());
        assert_eq!(best.unwrap().get_nbr_mosaics(), 2);

        // Test finding worst solution
        let worst = solutions.worst_by_comparator(SolutionComparator::LeastNbrMosaics);
        assert!(worst.is_some());
        assert_eq!(worst.unwrap().get_nbr_mosaics(), 5);
    }

    #[test]
    fn test_solution_sorting_trait_extended() {
        let mut solutions = vec![
            create_test_solution_with_different_areas(1000, 1000, 5),
            create_test_solution_with_different_areas(1000, 1000, 2),
            create_test_solution_with_different_areas(1000, 1000, 3),
            create_test_solution_with_different_areas(1000, 1000, 1),
            create_test_solution_with_different_areas(1000, 1000, 4),
        ];

        // Test unstable sorting
        solutions.sort_unstable_by_comparator(SolutionComparator::LeastNbrMosaics);
        for i in 0..solutions.len() - 1 {
            assert!(solutions[i].get_nbr_mosaics() <= solutions[i + 1].get_nbr_mosaics());
        }

        // Test is_sorted check
        assert!(solutions.is_sorted_by_comparator(SolutionComparator::LeastNbrMosaics));
        
        // Shuffle and verify it's no longer sorted
        solutions.reverse();
        assert!(!solutions.is_sorted_by_comparator(SolutionComparator::LeastNbrMosaics));

        // Test top_n functionality
        let top_3 = solutions.top_n_by_comparator(3, SolutionComparator::LeastNbrMosaics);
        assert_eq!(top_3.len(), 3);
        assert_eq!(top_3[0].get_nbr_mosaics(), 1);
        assert_eq!(top_3[1].get_nbr_mosaics(), 2);
        assert_eq!(top_3[2].get_nbr_mosaics(), 3);

        // Test top_n with n larger than collection size
        let all_solutions = solutions.top_n_by_comparator(10, SolutionComparator::LeastNbrMosaics);
        assert_eq!(all_solutions.len(), 5);

        // Test top_n with n = 0
        let empty_result = solutions.top_n_by_comparator(0, SolutionComparator::LeastNbrMosaics);
        assert!(empty_result.is_empty());
    }

    #[test]
    fn test_empty_collection_edge_cases() {
        let empty_solutions: Vec<Solution> = Vec::new();
        
        // Test best/worst on empty collection
        assert!(empty_solutions.best_by_comparator(SolutionComparator::LeastNbrMosaics).is_none());
        assert!(empty_solutions.worst_by_comparator(SolutionComparator::LeastNbrMosaics).is_none());
        
        // Test is_sorted on empty collection (should be true)
        assert!(empty_solutions.is_sorted_by_comparator(SolutionComparator::LeastNbrMosaics));
        
        // Test top_n on empty collection
        let top_n = empty_solutions.top_n_by_comparator(5, SolutionComparator::LeastNbrMosaics);
        assert!(top_n.is_empty());
    }

    #[test]
    fn test_single_element_collection() {
        let solutions = vec![create_test_solution_with_different_areas(1000, 1000, 3)];
        
        // Test best/worst on single element
        let best = solutions.best_by_comparator(SolutionComparator::LeastNbrMosaics);
        let worst = solutions.worst_by_comparator(SolutionComparator::LeastNbrMosaics);
        assert!(best.is_some());
        assert!(worst.is_some());
        assert_eq!(best.unwrap().get_nbr_mosaics(), worst.unwrap().get_nbr_mosaics());
        
        // Test is_sorted on single element (should be true)
        assert!(solutions.is_sorted_by_comparator(SolutionComparator::LeastNbrMosaics));
        
        // Test top_n on single element
        let top_1 = solutions.top_n_by_comparator(1, SolutionComparator::LeastNbrMosaics);
        assert_eq!(top_1.len(), 1);
        assert_eq!(top_1[0].get_nbr_mosaics(), 3);
    }

    #[test]
    fn test_equal_solutions() {
        let solution1 = create_test_solution_with_different_areas(1000, 1000, 2);
        let solution2 = create_test_solution_with_different_areas(1000, 1000, 2);
        
        // Solutions with same parameters should be equal for most comparators
        assert_eq!(compare_by_least_nbr_mosaics(&solution1, &solution2), std::cmp::Ordering::Equal);
        assert_eq!(compare_by_least_nbr_cuts(&solution1, &solution2), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_empty_solutions() {
        let solution1 = Solution::new();
        let solution2 = Solution::new();
        
        // Empty solutions should be equal
        assert_eq!(compare_by_least_wasted_area(&solution1, &solution2), std::cmp::Ordering::Equal);
        assert_eq!(compare_by_least_nbr_cuts(&solution1, &solution2), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_comparator_functional_equivalence() {
        // Test that the Rust comparators behave the same as the Java ones would
        
        // Test integer comparisons (cuts, mosaics, unused tiles)
        let solution_few_cuts = Solution::new();
        let solution_many_cuts = Solution::new();
        
        // Both have 0 cuts, should be equal
        assert_eq!(compare_by_least_nbr_cuts(&solution_few_cuts, &solution_many_cuts), std::cmp::Ordering::Equal);
        
        // Test mosaic count comparison
        let solution_few_mosaics = create_test_solution_with_different_areas(1000, 1000, 1);
        let solution_many_mosaics = create_test_solution_with_different_areas(1000, 1000, 3);
        
        assert_eq!(compare_by_least_nbr_mosaics(&solution_few_mosaics, &solution_many_mosaics), std::cmp::Ordering::Less);
        assert_eq!(compare_by_least_nbr_mosaics(&solution_many_mosaics, &solution_few_mosaics), std::cmp::Ordering::Greater);
        
        // Test area comparisons with overflow safety
        let solution_small_area = create_test_solution_with_different_areas(1000, 1000, 1);
        let solution_large_area = create_test_solution_with_different_areas(1100, 1050, 1);
        
        assert_eq!(compare_by_biggest_unused_tile_area(&solution_small_area, &solution_large_area), std::cmp::Ordering::Greater);
        assert_eq!(compare_by_least_wasted_area(&solution_small_area, &solution_large_area), std::cmp::Ordering::Less);
    }
}
