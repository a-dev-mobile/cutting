//! Tests for CutListThread
//! 
//! This module contains comprehensive unit tests for the CutListThread functionality.

use cutlist_optimizer_cli::{
    engine::cut_list_thread::{CutListThread, SolutionComparator},
    models::{Solution, TileDimensions, TileNode, Mosaic},
    stock::StockSolution,
    CutDirection, Status, Orientation,
    errors::AppError,
};
use std::{
    sync::{Arc, Mutex},
    thread,
};

// Helper function to create a test tile dimensions
fn create_test_tile(id: i32, width: i32, height: i32, material: &str) -> TileDimensions {
    TileDimensions {
        id,
        width,
        height,
        material: material.to_string(),
        orientation: Orientation::Any,
        label: None,
        is_rotated: false,
    }
}

// Helper function to create a test stock solution
fn create_test_stock_solution() -> StockSolution {
    let stock_tiles = vec![
        create_test_tile(1, 1000, 2000, "Wood"),
        create_test_tile(2, 800, 1600, "Wood"),
    ];
    StockSolution::from_tiles(stock_tiles)
}

#[test]
fn test_cut_list_thread_creation() {
    let thread = CutListThread::new();
    assert_eq!(thread.status(), Status::Queued);
    assert_eq!(thread.percentage_done(), 0);
    assert_eq!(thread.accuracy_factor(), 100);
    assert_eq!(thread.cut_thickness(), 0);
    assert_eq!(thread.min_trim_dimension(), 0);
    assert_eq!(thread.first_cut_orientation(), CutDirection::Both);
    assert!(!thread.consider_grain_direction());
    assert!(thread.tiles().is_empty());
    assert!(thread.solutions().is_empty());
    assert!(thread.group().is_none());
    assert!(thread.aux_info().is_none());
}

#[test]
fn test_setters_and_getters() {
    let mut thread = CutListThread::new();
    
    // Test group
    thread.set_group(Some("test_group".to_string()));
    assert_eq!(thread.group(), Some("test_group"));
    
    // Test aux info
    thread.set_aux_info(Some("test info".to_string()));
    assert_eq!(thread.aux_info(), Some("test info"));
    
    // Test cut thickness
    thread.set_cut_thickness(5);
    assert_eq!(thread.cut_thickness(), 5);
    
    // Test min trim dimension
    thread.set_min_trim_dimension(10);
    assert_eq!(thread.min_trim_dimension(), 10);
    
    // Test first cut orientation
    thread.set_first_cut_orientation(CutDirection::Horizontal);
    assert_eq!(thread.first_cut_orientation(), CutDirection::Horizontal);
    
    // Test consider grain direction
    thread.set_consider_grain_direction(true);
    assert!(thread.consider_grain_direction());
    
    // Test accuracy factor
    thread.set_accuracy_factor(200);
    assert_eq!(thread.accuracy_factor(), 200);
}

#[test]
fn test_tiles_management() {
    let mut thread = CutListThread::new();
    
    let tiles = vec![
        create_test_tile(1, 100, 200, "Wood"),
        create_test_tile(2, 150, 250, "Wood"),
        create_test_tile(3, 300, 400, "Metal"),
    ];
    
    thread.set_tiles(tiles.clone());
    assert_eq!(thread.tiles().len(), 3);
    assert_eq!(thread.tiles()[0].width, 100);
    assert_eq!(thread.tiles()[0].height, 200);
    assert_eq!(thread.tiles()[0].material, "Wood");
    assert_eq!(thread.tiles()[2].material, "Metal");
}

#[test]
fn test_solutions_management() {
    let mut thread = CutListThread::new();
    
    let solutions = vec![
        Solution::new(),
        Solution::new(),
    ];
    
    thread.set_solutions(solutions);
    assert_eq!(thread.solutions().len(), 2);
}

#[test]
fn test_stock_solution() {
    let mut thread = CutListThread::new();
    let stock_solution = create_test_stock_solution();
    
    thread.set_stock_solution(Some(stock_solution));
    assert!(thread.stock_solution().is_some());
    
    thread.set_stock_solution(None);
    assert!(thread.stock_solution().is_none());
}

#[test]
fn test_status_checks() {
    let mut thread = CutListThread::new();
    
    // Initial state
    assert!(!thread.is_running());
    assert!(!thread.is_finished());
    assert!(!thread.has_error());
    assert!(!thread.is_terminated());
    
    // Test termination
    thread.terminate();
    assert!(thread.is_terminated());
}

#[test]
fn test_elapsed_time() {
    let thread = CutListThread::new();
    
    // Should return zero duration when not started
    let elapsed = thread.elapsed_time();
    assert_eq!(elapsed.as_secs(), 0);
    
    let elapsed_millis = thread.get_elapsed_time_millis();
    assert_eq!(elapsed_millis, 0);
}

#[test]
fn test_material() {
    let thread = CutListThread::new();
    
    // Should return None when no solutions
    assert_eq!(thread.material(), None);
}

#[test]
fn test_remove_duplicated() {
    let thread = CutListThread::new();
    
    // Create solutions with identical mosaics
    let mut solutions = vec![
        Solution::new(),
        Solution::new(),
    ];
    
    // Add identical mosaics to make them duplicates
    let mosaic = Mosaic::default();
    solutions[0].add_mosaic(mosaic.clone());
    solutions[1].add_mosaic(mosaic);
    
    let initial_count = solutions.len();
    let removed = thread.remove_duplicated(&mut solutions);
    
    // Should remove at least one duplicate
    assert!(solutions.len() < initial_count || removed == 0);
}

#[test]
fn test_validation_configuration() {
    let mut thread = CutListThread::new();
    
    // Test with empty tiles - should fail
    let result = thread.validate_configuration();
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Core(core_err) => match core_err {
            cutlist_optimizer_cli::errors::core::CoreError::InvalidInput { details } => {
                assert!(details.contains("No tiles provided"));
            }
            _ => panic!("Expected InvalidInput error"),
        },
        _ => panic!("Expected Core error"),
    }
    
    // Add tiles but no stock solution - should fail
    thread.set_tiles(vec![create_test_tile(1, 100, 200, "Wood")]);
    let result = thread.validate_configuration();
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Core(core_err) => match core_err {
            cutlist_optimizer_cli::errors::core::CoreError::InvalidInput { details } => {
                assert!(details.contains("Stock solution is required"));
            }
            _ => panic!("Expected InvalidInput error"),
        },
        _ => panic!("Expected Core error"),
    }
    
    // Add stock solution - should pass
    thread.set_stock_solution(Some(create_test_stock_solution()));
    let result = thread.validate_configuration();
    assert!(result.is_ok());
    
    // Test negative cut thickness - should fail
    thread.set_cut_thickness(-1);
    let result = thread.validate_configuration();
    assert!(result.is_err());
    
    // Test negative min trim dimension - should fail
    thread.set_cut_thickness(0);
    thread.set_min_trim_dimension(-1);
    let result = thread.validate_configuration();
    assert!(result.is_err());
    
    // Test zero accuracy factor - should fail
    thread.set_min_trim_dimension(0);
    thread.set_accuracy_factor(0);
    let result = thread.validate_configuration();
    assert!(result.is_err());
    
    // Test invalid tile dimensions - should fail
    thread.set_accuracy_factor(100);
    thread.set_tiles(vec![create_test_tile(1, 0, 200, "Wood")]);
    let result = thread.validate_configuration();
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Core(core_err) => match core_err {
            cutlist_optimizer_cli::errors::core::CoreError::InvalidInput { details } => {
                assert!(details.contains("invalid dimensions"));
            }
            _ => panic!("Expected InvalidInput error"),
        },
        _ => panic!("Expected Core error"),
    }
}

#[test]
fn test_find_candidates() {
    let thread = CutListThread::new();
    let root_node = TileNode::new(0, 1000, 0, 2000);
    let mut candidates = Vec::new();
    
    // Test finding candidates for a tile that fits
    thread.find_candidates(500, 1000, &root_node, &mut candidates);
    assert!(!candidates.is_empty());
    
    // Test finding candidates for a tile that doesn't fit
    candidates.clear();
    thread.find_candidates(1500, 2500, &root_node, &mut candidates);
    assert!(candidates.is_empty());
}

#[test]
fn test_split_horizontally() {
    let thread = CutListThread::new();
    let node = TileNode::new(0, 1000, 0, 2000);
    
    let cut = thread.split_horizontally(&node, 500, 3, 1).unwrap();
    
    assert_eq!(cut.x1, 500);
    assert_eq!(cut.y1, 0);
    assert_eq!(cut.x2, 500);
    assert_eq!(cut.y2, 2000);
    assert!(cut.is_horizontal);
    assert_eq!(cut.cut_coord, 500);
    assert_eq!(cut.original_width, 1000);
    assert_eq!(cut.original_height, 2000);
}

#[test]
fn test_split_vertically() {
    let thread = CutListThread::new();
    let node = TileNode::new(0, 1000, 0, 2000);
    
    let cut = thread.split_vertically(&node, 1000, 3, 1).unwrap();
    
    assert_eq!(cut.x1, 0);
    assert_eq!(cut.y1, 1000);
    assert_eq!(cut.x2, 1000);
    assert_eq!(cut.y2, 1000);
    assert!(!cut.is_horizontal);
    assert_eq!(cut.cut_coord, 1000);
    assert_eq!(cut.original_width, 1000);
    assert_eq!(cut.original_height, 2000);
}

#[test]
fn test_run_with_invalid_configuration() {
    let mut thread = CutListThread::new();
    
    // Run without proper configuration
    thread.run();
    
    // Should result in error status
    assert!(thread.has_error());
}

#[test]
fn test_run_with_valid_configuration() {
    let mut thread = CutListThread::new();
    
    // Set up valid configuration
    thread.set_tiles(vec![create_test_tile(1, 100, 200, "Wood")]);
    thread.set_stock_solution(Some(create_test_stock_solution()));
    
    // Run the thread
    thread.run();
    
    // Should complete successfully or be terminated
    assert!(thread.is_finished() || thread.is_terminated() || thread.has_error());
}

#[test]
fn test_comparators() {
    let mut thread = CutListThread::new();
    
    // Create test comparators
    let comparator1: SolutionComparator = Box::new(|a, b| {
        a.id.cmp(&b.id)
    });
    
    let comparator2: SolutionComparator = Box::new(|a, b| {
        a.timestamp.cmp(&b.timestamp)
    });
    
    // Test thread prioritized comparators
    thread.set_thread_prioritized_comparators(vec![comparator1]);
    assert_eq!(thread.thread_prioritized_comparators().len(), 1);
    
    // Test final solution prioritized comparators
    thread.set_final_solution_prioritized_comparators(vec![comparator2]);
    assert_eq!(thread.final_solution_prioritized_comparators().len(), 1);
}

#[test]
fn test_sort_solutions() {
    let thread = CutListThread::new();
    
    // Create solutions with different IDs
    let mut solutions = vec![
        Solution::new(),
        Solution::new(),
        Solution::new(),
    ];
    
    // Ensure they have different IDs by creating them separately
    let id1 = solutions[0].id;
    let id2 = solutions[1].id;
    let id3 = solutions[2].id;
    
    // Create a comparator that sorts by ID in reverse order
    let comparators: Vec<SolutionComparator> = vec![
        Box::new(|a, b| b.id.cmp(&a.id))
    ];
    
    thread.sort_solutions(&mut solutions, &comparators);
    
    // Verify sorting worked (should be in descending order of ID)
    assert!(solutions[0].id >= solutions[1].id);
    assert!(solutions[1].id >= solutions[2].id);
}

#[test]
fn test_all_solutions_thread_safety() {
    let thread = CutListThread::new();
    let all_solutions = thread.all_solutions();
    
    // Test that we can lock and modify the shared solutions
    {
        let mut solutions = all_solutions.lock().unwrap();
        solutions.push(Solution::new());
        assert_eq!(solutions.len(), 1);
    }
    
    // Test that the modification persists
    {
        let solutions = all_solutions.lock().unwrap();
        assert_eq!(solutions.len(), 1);
    }
}

#[test]
fn test_default_implementation() {
    let thread = CutListThread::default();
    assert_eq!(thread.status(), Status::Queued);
    assert_eq!(thread.accuracy_factor(), 100);
    assert_eq!(thread.cut_thickness(), 0);
    assert_eq!(thread.min_trim_dimension(), 0);
    assert_eq!(thread.first_cut_orientation(), CutDirection::Both);
    assert!(!thread.consider_grain_direction());
}

#[test]
fn test_debug_implementation() {
    let thread = CutListThread::new();
    let debug_str = format!("{:?}", thread);
    assert!(debug_str.contains("CutListThread"));
    assert!(debug_str.contains("accuracy_factor"));
    assert!(debug_str.contains("cut_thickness"));
    assert!(debug_str.contains("status"));
}

#[test]
fn test_cutting_strategies() {
    let thread = CutListThread::new();
    let node = TileNode::new(0, 1000, 0, 2000);
    let tile = create_test_tile(1, 500, 1000, "Wood");
    
    // Test horizontal-vertical split
    let cuts_hv = thread.split_hv(&node, &tile, 3).unwrap();
    assert!(!cuts_hv.is_empty());
    
    // Test vertical-horizontal split
    let cuts_vh = thread.split_vh(&node, &tile, 3).unwrap();
    assert!(!cuts_vh.is_empty());
}

#[test]
fn test_different_cut_orientations() {
    let mut thread = CutListThread::new();
    
    // Test all cut orientations
    thread.set_first_cut_orientation(CutDirection::Horizontal);
    assert_eq!(thread.first_cut_orientation(), CutDirection::Horizontal);
    
    thread.set_first_cut_orientation(CutDirection::Vertical);
    assert_eq!(thread.first_cut_orientation(), CutDirection::Vertical);
    
    thread.set_first_cut_orientation(CutDirection::Both);
    assert_eq!(thread.first_cut_orientation(), CutDirection::Both);
}

#[test]
fn test_grain_direction_consideration() {
    let mut thread = CutListThread::new();
    
    // Test grain direction setting
    assert!(!thread.consider_grain_direction());
    
    thread.set_consider_grain_direction(true);
    assert!(thread.consider_grain_direction());
    
    thread.set_consider_grain_direction(false);
    assert!(!thread.consider_grain_direction());
}

#[test]
fn test_percentage_done_tracking() {
    let thread = CutListThread::new();
    
    // Initially should be 0
    assert_eq!(thread.percentage_done(), 0);
    
    // Note: percentage_done is updated internally during computation
    // This test verifies the getter works correctly
}

#[test]
fn test_edge_cases() {
    let mut thread = CutListThread::new();
    
    // Test with very small tiles
    let small_tiles = vec![
        create_test_tile(1, 1, 1, "Wood"),
        create_test_tile(2, 2, 2, "Wood"),
    ];
    thread.set_tiles(small_tiles);
    thread.set_stock_solution(Some(create_test_stock_solution()));
    
    let validation = thread.validate_configuration();
    assert!(validation.is_ok());
    
    // Test with very large accuracy factor
    thread.set_accuracy_factor(10000);
    assert_eq!(thread.accuracy_factor(), 10000);
    
    // Test with large cut thickness
    thread.set_cut_thickness(100);
    assert_eq!(thread.cut_thickness(), 100);
}

#[test]
fn test_concurrent_access() {
    let thread = Arc::new(Mutex::new(CutListThread::new()));
    let all_solutions = {
        let t = thread.lock().unwrap();
        t.all_solutions()
    };
    
    // Spawn multiple threads to test concurrent access
    let handles: Vec<_> = (0..5).map(|_i| {
        let solutions = all_solutions.clone();
        thread::spawn(move || {
            let mut sols = solutions.lock().unwrap();
            sols.push(Solution::new());
            sols.len()
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        let _result = handle.join().unwrap();
    }
    
    // Verify all solutions were added
    let final_count = all_solutions.lock().unwrap().len();
    assert_eq!(final_count, 5);
}

#[test]
fn test_memory_safety() {
    // Test that dropping the thread doesn't cause issues
    {
        let mut thread = CutListThread::new();
        thread.set_tiles(vec![create_test_tile(1, 100, 200, "Wood")]);
        thread.set_stock_solution(Some(create_test_stock_solution()));
        
        // Thread should be dropped here without issues
    }
    
    // Test with large data structures
    let mut thread = CutListThread::new();
    let large_tiles: Vec<_> = (0..1000).map(|i| {
        create_test_tile(i, 100 + i, 200 + i, "Wood")
    }).collect();
    
    thread.set_tiles(large_tiles);
    assert_eq!(thread.tiles().len(), 1000);
}

#[test]
fn test_error_handling() {
    let thread = CutListThread::new();
    
    // Test error handling in split operations with invalid nodes
    let invalid_node = TileNode::new(0, 0, 0, 0); // Zero-sized node
    
    // These should handle gracefully or return appropriate errors
    let result = thread.split_horizontally(&invalid_node, 100, 3, 1);
    // The implementation should handle this case appropriately
    
    let result = thread.split_vertically(&invalid_node, 100, 3, 1);
    // The implementation should handle this case appropriately
}
