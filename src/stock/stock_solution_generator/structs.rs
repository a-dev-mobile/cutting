use crate::models::TileDimensions;
use crate::stock::StockSolution;
use crate::constants::PerformanceConstants;
use std::collections::HashSet;

/// Configuration and state for generating stock solutions
/// 
/// This is the Rust equivalent of the Java StockSolutionGenerator class.
/// It generates optimal combinations of stock tiles to fit required tiles
/// using iterative backtracking with memoization and exclusion tracking.
#[derive(Debug, Clone)]
pub struct StockSolutionGenerator {
    /// The tiles that need to be fitted
    pub(crate) tiles_to_fit: Vec<TileDimensions>,
    
    /// Available stock tiles to choose from
    pub(crate) stock_tiles: Vec<TileDimensions>,
    
    /// Optional hint for maximum stock solution length
    pub(crate) max_stock_solution_length_hint: Option<usize>,
    
    /// Stock solutions to exclude from future generations
    pub(crate) stock_solutions_to_exclude: HashSet<StockSolution>,
    
    /// Previously returned stock tile indexes for iteration optimization
    pub(crate) previous_returned_stock_tiles_indexes: Vec<usize>,
    
    /// Previous index to iterate from for optimization
    pub(crate) prev_index_to_iterate: usize,
    
    /// Total area required by all tiles to fit
    pub(crate) required_area: i64,
    
    /// Maximum dimension among all tiles to fit
    pub(crate) required_max_dimension: i32,
    
    /// Smallest tile area among tiles to fit
    pub(crate) smallest_tile_area: i64,
    
    /// Pre-computed solution using all available panels
    pub(crate) all_panel_stock_solution: StockSolution,
}

/// Configuration for stock solution generation
#[derive(Debug, Clone)]
pub struct StockSolutionConfig {
    /// Maximum number of stock tiles in a solution
    pub max_stock_solution_length: usize,
}

impl Default for StockSolutionConfig {
    fn default() -> Self {
        Self {
            max_stock_solution_length: PerformanceConstants::DEFAULT_MAX_STOCK_SOLUTION_LENGTH,
        }
    }
}
