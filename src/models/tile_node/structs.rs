use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicU32;
use crate::models::Tile;

/// Static counter for generating unique node IDs
pub(super) static NEXT_ID: AtomicU32 = AtomicU32::new(0);

/// Represents a node in a tile cutting tree structure
/// 
/// This structure represents either a leaf node (final tile) or an internal node
/// that has been split into two child nodes. It maintains spatial information
/// through the contained Tile and tracks various properties for optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileNode {
    /// Unique identifier for this node
    pub id: u32,
    
    /// External identifier (can be set by user, defaults to None)
    pub external_id: Option<i32>,
    
    /// The tile representing the spatial bounds of this node
    pub tile: Tile,
    
    /// Whether this node represents a final cut (leaf node with actual content)
    pub is_final: bool,
    
    /// Whether the tile has been rotated from its original orientation
    pub is_rotated: bool,
    
    /// First child node (if this node has been split)
    pub child1: Option<Box<TileNode>>,
    
    /// Second child node (if this node has been split)
    pub child2: Option<Box<TileNode>>,
    
    /// Cached flag indicating if the entire area is used (optimization)
    pub is_area_totally_used: bool,
    
    /// Cached value of totally used area (optimization)
    pub totally_used_area: i64,
}
