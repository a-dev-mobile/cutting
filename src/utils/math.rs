//! Mathematical utility functions
//! 
//! This module provides common mathematical operations and calculations
//! used throughout the cutting optimization system.

/// Calculate percentage between two numbers
/// 
/// This function calculates what percentage the `part` represents of the `total`.
/// It handles the edge case where total is zero by returning 0.0.
/// 
/// # Arguments
/// * `part` - The partial value
/// * `total` - The total value
/// 
/// # Returns
/// The percentage as a floating point number (0.0 to 100.0)
/// 
/// # Examples
/// ```
/// use cutting::utils::math::percentage;
/// 
/// assert_eq!(percentage(25.0, 100.0), 25.0);
/// assert_eq!(percentage(0.0, 100.0), 0.0);
/// assert_eq!(percentage(50.0, 0.0), 0.0); // Edge case: division by zero
/// ```
pub fn percentage(part: f64, total: f64) -> f64 {
    if total == 0.0 {
        0.0
    } else {
        (part / total) * 100.0
    }
}

/// Calculate the efficiency ratio between used and total area/length
/// 
/// This is commonly used in cutting optimization to measure how efficiently
/// material is being used.
/// 
/// # Arguments
/// * `used` - The amount of material used
/// * `available` - The total amount of material available
/// 
/// # Returns
/// Efficiency ratio as a percentage (0.0 to 100.0)
pub fn efficiency_ratio(used: f64, available: f64) -> f64 {
    percentage(used, available)
}

/// Calculate waste percentage
/// 
/// This calculates the percentage of material that is wasted.
/// 
/// # Arguments
/// * `used` - The amount of material used
/// * `total` - The total amount of material
/// 
/// # Returns
/// Waste percentage (0.0 to 100.0)
pub fn waste_percentage(used: f64, total: f64) -> f64 {
    if total == 0.0 {
        0.0
    } else {
        percentage(total - used, total)
    }
}

/// Round a floating point number to a specified number of decimal places
/// 
/// # Arguments
/// * `value` - The value to round
/// * `decimal_places` - Number of decimal places to round to
/// 
/// # Returns
/// The rounded value
/// 
/// # Examples
/// ```
/// use cutting::utils::math::round_to_decimal_places;
/// 
/// assert_eq!(round_to_decimal_places(3.14159, 2), 3.14);
/// assert_eq!(round_to_decimal_places(2.5, 0), 3.0);
/// ```
pub fn round_to_decimal_places(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10_f64.powi(decimal_places as i32);
    (value * multiplier).round() / multiplier
}

/// Check if two floating point numbers are approximately equal
/// 
/// This function is useful for comparing floating point numbers where
/// exact equality might not work due to precision issues.
/// 
/// # Arguments
/// * `a` - First number
/// * `b` - Second number
/// * `epsilon` - The tolerance for comparison (default: 1e-10)
/// 
/// # Returns
/// True if the numbers are approximately equal
pub fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Check if two floating point numbers are approximately equal with default tolerance
pub fn approx_equal_default(a: f64, b: f64) -> bool {
    approx_equal(a, b, 1e-9)
}

/// Clamp a value between a minimum and maximum
/// 
/// # Arguments
/// * `value` - The value to clamp
/// * `min` - Minimum allowed value
/// * `max` - Maximum allowed value
/// 
/// # Returns
/// The clamped value
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation between two values
/// 
/// # Arguments
/// * `start` - Starting value
/// * `end` - Ending value
/// * `t` - Interpolation factor (0.0 to 1.0)
/// 
/// # Returns
/// Interpolated value
pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + t * (end - start)
}

/// Calculate the area of a rectangle
/// 
/// # Arguments
/// * `width` - Width of the rectangle
/// * `height` - Height of the rectangle
/// 
/// # Returns
/// The area
pub fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

/// Calculate the perimeter of a rectangle
/// 
/// # Arguments
/// * `width` - Width of the rectangle
/// * `height` - Height of the rectangle
/// 
/// # Returns
/// The perimeter
pub fn rectangle_perimeter(width: f64, height: f64) -> f64 {
    2.0 * (width + height)
}

/// Statistical functions
pub mod statistics {
    /// Calculate the mean (average) of a slice of numbers
    pub fn mean(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            None
        } else {
            Some(values.iter().sum::<f64>() / values.len() as f64)
        }
    }
    
    /// Calculate the median of a slice of numbers
    pub fn median(values: &mut [f64]) -> Option<f64> {
        if values.is_empty() {
            return None;
        }
        
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = values.len();
        
        if len % 2 == 0 {
            Some((values[len / 2 - 1] + values[len / 2]) / 2.0)
        } else {
            Some(values[len / 2])
        }
    }
    
    /// Find the minimum value in a slice
    pub fn min(values: &[f64]) -> Option<f64> {
        values.iter().fold(None, |acc, &x| {
            Some(acc.map_or(x, |y| x.min(y)))
        })
    }
    
    /// Find the maximum value in a slice
    pub fn max(values: &[f64]) -> Option<f64> {
        values.iter().fold(None, |acc, &x| {
            Some(acc.map_or(x, |y| x.max(y)))
        })
    }
    
    /// Calculate the range (max - min) of values
    pub fn range(values: &[f64]) -> Option<f64> {
        match (min(values), max(values)) {
            (Some(min_val), Some(max_val)) => Some(max_val - min_val),
            _ => None,
        }
    }
    
    /// Calculate the standard deviation of a slice of numbers
    pub fn standard_deviation(values: &[f64]) -> Option<f64> {
        let mean_val = mean(values)?;
        if values.len() <= 1 {
            return Some(0.0);
        }
        
        let variance = values.iter()
            .map(|x| (x - mean_val).powi(2))
            .sum::<f64>() / (values.len() - 1) as f64;
        
        Some(variance.sqrt())
    }
}

/// Conversion utilities
pub mod conversions {
    /// Convert millimeters to meters
    pub fn mm_to_m(mm: f64) -> f64 {
        mm / 1000.0
    }
    
    /// Convert meters to millimeters
    pub fn m_to_mm(m: f64) -> f64 {
        m * 1000.0
    }
    
    /// Convert centimeters to millimeters
    pub fn cm_to_mm(cm: f64) -> f64 {
        cm * 10.0
    }
    
    /// Convert millimeters to centimeters
    pub fn mm_to_cm(mm: f64) -> f64 {
        mm / 10.0
    }
    
    /// Convert inches to millimeters
    pub fn inches_to_mm(inches: f64) -> f64 {
        inches * 25.4
    }
    
    /// Convert millimeters to inches
    pub fn mm_to_inches(mm: f64) -> f64 {
        mm / 25.4
    }
}

/// Optimization-related mathematical functions
pub mod optimization {
    use super::*;
    
    /// Calculate the aspect ratio of a rectangle
    pub fn aspect_ratio(width: f64, height: f64) -> f64 {
        if height == 0.0 {
            f64::INFINITY
        } else {
            width / height
        }
    }
    
    /// Check if a rectangle fits within another rectangle
    pub fn fits_within(
        item_width: f64, 
        item_height: f64, 
        container_width: f64, 
        container_height: f64
    ) -> bool {
        (item_width <= container_width && item_height <= container_height) ||
        (item_width <= container_height && item_height <= container_width)
    }
    
    /// Calculate the utilization score for a cutting pattern
    /// This is used to evaluate how well a cutting pattern uses the available material
    pub fn utilization_score(used_area: f64, total_area: f64, penalty_factor: f64) -> f64 {
        let efficiency = efficiency_ratio(used_area, total_area);
        let waste = waste_percentage(used_area, total_area);
        
        // Higher efficiency is better, higher waste is worse
        efficiency - (waste * penalty_factor)
    }
}
