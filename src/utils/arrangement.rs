//! Arrangement utilities for generating permutations
//! 
//! This module provides functionality to generate all possible permutations
//! of a given collection of elements. It's useful for optimization algorithms
//! that need to try different arrangements of items.

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

/// Generate permutations with a limit on the number of results
/// 
/// This function is useful when you only need a subset of all possible permutations,
/// which can save memory and computation time for large input sets.
/// 
/// # Arguments
/// * `list` - A vector of elements to permute
/// * `limit` - Maximum number of permutations to generate
/// 
/// # Returns
/// A vector containing up to `limit` permutations
pub fn generate_permutations_limited<T: Clone>(list: Vec<T>, limit: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut count = 0;
    
    fn generate_limited<T: Clone>(
        mut list: Vec<T>, 
        result: &mut Vec<Vec<T>>, 
        count: &mut usize, 
        limit: usize
    ) {
        if *count >= limit {
            return;
        }
        
        if list.is_empty() {
            result.push(vec![]);
            *count += 1;
            return;
        }
        
        let first_element = list.remove(0);
        let mut sub_perms = Vec::new();
        generate_limited(list, &mut sub_perms, count, limit);
        
        for permutation in sub_perms {
            if *count >= limit {
                break;
            }
            
            for i in 0..=permutation.len() {
                if *count >= limit {
                    break;
                }
                
                let mut new_permutation = permutation.clone();
                new_permutation.insert(i, first_element.clone());
                result.push(new_permutation);
                *count += 1;
            }
        }
    }
    
    generate_limited(list, &mut result, &mut count, limit);
    result
}

/// Calculate the factorial of a number (useful for determining permutation count)
/// 
/// # Arguments
/// * `n` - The number to calculate factorial for
/// 
/// # Returns
/// The factorial of n, or None if overflow would occur
pub fn factorial(n: usize) -> Option<usize> {
    if n == 0 || n == 1 {
        return Some(1);
    }
    
    let mut result = 1usize;
    for i in 2..=n {
        result = result.checked_mul(i)?;
    }
    Some(result)
}

/// Get the expected number of permutations for a given input size
/// 
/// This is a convenience function that returns the factorial of the input size,
/// which represents the total number of permutations possible.
pub fn expected_permutation_count(input_size: usize) -> Option<usize> {
    factorial(input_size)
}
