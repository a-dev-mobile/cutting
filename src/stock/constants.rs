//! Constants for stock solution generation and management
//! 
//! This module centralizes all configuration constants used throughout the stock
//! solution generation system to ensure consistency and easy maintenance.

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
