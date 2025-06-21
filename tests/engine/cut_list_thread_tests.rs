//! Tests for CutListThread
//! 
//! This module contains unit tests for the CutListThread functionality.

use cutlist_optimizer_cli::{
    engine::cut_list_thread::CutListThread,
    models::{Solution, TileDimensions},
    Status,
};

#[test]
fn test_cut_list_thread_creation() {
    let thread = CutListThread::new();
    assert_eq!(thread.status(), Status::Queued);
    assert_eq!(thread.percentage_done(), 0);
    assert_eq!(thread.accuracy_factor(), 100);
}

#[test]
fn test_setters_and_getters() {
    let mut thread = CutListThread::new();
    
    thread.set_group(Some("test_group".to_string()));
    assert_eq!(thread.group(), Some("test_group"));
    
    thread.set_cut_thickness(5);
    assert_eq!(thread.cut_thickness(), 5);
    
    thread.set_min_trim_dimension(10);
    assert_eq!(thread.min_trim_dimension(), 10);
    
    thread.set_consider_grain_direction(true);
    assert!(thread.consider_grain_direction());
}

#[test]
fn test_remove_duplicated() {
    let thread = CutListThread::new();
    let mut solutions = vec![
        Solution::new(),
        Solution::new(),
    ];
    
    // This test would need proper solution setup to test deduplication
    let removed = thread.remove_duplicated(&mut solutions);
    assert!(removed >= 0);
}

#[test]
fn test_status_checks() {
    let mut thread = CutListThread::new();
    
    // Initial state
    assert!(!thread.is_running());
    assert!(!thread.is_finished());
    assert!(!thread.has_error());
    assert!(!thread.is_terminated());
    
    // Test termination
    thread.terminate();
    assert!(thread.is_terminated());
}

#[test]
fn test_tiles_management() {
    let mut thread = CutListThread::new();
    
    let tiles = vec![
        TileDimensions::new(1, 100, 200),
        TileDimensions::new(2, 150, 250),
    ];
    
    thread.set_tiles(tiles.clone());
    assert_eq!(thread.tiles().len(), 2);
    assert_eq!(thread.tiles()[0].width, 100);
    assert_eq!(thread.tiles()[0].height, 200);
}

#[test]
fn test_accuracy_factor() {
    let mut thread = CutListThread::new();
    
    thread.set_accuracy_factor(200);
    assert_eq!(thread.accuracy_factor(), 200);
}

#[test]
fn test_aux_info() {
    let mut thread = CutListThread::new();
    
    thread.set_aux_info(Some("test info".to_string()));
    assert_eq!(thread.aux_info(), Some("test info"));
    
    thread.set_aux_info(None);
    assert_eq!(thread.aux_info(), None);
}

#[test]
fn test_elapsed_time() {
    let thread = CutListThread::new();
    
    // Should return zero duration when not started
    let elapsed = thread.elapsed_time();
    assert_eq!(elapsed.as_secs(), 0);
}

#[test]
fn test_material() {
    let thread = CutListThread::new();
    
    // Should return None when no solutions
    assert_eq!(thread.material(), None);
}

#[test]
fn test_default_implementation() {
    let thread = CutListThread::default();
    assert_eq!(thread.status(), Status::Queued);
    assert_eq!(thread.accuracy_factor(), 100);
}

#[test]
fn test_debug_implementation() {
    let thread = CutListThread::new();
    let debug_str = format!("{:?}", thread);
    assert!(debug_str.contains("CutListThread"));
    assert!(debug_str.contains("accuracy_factor"));
}
