use std::time::{Duration, Instant};

/// Convert elapsed time to human readable format
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let millis = duration.subsec_millis();

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else if seconds > 0 {
        format!("{}.{}s", seconds, millis / 100)
    } else {
        format!("{}ms", millis)
    }
}

/// Simple timer for performance measurement
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            name: name.into(),
        }
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    pub fn finish(self) -> Duration {
        let elapsed = self.elapsed();
        tracing::info!("{} completed in {}", self.name, format_duration(elapsed));
        elapsed
    }
}

/// Calculate percentage between two numbers
pub fn percentage(part: f64, total: f64) -> f64 {
    if total == 0.0 {
        0.0
    } else {
        (part / total) * 100.0
    }
}

/// Arrangement utilities for generating permutations
pub mod arrangement {
    /// Generate all permutations of the given vector
    /// 
    /// This function generates all possible permutations of the input vector.
    /// The algorithm works recursively by removing the first element and 
    /// inserting it at every possible position in all permutations of the remaining elements.
    /// 
    /// # Arguments
    /// * `list` - A vector of elements to permute
    /// 
    /// # Returns
    /// A vector containing all permutations, where each permutation is a vector of T
    /// 
    /// # Examples
    /// ```
    /// use cutting::utils::arrangement::generate_permutations;
    /// 
    /// let input = vec![1, 2, 3];
    /// let perms = generate_permutations(input);
    /// assert_eq!(perms.len(), 6); // 3! = 6 permutations
    /// ```
    pub fn generate_permutations<T: Clone>(mut list: Vec<T>) -> Vec<Vec<T>> {
        // Base case: empty list has one permutation (empty permutation)
        if list.is_empty() {
            return vec![vec![]];
        }
        
        // Remove the first element
        let first_element = list.remove(0);
        let mut result = Vec::new();
        
        // Generate permutations of the remaining elements
        for permutation in generate_permutations(list) {
            // Insert the first element at every possible position
            for i in 0..=permutation.len() {
                let mut new_permutation = permutation.clone();
                new_permutation.insert(i, first_element.clone());
                result.push(new_permutation);
            }
        }
        
        result
    }
    
    /// Generate all permutations without consuming the input vector
    /// 
    /// This is a more memory-efficient version that borrows the input
    /// and only clones when necessary.
    /// 
    /// # Arguments
    /// * `list` - A slice of elements to permute
    /// 
    /// # Returns
    /// A vector containing all permutations
    pub fn generate_permutations_borrowed<T: Clone>(list: &[T]) -> Vec<Vec<T>> {
        generate_permutations(list.to_vec())
    }
    
    /// Generate permutations using an iterator-based approach for better memory efficiency
    /// 
    /// This version uses iterators and is more idiomatic Rust, though it still
    /// needs to collect results due to the recursive nature.
    pub fn generate_permutations_iter<T: Clone>(list: Vec<T>) -> impl Iterator<Item = Vec<T>> {
        generate_permutations(list).into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::arrangement::*;
    
    #[test]
    fn test_empty_permutations() {
        let empty: Vec<i32> = vec![];
        let result = generate_permutations(empty);
        let expected: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_single_element_permutations() {
        let single = vec![1];
        let result = generate_permutations(single);
        assert_eq!(result, vec![vec![1]]);
    }
    
    #[test]
    fn test_two_element_permutations() {
        let two = vec![1, 2];
        let result = generate_permutations(two);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec![1, 2]));
        assert!(result.contains(&vec![2, 1]));
    }
    
    #[test]
    fn test_three_element_permutations() {
        let three = vec![1, 2, 3];
        let result = generate_permutations(three);
        assert_eq!(result.len(), 6); // 3! = 6
        
        // Check that all expected permutations are present
        let expected = vec![
            vec![1, 2, 3], vec![2, 1, 3], vec![2, 3, 1],
            vec![1, 3, 2], vec![3, 1, 2], vec![3, 2, 1]
        ];
        
        for perm in expected {
            assert!(result.contains(&perm), "Missing permutation: {:?}", perm);
        }
    }
    
    #[test]
    fn test_string_permutations() {
        let strings = vec!["a".to_string(), "b".to_string()];
        let result = generate_permutations(strings);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec!["a".to_string(), "b".to_string()]));
        assert!(result.contains(&vec!["b".to_string(), "a".to_string()]));
    }
    
    #[test]
    fn test_borrowed_permutations() {
        let data = vec![1, 2, 3];
        let result = generate_permutations_borrowed(&data);
        assert_eq!(result.len(), 6);
        // Original data should be unchanged
        assert_eq!(data, vec![1, 2, 3]);
    }
}
