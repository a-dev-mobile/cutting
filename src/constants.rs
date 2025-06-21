//! Global constants for the cutting optimization system
//! 
//! This module centralizes all configuration constants used throughout the
//! application to ensure consistency and easy maintenance.

/// Stock solution generation configuration constants
pub struct StockConstants;

impl StockConstants {
    /// Minimum number of initial stock solutions to generate before considering
    /// the generation process as started. This ensures we have a baseline number
    /// of solutions available for immediate retrieval.
    pub const MIN_INIT_STOCK_SOLUTIONS_TO_GENERATE: usize = 10;
    
    /// Minimum number of stock solutions to generate when an "all fit" solution
    /// exists. When we find a solution that fits all tiles, we continue generating
    /// more solutions to provide alternatives and potentially better optimizations.
    pub const MIN_STOCK_SOLUTIONS_TO_GENERATE_WITH_ALL_FIT_SOLUTION: usize = 100;
    
    /// Maximum number of iterations allowed in blocking operations to prevent
    /// infinite loops. This acts as a safety mechanism for operations that wait
    /// for solutions or other conditions.
    pub const MAX_ITERATIONS: u32 = 10000;
    
    /// Sleep duration in milliseconds when waiting for solutions in the background
    /// generation thread. This controls how frequently the thread checks for
    /// termination conditions when it has generated enough solutions.
    pub const SOLUTION_WAIT_SLEEP_MS: u64 = 1000;
    
    /// Sleep duration in milliseconds for retry operations in blocking methods.
    /// This controls how frequently we retry operations like solution retrieval
    /// when waiting for new solutions to be generated.
    pub const RETRY_SLEEP_MS: u64 = 100;
}

/// Configuration default values for cutting optimization
pub struct ConfigurationDefaults;

impl ConfigurationDefaults {
    /// Default accuracy factor for cutting calculations. Higher values provide
    /// more precise calculations but may increase processing time.
    pub const DEFAULT_ACCURACY_FACTOR: i32 = 100;
    
    /// Default cut thickness in millimeters. This represents the width of the
    /// saw blade or cutting tool that removes material during cutting.
    pub const DEFAULT_CUT_THICKNESS: i32 = 3;
    
    /// Default minimum trim dimension in millimeters. Pieces smaller than this
    /// are considered waste and not usable for further cutting operations.
    pub const DEFAULT_MIN_TRIM_DIMENSION: i32 = 10;
    
    /// Default optimization factor (1-10 scale). Higher values provide more
    /// thorough optimization but require more processing time.
    pub const DEFAULT_OPTIMIZATION_FACTOR: i32 = 5;
    
    /// Maximum allowed optimization factor. Values above this are considered
    /// invalid and will cause configuration validation to fail.
    pub const MAX_OPTIMIZATION_FACTOR: i32 = 10;
    
    /// Minimum allowed optimization factor. Values below this are considered
    /// invalid and will cause configuration validation to fail.
    pub const MIN_OPTIMIZATION_FACTOR: i32 = 1;
}

/// Performance and threading configuration constants
pub struct PerformanceConstants;

impl PerformanceConstants {
    /// Default maximum length for stock solution arrays. This limits memory
    /// usage and prevents excessive solution generation.
    pub const DEFAULT_MAX_STOCK_SOLUTION_LENGTH: usize = 1000;
    
    /// Thread check interval in milliseconds. This controls how frequently
    /// background threads check for termination signals and status updates.
    pub const THREAD_CHECK_INTERVAL_MS: u64 = 1000;
    
    /// Progress update interval in milliseconds. This controls how frequently
    /// progress indicators are updated to avoid excessive logging or UI updates.
    pub const PROGRESS_UPDATE_INTERVAL_MS: u64 = 100;
}

/// Mathematical and conversion constants
pub struct MathConstants;

impl MathConstants {
    /// Percentage multiplier for converting ratios to percentages (0.5 -> 50%)
    pub const PERCENTAGE_MULTIPLIER: f64 = 100.0;
    
    /// Time conversion factor from seconds to milliseconds
    pub const SECONDS_TO_MILLIS: f64 = 1000.0;
    
    /// Time conversion factor from seconds to minutes
    pub const SECONDS_TO_MINUTES: f64 = 60.0;
    
    /// Time conversion factor from seconds to hours
    pub const SECONDS_TO_HOURS: f64 = 3600.0;
    
    /// Milliseconds divisor for formatting sub-second precision in timing displays
    pub const MILLIS_PRECISION_DIVISOR: u32 = 100;
}
