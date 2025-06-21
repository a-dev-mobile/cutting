use crate::models::TileDimensions;
use serde::{Deserialize, Serialize};

/// Represents a stock solution containing a collection of tile dimensions
/// This is the Rust equivalent of the Java StockSolution class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockSolution {
    pub(crate) stock_tile_dimensions: Vec<TileDimensions>,
}
