//! Validation and utility implementations for CutListThread
//! 
//! This module contains validation logic, sorting utilities, and other helper methods.

use crate::{
    models::Solution,
    error::{OptimizerError, Result},
};
use std::collections::HashSet;

use super::structs::{CutListThread, SolutionComparator};

impl CutListThread {
    /// Sort solutions using the provided comparators
    pub fn sort_solutions(&self, solutions: &mut Vec<Solution>, comparators: &[SolutionComparator]) {
        if comparators.is_empty() {
            return;
        }

        solutions.sort_by(|a, b| {
            for comparator in comparators {
                let result = comparator(a, b);
                if result != std::cmp::Ordering::Equal {
                    return result;
                }
            }
            std::cmp::Ordering::Equal
        });
    }

    /// Validate the thread configuration before execution
    pub fn validate_configuration(&self) -> Result<()> {
        // Validate tiles
        if self.tiles.is_empty() {
            return Err(OptimizerError::InvalidInput { 
                details: "No tiles provided for optimization".to_string() 
            });
        }

        // Validate cut thickness
        if self.cut_thickness < 0 {
            return Err(OptimizerError::InvalidInput {
                details: "Cut thickness cannot be negative".to_string()
            });
        }

        // Validate min trim dimension
        if self.min_trim_dimension < 0 {
            return Err(OptimizerError::InvalidInput {
                details: "Minimum trim dimension cannot be negative".to_string()
            });
        }

        // Validate accuracy factor
        if self.accuracy_factor == 0 {
            return Err(OptimizerError::InvalidInput {
                details: "Accuracy factor must be greater than zero".to_string()
            });
        }

        // Validate stock solution
        if self.stock_solution.is_none() {
            return Err(OptimizerError::InvalidInput {
                details: "Stock solution is required".to_string()
            });
        }

        // Validate tile dimensions
        for (index, tile) in self.tiles.iter().enumerate() {
            if tile.width <= 0 || tile.height <= 0 {
                return Err(OptimizerError::InvalidInput {
                    details: format!("Tile {} has invalid dimensions: {}x{}", 
                                   index, tile.width, tile.height)
                });
            }
        }

        Ok(())
    }

    /// Remove duplicate solutions from the list
    /// Returns the number of duplicates removed
    pub fn remove_duplicated(&self, solutions: &mut Vec<Solution>) -> usize {
        let mut seen = HashSet::new();
        let mut to_remove = Vec::new();
        let mut removed_count = 0;

        for (index, solution) in solutions.iter().enumerate() {
            let mut identifier = String::new();
            for mosaic in solution.get_mosaics() {
                identifier.push_str(&mosaic.root_tile_node().string_identifier());
            }

            if !seen.insert(identifier) {
                to_remove.push(index);
                removed_count += 1;
            }
        }

        // Remove in reverse order to maintain indices
        for &index in to_remove.iter().rev() {
            solutions.remove(index);
        }

        removed_count
    }

    /// Sort and limit solutions based on comparators and accuracy factor
    pub(crate) fn sort_and_limit_solutions(
        &self,
        solutions: &mut Vec<Solution>,
        _use_thread_comparators: bool,
    ) -> Result<()> {
        // For now, just limit by accuracy factor
        // In full implementation, this would use the comparators
        if solutions.len() > self.accuracy_factor as usize {
            solutions.truncate(self.accuracy_factor as usize);
        }
        Ok(())
    }
}
