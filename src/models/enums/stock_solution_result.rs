use crate::stock::StockSolution;

/// Result of stock solution generation
#[derive(Debug, Clone)]
pub enum StockSolutionResult {
    /// A valid solution was found
    Solution(StockSolution),
    /// No solution could be found with current constraints
    NoSolution,
    /// All possible solutions have been excluded
    AllExcluded,
}
