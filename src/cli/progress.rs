//! Progress reporting utilities for CLI operations

use std::time::{Duration, Instant};
use crate::logging::{log_info, log_progress};
use crate::stock::constants::PerformanceConstants;

/// A simple progress reporter for long-running operations
pub struct ProgressReporter {
    start_time: Instant,
    last_update: Instant,
    total_steps: Option<usize>,
    current_step: usize,
    message: String,
}

impl ProgressReporter {
    /// Create a new progress reporter
    pub fn new(message: String) -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_update: now,
            total_steps: None,
            current_step: 0,
            message,
        }
    }

    /// Create a new progress reporter with known total steps
    pub fn with_total(message: String, total_steps: usize) -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_update: now,
            total_steps: Some(total_steps),
            current_step: 0,
            message,
        }
    }

    /// Update progress to the next step
    pub fn step(&mut self) {
        self.current_step += 1;
        self.update();
    }

    /// Update progress with a custom message
    pub fn step_with_message(&mut self, message: String) {
        self.current_step += 1;
        self.message = message;
        self.update();
    }

    /// Set the current step directly
    pub fn set_step(&mut self, step: usize) {
        self.current_step = step;
        self.update();
    }

    /// Update the display (called automatically by step methods)
    fn update(&mut self) {
        let now = Instant::now();
        
        // Only update display every 100ms to avoid spam
        if now.duration_since(self.last_update) < Duration::from_millis(PerformanceConstants::PROGRESS_UPDATE_INTERVAL_MS) {
            return;
        }
        
        self.last_update = now;
        
        let elapsed = now.duration_since(self.start_time);
        
        if let Some(total) = self.total_steps {
            let percentage = (self.current_step as f64 / total as f64 * 100.0).min(100.0);
            log_progress!(
                "[{:6.2}%] {} ({}/{}) - {:?}",
                percentage, self.message, self.current_step, total, elapsed
            );
        } else {
            log_progress!(
                "[Step {}] {} - {:?}",
                self.current_step, self.message, elapsed
            );
        }
    }

    /// Finish the progress and show completion message
    pub fn finish(&self) {
        let elapsed = Instant::now().duration_since(self.start_time);
        if let Some(total) = self.total_steps {
            log_info!(
                "✓ {} completed ({}/{}) in {:?}",
                self.message, self.current_step, total, elapsed
            );
        } else {
            log_info!(
                "✓ {} completed ({} steps) in {:?}",
                self.message, self.current_step, elapsed
            );
        }
    }
}

/// Simple spinner for indeterminate progress
pub struct Spinner {
    chars: Vec<char>,
    current: usize,
    message: String,
}

impl Spinner {
    /// Create a new spinner
    pub fn new(message: String) -> Self {
        Self {
            chars: vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            current: 0,
            message,
        }
    }

    /// Update the spinner display
    pub fn tick(&mut self) {
        // Используем log_progress для спиннера
        log_progress!("{} {}", self.chars[self.current], self.message);
        self.current = (self.current + 1) % self.chars.len();
    }

    /// Finish the spinner with a completion message
    pub fn finish(&self, message: String) {
        log_info!("\r✓ {}", message);
    }
}
