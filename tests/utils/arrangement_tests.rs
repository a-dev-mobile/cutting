//! Tests for arrangement utilities
//! 
//! This module contains comprehensive tests for the arrangement module,
//! which provides permutation generation functionality.

use cutlist_optimizer_cli::utils::arrangement::*;

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
fn test_four_element_permutations_count() {
    let four = vec![1, 2, 3, 4];
    let result = generate_permutations(four);
    assert_eq!(result.len(), 24); // 4! = 24
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

#[test]
fn test_permutations_iter() {
    let data = vec![1, 2];
    let result: Vec<Vec<i32>> = generate_permutations_iter(data).collect();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&vec![1, 2]));
    assert!(result.contains(&vec![2, 1]));
}

#[test]
fn test_permutations_limited() {
    let data = vec![1, 2, 3, 4];
    let result = generate_permutations_limited(data, 5);
    assert!(result.len() <= 5);
    
    // All results should be valid permutations
    for perm in &result {
        assert_eq!(perm.len(), 4);
        // Check that all elements are present
        assert!(perm.contains(&1));
        assert!(perm.contains(&2));
        assert!(perm.contains(&3));
        assert!(perm.contains(&4));
    }
}

#[test]
fn test_permutations_limited_zero() {
    let data = vec![1, 2, 3];
    let result = generate_permutations_limited(data, 0);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_permutations_limited_more_than_total() {
    let data = vec![1, 2];
    let result = generate_permutations_limited(data, 10);
    assert_eq!(result.len(), 2); // Should not exceed total possible permutations
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), Some(1));
    assert_eq!(factorial(1), Some(1));
    assert_eq!(factorial(2), Some(2));
    assert_eq!(factorial(3), Some(6));
    assert_eq!(factorial(4), Some(24));
    assert_eq!(factorial(5), Some(120));
}

#[test]
fn test_factorial_large_number() {
    // Test that factorial handles large numbers gracefully
    let result = factorial(20);
    assert!(result.is_some());
    
    // Test overflow case
    let overflow_result = factorial(25);
    assert!(overflow_result.is_none());
}

#[test]
fn test_expected_permutation_count() {
    assert_eq!(expected_permutation_count(0), Some(1));
    assert_eq!(expected_permutation_count(1), Some(1));
    assert_eq!(expected_permutation_count(3), Some(6));
    assert_eq!(expected_permutation_count(4), Some(24));
}

#[test]
fn test_permutations_with_duplicates() {
    // Test behavior with duplicate elements
    let data = vec![1, 1, 2];
    let result = generate_permutations(data);
    assert_eq!(result.len(), 6); // Still generates all permutations, including duplicates
    
    // Should contain duplicate permutations
    let count_112 = result.iter().filter(|&perm| *perm == vec![1, 1, 2]).count();
    assert!(count_112 > 0);
}

#[test]
fn test_permutations_performance() {
    // Test with a reasonable size to ensure performance is acceptable
    let data: Vec<i32> = (1..=6).collect();
    let start = std::time::Instant::now();
    let result = generate_permutations(data);
    let duration = start.elapsed();
    
    assert_eq!(result.len(), 720); // 6! = 720
    assert!(duration.as_millis() < 1000, "Permutation generation took too long: {:?}", duration);
}

#[test]
fn test_permutations_memory_usage() {
    // Test that we don't run out of memory with reasonable input sizes
    let data: Vec<i32> = (1..=7).collect();
    let result = generate_permutations(data);
    assert_eq!(result.len(), 5040); // 7! = 5040
    
    // Verify some random permutations are present
    assert!(result.contains(&vec![1, 2, 3, 4, 5, 6, 7]));
    assert!(result.contains(&vec![7, 6, 5, 4, 3, 2, 1]));
}

#[test]
fn test_permutations_with_custom_types() {
    #[derive(Debug, Clone, PartialEq)]
    struct CustomType {
        value: i32,
    }
    
    let data = vec![
        CustomType { value: 1 },
        CustomType { value: 2 },
    ];
    
    let result = generate_permutations(data);
    assert_eq!(result.len(), 2);
    
    let expected1 = vec![CustomType { value: 1 }, CustomType { value: 2 }];
    let expected2 = vec![CustomType { value: 2 }, CustomType { value: 1 }];
    
    assert!(result.contains(&expected1));
    assert!(result.contains(&expected2));
}
