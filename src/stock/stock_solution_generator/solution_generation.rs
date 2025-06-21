use super::structs::{StockSolutionGenerator, StockSolutionConfig};
use crate::models::enums::StockSolutionResult;
use crate::stock::StockSolution;
use crate::{log_info, log_operation_start, log_operation_success};

impl StockSolutionGenerator {
    /// Generate the next stock solution
    pub fn generate_stock_solution(&mut self) -> StockSolutionResult {
        log_operation_start!("stock_solution_generation");
        log_info!("Starting stock solution generation: {} stock tiles, {} tiles to fit, required area: {}",
            self.stock_tiles.len(),
            self.tiles_to_fit.len(),
            self.required_area
        );

        // If all stock tiles are the same, use the all-panel solution
        if self.is_unique_stock_panel() {
            log_info!("Using unique stock panel optimization");
            if self.is_excluded(&self.all_panel_stock_solution) {
                log_info!("All-panel solution already excluded");
                return StockSolutionResult::AllExcluded;
            }
            self.stock_solutions_to_exclude.insert(self.all_panel_stock_solution.clone());
            log_operation_success!("stock_solution_generation");
            return StockSolutionResult::Solution(self.all_panel_stock_solution.clone());
        }

        // Calculate minimum number of tiles needed
        let min_tiles_needed = (self.required_area as f64 / self.get_biggest_stock_tile_area() as f64).ceil() as usize;
        log_info!("Minimum tiles needed: {}", min_tiles_needed);
        
        // Determine maximum solution length
        let max_length = if let Some(hint) = self.max_stock_solution_length_hint {
            if hint >= min_tiles_needed {
                hint
            } else {
                StockSolutionConfig::default().max_stock_solution_length
            }
        } else {
            StockSolutionConfig::default().max_stock_solution_length
        };
        log_info!("Maximum solution length: {}", max_length);

        // If we're using the default max length and haven't excluded the all-panel solution, use it
        if max_length == StockSolutionConfig::default().max_stock_solution_length 
            && !self.is_excluded(&self.all_panel_stock_solution) {
            log_info!("Using default max length with all-panel solution");
            self.stock_solutions_to_exclude.insert(self.all_panel_stock_solution.clone());
            log_operation_success!("stock_solution_generation");
            return StockSolutionResult::Solution(self.all_panel_stock_solution.clone());
        }

        // Try to find a solution with increasing number of tiles
        log_info!("Searching for solution with {}-{} tiles", min_tiles_needed, max_length.min(self.stock_tiles.len()));
        for num_tiles in min_tiles_needed..=max_length.min(self.stock_tiles.len()) {
            log_info!("Trying solution with {} tiles", num_tiles);
            if let Some(solution) = self.get_candidate_stock_solution(num_tiles) {
                log_info!("Found solution with {} tiles, total area: {}", solution.len(), solution.get_total_area());
                self.stock_solutions_to_exclude.insert(solution.clone());
                log_operation_success!("stock_solution_generation");
                return StockSolutionResult::Solution(solution);
            }
        }

        log_info!("No solution found with current constraints");
        StockSolutionResult::NoSolution
    }

    /// Get a candidate stock solution with the specified number of tiles
    pub(crate) fn get_candidate_stock_solution(&mut self, num_tiles: usize) -> Option<StockSolution> {
        let indexes = if self.previous_returned_stock_tiles_indexes.len() == num_tiles {
            self.previous_returned_stock_tiles_indexes.clone()
        } else {
            (0..num_tiles).collect()
        };

        let start_index = if self.previous_returned_stock_tiles_indexes.len() == num_tiles {
            self.prev_index_to_iterate
        } else {
            0
        };

        self.iterate(
            self.required_area,
            self.required_max_dimension,
            self.smallest_tile_area,
            num_tiles,
            indexes,
            start_index,
        )
    }
}
