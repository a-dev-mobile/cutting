//! Cut List Thread Structure Definition
//! 
//! This module contains the main CutListThread struct definition.

use crate::{
    models::{
        Solution, TileDimensions,
        task::Task,
    },
    stock::StockSolution,
    constants::ConfigurationDefaults,
    CutDirection, Status,
};
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

/// Type alias for solution comparator functions
pub type SolutionComparator = Box<dyn Fn(&Solution, &Solution) -> std::cmp::Ordering + Send + Sync>;

/// Cut List Thread - Main computation engine for cutting optimization
/// 
/// This struct represents a thread that computes cutting solutions for a given set of tiles.
/// It's designed to be run concurrently with other threads to explore different solution spaces.
pub struct CutListThread {
    // Configuration fields
    pub(crate) accuracy_factor: usize,
    pub(crate) cut_thickness: i32,
    pub(crate) min_trim_dimension: i32,
    pub(crate) first_cut_orientation: CutDirection,
    pub(crate) consider_grain_direction: bool,
    
    // Input data
    pub(crate) tiles: Vec<TileDimensions>,
    pub(crate) stock_solution: Option<StockSolution>,
    pub(crate) task: Option<Arc<Mutex<Task>>>,
    
    // Comparators for solution ranking
    pub(crate) thread_prioritized_comparators: Vec<SolutionComparator>,
    pub(crate) final_solution_prioritized_comparators: Vec<SolutionComparator>,
    
    // Results and state
    pub(crate) solutions: Vec<Solution>,
    pub(crate) all_solutions: Arc<Mutex<Vec<Solution>>>,
    pub(crate) status: Status,
    pub(crate) percentage_done: i32,
    pub(crate) start_time: Option<Instant>,
    
    // Metadata
    pub(crate) group: Option<String>,
    pub(crate) aux_info: Option<String>,
}

impl CutListThread {
    /// Create a new CutListThread with default configuration
    pub fn new() -> Self {
        Self {
            accuracy_factor: ConfigurationDefaults::DEFAULT_ACCURACY_FACTOR as usize,
            cut_thickness: 0,
            min_trim_dimension: 0,
            first_cut_orientation: CutDirection::Both,
            consider_grain_direction: false,
            tiles: Vec::new(),
            stock_solution: None,
            task: None,
            thread_prioritized_comparators: Vec::new(),
            final_solution_prioritized_comparators: Vec::new(),
            solutions: Vec::new(),
            all_solutions: Arc::new(Mutex::new(Vec::new())),
            status: Status::Queued,
            percentage_done: 0,
            start_time: None,
            group: None,
            aux_info: None,
        }
    }
}

impl Default for CutListThread {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for CutListThread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CutListThread")
            .field("accuracy_factor", &self.accuracy_factor)
            .field("cut_thickness", &self.cut_thickness)
            .field("min_trim_dimension", &self.min_trim_dimension)
            .field("first_cut_orientation", &self.first_cut_orientation)
            .field("consider_grain_direction", &self.consider_grain_direction)
            .field("tiles", &self.tiles)
            .field("stock_solution", &self.stock_solution)
            .field("task", &self.task)
            .field("thread_prioritized_comparators", &format!("{} comparators", self.thread_prioritized_comparators.len()))
            .field("final_solution_prioritized_comparators", &format!("{} comparators", self.final_solution_prioritized_comparators.len()))
            .field("solutions", &self.solutions)
            .field("all_solutions", &self.all_solutions)
            .field("status", &self.status)
            .field("percentage_done", &self.percentage_done)
            .field("start_time", &self.start_time)
            .field("group", &self.group)
            .field("aux_info", &self.aux_info)
            .finish()
    }
}
