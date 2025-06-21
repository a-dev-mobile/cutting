use serde::{Deserialize, Serialize};
use crate::{models::{Cut, TileDimensions, TileNode}, Orientation};

/// Represents a complete cutting solution for a piece of material
/// 
/// A Mosaic contains the root tile node representing the original material piece,
/// all cuts that have been made, and metadata about the material and orientation.
/// This is the primary result structure for cutting optimization algorithms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mosaic {
    /// List of all cuts made in this mosaic
    pub cuts: Vec<Cut>,
    
    /// Material type/name for this mosaic
    pub material: String,
    
    /// Orientation constraint for the material
    pub orientation: Orientation,
    
    /// Root node of the tile cutting tree
    pub root_tile_node: TileNode,
    
    /// Identifier for the stock/source material
    pub stock_id: i32,
}

impl Mosaic {
    /// Create a new Mosaic from another Mosaic (copy constructor equivalent)
    pub fn from_mosaic(other: &Mosaic) -> Self {
        Self {
            root_tile_node: TileNode::from_tile_node(&other.root_tile_node),
            cuts: other.cuts.clone(),
            stock_id: other.stock_id,
            material: other.material.clone(),
            orientation: other.orientation,
        }
    }

    /// Create a new Mosaic from a TileNode and material
    pub fn from_tile_node(tile_node: &TileNode, material: String) -> Self {
        Self {
            cuts: Vec::new(),
            root_tile_node: TileNode::from_tile_node(tile_node),
            stock_id: tile_node.external_id().unwrap_or(tile_node.id() as i32),
            material,
            orientation: Orientation::Any,
        }
    }

    /// Create a new Mosaic from TileDimensions
    pub fn from_tile_dimensions(tile_dimensions: &TileDimensions) -> Self {
        let mut root_node = TileNode::from_dimensions(tile_dimensions);
        root_node.set_external_id(Some(tile_dimensions.id));
        
        Self {
            cuts: Vec::new(),
            root_tile_node: root_node,
            material: tile_dimensions.material.clone(),
            orientation: tile_dimensions.orientation,
            stock_id: tile_dimensions.id,
        }
    }
}

impl Default for Mosaic {
    fn default() -> Self {
        Self {
            cuts: Vec::new(),
            material: String::from("DEFAULT"),
            orientation: Orientation::Any,
            root_tile_node: TileNode::default(),
            stock_id: 0,
        }
    }
}

impl PartialEq for Mosaic {
    fn eq(&self, other: &Self) -> bool {
        // Java implementation only compares root tile nodes
        self.root_tile_node == other.root_tile_node
    }
}

impl Eq for Mosaic {}
