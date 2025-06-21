use cutlist_optimizer_cli::models::{Cut, CutBuilder};

#[cfg(test)]
mod cut_tests {
    use super::*;

    #[test]
    fn test_cut_new() {
        let cut = Cut::new(0, 0, 100, 50, 100, 50, true, 25, 1, 2, 3);
        
        assert_eq!(cut.x1(), 0);
        assert_eq!(cut.y1(), 0);
        assert_eq!(cut.x2(), 100);
        assert_eq!(cut.y2(), 50);
        assert_eq!(cut.original_width(), 100);
        assert_eq!(cut.original_height(), 50);
        assert_eq!(cut.is_horizontal(), true);
        assert_eq!(cut.cut_coord(), 25);
        assert_eq!(cut.original_tile_id(), 1);
        assert_eq!(cut.child1_tile_id(), 2);
        assert_eq!(cut.child2_tile_id(), 3);
    }

    #[test]
    fn test_cut_from_cut() {
        let original = Cut::new(10, 20, 110, 70, 100, 50, false, 60, 5, 6, 7);
        let copy = Cut::from_cut(&original);
        
        assert_eq!(copy.x1(), original.x1());
        assert_eq!(copy.y1(), original.y1());
        assert_eq!(copy.x2(), original.x2());
        assert_eq!(copy.y2(), original.y2());
        assert_eq!(copy.original_width(), original.original_width());
        assert_eq!(copy.original_height(), original.original_height());
        assert_eq!(copy.is_horizontal(), original.is_horizontal());
        assert_eq!(copy.cut_coord(), original.cut_coord());
        assert_eq!(copy.original_tile_id(), original.original_tile_id());
        assert_eq!(copy.child1_tile_id(), original.child1_tile_id());
        assert_eq!(copy.child2_tile_id(), original.child2_tile_id());
    }

    #[test]
    fn test_cut_length() {
        // Horizontal cut: length should be width
        let horizontal_cut = Cut::new(0, 0, 100, 50, 100, 50, true, 25, 1, 2, 3);
        assert_eq!(horizontal_cut.length(), 150); // |100-0| + |50-0| = 100 + 50 = 150
        
        // Vertical cut: length should be height
        let vertical_cut = Cut::new(0, 0, 50, 100, 50, 100, false, 25, 1, 2, 3);
        assert_eq!(vertical_cut.length(), 150); // |50-0| + |100-0| = 50 + 100 = 150
    }

    #[test]
    fn test_cut_builder() {
        let cut = Cut::builder()
            .set_x1(5)
            .set_y1(10)
            .set_x2(105)
            .set_y2(60)
            .set_original_width(100)
            .set_original_height(50)
            .set_horizontal(true)
            .set_cut_coord(30)
            .set_original_tile_id(8)
            .set_child1_tile_id(9)
            .set_child2_tile_id(10)
            .build();
        
        assert_eq!(cut.x1(), 5);
        assert_eq!(cut.y1(), 10);
        assert_eq!(cut.x2(), 105);
        assert_eq!(cut.y2(), 60);
        assert_eq!(cut.original_width(), 100);
        assert_eq!(cut.original_height(), 50);
        assert_eq!(cut.is_horizontal(), true);
        assert_eq!(cut.cut_coord(), 30);
        assert_eq!(cut.original_tile_id(), 8);
        assert_eq!(cut.child1_tile_id(), 9);
        assert_eq!(cut.child2_tile_id(), 10);
    }

    #[test]
    fn test_cut_builder_getters() {
        let builder = CutBuilder::new()
            .set_x1(15)
            .set_y1(25)
            .set_original_width(200);
        
        assert_eq!(builder.x1(), 15);
        assert_eq!(builder.y1(), 25);
        assert_eq!(builder.original_width(), 200);
        assert_eq!(builder.x2(), 0); // default value
        assert_eq!(builder.is_horizontal(), false); // default value
    }

    #[test]
    fn test_cut_default() {
        let cut = Cut::default();
        
        assert_eq!(cut.x1(), 0);
        assert_eq!(cut.y1(), 0);
        assert_eq!(cut.x2(), 0);
        assert_eq!(cut.y2(), 0);
        assert_eq!(cut.original_width(), 0);
        assert_eq!(cut.original_height(), 0);
        assert_eq!(cut.is_horizontal(), false);
        assert_eq!(cut.cut_coord(), 0);
        assert_eq!(cut.original_tile_id(), 0);
        assert_eq!(cut.child1_tile_id(), 0);
        assert_eq!(cut.child2_tile_id(), 0);
    }

    #[test]
    fn test_cut_display() {
        let cut = Cut::new(0, 0, 100, 50, 100, 50, true, 25, 1, 2, 3);
        let display_string = format!("{}", cut);
        
        assert!(display_string.contains("Cut["));
        assert!(display_string.contains("(0, 0) -> (100, 50)"));
        assert!(display_string.contains("100x50"));
        assert!(display_string.contains("horizontal"));
        assert!(display_string.contains("cut at 25"));
        assert!(display_string.contains("tile 1 -> [2, 3]"));
    }

    #[test]
    fn test_cut_clone_and_equality() {
        let cut1 = Cut::new(0, 0, 100, 50, 100, 50, true, 25, 1, 2, 3);
        let cut2 = cut1.clone();
        
        assert_eq!(cut1, cut2);
        assert_ne!(&cut1 as *const _, &cut2 as *const _); // Different memory addresses
    }

    #[test]
    fn test_cut_serialization() {
        let cut = Cut::new(0, 0, 100, 50, 100, 50, true, 25, 1, 2, 3);
        
        // Test that serialization works (this will panic if serde traits are not properly implemented)
        let serialized = serde_json::to_string(&cut).expect("Failed to serialize Cut");
        let deserialized: Cut = serde_json::from_str(&serialized).expect("Failed to deserialize Cut");
        
        assert_eq!(cut, deserialized);
    }
}
