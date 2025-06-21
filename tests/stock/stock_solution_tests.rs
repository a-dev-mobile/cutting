use cutlist_optimizer_cli::stock::StockSolution;
use cutlist_optimizer_cli::models::TileDimensions;

#[cfg(test)]
mod tests {
    use cutlist_optimizer_cli::Orientation;

    use super::*;

    fn create_test_tile(id: i32, width: i32, height: i32) -> TileDimensions {
        TileDimensions {
            id,
            width,
            height,
            label: None,
            material: "TEST".to_string(),
            orientation: Orientation::Any,
            is_rotated: false,
        }
    }

    #[test]
    fn test_new_stock_solution() {
        let solution = StockSolution::new();
        assert!(solution.is_empty());
        assert_eq!(solution.len(), 0);
    }

    #[test]
    fn test_from_tiles() {
        let tiles = vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 150, 300),
        ];
        let solution = StockSolution::from_tiles(tiles.clone());
        assert_eq!(solution.len(), 2);
        assert_eq!(solution.get_stock_tile_dimensions(), &tiles);
    }

    #[test]
    fn test_from_slice() {
        let tiles = vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 150, 300),
        ];
        let solution = StockSolution::from_slice(&tiles);
        assert_eq!(solution.len(), 2);
        assert_eq!(solution.get_stock_tile_dimensions(), &tiles);
    }

    #[test]
    fn test_add_stock_tile() {
        let mut solution = StockSolution::new();
        let tile = create_test_tile(1, 100, 200);
        
        solution.add_stock_tile(tile.clone());
        assert_eq!(solution.len(), 1);
        assert_eq!(solution.get_stock_tile_dimensions()[0], tile);
    }

    #[test]
    fn test_sort_panels_asc() {
        let mut solution = StockSolution::from_tiles(vec![
            create_test_tile(1, 200, 300), // area: 60000
            create_test_tile(2, 100, 200), // area: 20000
            create_test_tile(3, 150, 250), // area: 37500
        ]);

        solution.sort_panels_asc();
        let tiles = solution.get_stock_tile_dimensions();
        assert_eq!(tiles[0].area(), 20000);
        assert_eq!(tiles[1].area(), 37500);
        assert_eq!(tiles[2].area(), 60000);
    }

    #[test]
    fn test_sort_panels_desc() {
        let mut solution = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200), // area: 20000
            create_test_tile(2, 200, 300), // area: 60000
            create_test_tile(3, 150, 250), // area: 37500
        ]);

        solution.sort_panels_desc();
        let tiles = solution.get_stock_tile_dimensions();
        assert_eq!(tiles[0].area(), 60000);
        assert_eq!(tiles[1].area(), 37500);
        assert_eq!(tiles[2].area(), 20000);
    }

    #[test]
    fn test_has_unique_panel_size_true() {
        let solution = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 100, 200),
            create_test_tile(3, 200, 100), // Same dimensions, different orientation
        ]);

        assert!(solution.has_unique_panel_size());
    }

    #[test]
    fn test_has_unique_panel_size_false() {
        let solution = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 150, 300),
        ]);

        assert!(!solution.has_unique_panel_size());
    }

    #[test]
    fn test_has_unique_panel_size_empty() {
        let solution = StockSolution::new();
        assert!(solution.has_unique_panel_size());
    }

    #[test]
    fn test_get_total_area() {
        let solution = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200), // area: 20000
            create_test_tile(2, 150, 300), // area: 45000
        ]);

        assert_eq!(solution.get_total_area(), 65000);
    }

    #[test]
    fn test_to_string() {
        let solution = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 150, 300),
        ]);

        assert_eq!(solution.to_string(), "[100x200][150x300]");
    }

    #[test]
    fn test_to_string_grouped() {
        let solution = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 100, 200),
            create_test_tile(3, 150, 300),
        ]);

        let grouped = solution.to_string_grouped();
        // Should contain both sizes with counts
        assert!(grouped.contains("100x200*2"));
        assert!(grouped.contains("150x300*1"));
    }

    #[test]
    fn test_equality_same_tiles() {
        let solution1 = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 150, 300),
        ]);

        let solution2 = StockSolution::from_tiles(vec![
            create_test_tile(3, 100, 200),
            create_test_tile(4, 150, 300),
        ]);

        assert_eq!(solution1, solution2);
    }

    #[test]
    fn test_equality_different_order() {
        let solution1 = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 150, 300),
        ]);

        let solution2 = StockSolution::from_tiles(vec![
            create_test_tile(3, 150, 300),
            create_test_tile(4, 100, 200),
        ]);

        assert_eq!(solution1, solution2);
    }

    #[test]
    fn test_equality_rotated_tiles() {
        let solution1 = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200),
        ]);

        let solution2 = StockSolution::from_tiles(vec![
            create_test_tile(2, 200, 100), // Rotated version
        ]);

        assert_eq!(solution1, solution2);
    }

    #[test]
    fn test_inequality_different_sizes() {
        let solution1 = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200),
        ]);

        let solution2 = StockSolution::from_tiles(vec![
            create_test_tile(2, 150, 300),
        ]);

        assert_ne!(solution1, solution2);
    }

    #[test]
    fn test_inequality_different_counts() {
        let solution1 = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200),
        ]);

        let solution2 = StockSolution::from_tiles(vec![
            create_test_tile(2, 100, 200),
            create_test_tile(3, 100, 200),
        ]);

        assert_ne!(solution1, solution2);
    }

    #[test]
    fn test_iterator() {
        let tiles = vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 150, 300),
        ];
        let solution = StockSolution::from_tiles(tiles.clone());

        let collected: Vec<_> = solution.iter().collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(*collected[0], tiles[0]);
        assert_eq!(*collected[1], tiles[1]);
    }

    #[test]
    fn test_into_iterator() {
        let tiles = vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 150, 300),
        ];
        let solution = StockSolution::from_tiles(tiles.clone());

        let collected: Vec<_> = solution.into_iter().collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0], tiles[0]);
        assert_eq!(collected[1], tiles[1]);
    }

    #[test]
    fn test_from_vec_conversion() {
        let tiles = vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 150, 300),
        ];
        let solution: StockSolution = tiles.clone().into();
        assert_eq!(solution.get_stock_tile_dimensions(), &tiles);
    }

    #[test]
    fn test_from_slice_conversion() {
        let tiles = vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 150, 300),
        ];
        let solution: StockSolution = tiles.as_slice().into();
        assert_eq!(solution.get_stock_tile_dimensions(), &tiles);
    }

    #[test]
    fn test_display_trait() {
        let solution = StockSolution::from_tiles(vec![
            create_test_tile(1, 100, 200),
            create_test_tile(2, 150, 300),
        ]);

        let display_string = format!("{}", solution);
        assert_eq!(display_string, "[100x200][150x300]");
    }
}
