//! Tests for collection utilities
//!
//! This test suite validates the collection utility functions for tile processing.

use cutlist_optimizer_cli::engine::service::computation::grouping::CollectionUtils;
use cutlist_optimizer_cli::models::{
    tile_dimensions::structs::TileDimensions,
    task::structs::Task,
    enums::Orientation,
};

fn create_test_tile(id: i32, width: i32, height: i32, material: &str) -> TileDimensions {
    TileDimensions {
        id,
        width,
        height,
        label: None,
        material: material.to_string(),
        orientation: Orientation::Any,
        is_rotated: false,
    }
}

fn create_test_tile_with_label(id: i32, width: i32, height: i32, material: &str, label: &str) -> TileDimensions {
    TileDimensions {
        id,
        width,
        height,
        label: Some(label.to_string()),
        material: material.to_string(),
        orientation: Orientation::Any,
        is_rotated: false,
    }
}

fn create_test_task() -> Task {
    Task::new("test_task_123".to_string())
}

#[cfg(test)]
mod get_tile_dimensions_per_material_tests {
    use super::*;

    #[test]
    fn test_basic_grouping() {
        let tiles = vec![
            create_test_tile(1, 100, 200, "wood"),
            create_test_tile(2, 150, 250, "metal"),
            create_test_tile(3, 120, 180, "wood"),
            create_test_tile(4, 200, 300, "plastic"),
        ];

        let result = CollectionUtils::get_tile_dimensions_per_material(&tiles).unwrap();
        
        assert_eq!(result.len(), 3);
        assert_eq!(result.get("wood").unwrap().len(), 2);
        assert_eq!(result.get("metal").unwrap().len(), 1);
        assert_eq!(result.get("plastic").unwrap().len(), 1);
        
        // Verify wood tiles
        let wood_tiles = result.get("wood").unwrap();
        assert_eq!(wood_tiles[0].id, 1);
        assert_eq!(wood_tiles[1].id, 3);
    }

    #[test]
    fn test_single_material() {
        let tiles = vec![
            create_test_tile(1, 100, 200, "wood"),
            create_test_tile(2, 150, 250, "wood"),
            create_test_tile(3, 120, 180, "wood"),
        ];

        let result = CollectionUtils::get_tile_dimensions_per_material(&tiles).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("wood").unwrap().len(), 3);
    }

    #[test]
    fn test_empty_input() {
        let tiles = vec![];

        let result = CollectionUtils::get_tile_dimensions_per_material(&tiles);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_single_tile() {
        let tiles = vec![create_test_tile(1, 100, 200, "wood")];

        let result = CollectionUtils::get_tile_dimensions_per_material(&tiles).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("wood").unwrap().len(), 1);
    }

    #[test]
    fn test_material_name_variations() {
        let tiles = vec![
            create_test_tile(1, 100, 200, "Wood"),
            create_test_tile(2, 150, 250, "wood"),
            create_test_tile(3, 120, 180, "WOOD"),
        ];

        let result = CollectionUtils::get_tile_dimensions_per_material(&tiles).unwrap();
        
        // Should treat as different materials (case sensitive)
        assert_eq!(result.len(), 3);
        assert_eq!(result.get("Wood").unwrap().len(), 1);
        assert_eq!(result.get("wood").unwrap().len(), 1);
        assert_eq!(result.get("WOOD").unwrap().len(), 1);
    }
}

#[cfg(test)]
mod is_one_dimensional_optimization_tests {
    use super::*;

    #[test]
    fn test_true_case_shared_width() {
        let tiles = vec![
            create_test_tile(1, 100, 200, "wood"),
            create_test_tile(2, 100, 300, "wood"), // shares width=100
            create_test_tile(3, 100, 150, "wood"), // shares width=100
        ];
        let stock_tiles = vec![
            create_test_tile(4, 100, 400, "wood"), // shares width=100
            create_test_tile(5, 100, 500, "wood"), // shares width=100
        ];

        let result = CollectionUtils::is_one_dimensional_optimization(&tiles, &stock_tiles).unwrap();
        assert!(result);
    }

    #[test]
    fn test_true_case_shared_height() {
        let tiles = vec![
            create_test_tile(1, 200, 100, "wood"),
            create_test_tile(2, 300, 100, "wood"), // shares height=100
            create_test_tile(3, 150, 100, "wood"), // shares height=100
        ];
        let stock_tiles = vec![
            create_test_tile(4, 400, 100, "wood"), // shares height=100
            create_test_tile(5, 500, 100, "wood"), // shares height=100
        ];

        let result = CollectionUtils::is_one_dimensional_optimization(&tiles, &stock_tiles).unwrap();
        assert!(result);
    }

    #[test]
    fn test_true_case_mixed_orientation() {
        let tiles = vec![
            create_test_tile(1, 100, 200, "wood"),
            create_test_tile(2, 300, 100, "wood"), // shares dimension 100 (as height for first, width for second)
        ];
        let stock_tiles = vec![
            create_test_tile(3, 100, 400, "wood"), // shares dimension 100
        ];

        let result = CollectionUtils::is_one_dimensional_optimization(&tiles, &stock_tiles).unwrap();
        assert!(result);
    }

    #[test]
    fn test_false_case_no_shared_dimensions() {
        let tiles = vec![
            create_test_tile(1, 100, 200, "wood"),
            create_test_tile(2, 150, 250, "wood"), // no shared dimensions
        ];
        let stock_tiles = vec![
            create_test_tile(3, 300, 400, "wood"), // no shared dimensions
        ];

        let result = CollectionUtils::is_one_dimensional_optimization(&tiles, &stock_tiles).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_false_case_stock_breaks_pattern() {
        let tiles = vec![
            create_test_tile(1, 100, 200, "wood"),
            create_test_tile(2, 100, 300, "wood"), // shares width=100
        ];
        let stock_tiles = vec![
            create_test_tile(3, 100, 400, "wood"), // shares width=100
            create_test_tile(4, 500, 600, "wood"), // breaks the pattern
        ];

        let result = CollectionUtils::is_one_dimensional_optimization(&tiles, &stock_tiles).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_complex_java_algorithm_simulation() {
        // This test simulates the exact Java algorithm behavior
        
        // Start with dimensions [100, 200] from first tile
        let tiles = vec![
            create_test_tile(1, 100, 200, "wood"),
            create_test_tile(2, 150, 100, "wood"), // removes 200, keeps 100
            create_test_tile(3, 100, 250, "wood"), // keeps 100
        ];
        let stock_tiles = vec![
            create_test_tile(4, 100, 300, "wood"), // keeps 100
        ];

        let result = CollectionUtils::is_one_dimensional_optimization(&tiles, &stock_tiles).unwrap();
        assert!(result); // Should be true because 100 is shared by all
    }

    #[test]
    fn test_empty_tiles_error() {
        let tiles = vec![];
        let stock_tiles = vec![create_test_tile(1, 100, 200, "wood")];

        let result = CollectionUtils::is_one_dimensional_optimization(&tiles, &stock_tiles);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_empty_stock_tiles_error() {
        let tiles = vec![create_test_tile(1, 100, 200, "wood")];
        let stock_tiles = vec![];

        let result = CollectionUtils::is_one_dimensional_optimization(&tiles, &stock_tiles);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_single_tile_single_stock() {
        let tiles = vec![create_test_tile(1, 100, 200, "wood")];
        let stock_tiles = vec![create_test_tile(2, 100, 300, "wood")];

        let result = CollectionUtils::is_one_dimensional_optimization(&tiles, &stock_tiles).unwrap();
        assert!(result); // Shares width=100
    }
}

#[cfg(test)]
mod generate_groups_tests {
    use super::*;

    #[test]
    fn test_basic_grouping() {
        let tiles = vec![
            create_test_tile(1, 100, 200, "wood"),
            create_test_tile(2, 100, 200, "wood"), // duplicate
            create_test_tile(3, 150, 250, "wood"), // different
        ];
        let stock_tiles = vec![
            create_test_tile(4, 100, 200, "wood"),
        ];
        let task = create_test_task();

        let result = CollectionUtils::generate_groups(&tiles, &stock_tiles, &task).unwrap();
        assert_eq!(result.len(), 3);
        
        // All should be in group 0 initially (small dataset)
        for grouped_tile in &result {
            assert_eq!(grouped_tile.get_group(), 0);
        }
    }

    #[test]
    fn test_group_splitting_large_dataset() {
        // Create 200 identical tiles to trigger group splitting
        let mut tiles = Vec::new();
        for i in 0..200 {
            tiles.push(create_test_tile(i, 100, 200, "wood"));
        }
        
        let stock_tiles = vec![create_test_tile(1000, 100, 200, "wood")];
        let task = create_test_task();

        let result = CollectionUtils::generate_groups(&tiles, &stock_tiles, &task).unwrap();
        assert_eq!(result.len(), 200);
        
        // Should have multiple groups due to splitting logic
        let mut groups = std::collections::HashSet::new();
        for grouped_tile in &result {
            groups.insert(grouped_tile.get_group());
        }
        assert!(groups.len() > 1, "Expected multiple groups for large dataset");
    }

    #[test]
    fn test_one_dimensional_optimization_grouping() {
        // Create tiles that share a common dimension (one-dimensional)
        let tiles = vec![
            create_test_tile(1, 100, 200, "wood"),
            create_test_tile(2, 100, 300, "wood"), // shares width=100
            create_test_tile(3, 100, 150, "wood"), // shares width=100
        ];
        let stock_tiles = vec![
            create_test_tile(4, 100, 400, "wood"), // shares width=100
        ];
        let task = create_test_task();

        let result = CollectionUtils::generate_groups(&tiles, &stock_tiles, &task).unwrap();
        assert_eq!(result.len(), 3);
        
        // In one-dimensional optimization, group size limit is 1, so no splitting
        for grouped_tile in &result {
            assert_eq!(grouped_tile.get_group(), 0);
        }
    }

    #[test]
    fn test_tiles_with_different_properties() {
        let tiles = vec![
            create_test_tile_with_label(1, 100, 200, "wood", "label1"),
            create_test_tile_with_label(2, 100, 200, "wood", "label2"), // same dimensions, different label
            create_test_tile(3, 100, 200, "metal"), // same dimensions, different material
        ];
        let stock_tiles = vec![create_test_tile(4, 100, 200, "wood")];
        let task = create_test_task();

        let result = CollectionUtils::generate_groups(&tiles, &stock_tiles, &task).unwrap();
        assert_eq!(result.len(), 3);
        
        // Each should be treated as different due to different toString() representations
        // (different labels and materials create different keys)
    }

    #[test]
    fn test_empty_tiles_error() {
        let tiles = vec![];
        let stock_tiles = vec![create_test_tile(1, 100, 200, "wood")];
        let task = create_test_task();

        let result = CollectionUtils::generate_groups(&tiles, &stock_tiles, &task);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_empty_stock_tiles_error() {
        let tiles = vec![create_test_tile(1, 100, 200, "wood")];
        let stock_tiles = vec![];
        let task = create_test_task();

        let result = CollectionUtils::generate_groups(&tiles, &stock_tiles, &task);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_single_tile() {
        let tiles = vec![create_test_tile(1, 100, 200, "wood")];
        let stock_tiles = vec![create_test_tile(2, 100, 200, "wood")];
        let task = create_test_task();

        let result = CollectionUtils::generate_groups(&tiles, &stock_tiles, &task).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get_group(), 0);
        assert_eq!(result[0].id(), 1);
    }

    #[test]
    fn test_group_key_generation() {
        let tiles = vec![
            create_test_tile_with_label(1, 100, 200, "wood", "A"),
            create_test_tile_with_label(2, 100, 200, "wood", "B"), // Different label = different key
        ];
        let stock_tiles = vec![create_test_tile(3, 100, 200, "wood")];
        let task = create_test_task();

        let result = CollectionUtils::generate_groups(&tiles, &stock_tiles, &task).unwrap();
        assert_eq!(result.len(), 2);
        
        // Both should be in group 0 since they're treated as different tile types
        assert_eq!(result[0].get_group(), 0);
        assert_eq!(result[1].get_group(), 0);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_workflow_integration() {
        // Create a realistic dataset
        let tiles = vec![
            create_test_tile(1, 100, 200, "wood"),
            create_test_tile(2, 100, 200, "wood"), // duplicate
            create_test_tile(3, 150, 250, "metal"),
            create_test_tile(4, 100, 300, "wood"), // shares width with first two
        ];
        let stock_tiles = vec![
            create_test_tile(5, 100, 400, "wood"),
            create_test_tile(6, 200, 300, "metal"),
        ];
        let task = create_test_task();

        // Test material grouping
        let material_groups = CollectionUtils::get_tile_dimensions_per_material(&tiles).unwrap();
        assert_eq!(material_groups.len(), 2);
        assert_eq!(material_groups.get("wood").unwrap().len(), 3);
        assert_eq!(material_groups.get("metal").unwrap().len(), 1);

        // Test one-dimensional optimization check
        let wood_tiles = material_groups.get("wood").unwrap();
        let wood_stock: Vec<_> = stock_tiles.iter()
            .filter(|t| t.material == "wood")
            .cloned()
            .collect();
        
        let is_one_dim = CollectionUtils::is_one_dimensional_optimization(wood_tiles, &wood_stock).unwrap();
        assert!(is_one_dim); // Should be true because all share width=100

        // Test group generation
        let groups = CollectionUtils::generate_groups(wood_tiles, &wood_stock, &task).unwrap();
        assert_eq!(groups.len(), 3);
    }

    #[test]
    fn test_error_propagation() {
        let empty_tiles = vec![];
        let stock_tiles = vec![create_test_tile(1, 100, 200, "wood")];
        let task = create_test_task();

        // All methods should properly handle and propagate errors
        assert!(CollectionUtils::get_tile_dimensions_per_material(&empty_tiles).is_err());
        assert!(CollectionUtils::is_one_dimensional_optimization(&empty_tiles, &stock_tiles).is_err());
        assert!(CollectionUtils::generate_groups(&empty_tiles, &stock_tiles, &task).is_err());
    }

    #[test]
    fn test_performance_characteristics() {
        // Create a larger dataset to test performance
        let mut tiles = Vec::new();
        for i in 0..1000 {
            tiles.push(create_test_tile(i, 100 + (i % 10), 200 + (i % 5), "wood"));
        }
        let stock_tiles = vec![create_test_tile(10000, 100, 200, "wood")];
        let task = create_test_task();

        let start = std::time::Instant::now();
        
        // Test all methods with larger dataset
        let _material_groups = CollectionUtils::get_tile_dimensions_per_material(&tiles).unwrap();
        let _is_one_dim = CollectionUtils::is_one_dimensional_optimization(&tiles, &stock_tiles).unwrap();
        let _groups = CollectionUtils::generate_groups(&tiles, &stock_tiles, &task).unwrap();
        
        let duration = start.elapsed();
        
        // Should complete within reasonable time (adjust threshold as needed)
        assert!(duration.as_millis() < 1000, "Operations took too long: {:?}", duration);
    }
}
