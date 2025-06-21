use super::structs::StockSolutionGenerator;
use crate::error::AppError;
use crate::models::TileDimensions;
use crate::stock::StockSolution;
use std::collections::HashSet;

impl StockSolutionGenerator {
    /// Create a new StockSolutionGenerator with tiles to fit and available stock tiles
    pub fn new(
        tiles_to_fit: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
        max_stock_solution_length_hint: Option<usize>,
    ) -> Result<Self, AppError> {
        if tiles_to_fit.is_empty() {
            return Err(AppError::NoTilesToFit);
        }
        if stock_tiles.is_empty() {
            return Err(AppError::NoStockTiles);
        }

        let mut generator = Self {
            tiles_to_fit,
            stock_tiles,
            max_stock_solution_length_hint,
            stock_solutions_to_exclude: HashSet::new(),
            previous_returned_stock_tiles_indexes: Vec::new(),
            prev_index_to_iterate: 0,
            required_area: 0,
            required_max_dimension: 0,
            smallest_tile_area: i64::MAX,
            all_panel_stock_solution: StockSolution::new(),
        };

        // Sort stock tiles by area (ascending)
        generator.sort_stock_tiles_area_asc();
        
        // Calculate required metrics
        generator.calc_required_area();
        
        // Generate the all-panel stock solution
        generator.all_panel_stock_solution = generator.gen_all_panel_stock_solution();

        Ok(generator)
    }

    /// Create a new StockSolutionGenerator without length hint
    pub fn new_simple(
        tiles_to_fit: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
    ) -> Result<Self, AppError> {
        Self::new(tiles_to_fit, stock_tiles, None)
    }
}

impl Default for StockSolutionGenerator {
    fn default() -> Self {
        Self {
            tiles_to_fit: Vec::new(),
            stock_tiles: Vec::new(),
            max_stock_solution_length_hint: None,
            stock_solutions_to_exclude: HashSet::new(),
            previous_returned_stock_tiles_indexes: Vec::new(),
            prev_index_to_iterate: 0,
            required_area: 0,
            required_max_dimension: 0,
            smallest_tile_area: i64::MAX,
            all_panel_stock_solution: StockSolution::new(),
        }
    }
}
