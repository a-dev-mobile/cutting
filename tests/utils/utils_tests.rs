//! Tests for utils module functionality

use cutlist_optimizer_cli::utils::*;
use cutlist_optimizer_cli::utils::validation;
use cutlist_optimizer_cli::utils::misc;

#[test]
fn test_util_error_display() {
    let err = UtilError::InvalidInput("test error".to_string());
    assert_eq!(err.to_string(), "Invalid input: test error");
}

#[test]
fn test_validation_positive() {
    assert!(validation::validate_positive(1.0, "test").is_ok());
    assert!(validation::validate_positive(-1.0, "test").is_err());
    assert!(validation::validate_positive(0.0, "test").is_err());
}

#[test]
fn test_validation_non_negative() {
    assert!(validation::validate_non_negative(1.0, "test").is_ok());
    assert!(validation::validate_non_negative(0.0, "test").is_ok());
    assert!(validation::validate_non_negative(-1.0, "test").is_err());
}

#[test]
fn test_validation_not_empty() {
    let empty: Vec<i32> = vec![];
    let non_empty = vec![1, 2, 3];
    
    assert!(validation::validate_not_empty(&non_empty, "test").is_ok());
    assert!(validation::validate_not_empty(&empty, "test").is_err());
}

#[test]
fn test_validation_range() {
    assert!(validation::validate_range(5.0, 0.0, 10.0, "test").is_ok());
    assert!(validation::validate_range(-1.0, 0.0, 10.0, "test").is_err());
    assert!(validation::validate_range(11.0, 0.0, 10.0, "test").is_err());
}

#[test]
fn test_misc_format_number() {
    assert_eq!(misc::format_number(123.45), "123.45");
    assert_eq!(misc::format_number(1234.56), "1234.56");
}

#[test]
fn test_misc_truncate_string() {
    assert_eq!(misc::truncate_string("hello", 10), "hello");
    assert_eq!(misc::truncate_string("hello world", 8), "hello...");
    assert_eq!(misc::truncate_string("hi", 2), "hi");
}
