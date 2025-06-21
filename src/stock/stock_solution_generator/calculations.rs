use super::structs::{StockSolutionGenerator, StockSolutionConfig};
use crate::models::TileDimensions;
use crate::stock::StockSolution;

impl StockSolutionGenerator {
    /// Get the total required area
    pub fn get_required_area(&self) -> i64 {
        self.required_area
    }

    /// Calculate required area and dimensions from tiles to fit
    pub(crate) fn calc_required_area(&mut self) {
        self.required_area = 0;
        self.required_max_dimension = 0;
        self.smallest_tile_area = i64::MAX;

        for tile in &self.tiles_to_fit {
            let area = tile.area() as i64;
            self.required_area += area;
            
            if tile.max_dimension() > self.required_max_dimension {
                self.required_max_dimension = tile.max_dimension();
            }
            
            if area < self.smallest_tile_area {
                self.smallest_tile_area = area;
            }
        }
    }

    /// Get the area of the biggest stock tile
    pub(crate) fn get_biggest_stock_tile_area(&self) -> i64 {
        self.stock_tiles.iter()
            .map(|tile| tile.area() as i64)
            .max()
            .unwrap_or(0)
    }

    /// Check if all stock tiles have the same ID (unique panel type)
    pub(crate) fn is_unique_stock_panel(&self) -> bool {
        if self.stock_tiles.is_empty() {
            return true;
        }

        let first_id = self.stock_tiles[0].id;
        self.stock_tiles.iter().all(|tile| tile.id == first_id)
    }

    /// Generate a stock solution using all available panels
    pub(crate) fn gen_all_panel_stock_solution(&self) -> StockSolution {
        let max_tiles = StockSolutionConfig::default().max_stock_solution_length
            .min(self.stock_tiles.len());
        
        let tiles: Vec<TileDimensions> = self.stock_tiles.iter()
            .rev() // Take from the end (largest tiles first after sorting)
            .take(max_tiles)
            .cloned()
            .collect();

        let mut solution = StockSolution::from_tiles(tiles);
        solution.sort_panels_asc();
        solution
    }
}
