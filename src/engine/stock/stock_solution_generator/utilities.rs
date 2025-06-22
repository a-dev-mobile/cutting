use super::structs::StockSolutionGenerator;
use crate::models::TileDimensions;
use super::super::StockSolution;
use std::collections::HashSet;

impl StockSolutionGenerator {
    /// Check if a stock solution is excluded
    pub(crate) fn is_excluded(&self, solution: &StockSolution) -> bool {
        self.stock_solutions_to_exclude.contains(solution)
    }

    /// Check if a list of stock tiles (by indexes) is excluded
    pub(crate) fn is_excluded_by_indexes(&self, indexes: &[usize]) -> bool {
        if self.stock_solutions_to_exclude.is_empty() {
            return false;
        }

        let tiles: Vec<TileDimensions> = indexes.iter()
            .map(|&i| self.stock_tiles[i].clone())
            .collect();
        
        let solution = StockSolution::from_tiles(tiles);
        self.is_excluded(&solution)
    }

    /// Check if all indexes in the list are unique
    pub(crate) fn is_valid_indexes(&self, indexes: &[usize]) -> bool {
        let mut seen = HashSet::new();
        indexes.iter().all(|&i| seen.insert(i))
    }

    /// Sort stock tiles by area in ascending order
    pub(crate) fn sort_stock_tiles_area_asc(&mut self) {
        self.stock_tiles.sort_by(|a, b| a.area().cmp(&b.area()));
    }
}
