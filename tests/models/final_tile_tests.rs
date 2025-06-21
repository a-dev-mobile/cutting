//! Tests for FinalTile model

use cutlist_optimizer_cli::models::FinalTile;

#[test]
fn test_final_tile_default() {
    let tile = FinalTile::default();
    assert_eq!(tile.get_request_obj_id(), 0);
    assert_eq!(tile.get_width(), 0.0);
    assert_eq!(tile.get_height(), 0.0);
    assert_eq!(tile.get_label(), None);
    assert_eq!(tile.get_count(), 0);
}

#[test]
fn test_final_tile_new() {
    let tile = FinalTile::new();
    assert_eq!(tile.get_request_obj_id(), 0);
    assert_eq!(tile.get_width(), 0.0);
    assert_eq!(tile.get_height(), 0.0);
    assert_eq!(tile.get_label(), None);
    assert_eq!(tile.get_count(), 0);
}

#[test]
fn test_final_tile_setters() {
    let mut tile = FinalTile::new();
    
    tile.set_request_obj_id(123);
    tile.set_width(250.0);
    tile.set_height(180.0);
    tile.set_label(Some("Final Tile".to_string()));
    tile.set_count(7);
    
    assert_eq!(tile.get_request_obj_id(), 123);
    assert_eq!(tile.get_width(), 250.0);
    assert_eq!(tile.get_height(), 180.0);
    assert_eq!(tile.get_label(), Some("Final Tile"));
    assert_eq!(tile.get_count(), 7);
}

#[test]
fn test_final_tile_count_plus_plus() {
    let mut tile = FinalTile::new();
    tile.set_count(5);
    
    let previous_count = tile.count_plus_plus();
    assert_eq!(previous_count, 5);
    assert_eq!(tile.get_count(), 6);
    
    let previous_count = tile.count_plus_plus();
    assert_eq!(previous_count, 6);
    assert_eq!(tile.get_count(), 7);
}

#[test]
fn test_final_tile_area_calculations() {
    let mut tile = FinalTile::new();
    tile.set_width(12.0);
    tile.set_height(8.0);
    tile.set_count(4);
    
    assert_eq!(tile.area(), 96.0);
    assert_eq!(tile.total_area(), 384.0);
}

#[test]
fn test_final_tile_clone() {
    let mut original = FinalTile::new();
    original.set_request_obj_id(999);
    original.set_label(Some("Cloned Tile".to_string()));
    
    let cloned = original.clone();
    assert_eq!(original, cloned);
    assert_eq!(cloned.get_request_obj_id(), 999);
    assert_eq!(cloned.get_label(), Some("Cloned Tile"));
}

#[test]
fn test_final_tile_count_plus_plus_from_zero() {
    let mut tile = FinalTile::new();
    assert_eq!(tile.get_count(), 0);
    
    let previous = tile.count_plus_plus();
    assert_eq!(previous, 0);
    assert_eq!(tile.get_count(), 1);
}
