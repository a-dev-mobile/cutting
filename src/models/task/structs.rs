//! Task structure definition for cut list optimization
//! 
//! This module contains the main Task struct that manages the lifecycle of cutting calculations,
//! coordinates multiple threads, tracks progress, and aggregates solutions.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{debug, info, warn, error};

use crate::{
    models::{
        CalculationRequest, CalculationResponse, Solution, TileDimensions,
        enums::Status,
    },
    engine::cut_list_thread::CutListThread,
    error::TaskError,
};

/// Task represents a complete cutting optimization job with thread management and progress tracking
#[derive(Debug)]
pub struct Task {
    // Core identification
    id: String,
    
    // Request and response data
    calculation_request: Option<CalculationRequest>,
    solution: Arc<RwLock<Option<CalculationResponse>>>,
    
    // Status and timing
    status: Arc<RwLock<Status>>,
    start_time: SystemTime,
    end_time: Arc<Mutex<Option<SystemTime>>>,
    last_queried: Arc<Mutex<SystemTime>>,
    
    // Thread management
    threads: Arc<Mutex<Vec<Arc<Mutex<CutListThread>>>>>,
    
    // Progress tracking per material
    per_material_percentage_done: Arc<Mutex<HashMap<String, i32>>>,
    
    // Solutions per material
    solutions: Arc<Mutex<HashMap<String, Vec<Solution>>>>,
    
    // Thread group rankings for optimization
    thread_group_rankings: Arc<Mutex<HashMap<String, HashMap<String, i32>>>>,
    
    // Material-specific data
    tile_dimensions_per_material: Option<HashMap<String, Vec<TileDimensions>>>,
    stock_dimensions_per_material: Option<HashMap<String, Vec<TileDimensions>>>,
    no_material_tiles: Vec<TileDimensions>,
    
    // Configuration
    factor: f64,
    is_min_trim_dimension_influenced: bool,
    
    // Logging
    log: Arc<Mutex<String>>,
}

impl Task {
    /// Create a new Task with the given ID
    pub fn new(id: String) -> Self {
        let now = SystemTime::now();
        
        Self {
            id,
            calculation_request: None,
            solution: Arc::new(RwLock::new(None)),
            status: Arc::new(RwLock::new(Status::Queued)), // Java uses IDLE, but our enum uses Queued
            start_time: now,
            end_time: Arc::new(Mutex::new(None)),
            last_queried: Arc::new(Mutex::new(now)),
            threads: Arc::new(Mutex::new(Vec::new())),
            per_material_percentage_done: Arc::new(Mutex::new(HashMap::new())),
            solutions: Arc::new(Mutex::new(HashMap::new())),
            thread_group_rankings: Arc::new(Mutex::new(HashMap::new())),
            tile_dimensions_per_material: None,
            stock_dimensions_per_material: None,
            no_material_tiles: Vec::new(),
            factor: 1.0,
            is_min_trim_dimension_influenced: false,
            log: Arc::new(Mutex::new(String::new())),
        }
    }

    // ===== Basic Getters and Setters =====

    /// Get the task ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set the task ID
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    /// Get the current status
    pub fn status(&self) -> Status {
        *self.status.read().unwrap()
    }

    /// Get the calculation request
    pub fn calculation_request(&self) -> &Option<CalculationRequest> {
        &self.calculation_request
    }

    /// Set the calculation request
    pub fn set_calculation_request(&mut self, request: CalculationRequest) {
        self.calculation_request = Some(request);
    }

    /// Get the solution (returns a clone due to RwLock)
    pub fn solution(&self) -> Option<CalculationResponse> {
        self.solution.read().unwrap().clone()
    }

    /// Set the solution
    pub fn set_solution(&self, solution: CalculationResponse) {
        *self.solution.write().unwrap() = Some(solution);
    }

    /// Get the factor
    pub fn factor(&self) -> f64 {
        self.factor
    }

    /// Set the factor
    pub fn set_factor(&mut self, factor: f64) {
        self.factor = factor;
    }

    /// Check if min trim dimension is influenced
    pub fn is_min_trim_dimension_influenced(&self) -> bool {
        self.is_min_trim_dimension_influenced
    }

    /// Set min trim dimension influenced flag
    pub fn set_min_trim_dimension_influenced(&mut self, influenced: bool) {
        self.is_min_trim_dimension_influenced = influenced;
    }

    /// Get no material tiles
    pub fn no_material_tiles(&self) -> &Vec<TileDimensions> {
        &self.no_material_tiles
    }

    /// Set no material tiles
    pub fn set_no_material_tiles(&mut self, tiles: Vec<TileDimensions>) {
        self.no_material_tiles = tiles;
    }

    /// Get tile dimensions per material
    pub fn tile_dimensions_per_material(&self) -> &Option<HashMap<String, Vec<TileDimensions>>> {
        &self.tile_dimensions_per_material
    }

    /// Set tile dimensions per material
    pub fn set_tile_dimensions_per_material(&mut self, dimensions: HashMap<String, Vec<TileDimensions>>) {
        self.tile_dimensions_per_material = Some(dimensions);
    }

    /// Get stock dimensions per material
    pub fn stock_dimensions_per_material(&self) -> &Option<HashMap<String, Vec<TileDimensions>>> {
        &self.stock_dimensions_per_material
    }

    /// Set stock dimensions per material
    pub fn set_stock_dimensions_per_material(&mut self, dimensions: HashMap<String, Vec<TileDimensions>>) {
        self.stock_dimensions_per_material = Some(dimensions);
    }

    // ===== Status Management =====

    /// Check if the task is currently running
    pub fn is_running(&self) -> bool {
        matches!(self.status(), Status::Running)
    }

    /// Set the task status to running
    /// Returns Ok(()) if successful, Err if task is not in a valid state to start
    pub fn set_running_status(&self) -> Result<(), TaskError> {
        let mut status = self.status.write().unwrap();
        if *status != Status::Queued {
            return Err(TaskError::InvalidStatusTransition {
                from: *status,
                to: Status::Running,
            });
        }
        *status = Status::Running;
        info!("Task {} set to running status", self.id);
        Ok(())
    }

    /// Stop the task
    /// Returns Ok(()) if successful, Err if task is not running
    pub fn stop(&self) -> Result<(), TaskError> {
        let mut status = self.status.write().unwrap();
        if *status != Status::Running {
            return Err(TaskError::InvalidStatusTransition {
                from: *status,
                to: Status::Finished, // Assuming stop means finished
            });
        }
        *status = Status::Finished;
        self.set_end_time();
        info!("Task {} stopped", self.id);
        Ok(())
    }

    /// Terminate the task
    /// Returns Ok(()) if successful, Err if task is not running
    pub fn terminate(&self) -> Result<(), TaskError> {
        let mut status = self.status.write().unwrap();
        if *status != Status::Running {
            return Err(TaskError::InvalidStatusTransition {
                from: *status,
                to: Status::Terminated,
            });
        }
        *status = Status::Terminated;
        self.set_end_time();
        warn!("Task {} terminated", self.id);
        Ok(())
    }

    /// Set the task status to error
    pub fn terminate_error(&self) {
        let mut status = self.status.write().unwrap();
        *status = Status::Error;
        self.set_end_time();
        error!("Task {} terminated with error", self.id);
    }

    // ===== Time Management =====

    /// Get the start time as milliseconds since epoch
    pub fn start_time(&self) -> u64 {
        self.start_time
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Get the end time as milliseconds since epoch
    pub fn end_time(&self) -> u64 {
        self.end_time
            .lock()
            .unwrap()
            .map(|t| {
                t.duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64
            })
            .unwrap_or(0)
    }

    /// Set the end time to now
    fn set_end_time(&self) {
        *self.end_time.lock().unwrap() = Some(SystemTime::now());
    }

    /// Get elapsed time in milliseconds
    pub fn elapsed_time(&self) -> u64 {
        let end_time = self.end_time.lock().unwrap();
        let end = end_time.unwrap_or_else(SystemTime::now);
        end.duration_since(self.start_time)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Get last queried time as milliseconds since epoch
    pub fn last_queried(&self) -> u64 {
        self.last_queried
            .lock()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Set last queried time to now
    pub fn set_last_queried(&self) {
        *self.last_queried.lock().unwrap() = SystemTime::now();
    }

    // ===== Material Management =====

    /// Add a material to compute
    pub fn add_material_to_compute(&self, material: String) {
        {
            let mut solutions = self.solutions.lock().unwrap();
            solutions.insert(material.clone(), Vec::new());
        }
        {
            let mut percentages = self.per_material_percentage_done.lock().unwrap();
            percentages.insert(material.clone(), 0);
        }
        {
            let mut rankings = self.thread_group_rankings.lock().unwrap();
            rankings.insert(material, HashMap::new());
        }
    }

    /// Get solutions for a specific material
    pub fn solutions(&self, material: &str) -> Option<Vec<Solution>> {
        self.solutions
            .lock()
            .unwrap()
            .get(material)
            .cloned()
    }

    /// Set material percentage done
    pub fn set_material_percentage_done(&self, material: String, percentage: i32) {
        {
            let mut percentages = self.per_material_percentage_done.lock().unwrap();
            percentages.insert(material, percentage);
        }
        
        if percentage == 100 {
            self.check_if_finished();
        }
    }

    /// Get overall percentage done (average across all materials)
    pub fn percentage_done(&self) -> i32 {
        let percentages = self.per_material_percentage_done.lock().unwrap();
        if percentages.is_empty() {
            return 0;
        }

        let total: i32 = percentages.values().sum();
        total / percentages.len() as i32
    }

    /// Check if all materials are finished and update status accordingly
    pub fn check_if_finished(&self) {
        if matches!(self.status(), Status::Finished) {
            return;
        }

        let percentages = self.per_material_percentage_done.lock().unwrap();
        let all_finished = percentages.values().all(|&p| p == 100);

        if all_finished {
            let mut status = self.status.write().unwrap();
            *status = Status::Finished;
            self.set_end_time();
            
            if self.solution.read().unwrap().is_none() {
                // Build solution if not already built
                drop(status); // Release the lock before calling build_solution
                if let Some(solution) = self.build_solution() {
                    *self.solution.write().unwrap() = Some(solution);
                }
            }
            
            info!("Task {} finished", self.id);
        }
    }

    // ===== Thread Management =====

    /// Add a thread to the task
    pub fn add_thread(&self, thread: Arc<Mutex<CutListThread>>) {
        let mut threads = self.threads.lock().unwrap();
        threads.push(thread);
    }

    /// Get number of running threads
    pub fn nbr_running_threads(&self) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Running)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get number of queued threads
    pub fn nbr_queued_threads(&self) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Queued)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get number of finished threads
    pub fn nbr_finished_threads(&self) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Finished)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get number of finished threads for a specific material
    pub fn nbr_finished_threads_for_material(&self, material: &str) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Finished) && 
                    t.material().map_or(false, |m| m == material)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get number of terminated threads
    pub fn nbr_terminated_threads(&self) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Terminated)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get number of error threads
    pub fn nbr_error_threads(&self) -> usize {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter(|thread| {
                if let Ok(t) = thread.lock() {
                    matches!(t.status(), Status::Error)
                } else {
                    false
                }
            })
            .count()
    }

    /// Get maximum thread progress percentage
    pub fn max_thread_progress_percentage(&self) -> i32 {
        let threads = self.threads.lock().unwrap();
        threads
            .iter()
            .filter_map(|thread| {
                thread.lock().ok().map(|t| t.percentage_done())
            })
            .max()
            .unwrap_or(0)
    }

    /// Get total number of threads
    pub fn nbr_total_threads(&self) -> usize {
        self.threads.lock().unwrap().len()
    }

    // ===== Thread Group Rankings =====

    /// Get thread group rankings for a material
    pub fn thread_group_rankings(&self, material: &str) -> Option<HashMap<String, i32>> {
        self.thread_group_rankings
            .lock()
            .unwrap()
            .get(material)
            .cloned()
    }

    /// Increment thread group rankings
    pub fn increment_thread_group_rankings(&self, material: &str, thread_group: &str) {
        let mut rankings = self.thread_group_rankings.lock().unwrap();
        let material_rankings = rankings
            .entry(material.to_string())
            .or_insert_with(HashMap::new);
        
        let count = material_rankings
            .entry(thread_group.to_string())
            .or_insert(0);
        *count += 1;
    }

    // ===== Logging =====

    /// Get the current log
    pub fn log(&self) -> String {
        self.log.lock().unwrap().clone()
    }

    /// Set the log content
    pub fn set_log(&self, log_content: String) {
        *self.log.lock().unwrap() = log_content;
    }

    /// Append a line to the log
    pub fn append_line_to_log(&self, line: &str) {
        let mut log = self.log.lock().unwrap();
        if !log.is_empty() {
            log.push('\n');
        }
        log.push_str(line);
    }

    // ===== Solution Management =====

    /// Check if the task has a solution
    pub fn has_solution(&self) -> bool {
        self.solution.read().unwrap().is_some()
    }

    /// Check if the solution has all panels fitting
    pub fn has_solution_all_fit(&self) -> bool {
        self.has_solution() && 
        self.solution
            .read()
            .unwrap()
            .as_ref()
            .map(|s| s.no_fit_panels.is_empty())
            .unwrap_or(false)
    }

    /// Build the final solution from all thread solutions
    /// Returns the built solution or None if no calculation request exists
    pub fn build_solution(&self) -> Option<CalculationResponse> {
        // This would typically use a CalculationResponseBuilder
        // For now, we'll create a placeholder implementation
        debug!("Building solution for task {}", self.id);
        
        // In a real implementation, this would:
        // 1. Collect all solutions from threads
        // 2. Apply optimization algorithms
        // 3. Build the final CalculationResponse
        
        if let Some(request) = &self.calculation_request {
            let response = CalculationResponse {
                version: "1.0.0".to_string(),
                edge_bands: None,
                elapsed_time: self.elapsed_time(),
                id: Some(self.id.clone()),
                panels: Some(Vec::new()), // Would be populated with actual results
                request: Some(request.clone()),
                solution_elapsed_time: Some(self.elapsed_time()),
                task_id: Some(self.id.clone()),
                total_cut_length: 0.0,
                total_nbr_cuts: 0,
                total_used_area: 0.0,
                total_used_area_ratio: 0.0,
                total_wasted_area: 0.0,
                used_stock_panels: None,
                no_fit_panels: Vec::new(),
                mosaics: Vec::new(),
            };
            
            Some(response)
        } else {
            None
        }
    }

    /// Build and set the solution for this task
    pub fn build_and_set_solution(&self) {
        if let Some(solution) = self.build_solution() {
            *self.solution.write().unwrap() = Some(solution);
        }
    }
}

// Thread-safe cloning for Arc<Task>
impl Clone for Task {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            calculation_request: self.calculation_request.clone(),
            solution: Arc::new(RwLock::new(self.solution.read().unwrap().clone())),
            status: Arc::new(RwLock::new(self.status())),
            start_time: self.start_time,
            end_time: Arc::clone(&self.end_time),
            last_queried: Arc::new(Mutex::new(*self.last_queried.lock().unwrap())),
            threads: Arc::clone(&self.threads), // Share threads instead of creating empty Vec
            per_material_percentage_done: Arc::clone(&self.per_material_percentage_done),
            solutions: Arc::clone(&self.solutions),
            thread_group_rankings: Arc::clone(&self.thread_group_rankings),
            tile_dimensions_per_material: self.tile_dimensions_per_material.clone(),
            stock_dimensions_per_material: self.stock_dimensions_per_material.clone(),
            no_material_tiles: self.no_material_tiles.clone(),
            factor: self.factor,
            is_min_trim_dimension_influenced: self.is_min_trim_dimension_influenced,
            log: Arc::clone(&self.log),
        }
    }
}
