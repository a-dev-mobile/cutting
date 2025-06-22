use cutlist_optimizer_cli::engine::service::computation::PermutationUtils;
use cutlist_optimizer_cli::models::tile_dimensions::structs::TileDimensions;
use cutlist_optimizer_cli::models::enums::orientation::Orientation;

fn create_test_tile(id: i32, width: i32, height: i32, material: &str) -> TileDimensions {
    TileDimensions {
        id,
        width,
        height,
        label: Some(format!("Tile {}", id)),
        material: material.to_string(),
        orientation: Orientation::Any,
        is_rotated: false,
    }
}

#[test]
fn test_remove_duplicated_permutations() {
    let tiles = vec![
        create_test_tile(1, 100, 200, "Wood"),
        create_test_tile(2, 100, 200, "Wood"), // Duplicate
        create_test_tile(3, 150, 300, "Metal"),
    ];

    let result = PermutationUtils::remove_duplicated_permutations(tiles);
    assert_eq!(result.len(), 2);
}

#[test]
fn test_are_equivalent_tiles() {
    let tile1 = create_test_tile(1, 100, 200, "Wood");
    let tile2 = create_test_tile(2, 100, 200, "Wood");
    let tile3 = create_test_tile(3, 150, 300, "Wood");

    assert!(PermutationUtils::are_equivalent_tiles(&tile1, &tile2));
    assert!(!PermutationUtils::are_equivalent_tiles(&tile1, &tile3));
}

#[test]
fn test_generate_tile_key() {
    let tile = create_test_tile(1, 100, 200, "Wood");
    let key = PermutationUtils::generate_tile_key(&tile);
    assert_eq!(key, "100x200_Wood_Any");
}

#[test]
fn test_group_equivalent_tiles() {
    let tiles = vec![
        create_test_tile(1, 100, 200, "Wood"),
        create_test_tile(2, 100, 200, "Wood"), // Same group
        create_test_tile(3, 150, 300, "Metal"), // Different group
    ];

    let groups = PermutationUtils::group_equivalent_tiles(tiles);
    assert_eq!(groups.len(), 2);
    assert_eq!(groups[0].len(), 2); // Wood group
    assert_eq!(groups[1].len(), 1); // Metal group
}

#[test]
fn test_calculate_group_total_count() {
    let tiles = vec![
        create_test_tile(1, 100, 200, "Wood"),
        create_test_tile(2, 100, 200, "Wood"),
        create_test_tile(3, 150, 300, "Metal"),
    ];

    let count = PermutationUtils::calculate_group_total_count(&tiles);
    assert_eq!(count, 3);
}

#[test]
fn test_get_max_count_from_group() {
    let tiles = vec![
        create_test_tile(1, 100, 200, "Wood"),
        create_test_tile(2, 100, 200, "Wood"),
    ];

    let max_count = PermutationUtils::get_max_count_from_group(&tiles);
    assert_eq!(max_count, 1);

    let empty_tiles: Vec<TileDimensions> = vec![];
    let empty_max_count = PermutationUtils::get_max_count_from_group(&empty_tiles);
    assert_eq!(empty_max_count, 0);
}

#[test]
fn test_create_representatives_from_groups() {
    let groups = vec![
        vec![
            create_test_tile(1, 100, 200, "Wood"),
            create_test_tile(2, 100, 200, "Wood"),
        ],
        vec![
            create_test_tile(3, 150, 300, "Metal"),
        ],
    ];

    let representatives = PermutationUtils::create_representatives_from_groups(&groups);
    assert_eq!(representatives.len(), 2);
    assert_eq!(representatives[0].material, "Wood");
    assert_eq!(representatives[1].material, "Metal");
}

#[test]
fn test_validate_permutation_removal() {
    let original_tiles = vec![
        create_test_tile(1, 100, 200, "Wood"),
        create_test_tile(2, 100, 200, "Wood"), // Duplicate
        create_test_tile(3, 150, 300, "Metal"),
    ];

    let processed_tiles = vec![
        create_test_tile(1, 100, 200, "Wood"),
        create_test_tile(3, 150, 300, "Metal"),
    ];

    assert!(PermutationUtils::validate_permutation_removal(&original_tiles, &processed_tiles));

    // Test invalid case where processed has more tiles than original
    let invalid_processed = vec![
        create_test_tile(1, 100, 200, "Wood"),
        create_test_tile(2, 100, 200, "Wood"),
        create_test_tile(3, 150, 300, "Metal"),
        create_test_tile(4, 200, 400, "Plastic"),
    ];

    assert!(!PermutationUtils::validate_permutation_removal(&original_tiles, &invalid_processed));
}

#[test]
fn test_edge_cases() {
    // Test with empty input
    let empty_tiles: Vec<TileDimensions> = vec![];
    let result = PermutationUtils::remove_duplicated_permutations(empty_tiles);
    assert_eq!(result.len(), 0);

    // Test with single tile
    let single_tile = vec![create_test_tile(1, 100, 200, "Wood")];
    let result = PermutationUtils::remove_duplicated_permutations(single_tile);
    assert_eq!(result.len(), 1);

    // Test grouping with empty input
    let empty_tiles: Vec<TileDimensions> = vec![];
    let groups = PermutationUtils::group_equivalent_tiles(empty_tiles);
    assert_eq!(groups.len(), 0);
}

#[test]
fn test_different_orientations() {
    let tile_horizontal = TileDimensions {
        id: 1,
        width: 100,
        height: 200,
        label: Some("Tile 1".to_string()),
        material: "Wood".to_string(),
        orientation: Orientation::Horizontal,
        is_rotated: false,
    };

    let tile_vertical = TileDimensions {
        id: 2,
        width: 100,
        height: 200,
        label: Some("Tile 2".to_string()),
        material: "Wood".to_string(),
        orientation: Orientation::Vertical,
        is_rotated: false,
    };

    // Should not be equivalent due to different orientations
    assert!(!PermutationUtils::are_equivalent_tiles(&tile_horizontal, &tile_vertical));

    // Test grouping with different orientations
    let tiles = vec![tile_horizontal, tile_vertical];
    let groups = PermutationUtils::group_equivalent_tiles(tiles);
    assert_eq!(groups.len(), 2); // Should create separate groups
}
