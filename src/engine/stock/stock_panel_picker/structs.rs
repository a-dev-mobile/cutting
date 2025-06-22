//! Stock Panel Picker structures for managing stock solution generation and retrieval
//! 
//! This module provides the Rust equivalent of the Java StockPanelPicker

use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tokio::sync::mpsc;
use crate::models::{task::Task, TileDimensions};
use crate::engine::stock::{StockSolution, StockSolutionGenerator};
use crate::errors::Result;


/// Result type for stock panel picker operations
pub type StockPanelPickerResult<T> = Result<T>;

/// Stock Panel Picker manages the generation and retrieval of stock solutions
/// 
/// This is the Rust equivalent of the Java StockPanelPicker class.
/// It provides thread-safe access to generated stock solutions with background generation.
#[derive(Debug)]
pub struct StockPanelPicker {
    /// The stock solution generator
    pub(crate) stock_solution_generator: StockSolutionGenerator,
    
    /// Reference to the task for status checking
    pub(crate) task: Arc<Task>,
    
    /// Generated stock solutions (thread-safe)
    pub(crate) stock_solutions: Arc<Mutex<Vec<StockSolution>>>,
    
    /// Maximum retrieved index for optimization
    pub(crate) max_retrieved_idx: Arc<Mutex<usize>>,
    
    /// Background thread handle for solution generation
    pub(crate) generation_thread: Arc<Mutex<Option<JoinHandle<()>>>>,
    
    /// Channel for communicating with the generation thread
    pub(crate) shutdown_sender: Arc<Mutex<Option<mpsc::UnboundedSender<()>>>>,
}

/// Builder pattern for creating StockPanelPicker instances
#[derive(Debug, Clone)]
pub struct StockPanelPickerBuilder {
    pub(crate) tiles_to_fit: Option<Vec<TileDimensions>>,
    pub(crate) stock_tiles: Option<Vec<TileDimensions>>,
    pub(crate) task: Option<Arc<Task>>,
    pub(crate) max_stock_solution_length_hint: Option<usize>,
}

impl Default for StockPanelPickerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for solution sorting behavior
#[derive(Debug, Clone)]
pub struct SolutionSortConfig {
    /// Whether to sort solutions by total area
    pub sort_by_area: bool,
    
    /// Whether to sort in ascending order (true) or descending (false)
    pub ascending: bool,
}

impl Default for SolutionSortConfig {
    fn default() -> Self {
        Self {
            sort_by_area: true,
            ascending: true, // Sort by smallest area first (like Java implementation)
        }
    }
}

/// Statistics about the stock panel picker state
#[derive(Debug, Clone)]
pub struct StockPanelPickerStats {
    /// Total number of generated solutions
    pub total_solutions: usize,
    
    /// Maximum retrieved index
    pub max_retrieved_idx: usize,
    
    /// Whether the generation thread is still active
    pub is_generating: bool,
    
    /// Required area from the generator
    pub required_area: i64,
}
