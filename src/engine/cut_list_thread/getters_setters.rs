//! Getter and setter implementations for CutListThread
//! 
//! This module contains all the getter and setter methods for the CutListThread struct.

use crate::{
    models::{
        Solution, TileDimensions,
        task::Task,
    },
    CutDirection, Status,
};
use crate::engine::stock::StockSolution;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use super::structs::{CutListThread, SolutionComparator};

impl CutListThread {
    // Basic property getters and setters
    
    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }

    pub fn set_group(&mut self, group: Option<String>) {
        self.group = group;
    }

    pub fn aux_info(&self) -> Option<&str> {
        self.aux_info.as_deref()
    }

    pub fn set_aux_info(&mut self, aux_info: Option<String>) {
        self.aux_info = aux_info;
    }

    pub fn task(&self) -> Option<Arc<Mutex<Task>>> {
        self.task.clone()
    }

    pub fn set_task(&mut self, task: Option<Arc<Mutex<Task>>>) {
        self.task = task;
    }

    // Comparator getters and setters
    
    pub fn thread_prioritized_comparators(&self) -> &[SolutionComparator] {
        &self.thread_prioritized_comparators
    }

    pub fn set_thread_prioritized_comparators(&mut self, comparators: Vec<SolutionComparator>) {
        self.thread_prioritized_comparators = comparators;
    }

    pub fn final_solution_prioritized_comparators(&self) -> &[SolutionComparator] {
        &self.final_solution_prioritized_comparators
    }

    pub fn set_final_solution_prioritized_comparators(&mut self, comparators: Vec<SolutionComparator>) {
        self.final_solution_prioritized_comparators = comparators;
    }

    // Status and configuration getters and setters
    
    pub fn status(&self) -> Status {
        self.status
    }

    pub fn cut_thickness(&self) -> i32 {
        self.cut_thickness
    }

    pub fn set_cut_thickness(&mut self, thickness: i32) {
        self.cut_thickness = thickness;
    }

    pub fn min_trim_dimension(&self) -> i32 {
        self.min_trim_dimension
    }

    pub fn set_min_trim_dimension(&mut self, dimension: i32) {
        self.min_trim_dimension = dimension;
    }

    pub fn first_cut_orientation(&self) -> CutDirection {
        self.first_cut_orientation
    }

    pub fn set_first_cut_orientation(&mut self, orientation: CutDirection) {
        self.first_cut_orientation = orientation;
    }

    pub fn consider_grain_direction(&self) -> bool {
        self.consider_grain_direction
    }

    pub fn set_consider_grain_direction(&mut self, consider: bool) {
        self.consider_grain_direction = consider;
    }

    pub fn percentage_done(&self) -> i32 {
        self.percentage_done
    }

    // Data collection getters and setters
    
    pub fn tiles(&self) -> &[TileDimensions] {
        &self.tiles
    }

    pub fn set_tiles(&mut self, tiles: Vec<TileDimensions>) {
        self.tiles = tiles;
    }

    pub fn solutions(&self) -> &[Solution] {
        &self.solutions
    }

    pub fn set_solutions(&mut self, solutions: Vec<Solution>) {
        self.solutions = solutions;
    }

    pub fn accuracy_factor(&self) -> usize {
        self.accuracy_factor
    }

    pub fn set_accuracy_factor(&mut self, factor: usize) {
        self.accuracy_factor = factor;
    }

    pub fn all_solutions(&self) -> Arc<Mutex<Vec<Solution>>> {
        self.all_solutions.clone()
    }

    pub fn set_all_solutions(&mut self, solutions: Arc<Mutex<Vec<Solution>>>) {
        self.all_solutions = solutions;
    }

    pub fn stock_solution(&self) -> Option<&StockSolution> {
        self.stock_solution.as_ref()
    }

    pub fn set_stock_solution(&mut self, stock_solution: Option<StockSolution>) {
        self.stock_solution = stock_solution;
    }

    // Computed property getters
    
    /// Get the material from the first solution (if any)
    pub fn material(&self) -> Option<String> {
        if let Ok(all_solutions) = self.all_solutions.lock() {
            all_solutions.first()
                .and_then(|s| s.get_material())
                .map(|m| m.to_string())
        } else {
            None
        }
    }

    /// Get elapsed time since thread started
    pub fn elapsed_time(&self) -> Duration {
        self.start_time
            .map(|start| start.elapsed())
            .unwrap_or_default()
    }

    /// Get the elapsed time in milliseconds since the thread started
    pub fn get_elapsed_time_millis(&self) -> u64 {
        self.elapsed_time().as_millis() as u64
    }
}
