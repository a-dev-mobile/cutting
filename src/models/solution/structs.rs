use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::atomic::AtomicU32;
use crate::models::{Mosaic, TileDimensions};

/// Static counter for generating unique solution IDs
pub(super) static ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Represents a complete cutting solution containing multiple mosaics
/// 
/// This is the Rust equivalent of the Java Solution class. It contains
/// all the mosaics (cutting patterns), unused stock panels, and panels
/// that couldn't be fit into any mosaic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    /// Unique identifier for this solution
    pub id: u32,
    
    /// Timestamp when this solution was created (milliseconds since Unix epoch)
    pub timestamp: u64,
    
    /// List of mosaics (cutting patterns) in this solution
    pub mosaics: Vec<Mosaic>,
    
    /// Panels that couldn't be fit into any mosaic
    pub no_fit_panels: Vec<TileDimensions>,
    
    /// Unused stock panels available for cutting
    pub unused_stock_panels: VecDeque<TileDimensions>,
    
    /// Optional auxiliary information
    pub aux_info: Option<String>,
    
    /// Optional creator thread group identifier
    pub creator_thread_group: Option<String>,
}
