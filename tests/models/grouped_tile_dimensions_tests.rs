#[cfg(test)]
mod grouped_tile_dimensions_tests {

    use cutlist_optimizer_cli::models::grouped_tile_dimensions::GroupedTileDimensions;
    use cutlist_optimizer_cli::models::tile_dimensions::TileDimensions;

    #[test]
    fn test_constructors() {
        // Test new constructor
        let grouped1 = GroupedTileDimensions::new(100, 200, 5);
        assert_eq!(grouped1.width(), 100);
        assert_eq!(grouped1.height(), 200);
        assert_eq!(grouped1.get_group(), 5);

        // Test from_tile_dimensions constructor
        let tile = TileDimensions::new(1, 150, 250);
        let grouped2 = GroupedTileDimensions::from_tile_dimensions(tile, 10);
        assert_eq!(grouped2.id(), 1);
        assert_eq!(grouped2.width(), 150);
        assert_eq!(grouped2.height(), 250);
        assert_eq!(grouped2.get_group(), 10);

        // Test copy constructor
        let grouped3 = GroupedTileDimensions::from_grouped(&grouped2);
        assert_eq!(grouped3, grouped2);
    }

    #[test]
    fn test_display() {
        let grouped = GroupedTileDimensions::with_id(42, 100, 200, 5);
        let display_str = format!("{}", grouped);
        assert_eq!(display_str, "id=42, gropup=5[100x200]");
    }

    #[test]
    fn test_equality() {
        let grouped1 = GroupedTileDimensions::new(100, 200, 5);
        let grouped2 = GroupedTileDimensions::new(100, 200, 5);
        let grouped3 = GroupedTileDimensions::new(100, 200, 6); // Different group
        let grouped4 = GroupedTileDimensions::new(150, 200, 5); // Different dimensions

        assert_eq!(grouped1, grouped2);
        assert_ne!(grouped1, grouped3);
        assert_ne!(grouped1, grouped4);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;

        let grouped1 = GroupedTileDimensions::new(100, 200, 5);
        let grouped2 = GroupedTileDimensions::new(100, 200, 5);
        let grouped3 = GroupedTileDimensions::new(100, 200, 6);

        let mut map = HashMap::new();
        map.insert(grouped1.clone(), "value1");

        // Same dimensions and group should find the same entry
        assert!(map.contains_key(&grouped2));

        // Different group should not find the entry
        assert!(!map.contains_key(&grouped3));
    }

    #[test]
    fn test_delegation() {
        let mut grouped = GroupedTileDimensions::new(100, 200, 5);

        assert_eq!(grouped.area(), 20000);
        assert!(grouped.can_rotate());

        grouped.rotate_90();
        assert_eq!(grouped.width(), 200);
        assert_eq!(grouped.height(), 100);
    }

    #[test]
    fn test_java_equivalence() {
        // Test that the Rust implementation behaves like the Java version
        let grouped1 = GroupedTileDimensions::with_id(1, 100, 200, 5);
        let grouped2 = GroupedTileDimensions::with_id(1, 100, 200, 5);
        let grouped3 = GroupedTileDimensions::with_id(2, 100, 200, 5); // Different id

        // Should be equal despite different object instances (like Java)
        assert_eq!(grouped1, grouped2);

        // Should not be equal due to different id
        assert_ne!(grouped1, grouped3);

        // Test toString equivalent
        let display_str = format!("{}", grouped1);
        assert!(display_str.contains("id=1"));
        assert!(display_str.contains("gropup=5")); // Note: keeping the typo from Java
        assert!(display_str.contains("[100x200]"));
    }
}
