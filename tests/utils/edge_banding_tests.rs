//! Tests for edge banding utilities
//! 
//! This module contains comprehensive tests for the edge banding calculation
//! functionality, including basic calculations, error handling, and edge cases.

use cutlist_optimizer_cli::utils::edge_banding::*;
use cutlist_optimizer_cli::models::{Edge, Panel, TileNode};

fn create_test_panel(id: i32, edge: Option<Edge>) -> Panel {
    Panel {
        id,
        width: Some("100".to_string()),
        height: Some("200".to_string()),
        count: 1,
        material: "wood".to_string(),
        enabled: true,
        orientation: 0,
        label: None,
        edge,
    }
}

fn create_test_tile_node(external_id: i32, width: i32, height: i32, is_rotated: bool) -> TileNode {
    let mut node = TileNode::new(0, width, 0, height);
    node.set_external_id(Some(external_id));
    node.set_rotated(is_rotated);
    node
}

#[test]
fn test_calc_edge_bands_basic() {
    let edge = Edge {
        top: Some("edge_material_1".to_string()),
        left: Some("edge_material_2".to_string()),
        bottom: Some("edge_material_1".to_string()),
        right: Some("edge_material_2".to_string()),
    };
    
    let panels = vec![create_test_panel(1, Some(edge))];
    let tile_nodes = vec![create_test_tile_node(1, 100, 200, false)];
    
    let result = calc_edge_bands(&tile_nodes, &panels, 1.0);
    
    assert_eq!(result.get("edge_material_1"), Some(&200.0)); // top + bottom = 100 + 100
    assert_eq!(result.get("edge_material_2"), Some(&400.0)); // left + right = 200 + 200
}

#[test]
fn test_calc_edge_bands_rotated() {
    let edge = Edge {
        top: Some("edge_material_1".to_string()),
        left: Some("edge_material_2".to_string()),
        bottom: Some("edge_material_1".to_string()),
        right: Some("edge_material_2".to_string()),
    };
    
    let panels = vec![create_test_panel(1, Some(edge))];
    let tile_nodes = vec![create_test_tile_node(1, 100, 200, true)]; // rotated
    
    let result = calc_edge_bands(&tile_nodes, &panels, 1.0);
    
    // When rotated: width becomes height for horizontal edges, height becomes width for vertical edges
    assert_eq!(result.get("edge_material_1"), Some(&400.0)); // top + bottom = 200 + 200
    assert_eq!(result.get("edge_material_2"), Some(&200.0)); // left + right = 100 + 100
}

#[test]
fn test_calc_edge_bands_with_scale_factor() {
    let edge = Edge {
        top: Some("edge_material_1".to_string()),
        left: None,
        bottom: None,
        right: None,
    };
    
    let panels = vec![create_test_panel(1, Some(edge))];
    let tile_nodes = vec![create_test_tile_node(1, 1000, 2000, false)];
    
    let result = calc_edge_bands(&tile_nodes, &panels, 1000.0); // Convert mm to meters
    
    assert_eq!(result.get("edge_material_1"), Some(&1.0)); // 1000 / 1000 = 1.0
}

#[test]
fn test_calc_edge_bands_no_edge_config() {
    let panels = vec![create_test_panel(1, None)];
    let tile_nodes = vec![create_test_tile_node(1, 100, 200, false)];
    
    let result = calc_edge_bands(&tile_nodes, &panels, 1.0);
    
    assert!(result.is_empty());
}

#[test]
fn test_calc_edge_bands_partial_edges() {
    let edge = Edge {
        top: Some("edge_material_1".to_string()),
        left: None,
        bottom: Some("edge_material_2".to_string()),
        right: None,
    };
    
    let panels = vec![create_test_panel(1, Some(edge))];
    let tile_nodes = vec![create_test_tile_node(1, 100, 200, false)];
    
    let result = calc_edge_bands(&tile_nodes, &panels, 1.0);
    
    assert_eq!(result.get("edge_material_1"), Some(&100.0)); // top only
    assert_eq!(result.get("edge_material_2"), Some(&100.0)); // bottom only
    assert_eq!(result.len(), 2);
}

#[test]
fn test_calc_edge_bands_multiple_panels() {
    let edge1 = Edge {
        top: Some("material_a".to_string()),
        left: Some("material_b".to_string()),
        bottom: None,
        right: None,
    };
    
    let edge2 = Edge {
        top: Some("material_a".to_string()),
        left: None,
        bottom: Some("material_c".to_string()),
        right: None,
    };
    
    let panels = vec![
        create_test_panel(1, Some(edge1)),
        create_test_panel(2, Some(edge2)),
    ];
    
    let tile_nodes = vec![
        create_test_tile_node(1, 100, 200, false),
        create_test_tile_node(2, 150, 300, false),
    ];
    
    let result = calc_edge_bands(&tile_nodes, &panels, 1.0);
    
    assert_eq!(result.get("material_a"), Some(&250.0)); // 100 + 150
    assert_eq!(result.get("material_b"), Some(&200.0)); // 200 from panel 1
    assert_eq!(result.get("material_c"), Some(&150.0)); // 150 from panel 2
}

#[test]
fn test_calc_edge_bands_safe_invalid_scale() {
    let panels = vec![];
    let tile_nodes = vec![];
    
    let result = calc_edge_bands_safe(&tile_nodes, &panels, 0.0);
    
    assert!(matches!(result, Err(EdgeBandingError::InvalidScaleFactor(0.0))));
    
    let result_negative = calc_edge_bands_safe(&tile_nodes, &panels, -1.0);
    assert!(matches!(result_negative, Err(EdgeBandingError::InvalidScaleFactor(-1.0))));
}

#[test]
fn test_calc_edge_bands_safe_panel_not_found() {
    let edge = Edge {
        top: Some("edge_material_1".to_string()),
        left: None,
        bottom: None,
        right: None,
    };
    
    let panels = vec![create_test_panel(1, Some(edge))];
    let tile_nodes = vec![create_test_tile_node(2, 100, 200, false)]; // Different ID
    
    let result = calc_edge_bands_safe(&tile_nodes, &panels, 1.0);
    
    assert!(matches!(result, Err(EdgeBandingError::PanelNotFound(1))));
}

#[test]
fn test_calc_edge_bands_safe_success() {
    let edge = Edge {
        top: Some("edge_material_1".to_string()),
        left: None,
        bottom: None,
        right: None,
    };
    
    let panels = vec![create_test_panel(1, Some(edge))];
    let tile_nodes = vec![create_test_tile_node(1, 100, 200, false)];
    
    let result = calc_edge_bands_safe(&tile_nodes, &panels, 1.0);
    
    assert!(result.is_ok());
    let edge_map = result.unwrap();
    assert_eq!(edge_map.get("edge_material_1"), Some(&100.0));
}

#[test]
fn test_calc_edge_bands_detailed() {
    let edge1 = Edge {
        top: Some("material_a".to_string()),
        left: Some("material_b".to_string()),
        bottom: None,
        right: None,
    };
    
    let edge2 = Edge {
        top: Some("material_a".to_string()),
        left: None,
        bottom: Some("material_c".to_string()),
        right: None,
    };
    
    let panels = vec![
        create_test_panel(1, Some(edge1)),
        create_test_panel(2, Some(edge2)),
    ];
    
    let tile_nodes = vec![
        create_test_tile_node(1, 100, 200, false),
        create_test_tile_node(2, 150, 300, false),
    ];
    
    let result = calc_edge_bands_detailed(&tile_nodes, &panels, 1.0).unwrap();
    
    assert_eq!(result.len(), 2);
    
    let panel1_edges = result.get(&1).unwrap();
    assert_eq!(panel1_edges.get("material_a"), Some(&100.0)); // top
    assert_eq!(panel1_edges.get("material_b"), Some(&200.0)); // left
    
    let panel2_edges = result.get(&2).unwrap();
    assert_eq!(panel2_edges.get("material_a"), Some(&150.0)); // top
    assert_eq!(panel2_edges.get("material_c"), Some(&150.0)); // bottom
}

#[test]
fn test_calc_material_total() {
    let edge = Edge {
        top: Some("material_a".to_string()),
        left: Some("material_a".to_string()),
        bottom: Some("material_b".to_string()),
        right: Some("material_b".to_string()),
    };
    
    let panels = vec![create_test_panel(1, Some(edge))];
    let tile_nodes = vec![create_test_tile_node(1, 100, 200, false)];
    
    let result_a = calc_material_total(&tile_nodes, &panels, "material_a", 1.0).unwrap();
    let result_b = calc_material_total(&tile_nodes, &panels, "material_b", 1.0).unwrap();
    let result_c = calc_material_total(&tile_nodes, &panels, "material_c", 1.0).unwrap();
    
    assert_eq!(result_a, 300.0); // top + left = 100 + 200
    assert_eq!(result_b, 300.0); // bottom + right = 100 + 200
    assert_eq!(result_c, 0.0); // not used
}

#[test]
fn test_get_material_summary() {
    let edge = Edge {
        top: Some("material_b".to_string()),
        left: Some("material_a".to_string()),
        bottom: Some("material_c".to_string()),
        right: Some("material_a".to_string()),
    };
    
    let panels = vec![create_test_panel(1, Some(edge))];
    let tile_nodes = vec![create_test_tile_node(1, 100, 200, false)];
    
    let result = get_material_summary(&tile_nodes, &panels, 1.0).unwrap();
    
    // Should be sorted by material name
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], ("material_a".to_string(), 400.0)); // left + right = 200 + 200
    assert_eq!(result[1], ("material_b".to_string(), 100.0)); // top = 100
    assert_eq!(result[2], ("material_c".to_string(), 100.0)); // bottom = 100
}

#[test]
fn test_edge_banding_error_display() {
    let err1 = EdgeBandingError::PanelNotFound(123);
    assert_eq!(err1.to_string(), "Panel with ID 123 not found in tile nodes");
    
    let err2 = EdgeBandingError::InvalidScaleFactor(-1.5);
    assert_eq!(err2.to_string(), "Invalid scale factor: -1.5. Must be positive");
    
    let err3 = EdgeBandingError::MissingEdgeConfig(456);
    assert_eq!(err3.to_string(), "Panel with ID 456 has no edge configuration");
}

#[test]
fn test_edge_banding_with_zero_dimensions() {
    let edge = Edge {
        top: Some("material_a".to_string()),
        left: Some("material_a".to_string()),
        bottom: Some("material_a".to_string()),
        right: Some("material_a".to_string()),
    };
    
    let panels = vec![create_test_panel(1, Some(edge))];
    let tile_nodes = vec![create_test_tile_node(1, 0, 200, false)];
    
    let result = calc_edge_bands(&tile_nodes, &panels, 1.0);
    
    // Should handle zero dimensions gracefully
    assert_eq!(result.get("material_a"), Some(&400.0)); // left + right = 200 + 200, top + bottom = 0 + 0
}

#[test]
fn test_edge_banding_large_scale_factor() {
    let edge = Edge {
        top: Some("material_a".to_string()),
        left: None,
        bottom: None,
        right: None,
    };
    
    let panels = vec![create_test_panel(1, Some(edge))];
    let tile_nodes = vec![create_test_tile_node(1, 1000000, 2000000, false)];
    
    let result = calc_edge_bands(&tile_nodes, &panels, 1000000.0);
    
    assert_eq!(result.get("material_a"), Some(&1.0)); // 1000000 / 1000000 = 1.0
}

#[test]
fn test_edge_banding_empty_inputs() {
    let result = calc_edge_bands(&[], &[], 1.0);
    assert!(result.is_empty());
    
    let result_safe = calc_edge_bands_safe(&[], &[], 1.0).unwrap();
    assert!(result_safe.is_empty());
    
    let result_detailed = calc_edge_bands_detailed(&[], &[], 1.0).unwrap();
    assert!(result_detailed.is_empty());
}
