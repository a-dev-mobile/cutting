use cutlist_optimizer_cli::models::{Mosaic, TileDimensions, TileNode};

#[cfg(test)]
mod mosaic_tests {
    use cutlist_optimizer_cli::Orientation;

    use super::*;

    #[test]
    fn test_mosaic_from_tile_dimensions() {
        let tile_dimensions = TileDimensions {
            id: 1,
            width: 100,
            height: 200,
            label: Some("Test Tile".to_string()),
            material: "Wood".to_string(),
            orientation: Orientation::Horizontal,
            is_rotated: false,
        };

        let mosaic = Mosaic::from_tile_dimensions(&tile_dimensions);

        assert_eq!(mosaic.stock_id(), 1);
        assert_eq!(mosaic.material(), "Wood");
        assert_eq!(mosaic.orientation(), Orientation::Horizontal);
        assert_eq!(mosaic.width(), 100);
        assert_eq!(mosaic.height(), 200);
        assert_eq!(mosaic.nbr_cuts(), 0);
        assert_eq!(mosaic.total_area(), 20000);
    }

    #[test]
    fn test_mosaic_from_tile_node() {
        let tile_node = TileNode::new(0, 100, 0, 200);
        let material = "Metal".to_string();

        let mosaic = Mosaic::from_tile_node(&tile_node, material);

        assert_eq!(mosaic.material(), "Metal");
        assert_eq!(mosaic.orientation(), Orientation::Any);
        assert_eq!(mosaic.width(), 100);
        assert_eq!(mosaic.height(), 200);
        assert_eq!(mosaic.nbr_cuts(), 0);
    }

    #[test]
    fn test_mosaic_copy_constructor() {
        let tile_dimensions = TileDimensions {
            id: 2,
            width: 150,
            height: 300,
            label: None,
            material: "Plastic".to_string(),
            orientation: Orientation::Vertical,
            is_rotated: false,
        };

        let original = Mosaic::from_tile_dimensions(&tile_dimensions);
        let copy = Mosaic::from_mosaic(&original);

        assert_eq!(original.stock_id(), copy.stock_id());
        assert_eq!(original.material(), copy.material());
        assert_eq!(original.orientation(), copy.orientation());
        assert_eq!(original.width(), copy.width());
        assert_eq!(original.height(), copy.height());
        assert_eq!(original, copy); // Test PartialEq implementation
    }

    #[test]
    fn test_mosaic_setters_and_getters() {
        let mut mosaic = Mosaic::default();

        mosaic.set_stock_id(42);
        mosaic.set_material("Glass".to_string());
        mosaic.set_orientation(Orientation::Horizontal);

        assert_eq!(mosaic.stock_id(), 42);
        assert_eq!(mosaic.material(), "Glass");
        assert_eq!(mosaic.orientation(), Orientation::Horizontal);
    }

    #[test]
    fn test_mosaic_cuts_management() {
        use cutlist_optimizer_cli::models::CutBuilder;

        let mut mosaic = Mosaic::default();
        assert!(!mosaic.has_cuts());
        assert_eq!(mosaic.nbr_cuts(), 0);

        let cut = CutBuilder::new()
            .set_x1(0).set_y1(0).set_x2(100).set_y2(50)
            .set_original_width(100).set_original_height(100)
            .set_horizontal(true).set_cut_coord(50)
            .set_original_tile_id(1).set_child1_tile_id(2).set_child2_tile_id(3)
            .build();

        mosaic.add_cut(cut);
        assert!(mosaic.has_cuts());
        assert_eq!(mosaic.nbr_cuts(), 1);

        mosaic.clear_cuts();
        assert!(!mosaic.has_cuts());
        assert_eq!(mosaic.nbr_cuts(), 0);
    }

    #[test]
    fn test_mosaic_area_calculations() {
        let tile_dimensions = TileDimensions {
            id: 3,
            width: 100,
            height: 100,
            label: None,
            material: "Wood".to_string(),
            orientation: Orientation::Any,
            is_rotated: false,
        };

        let mut mosaic = Mosaic::from_tile_dimensions(&tile_dimensions);
        
        assert_eq!(mosaic.total_area(), 10000);
        // Initially no final tiles, so used area should be 0
        assert_eq!(mosaic.used_area(), 0);
        assert_eq!(mosaic.unused_area(), 10000);
        assert_eq!(mosaic.efficiency(), 0.0);
        assert_eq!(mosaic.waste_ratio(), 1.0);
    }

    #[test]
    fn test_mosaic_hv_diff() {
        let mosaic = Mosaic::default();
        // With no final tiles, both horizontal and vertical counts are 0
        assert_eq!(mosaic.hv_diff(), 0.0);
    }

    #[test]
    fn test_mosaic_center_of_mass_distance() {
        let mosaic = Mosaic::default();
        // With no used area, distance should be 0
        assert_eq!(mosaic.center_of_mass_distance_to_origin(), 0.0);
    }

    #[test]
    fn test_mosaic_biggest_unused_tile() {
        let mosaic = Mosaic::default();
        // Default mosaic should have one unused tile (the root)
        let biggest = mosaic.biggest_unused_tile();
        assert!(biggest.is_some());
    }

    #[test]
    fn test_mosaic_depth() {
        let mosaic = Mosaic::default();
        // Default mosaic has no cuts, so depth should be 0
        assert_eq!(mosaic.depth(), 0);
    }

    #[test]
    fn test_mosaic_tile_counts() {
        let mosaic = Mosaic::default();
        
        assert_eq!(mosaic.final_tile_count(), 0);
        assert_eq!(mosaic.unused_tile_count(), 1); // Root tile is unused
        assert!(!mosaic.has_final_tiles());
    }

    #[test]
    fn test_mosaic_to_tile_dimensions() {
        let original_dimensions = TileDimensions {
            id: 5,
            width: 200,
            height: 150,
            label: Some("Original".to_string()),
            material: "Aluminum".to_string(),
            orientation: Orientation::Vertical,
            is_rotated: true,
        };

        let mosaic = Mosaic::from_tile_dimensions(&original_dimensions);
        let converted = mosaic.to_tile_dimensions();

        assert_eq!(converted.id, 5);
        assert_eq!(converted.width, 200);
        assert_eq!(converted.height, 150);
        assert_eq!(converted.material, "Aluminum");
        assert_eq!(converted.orientation, Orientation::Vertical);
        assert_eq!(converted.is_rotated, false); // Mosaic doesn't track rotation
        assert_eq!(converted.label, None); // Mosaic doesn't preserve label
    }

    #[test]
    fn test_mosaic_display() {
        let tile_dimensions = TileDimensions {
            id: 10,
            width: 50,
            height: 60,
            label: None,
            material: "TestMaterial".to_string(),
            orientation: Orientation::Any,
            is_rotated: false,
        };

        let mosaic = Mosaic::from_tile_dimensions(&tile_dimensions);
        let display_string = format!("{}", mosaic);
        
        assert!(display_string.contains("stock_id=10"));
        assert!(display_string.contains("material=TestMaterial"));
        assert!(display_string.contains("cuts=0"));
        assert!(display_string.contains("area=3000"));
    }

    #[test]
    fn test_mosaic_equality() {
        let tile_dimensions = TileDimensions {
            id: 1,
            width: 100,
            height: 100,
            label: None,
            material: "Wood".to_string(),
            orientation: Orientation::Any,
            is_rotated: false,
        };

        let mosaic1 = Mosaic::from_tile_dimensions(&tile_dimensions);
        let mosaic2 = Mosaic::from_mosaic(&mosaic1); // Use copy constructor to get same TileNode ID

        // They should be equal because they have the same root tile node structure
        assert_eq!(mosaic1, mosaic2);

        // Create a different mosaic
        let different_dimensions = TileDimensions {
            id: 2,
            width: 200,
            height: 100,
            label: None,
            material: "Wood".to_string(),
            orientation: Orientation::Any,
            is_rotated: false,
        };

        let mosaic3 = Mosaic::from_tile_dimensions(&different_dimensions);
        assert_ne!(mosaic1, mosaic3);
    }
}
