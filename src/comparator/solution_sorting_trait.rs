//! Solution sorting trait and implementations
//! 
//! This module contains the `SolutionSorting` trait which provides convenient
//! sorting methods for vectors of solutions.

use std::cmp::Ordering;
use crate::models::Solution;
use super::SolutionComparator;

/// Extension trait for Vec<Solution> to provide convenient sorting methods
pub trait SolutionSorting {
    /// Sort solutions using the specified comparator
    fn sort_by_comparator(&mut self, comparator: SolutionComparator);
    
    /// Sort solutions using a custom comparison function
    fn sort_by_custom<F>(&mut self, compare: F)
    where
        F: FnMut(&Solution, &Solution) -> Ordering;
    
    /// Get the best solution according to the specified comparator
    /// 
    /// Returns `None` if the vector is empty.
    fn best_by_comparator(&self, comparator: SolutionComparator) -> Option<&Solution>;
    
    /// Get the worst solution according to the specified comparator
    /// 
    /// Returns `None` if the vector is empty.
    fn worst_by_comparator(&self, comparator: SolutionComparator) -> Option<&Solution>;
}

impl SolutionSorting for Vec<Solution> {
    fn sort_by_comparator(&mut self, comparator: SolutionComparator) {
        self.sort_by(comparator.compare_fn());
    }
    
    fn sort_by_custom<F>(&mut self, mut compare: F)
    where
        F: FnMut(&Solution, &Solution) -> Ordering,
    {
        self.sort_by(|a, b| compare(a, b));
    }
    
    fn best_by_comparator(&self, comparator: SolutionComparator) -> Option<&Solution> {
        self.iter().min_by(|a, b| comparator.compare_fn()(a, b))
    }
    
    fn worst_by_comparator(&self, comparator: SolutionComparator) -> Option<&Solution> {
        self.iter().max_by(|a, b| comparator.compare_fn()(a, b))
    }
}
