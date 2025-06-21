//! Task structure definition for cut list optimization
//! 
//! This module contains the main Task struct that manages the lifecycle of cutting calculations,
//! coordinates multiple threads, tracks progress, and aggregates solutions.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
    time::SystemTime,
};

use crate::{
    models::{
        CalculationRequest, CalculationResponse, Solution, TileDimensions,
        enums::Status,
    },
    engine::cut_list_thread::CutListThread,
};

/// Task represents a complete cutting optimization job with thread management and progress tracking
#[derive(Debug)]
pub struct Task {
    // Core identification
    pub(crate) id: String,
    
    // Request and response data
    pub(crate) calculation_request: Option<CalculationRequest>,
    pub(crate) solution: Arc<RwLock<Option<CalculationResponse>>>,
    
    // Status and timing
    pub(crate) status: Arc<RwLock<Status>>,
    pub(crate) start_time: SystemTime,
    pub(crate) end_time: Arc<Mutex<Option<SystemTime>>>,
    pub(crate) last_queried: Arc<Mutex<SystemTime>>,
    
    // Thread management
    pub(crate) threads: Arc<Mutex<Vec<Arc<Mutex<CutListThread>>>>>,
    
    // Progress tracking per material
    pub(crate) per_material_percentage_done: Arc<Mutex<HashMap<String, i32>>>,
    
    // Solutions per material
    pub(crate) solutions: Arc<Mutex<HashMap<String, Vec<Solution>>>>,
    
    // Thread group rankings for optimization
    pub(crate) thread_group_rankings: Arc<Mutex<HashMap<String, HashMap<String, i32>>>>,
    
    // Material-specific data
    pub(crate) tile_dimensions_per_material: Option<HashMap<String, Vec<TileDimensions>>>,
    pub(crate) stock_dimensions_per_material: Option<HashMap<String, Vec<TileDimensions>>>,
    pub(crate) no_material_tiles: Vec<TileDimensions>,
    
    // Configuration
    pub(crate) factor: f64,
    pub(crate) is_min_trim_dimension_influenced: bool,
    
    // Logging
    pub(crate) log: Arc<Mutex<String>>,
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
}

// Thread-safe cloning for Arc<Task>
impl Clone for Task {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            calculation_request: self.calculation_request.clone(),
            solution: Arc::new(RwLock::new(self.solution.read().unwrap().clone())),
            status: Arc::new(RwLock::new(*self.status.read().unwrap())),
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
