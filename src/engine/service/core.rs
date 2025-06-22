//! Core service implementation
//!
//! This module contains the main service struct and its core functionality,
//! including initialization, configuration, and basic state management.

use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
use tokio::sync::{Semaphore, mpsc, Mutex};
use chrono::{DateTime, Utc};
use tracing::trace;

use crate::{
    errors::Result,
    models::{
        tile_dimensions::structs::TileDimensions,
        panel::structs::Panel,
    },
    engine::{
        watch_dog::core::WatchDog,
        running_tasks::structs::RunningTasks,
    },
};

/// Task executor for managing computation threads
#[derive(Debug)]
pub struct TaskExecutor {
    /// Semaphore to limit concurrent threads
    pub semaphore: Arc<Semaphore>,
    /// Channel for task requests
    pub task_sender: mpsc::UnboundedSender<String>,
    /// Task receiver
    pub task_receiver: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    /// Maximum concurrent threads
    pub max_concurrent_threads: usize,
    /// Active thread count
    pub active_count: Arc<AtomicU64>,
    /// Completed task count
    pub completed_count: Arc<AtomicU64>,
}

/// Permutation thread spawner for managing computation threads
#[derive(Debug)]
pub struct PermutationThreadSpawner {
    max_alive_spawner_threads: usize,
    interval_between_max_alive_check: u64,
    nbr_total_threads: Arc<AtomicU64>,
    nbr_unfinished_threads: Arc<AtomicU64>,
}

/// Progress tracker for monitoring task progress
#[derive(Debug)]
pub struct ProgressTracker {
    total_permutations: usize,
    task_id: String,
    material: String,
}

/// Main implementation of the CutList Optimizer Service
/// 
/// This struct provides the concrete implementation of task management,
/// optimization execution, and service lifecycle operations.
#[derive(Debug)]
pub struct CutListOptimizerServiceImpl {
    /// Whether multiple tasks per client are allowed
    allow_multiple_tasks_per_client: AtomicBool,
    /// Task ID counter for generating unique task IDs
    pub(crate) task_id_counter: AtomicU64,
    /// Service initialization status
    is_initialized: AtomicBool,
    /// Service shutdown status
    is_shutdown: AtomicBool,
    /// Thread coordination semaphore
    thread_semaphore: Arc<Semaphore>,
    /// Maximum threads per task
    max_threads_per_task: usize,
    /// Service start time
    start_time: DateTime<Utc>,
    /// Running tasks manager
    running_tasks: Option<Arc<RunningTasks>>,
    /// Task executor
    task_executor: Option<Arc<TaskExecutor>>,
    /// Watch dog for monitoring
    watch_dog: Option<Arc<WatchDog>>,
    /// Date format for task ID generation
    date_format: String,
}

/// Constants from Java implementation
pub const MAX_PERMUTATION_ITERATIONS: usize = 1000;
pub const MAX_STOCK_ITERATIONS: usize = 1000;
pub const MAX_ALLOWED_DIGITS: usize = 6;
pub const THREAD_QUEUE_SIZE: usize = 1000;
pub const MAX_ACTIVE_THREADS_PER_TASK: usize = 5;
pub const MAX_PERMUTATIONS_WITH_SOLUTION: usize = 150;
pub const MAX_PANELS_LIMIT: usize = 5000;
pub const MAX_STOCK_PANELS_LIMIT: usize = 5000;

impl CutListOptimizerServiceImpl {
    /// Create a new service instance
    pub fn new() -> Self {
        Self {
            allow_multiple_tasks_per_client: AtomicBool::new(false),
            task_id_counter: AtomicU64::new(0),
            is_initialized: AtomicBool::new(false),
            is_shutdown: AtomicBool::new(false),
            thread_semaphore: Arc::new(Semaphore::new(MAX_ACTIVE_THREADS_PER_TASK)),
            max_threads_per_task: MAX_ACTIVE_THREADS_PER_TASK,
            start_time: Utc::now(),
            running_tasks: None,
            task_executor: None,
            watch_dog: None,
            date_format: "%Y%m%d%H%M".to_string(),
        }
    }

    /// Generate a unique task ID (following Java pattern)
    pub(crate) fn generate_task_id(&self) -> String {
        let now = Utc::now();
        let date_part = now.format(&self.date_format).to_string();
        let counter = self.task_id_counter.fetch_add(1, Ordering::Relaxed);
        format!("{}{}", date_part, counter)
    }

    /// Check if the service is initialized
    pub(crate) fn ensure_initialized(&self) -> Result<()> {
        if !self.is_initialized.load(Ordering::Relaxed) {
            return Err(crate::errors::AppError::invalid_configuration(
                "Service not initialized"
            ));
        }
        Ok(())
    }

    /// Check if the service is not shutdown
    pub(crate) fn ensure_not_shutdown(&self) -> Result<()> {
        if self.is_shutdown.load(Ordering::Relaxed) {
            return Err(crate::errors::AppError::invalid_configuration(
                "Service is shutdown"
            ));
        }
        Ok(())
    }

    /// Get the current allow multiple tasks per client setting
    pub(crate) fn get_allow_multiple_tasks_per_client(&self) -> bool {
        self.allow_multiple_tasks_per_client.load(Ordering::Relaxed)
    }

    /// Set the allow multiple tasks per client setting
    pub(crate) fn set_allow_multiple_tasks_per_client_internal(&self, allow: bool) {
        self.allow_multiple_tasks_per_client.store(allow, Ordering::Relaxed);
    }

    /// Check if the service is initialized
    pub(crate) fn is_initialized(&self) -> bool {
        self.is_initialized.load(Ordering::Relaxed)
    }

    /// Set the initialization status
    pub(crate) fn set_initialized(&self, initialized: bool) {
        self.is_initialized.store(initialized, Ordering::Relaxed);
    }

    /// Check if the service is shutdown
    pub(crate) fn is_shutdown(&self) -> bool {
        self.is_shutdown.load(Ordering::Relaxed)
    }

    /// Set the shutdown status
    pub(crate) fn set_shutdown(&self, shutdown: bool) {
        self.is_shutdown.store(shutdown, Ordering::Relaxed);
    }

    /// Remove duplicated permutations (ported from Java)
    pub(crate) fn remove_duplicated_permutations(&self, permutations: &mut Vec<Vec<TileDimensions>>) -> usize {
        let mut hash_codes = Vec::new();
        let mut removed_count = 0;
        
        permutations.retain(|permutation| {
            let mut hash_code = 0i32;
            for tile in permutation {
                hash_code = hash_code.wrapping_mul(31).wrapping_add(tile.dimensions_hash() as i32);
            }
            
            if hash_codes.contains(&hash_code) {
                removed_count += 1;
                false
            } else {
                hash_codes.push(hash_code);
                true
            }
        });
        
        removed_count
    }

    /// Get number of decimal places in a string (delegated to DecimalPlaceCounter)
    pub fn get_nbr_decimal_places(&self, value: &str) -> usize {
        super::decimal_places::DecimalPlaceCounter::get_nbr_decimal_places(value)
    }

    /// Get number of integer places in a string (delegated to DecimalPlaceCounter)
    pub fn get_nbr_integer_places(&self, value: &str) -> usize {
        super::decimal_places::DecimalPlaceCounter::get_nbr_integer_places(value)
    }

    /// Get maximum number of decimal places from panels (delegated to DecimalPlaceCounter)
    pub fn get_max_nbr_decimal_places(&self, panels: &[Panel]) -> usize {
        super::decimal_places::DecimalPlaceCounter::get_max_nbr_decimal_places(panels)
    }

    /// Get maximum number of integer places from panels (delegated to DecimalPlaceCounter)
    pub fn get_max_nbr_integer_places(&self, panels: &[Panel]) -> usize {
        super::decimal_places::DecimalPlaceCounter::get_max_nbr_integer_places(panels)
    }

    /// Check if thread is eligible to start (ported from Java)
    pub(crate) fn is_thread_eligible_to_start(&self, _group: &str, _task: &crate::models::task::structs::Task, _material: &str) -> bool {
        // Simplified implementation - in full version would check thread group rankings
        true
    }
}

impl TaskExecutor {
    pub fn new(max_concurrent_threads: usize) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent_threads)),
            task_sender: sender,
            task_receiver: Arc::new(Mutex::new(receiver)),
            max_concurrent_threads,
            active_count: Arc::new(AtomicU64::new(0)),
            completed_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn get_active_count(&self) -> u64 {
        self.active_count.load(Ordering::Relaxed)
    }

    pub fn get_completed_task_count(&self) -> u64 {
        self.completed_count.load(Ordering::Relaxed)
    }

    pub fn get_queue_size(&self) -> usize {
        // Approximation since we can't get exact queue size from unbounded channel
        0
    }
}

impl PermutationThreadSpawner {
    pub fn new() -> Self {
        Self {
            max_alive_spawner_threads: MAX_ACTIVE_THREADS_PER_TASK,
            interval_between_max_alive_check: 1000,
            nbr_total_threads: Arc::new(AtomicU64::new(0)),
            nbr_unfinished_threads: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn set_max_alive_spawner_threads(&mut self, max: usize) {
        self.max_alive_spawner_threads = max;
    }

    pub fn set_interval_between_max_alive_check(&mut self, interval: u64) {
        self.interval_between_max_alive_check = interval;
    }

    pub fn get_nbr_total_threads(&self) -> u64 {
        self.nbr_total_threads.load(Ordering::Relaxed)
    }

    pub fn get_nbr_unfinished_threads(&self) -> u64 {
        self.nbr_unfinished_threads.load(Ordering::Relaxed)
    }

    pub async fn spawn<F>(&self, task: F) 
    where 
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.nbr_total_threads.fetch_add(1, Ordering::Relaxed);
        self.nbr_unfinished_threads.fetch_add(1, Ordering::Relaxed);
        
        let unfinished_counter = Arc::clone(&self.nbr_unfinished_threads);
        
        tokio::spawn(async move {
            task.await;
            unfinished_counter.fetch_sub(1, Ordering::Relaxed);
        });
    }
}

impl ProgressTracker {
    pub fn new(total_permutations: usize, task_id: String, material: String) -> Self {
        Self {
            total_permutations,
            task_id,
            material,
        }
    }

    pub fn refresh_task_status_info(&self) {
        // Implementation for refreshing task status
        trace!("Refreshing task status for task[{}] material[{}]", self.task_id, self.material);
    }
}

impl Default for CutListOptimizerServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
