use cutlist_optimizer_cli::models::{Tile, TileDimensions};

#[cfg(test)]
mod tile_tests {
    use super::*;

    #[test]
    fn test_tile_from_dimensions() {
        let tile_dimensions = TileDimensions::new(1, 100, 50);
        let tile = Tile::from_dimensions(&tile_dimensions);
        
        assert_eq!(tile.x1(), 0);
        assert_eq!(tile.x2(), 100);
        assert_eq!(tile.y1(), 0);
        assert_eq!(tile.y2(), 50);
        assert_eq!(tile.width(), 100);
        assert_eq!(tile.height(), 50);
    }

    #[test]
    fn test_tile_new() {
        let tile = Tile::new(10, 110, 20, 70);
        
        assert_eq!(tile.x1(), 10);
        assert_eq!(tile.x2(), 110);
        assert_eq!(tile.y1(), 20);
        assert_eq!(tile.y2(), 70);
        assert_eq!(tile.width(), 100);
        assert_eq!(tile.height(), 50);
    }

    #[test]
    fn test_tile_from_tile() {
        let original = Tile::new(5, 15, 10, 20);
        let copy = Tile::from_tile(&original);
        
        assert_eq!(original, copy);
        assert_eq!(copy.x1(), 5);
        assert_eq!(copy.x2(), 15);
        assert_eq!(copy.y1(), 10);
        assert_eq!(copy.y2(), 20);
    }

    #[test]
    fn test_tile_dimensions() {
        let tile = Tile::new(0, 100, 0, 50);
        
        assert_eq!(tile.width(), 100);
        assert_eq!(tile.height(), 50);
        assert_eq!(tile.area(), 5000);
        assert_eq!(tile.max_side(), 100);
        assert_eq!(tile.min_side(), 50);
    }

    #[test]
    fn test_tile_orientation() {
        let horizontal_tile = Tile::new(0, 100, 0, 50);
        assert!(horizontal_tile.is_horizontal());
        assert!(!horizontal_tile.is_vertical());
        assert!(!horizontal_tile.is_square());

        let vertical_tile = Tile::new(0, 50, 0, 100);
        assert!(!vertical_tile.is_horizontal());
        assert!(vertical_tile.is_vertical());
        assert!(!vertical_tile.is_square());

        let square_tile = Tile::new(0, 50, 0, 50);
        assert!(!square_tile.is_horizontal());
        assert!(!square_tile.is_vertical());
        assert!(square_tile.is_square());
    }

    #[test]
    fn test_tile_equality() {
        let tile1 = Tile::new(0, 100, 0, 50);
        let tile2 = Tile::new(0, 100, 0, 50);
        let tile3 = Tile::new(10, 110, 0, 50);
        
        assert_eq!(tile1, tile2);
        assert_ne!(tile1, tile3);
    }

    #[test]
    fn test_tile_contains_point() {
        let tile = Tile::new(10, 20, 5, 15);
        
        assert!(tile.contains_point(15, 10));
        assert!(tile.contains_point(10, 5));
        assert!(!tile.contains_point(20, 10)); // x2 is exclusive
        assert!(!tile.contains_point(15, 15)); // y2 is exclusive
        assert!(!tile.contains_point(5, 10));  // outside x1
        assert!(!tile.contains_point(15, 3));  // outside y1
    }

    #[test]
    fn test_tile_overlaps() {
        let tile1 = Tile::new(0, 10, 0, 10);
        let tile2 = Tile::new(5, 15, 5, 15);
        let tile3 = Tile::new(10, 20, 10, 20);
        
        assert!(tile1.overlaps_with(&tile2));
        assert!(tile2.overlaps_with(&tile1));
        assert!(!tile1.overlaps_with(&tile3));
        assert!(!tile3.overlaps_with(&tile1));
    }

    #[test]
    fn test_tile_translation() {
        let mut tile = Tile::new(0, 10, 0, 5);
        tile.translate(5, 3);
        
        assert_eq!(tile.x1(), 5);
        assert_eq!(tile.x2(), 15);
        assert_eq!(tile.y1(), 3);
        assert_eq!(tile.y2(), 8);
        
        let translated = tile.translated(-2, 1);
        assert_eq!(translated.x1(), 3);
        assert_eq!(translated.x2(), 13);
        assert_eq!(translated.y1(), 4);
        assert_eq!(translated.y2(), 9);
        
        // Original should be unchanged
        assert_eq!(tile.x1(), 5);
        assert_eq!(tile.y1(), 3);
    }

    #[test]
    fn test_tile_default() {
        let tile = Tile::default();
        assert_eq!(tile.x1(), 0);
        assert_eq!(tile.x2(), 0);
        assert_eq!(tile.y1(), 0);
        assert_eq!(tile.y2(), 0);
        assert_eq!(tile.width(), 0);
        assert_eq!(tile.height(), 0);
        assert_eq!(tile.area(), 0);
    }

    #[test]
    fn test_tile_display() {
        let tile = Tile::new(10, 20, 5, 15);
        let display_string = format!("{}", tile);
        assert_eq!(display_string, "Tile[(10, 5) -> (20, 15), 10x10]");
    }

    #[test]
    fn test_large_area_calculation() {
        // Test that area calculation uses i64 to prevent overflow
        let tile = Tile::new(0, 50000, 0, 50000);
        assert_eq!(tile.area(), 2_500_000_000_i64);
    }

    #[test]
    fn test_clone_and_hash() {
        let tile = Tile::new(1, 2, 3, 4);
        let cloned = tile.clone();
        
        assert_eq!(tile, cloned);
        
        // Test that it can be used in hash-based collections
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(tile);
        set.insert(cloned);
        assert_eq!(set.len(), 1); // Should be the same tile
    }
}
