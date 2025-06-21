//! Edge banding calculation utilities
//! 
//! This module provides functionality to calculate edge banding requirements
//! for panels based on their placement in tile nodes and edge configurations.

use std::collections::HashMap;
use crate::models::{TileNode, Panel};

/// Calculate edge banding requirements for a list of tile nodes and panels
/// 
/// This function calculates the total length of edge banding material needed
/// for each edge type based on the panel placements and their edge configurations.
/// 
/// # Arguments
/// * `tile_nodes` - List of tile nodes representing panel placements
/// * `panels` - List of panels with edge configurations
/// * `scale_factor` - Factor to convert dimensions to the desired unit (e.g., mm to meters)
/// 
/// # Returns
/// A HashMap mapping edge material names to their required lengths
/// 
/// # Examples
/// ```
/// use cutlist_optimizer_cli::utils::edge_banding::calc_edge_bands;
/// use cutlist_optimizer_cli::models::{TileNode, Panel, Edge};
/// use std::collections::HashMap;
/// 
/// let tile_nodes = vec![/* tile nodes */];
/// let panels = vec![/* panels with edge configs */];
/// let scale_factor = 1000.0; // Convert mm to meters
/// 
/// let edge_requirements = calc_edge_bands(&tile_nodes, &panels, scale_factor);
/// ```
pub fn calc_edge_bands(
    tile_nodes: &[TileNode], 
    panels: &[Panel], 
    scale_factor: f64
) -> HashMap<String, f64> {
    let mut edge_map: HashMap<String, f64> = HashMap::new();
    
    // Iterate through all panels that have edge configurations
    for panel in panels.iter().filter(|p| p.edge.is_some()) {
        let edge = panel.edge.as_ref().unwrap();
        
        // Find the corresponding tile node for this panel
        if let Some(tile_node) = find_tile_node_by_panel_id(tile_nodes, panel.id) {
            // Calculate dimensions based on rotation
            let (width_for_horizontal, height_for_vertical) = if tile_node.is_rotated() {
                (tile_node.height(), tile_node.width())
            } else {
                (tile_node.width(), tile_node.height())
            };
            
            // Add edge banding for each side that has a material specified
            add_edge_banding(&mut edge_map, &edge.top, width_for_horizontal as f64, scale_factor);
            add_edge_banding(&mut edge_map, &edge.left, height_for_vertical as f64, scale_factor);
            add_edge_banding(&mut edge_map, &edge.bottom, width_for_horizontal as f64, scale_factor);
            add_edge_banding(&mut edge_map, &edge.right, height_for_vertical as f64, scale_factor);
        }
    }
    
    edge_map
}

/// Find a tile node by panel ID
/// 
/// Searches through the tile nodes to find one with the matching external_id
fn find_tile_node_by_panel_id(tile_nodes: &[TileNode], panel_id: i32) -> Option<&TileNode> {
    tile_nodes.iter().find(|node| {
        node.external_id() == Some(panel_id)
    })
}

/// Add edge banding length to the map for a specific material
/// 
/// # Arguments
/// * `edge_map` - Mutable reference to the edge banding map
/// * `material` - Optional material name
/// * `length` - Length to add (in original units)
/// * `scale_factor` - Factor to scale the length
fn add_edge_banding(
    edge_map: &mut HashMap<String, f64>, 
    material: &Option<String>, 
    length: f64, 
    scale_factor: f64
) {
    if let Some(material_name) = material {
        let scaled_length = length / scale_factor;
        *edge_map.entry(material_name.clone()).or_insert(0.0) += scaled_length;
    }
}

/// Result type for edge banding calculations that can handle errors
pub type EdgeBandingResult<T> = Result<T, EdgeBandingError>;

/// Errors that can occur during edge banding calculations
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeBandingError {
    /// Panel not found in tile nodes
    PanelNotFound(i32),
    /// Invalid scale factor (zero or negative)
    InvalidScaleFactor(f64),
    /// Missing edge configuration
    MissingEdgeConfig(i32),
}

impl std::fmt::Display for EdgeBandingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeBandingError::PanelNotFound(id) => {
                write!(f, "Panel with ID {} not found in tile nodes", id)
            }
            EdgeBandingError::InvalidScaleFactor(factor) => {
                write!(f, "Invalid scale factor: {}. Must be positive", factor)
            }
            EdgeBandingError::MissingEdgeConfig(id) => {
                write!(f, "Panel with ID {} has no edge configuration", id)
            }
        }
    }
}

impl std::error::Error for EdgeBandingError {}

/// Enhanced version with error handling and validation
/// 
/// This version provides better error handling and validation compared to the basic version.
/// It returns a Result type that can indicate various error conditions.
pub fn calc_edge_bands_safe(
    tile_nodes: &[TileNode], 
    panels: &[Panel], 
    scale_factor: f64
) -> EdgeBandingResult<HashMap<String, f64>> {
    // Validate scale factor
    if scale_factor <= 0.0 {
        return Err(EdgeBandingError::InvalidScaleFactor(scale_factor));
    }
    
    let mut edge_map: HashMap<String, f64> = HashMap::new();
    
    for panel in panels {
        // Skip panels without edge configuration
        let edge = match &panel.edge {
            Some(e) => e,
            None => continue,
        };
        
        // Find corresponding tile node
        let tile_node = find_tile_node_by_panel_id(tile_nodes, panel.id)
            .ok_or(EdgeBandingError::PanelNotFound(panel.id))?;
        
        // Calculate dimensions based on rotation
        let (width_for_horizontal, height_for_vertical) = if tile_node.is_rotated() {
            (tile_node.height(), tile_node.width())
        } else {
            (tile_node.width(), tile_node.height())
        };
        
        // Add edge banding for each side
        add_edge_banding(&mut edge_map, &edge.top, width_for_horizontal as f64, scale_factor);
        add_edge_banding(&mut edge_map, &edge.left, height_for_vertical as f64, scale_factor);
        add_edge_banding(&mut edge_map, &edge.bottom, width_for_horizontal as f64, scale_factor);
        add_edge_banding(&mut edge_map, &edge.right, height_for_vertical as f64, scale_factor);
    }
    
    Ok(edge_map)
}

/// Calculate edge banding with detailed breakdown per panel
/// 
/// Returns a more detailed result showing edge banding requirements per panel
pub fn calc_edge_bands_detailed(
    tile_nodes: &[TileNode], 
    panels: &[Panel], 
    scale_factor: f64
) -> EdgeBandingResult<HashMap<i32, HashMap<String, f64>>> {
    if scale_factor <= 0.0 {
        return Err(EdgeBandingError::InvalidScaleFactor(scale_factor));
    }
    
    let mut detailed_map: HashMap<i32, HashMap<String, f64>> = HashMap::new();
    
    for panel in panels {
        let edge = match &panel.edge {
            Some(e) => e,
            None => continue,
        };
        
        let tile_node = find_tile_node_by_panel_id(tile_nodes, panel.id)
            .ok_or(EdgeBandingError::PanelNotFound(panel.id))?;
        
        let mut panel_edges: HashMap<String, f64> = HashMap::new();
        
        let (width_for_horizontal, height_for_vertical) = if tile_node.is_rotated() {
            (tile_node.height(), tile_node.width())
        } else {
            (tile_node.width(), tile_node.height())
        };
        
        // Add edge banding for each side to this panel's map
        add_edge_banding(&mut panel_edges, &edge.top, width_for_horizontal as f64, scale_factor);
        add_edge_banding(&mut panel_edges, &edge.left, height_for_vertical as f64, scale_factor);
        add_edge_banding(&mut panel_edges, &edge.bottom, width_for_horizontal as f64, scale_factor);
        add_edge_banding(&mut panel_edges, &edge.right, height_for_vertical as f64, scale_factor);
        
        if !panel_edges.is_empty() {
            detailed_map.insert(panel.id, panel_edges);
        }
    }
    
    Ok(detailed_map)
}

/// Calculate total edge banding length for a specific material
/// 
/// This is a convenience function to get the total length needed for a specific material type.
pub fn calc_material_total(
    tile_nodes: &[TileNode], 
    panels: &[Panel], 
    material_name: &str,
    scale_factor: f64
) -> EdgeBandingResult<f64> {
    let edge_map = calc_edge_bands_safe(tile_nodes, panels, scale_factor)?;
    Ok(edge_map.get(material_name).copied().unwrap_or(0.0))
}

/// Get a summary of all materials and their requirements
/// 
/// Returns a sorted vector of (material_name, total_length) tuples
pub fn get_material_summary(
    tile_nodes: &[TileNode], 
    panels: &[Panel], 
    scale_factor: f64
) -> EdgeBandingResult<Vec<(String, f64)>> {
    let edge_map = calc_edge_bands_safe(tile_nodes, panels, scale_factor)?;
    let mut summary: Vec<(String, f64)> = edge_map.into_iter().collect();
    summary.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by material name
    Ok(summary)
}
