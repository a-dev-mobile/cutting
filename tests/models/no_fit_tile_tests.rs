//! Tests for NoFitTile model

use cutlist_optimizer_cli::models::NoFitTile;

#[test]
fn test_no_fit_tile_default() {
    let tile = NoFitTile::default();
    assert_eq!(tile.get_id(), 0);
    assert_eq!(tile.get_width(), 0.0);
    assert_eq!(tile.get_height(), 0.0);
    assert_eq!(tile.get_count(), 0);
    assert_eq!(tile.get_label(), None);
    assert_eq!(tile.get_material(), None);
}

#[test]
fn test_no_fit_tile_new() {
    let tile = NoFitTile::new(1, 100.0, 50.0, 5);
    assert_eq!(tile.get_id(), 1);
    assert_eq!(tile.get_width(), 100.0);
    assert_eq!(tile.get_height(), 50.0);
    assert_eq!(tile.get_count(), 5);
    assert_eq!(tile.get_label(), None);
    assert_eq!(tile.get_material(), None);
}

#[test]
fn test_no_fit_tile_setters() {
    let mut tile = NoFitTile::default();
    
    tile.set_id(42);
    tile.set_width(200.0);
    tile.set_height(150.0);
    tile.set_count(10);
    tile.set_label(Some("Test Label".to_string()));
    tile.set_material(Some("Wood".to_string()));
    
    assert_eq!(tile.get_id(), 42);
    assert_eq!(tile.get_width(), 200.0);
    assert_eq!(tile.get_height(), 150.0);
    assert_eq!(tile.get_count(), 10);
    assert_eq!(tile.get_label(), Some("Test Label"));
    assert_eq!(tile.get_material(), Some("Wood"));
}

#[test]
fn test_no_fit_tile_area_calculations() {
    let tile = NoFitTile::new(1, 10.0, 5.0, 3);
    assert_eq!(tile.area(), 50.0);
    assert_eq!(tile.total_area(), 150.0);
}

#[test]
fn test_no_fit_tile_clone() {
    let mut original = NoFitTile::new(1, 100.0, 50.0, 2);
    original.set_label(Some("Original".to_string()));
    
    let cloned = original.clone();
    assert_eq!(original, cloned);
    assert_eq!(cloned.get_label(), Some("Original"));
}
