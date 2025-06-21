use cutlist_optimizer_cli::models::tile::{TileDimensions, Orientation};

#[test]
fn test_tile_creation() {
    let tile = TileDimensions::new(1, 100, 200);
    assert_eq!(tile.area(), 20000);
    assert!(!tile.is_square());
    assert!(!tile.is_horizontal());
}

#[test]
fn test_tile_rotation() {
    let mut tile = TileDimensions::new(1, 100, 200);
    tile.rotate_90();
    assert_eq!(tile.width, 200);
    assert_eq!(tile.height, 100);
    assert!(tile.is_rotated);
}

#[test]
fn test_tile_fits() {
    let small = TileDimensions::new(1, 50, 100);
    let large = TileDimensions::new(2, 200, 300);
    assert!(small.fits(&large));
    assert!(!large.fits(&small));
}

#[test]
fn test_orientation_constraints() {
    let mut tile = TileDimensions {
        id: 1,
        width: 100,
        height: 200,
        label: None,
        material: "Wood".to_string(),
        orientation: Orientation::Horizontal,
        is_rotated: false,
    };
    
    // Tile with horizontal orientation should not be able to rotate
    assert!(!tile.can_rotate());
    tile.rotate_90();
    // Dimensions should remain unchanged
    assert_eq!(tile.width, 100);
    assert_eq!(tile.height, 200);
    assert!(!tile.is_rotated);
}

#[test]
fn test_dimensions_hash() {
    let tile1 = TileDimensions::new(1, 100, 200);
    let tile2 = TileDimensions::new(2, 200, 100);
    
    // Tiles with swapped dimensions should have the same hash
    assert_eq!(tile1.dimensions_hash(), tile2.dimensions_hash());
}

#[test]
fn test_has_same_dimensions() {
    let tile1 = TileDimensions::new(1, 100, 200);
    let tile2 = TileDimensions::new(2, 200, 100);
    let tile3 = TileDimensions::new(3, 150, 250);
    
    assert!(tile1.has_same_dimensions(&tile2));
    assert!(!tile1.has_same_dimensions(&tile3));
}

#[test]
fn test_max_dimension() {
    let tile = TileDimensions::new(1, 100, 200);
    assert_eq!(tile.max_dimension(), 200);
    
    let square_tile = TileDimensions::new(2, 150, 150);
    assert_eq!(square_tile.max_dimension(), 150);
}

#[test]
fn test_dimensions_string() {
    let tile = TileDimensions::new(1, 100, 200);
    assert_eq!(tile.dimensions_string(), "100x200");
}

#[test]
fn test_tile_with_label() {
    let mut tile = TileDimensions::new(1, 100, 200);
    tile.label = Some("Test Tile".to_string());
    
    assert_eq!(tile.label, Some("Test Tile".to_string()));
    assert_eq!(tile.material, "DEFAULT");
}

#[test]
fn test_orientation_any_allows_rotation() {
    let mut tile = TileDimensions {
        id: 1,
        width: 100,
        height: 200,
        label: None,
        material: "Wood".to_string(),
        orientation: Orientation::Any,
        is_rotated: false,
    };
    
    assert!(tile.can_rotate());
    tile.rotate_90();
    assert_eq!(tile.width, 200);
    assert_eq!(tile.height, 100);
    assert!(tile.is_rotated);
}

#[test]
fn test_vertical_orientation_no_rotation() {
    let mut tile = TileDimensions {
        id: 1,
        width: 100,
        height: 200,
        label: None,
        material: "Wood".to_string(),
        orientation: Orientation::Vertical,
        is_rotated: false,
    };
    
    assert!(!tile.can_rotate());
    tile.rotate_90();
    // Should remain unchanged
    assert_eq!(tile.width, 100);
    assert_eq!(tile.height, 200);
    assert!(!tile.is_rotated);
}
