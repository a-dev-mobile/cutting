//! Timing and performance measurement utilities
//! 
//! This module provides utilities for measuring execution time and formatting
//! duration values in human-readable formats.

use std::time::{Duration, Instant};

/// Convert elapsed time to human readable format
/// 
/// This function formats a Duration into a human-readable string representation.
/// It automatically chooses the most appropriate unit based on the duration length.
/// 
/// # Arguments
/// * `duration` - The duration to format
/// 
/// # Returns
/// A formatted string representing the duration
/// 
/// # Examples
/// ```
/// use cutting::utils::timing::format_duration;
/// use std::time::Duration;
/// 
/// assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
/// assert_eq!(format_duration(Duration::from_secs(65)), "1m 5s");
/// assert_eq!(format_duration(Duration::from_secs(3665)), "1h 1m 5s");
/// ```
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
/// 
/// This struct provides an easy way to measure execution time of code blocks.
/// It automatically logs the elapsed time when dropped or when `finish()` is called.
/// 
/// # Examples
/// ```
/// use cutting::utils::timing::Timer;
/// 
/// let timer = Timer::new("My operation");
/// // ... do some work ...
/// let elapsed = timer.finish(); // Logs and returns elapsed time
/// ```
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    /// Create a new timer with the given name
    /// 
    /// The timer starts measuring immediately upon creation.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            name: name.into(),
        }
    }
    
    /// Get the elapsed time since the timer was created
    /// 
    /// This method does not consume the timer, so it can be called multiple times.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    /// Finish the timer and log the elapsed time
    /// 
    /// This method consumes the timer and returns the final elapsed time.
    /// It also logs the result using the tracing crate.
    pub fn finish(self) -> Duration {
        let elapsed = self.elapsed();
        tracing::info!("{} completed in {}", self.name, format_duration(elapsed));
        elapsed
    }
    
    /// Reset the timer to start measuring from now
    /// 
    /// This method resets the start time to the current instant.
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
    
    /// Get a checkpoint duration without finishing the timer
    /// 
    /// This method returns the elapsed time and logs it, but keeps the timer running.
    pub fn checkpoint(&self, checkpoint_name: &str) -> Duration {
        let elapsed = self.elapsed();
        tracing::debug!("{} - {}: {}", self.name, checkpoint_name, format_duration(elapsed));
        elapsed
    }
}

impl Drop for Timer {
    /// Automatically log the elapsed time when the timer is dropped
    fn drop(&mut self) {
        let elapsed = self.elapsed();
        tracing::debug!("{} dropped after {}", self.name, format_duration(elapsed));
    }
}

/// Measure the execution time of a closure
/// 
/// This function executes the provided closure and returns both its result
/// and the time it took to execute.
/// 
/// # Arguments
/// * `name` - A name for the operation (used in logging)
/// * `f` - The closure to execute and measure
/// 
/// # Returns
/// A tuple containing (result, elapsed_duration)
/// 
/// # Examples
/// ```
/// use cutting::utils::timing::measure_time;
/// 
/// let (result, duration) = measure_time("calculation", || {
///     // Some expensive calculation
///     42
/// });
/// ```
pub fn measure_time<F, R>(name: &str, f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let timer = Timer::new(name);
    let result = f();
    let duration = timer.finish();
    (result, duration)
}

/// Measure execution time and only return the result
/// 
/// This is a convenience function that measures time but only returns the result,
/// automatically logging the duration.
pub fn timed<F, R>(name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let (result, _) = measure_time(name, f);
    result
}

/// Convert duration to different time units
pub mod conversions {
    use std::time::Duration;
    
    /// Convert duration to milliseconds as f64
    pub fn to_millis_f64(duration: Duration) -> f64 {
        duration.as_secs_f64() * 1000.0
    }
    
    /// Convert duration to seconds as f64
    pub fn to_seconds_f64(duration: Duration) -> f64 {
        duration.as_secs_f64()
    }
    
    /// Convert duration to minutes as f64
    pub fn to_minutes_f64(duration: Duration) -> f64 {
        duration.as_secs_f64() / 60.0
    }
    
    /// Convert duration to hours as f64
    pub fn to_hours_f64(duration: Duration) -> f64 {
        duration.as_secs_f64() / 3600.0
    }
}

/// Performance measurement utilities
pub mod performance {
    use super::*;
    use std::collections::VecDeque;
    
    /// A rolling average calculator for performance metrics
    pub struct RollingAverage {
        values: VecDeque<Duration>,
        max_samples: usize,
    }
    
    impl RollingAverage {
        /// Create a new rolling average calculator
        pub fn new(max_samples: usize) -> Self {
            Self {
                values: VecDeque::with_capacity(max_samples),
                max_samples,
            }
        }
        
        /// Add a new sample to the rolling average
        pub fn add_sample(&mut self, duration: Duration) {
            if self.values.len() >= self.max_samples {
                self.values.pop_front();
            }
            self.values.push_back(duration);
        }
        
        /// Get the current average duration
        pub fn average(&self) -> Option<Duration> {
            if self.values.is_empty() {
                return None;
            }
            
            let total_nanos: u128 = self.values.iter()
                .map(|d| d.as_nanos())
                .sum();
            
            let avg_nanos = total_nanos / self.values.len() as u128;
            Some(Duration::from_nanos(avg_nanos as u64))
        }
        
        /// Get the number of samples currently stored
        pub fn sample_count(&self) -> usize {
            self.values.len()
        }
        
        /// Clear all samples
        pub fn clear(&mut self) {
            self.values.clear();
        }
    }
}
