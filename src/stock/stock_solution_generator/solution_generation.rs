use super::structs::{StockSolutionGenerator, StockSolutionConfig};
use crate::models::enums::StockSolutionResult;
use crate::stock::StockSolution;

impl StockSolutionGenerator {
    /// Generate the next stock solution
    pub fn generate_stock_solution(&mut self) -> StockSolutionResult {
        // If all stock tiles are the same, use the all-panel solution
        if self.is_unique_stock_panel() {
            if self.is_excluded(&self.all_panel_stock_solution) {
                return StockSolutionResult::AllExcluded;
            }
            self.stock_solutions_to_exclude.insert(self.all_panel_stock_solution.clone());
            return StockSolutionResult::Solution(self.all_panel_stock_solution.clone());
        }

        // Calculate minimum number of tiles needed
        let min_tiles_needed = (self.required_area as f64 / self.get_biggest_stock_tile_area() as f64).ceil() as usize;
        
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

        // If we're using the default max length and haven't excluded the all-panel solution, use it
        if max_length == StockSolutionConfig::default().max_stock_solution_length 
            && !self.is_excluded(&self.all_panel_stock_solution) {
            self.stock_solutions_to_exclude.insert(self.all_panel_stock_solution.clone());
            return StockSolutionResult::Solution(self.all_panel_stock_solution.clone());
        }

        // Try to find a solution with increasing number of tiles
        for num_tiles in min_tiles_needed..=max_length.min(self.stock_tiles.len()) {
            if let Some(solution) = self.get_candidate_stock_solution(num_tiles) {
                self.stock_solutions_to_exclude.insert(solution.clone());
                return StockSolutionResult::Solution(solution);
            }
        }

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
