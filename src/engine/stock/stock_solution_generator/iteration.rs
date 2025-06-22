use super::structs::StockSolutionGenerator;
use crate::models::TileDimensions;
use super::super::StockSolution;
use std::collections::HashSet;

impl StockSolutionGenerator {
    /// Main iteration algorithm for finding valid stock solutions
    pub(crate) fn iterate(
        &mut self,
        required_area: i64,
        required_max_dimension: i32,
        smallest_tile_area: i64,
        num_tiles: usize,
        mut indexes: Vec<usize>,
        iteration_index: usize,
    ) -> Option<StockSolution> {
        // Validate uniqueness of indexes up to iteration_index
        let mut seen = HashSet::new();
        for i in 0..iteration_index {
            if !seen.insert(indexes[i]) {
                return None; // Duplicate found
            }
        }

        // If we're not at the last position, recurse
        if iteration_index < num_tiles - 1 {
            let mut last_width = 0;
            let mut last_height = 0;
            let mut current_index = 0;

            while current_index < self.stock_tiles.len() {
                let _old_index = current_index;
                
                // Try recursive call
                if let Some(solution) = self.iterate(
                    required_area,
                    required_max_dimension,
                    smallest_tile_area,
                    num_tiles,
                    indexes.clone(),
                    iteration_index + 1,
                ) {
                    return Some(solution);
                }

                // Skip tiles with same dimensions and insufficient area
                loop {
                    current_index += 1;
                    if current_index >= self.stock_tiles.len() {
                        break;
                    }
                    let tile = &self.stock_tiles[current_index];
                    if tile.width != last_width || tile.height != last_height {
                        if tile.area() as i64 >= smallest_tile_area {
                            break;
                        }
                    }
                }

                if current_index < self.stock_tiles.len() {
                    last_width = self.stock_tiles[current_index].width;
                    last_height = self.stock_tiles[current_index].height;
                    
                    // Update indexes from iteration_index onwards
                    let mut idx = iteration_index;
                    let mut tile_idx = current_index;
                    while idx < indexes.len() && tile_idx < self.stock_tiles.len() {
                        indexes[idx] = tile_idx;
                        idx += 1;
                        tile_idx += 1;
                    }
                }
            }
        }

        // Check if current combination is valid
        loop {
            let mut total_area = required_area;
            let mut has_required_dimension = false;

            // Calculate remaining area and check dimensions
            for &index in &indexes {
                let tile = &self.stock_tiles[index];
                total_area -= tile.area() as i64;
                if tile.max_dimension() >= required_max_dimension {
                    has_required_dimension = true;
                }
            }

            // Check if this is a valid solution
            if total_area <= 0 
                && has_required_dimension 
                && self.is_valid_indexes(&indexes) 
                && !self.is_excluded_by_indexes(&indexes) {
                
                let tiles: Vec<TileDimensions> = indexes.iter()
                    .map(|&i| self.stock_tiles[i].clone())
                    .collect();
                
                let mut solution = StockSolution::from_tiles(tiles);
                solution.sort_panels_asc();

                // Update state for next iteration
                self.previous_returned_stock_tiles_indexes = indexes.clone();
                self.prev_index_to_iterate = iteration_index;

                return Some(solution);
            }

            // Try to get next unused stock tile
            if let Some(next_index) = self.get_next_unused_stock_tile(
                &indexes,
                indexes[iteration_index],
                &self.stock_tiles[indexes[iteration_index]],
            ) {
                indexes[iteration_index] = next_index;
            } else {
                break;
            }
        }

        None
    }

    /// Get the next unused stock tile index that can fit the given tile
    pub(crate) fn get_next_unused_stock_tile(
        &self,
        used_indexes: &[usize],
        start_index: usize,
        min_tile: &TileDimensions,
    ) -> Option<usize> {
        for i in (start_index + 1)..self.stock_tiles.len() {
            if !used_indexes.contains(&i) {
                let stock_tile = &self.stock_tiles[i];
                if stock_tile.width >= min_tile.width || stock_tile.height >= min_tile.height {
                    return Some(i);
                }
            }
        }
        None
    }
}
