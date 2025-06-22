//! Decimal and integer place counting utilities
//!
//! This module provides production-ready implementations of decimal and integer
//! place counting methods, converted from the original Java implementation.

use crate::models::panel::structs::Panel;
use crate::errors::{Result, AppError};

/// Utility functions for counting decimal and integer places in numeric strings
pub struct DecimalPlaceCounter;

impl DecimalPlaceCounter {
    /// Get number of decimal places in a string representation of a number
    /// 
    /// # Arguments
    /// * `value` - String representation of a number
    /// 
    /// # Returns
    /// Number of decimal places (0 if no decimal point found)
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::engine::service::decimal_places::DecimalPlaceCounter;
    /// 
    /// assert_eq!(DecimalPlaceCounter::get_nbr_decimal_places("123.45"), 2);
    /// assert_eq!(DecimalPlaceCounter::get_nbr_decimal_places("123"), 0);
    /// assert_eq!(DecimalPlaceCounter::get_nbr_decimal_places(""), 0);
    /// ```
    pub fn get_nbr_decimal_places(value: &str) -> usize {
        // Handle null/empty input safely
        if value.is_empty() {
            return 0;
        }
        
        // Find decimal point position
        if let Some(dot_index) = value.find('.') {
            // Ensure we don't underflow on edge cases like "123."
            value.len().saturating_sub(dot_index + 1)
        } else {
            0
        }
    }

    /// Get number of integer places in a string representation of a number
    /// 
    /// # Arguments
    /// * `value` - String representation of a number
    /// 
    /// # Returns
    /// Number of integer places (0 for empty string)
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::engine::service::decimal_places::DecimalPlaceCounter;
    /// 
    /// assert_eq!(DecimalPlaceCounter::get_nbr_integer_places("123.45"), 3);
    /// assert_eq!(DecimalPlaceCounter::get_nbr_integer_places("123"), 3);
    /// assert_eq!(DecimalPlaceCounter::get_nbr_integer_places(""), 0);
    /// ```
    pub fn get_nbr_integer_places(value: &str) -> usize {
        // Handle null/empty input safely
        if value.is_empty() {
            return 0;
        }
        
        // Find decimal point position
        if let Some(dot_index) = value.find('.') {
            dot_index
        } else {
            value.len()
        }
    }

    /// Get maximum number of decimal places from a collection of panels
    /// 
    /// # Arguments
    /// * `panels` - Slice of Panel structs to analyze
    /// 
    /// # Returns
    /// Maximum number of decimal places found across all enabled panels
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::engine::service::decimal_places::DecimalPlaceCounter;
    /// use cutlist_optimizer_cli::models::panel::structs::Panel;
    /// 
    /// let panels = vec![
    ///     Panel { width: Some("123.45".to_string()), height: Some("67.89".to_string()), enabled: true, ..Default::default() },
    ///     Panel { width: Some("1.0".to_string()), height: Some("2.123".to_string()), enabled: true, ..Default::default() },
    /// ];
    /// 
    /// assert_eq!(DecimalPlaceCounter::get_max_nbr_decimal_places(&panels), 3);
    /// ```
    pub fn get_max_nbr_decimal_places(panels: &[Panel]) -> usize {
        panels
            .iter()
            .filter(|panel| panel.enabled)
            .map(|panel| {
                let width_str = panel.width.as_deref().unwrap_or("0");
                let height_str = panel.height.as_deref().unwrap_or("0");
                std::cmp::max(
                    Self::get_nbr_decimal_places(width_str),
                    Self::get_nbr_decimal_places(height_str)
                )
            })
            .max()
            .unwrap_or(0)
    }

    /// Get maximum number of integer places from a collection of panels
    /// 
    /// # Arguments
    /// * `panels` - Slice of Panel structs to analyze
    /// 
    /// # Returns
    /// Maximum number of integer places found across all enabled panels
    /// 
    /// # Examples
    /// ```
    /// use cutlist_optimizer_cli::engine::service::decimal_places::DecimalPlaceCounter;
    /// use cutlist_optimizer_cli::models::panel::structs::Panel;
    /// 
    /// let panels = vec![
    ///     Panel { width: Some("123.45".to_string()), height: Some("67.89".to_string()), enabled: true, ..Default::default() },
    ///     Panel { width: Some("1.0".to_string()), height: Some("2.123".to_string()), enabled: true, ..Default::default() },
    /// ];
    /// 
    /// assert_eq!(DecimalPlaceCounter::get_max_nbr_integer_places(&panels), 3);
    /// ```
    pub fn get_max_nbr_integer_places(panels: &[Panel]) -> usize {
        panels
            .iter()
            .filter(|panel| panel.enabled)
            .map(|panel| {
                let width_str = panel.width.as_deref().unwrap_or("0");
                let height_str = panel.height.as_deref().unwrap_or("0");
                std::cmp::max(
                    Self::get_nbr_integer_places(width_str),
                    Self::get_nbr_integer_places(height_str)
                )
            })
            .max()
            .unwrap_or(0)
    }

    /// Validate numeric string format and extract decimal/integer places
    /// 
    /// # Arguments
    /// * `value` - String to validate and analyze
    /// 
    /// # Returns
    /// Result containing tuple of (integer_places, decimal_places) or error
    /// 
    /// # Errors
    /// Returns error if the string is not a valid numeric format
    pub fn validate_and_count_places(value: &str) -> Result<(usize, usize)> {
        if value.is_empty() {
            return Err(AppError::invalid_input("Empty numeric string"));
        }

        // Basic validation - check if it's a valid number format
        if let Err(_) = value.parse::<f64>() {
            return Err(AppError::invalid_input(&format!("Invalid numeric format: '{}'", value)));
        }

        let integer_places = Self::get_nbr_integer_places(value);
        let decimal_places = Self::get_nbr_decimal_places(value);

        Ok((integer_places, decimal_places))
    }

    /// Check if total digits (integer + decimal) exceed maximum allowed
    /// 
    /// # Arguments
    /// * `panels` - Panels to check
    /// * `max_allowed_digits` - Maximum total digits allowed
    /// 
    /// # Returns
    /// Result indicating if validation passed, or error with details
    pub fn validate_digit_limits(panels: &[Panel], max_allowed_digits: usize) -> Result<()> {
        let max_decimal = Self::get_max_nbr_decimal_places(panels);
        let max_integer = Self::get_max_nbr_integer_places(panels);
        let total_digits = max_decimal + max_integer;

        if total_digits > max_allowed_digits {
            return Err(AppError::invalid_input(&format!(
                "Maximum allowed digits exceeded: decimal[{}] + integer[{}] = {} > max[{}]",
                max_decimal, max_integer, total_digits, max_allowed_digits
            )));
        }

        Ok(())
    }
}
