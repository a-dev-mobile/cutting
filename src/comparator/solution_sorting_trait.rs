//! Solution sorting trait and implementations
//! 
//! This module contains the `SolutionSorting` trait which provides convenient
//! sorting methods for vectors of solutions.

use std::cmp::Ordering;
use crate::models::Solution;
use super::SolutionComparator;

/// Extension trait for Vec<Solution> to provide convenient sorting methods
/// 
/// This trait provides high-performance sorting operations with proper error handling
/// and memory-efficient implementations for solution comparison and ranking.
pub trait SolutionSorting {
    /// Sort solutions using the specified comparator
    /// 
    /// # Performance
    /// Uses Rust's optimized `sort_by` which is typically a hybrid of merge sort
    /// and insertion sort, providing O(n log n) worst-case performance.
    fn sort_by_comparator(&mut self, comparator: SolutionComparator);
    
    /// Sort solutions using a custom comparison function
    /// 
    /// # Arguments
    /// * `compare` - Custom comparison function that must provide consistent ordering
    /// 
    /// # Performance
    /// Same performance characteristics as `sort_by_comparator`.
    fn sort_by_custom<F>(&mut self, compare: F)
    where
        F: FnMut(&Solution, &Solution) -> Ordering;
    
    /// Get the best solution according to the specified comparator
    /// 
    /// Returns `None` if the vector is empty.
    /// 
    /// # Performance
    /// O(n) time complexity, single pass through the collection.
    fn best_by_comparator(&self, comparator: SolutionComparator) -> Option<&Solution>;
    
    /// Get the worst solution according to the specified comparator
    /// 
    /// Returns `None` if the vector is empty.
    /// 
    /// # Performance
    /// O(n) time complexity, single pass through the collection.
    fn worst_by_comparator(&self, comparator: SolutionComparator) -> Option<&Solution>;
    
    /// Sort solutions in-place using an unstable sort for better performance
    /// 
    /// This is faster than stable sorting when the relative order of equal
    /// elements doesn't matter.
    /// 
    /// # Performance
    /// Uses `sort_unstable_by` which is typically faster than stable sorting
    /// and uses less memory.
    fn sort_unstable_by_comparator(&mut self, comparator: SolutionComparator);
    
    /// Check if solutions are already sorted according to the comparator
    /// 
    /// # Returns
    /// `true` if the solutions are in sorted order, `false` otherwise.
    /// 
    /// # Performance
    /// O(n) time complexity, early termination on first out-of-order pair.
    fn is_sorted_by_comparator(&self, comparator: SolutionComparator) -> bool;
    
    /// Get the top N solutions according to the specified comparator
    /// 
    /// This is more efficient than sorting the entire collection when you
    /// only need the best few solutions.
    /// 
    /// # Arguments
    /// * `n` - Number of top solutions to return
    /// * `comparator` - Comparator to use for ranking
    /// 
    /// # Returns
    /// Vector containing up to N best solutions in sorted order.
    /// 
    /// # Performance
    /// O(n log k) where k is the requested number of solutions.
    fn top_n_by_comparator(&self, n: usize, comparator: SolutionComparator) -> Vec<&Solution>;
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
    
    fn sort_unstable_by_comparator(&mut self, comparator: SolutionComparator) {
        self.sort_unstable_by(comparator.compare_fn());
    }
    
    fn is_sorted_by_comparator(&self, comparator: SolutionComparator) -> bool {
        let compare_fn = comparator.compare_fn();
        self.windows(2).all(|pair| {
            match compare_fn(&pair[0], &pair[1]) {
                Ordering::Less | Ordering::Equal => true,
                Ordering::Greater => false,
            }
        })
    }
    
    fn top_n_by_comparator(&self, n: usize, comparator: SolutionComparator) -> Vec<&Solution> {
        if n == 0 || self.is_empty() {
            return Vec::new();
        }
        
        if n >= self.len() {
            // If requesting all or more solutions, just sort and return all
            let mut solutions: Vec<&Solution> = self.iter().collect();
            solutions.sort_by(|a, b| comparator.compare_fn()(a, b));
            return solutions;
        }
        
        // Use a more efficient approach for getting top N
        // Create a vector of references and partially sort
        let mut solutions: Vec<&Solution> = self.iter().collect();
        
        // Use select_nth_unstable for O(n) average case performance
        // This puts the nth element in its correct position and partitions around it
        solutions.select_nth_unstable_by(n - 1, |a, b| comparator.compare_fn()(a, b));
        
        // Sort only the first n elements
        solutions[..n].sort_by(|a, b| comparator.compare_fn()(a, b));
        
        // Return only the top n
        solutions.into_iter().take(n).collect()
    }
}
