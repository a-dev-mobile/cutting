//! Core service implementation
//! This module contains only the main service struct and basic utilities

use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
use tokio::sync::Semaphore;
use chrono::{DateTime, Utc};

use crate::{
    errors::Result,
    engine::{
        watch_dog::core::WatchDog,
        running_tasks::structs::RunningTasks,
    },
    constants::EngineConstants,
};

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
    /// Watch dog for monitoring
    watch_dog: Option<Arc<WatchDog>>,
    /// Date format for task ID generation
    date_format: String,
}

impl CutListOptimizerServiceImpl {
    /// Create a new service instance
    pub fn new() -> Self {
        Self {
            allow_multiple_tasks_per_client: AtomicBool::new(false),
            task_id_counter: AtomicU64::new(0),
            is_initialized: AtomicBool::new(false),
            is_shutdown: AtomicBool::new(false),
            thread_semaphore: Arc::new(Semaphore::new(EngineConstants::MAX_ACTIVE_THREADS_PER_TASK)),
            max_threads_per_task: EngineConstants::MAX_ACTIVE_THREADS_PER_TASK,
            start_time: Utc::now(),
            running_tasks: None,
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
    pub fn is_initialized(&self) -> bool {
        self.is_initialized.load(Ordering::Relaxed)
    }

    /// Set the initialization status
    pub(crate) fn set_initialized(&self, initialized: bool) {
        self.is_initialized.store(initialized, Ordering::Relaxed);
    }

    /// Check if the service is shutdown
    pub fn is_shutdown(&self) -> bool {
        self.is_shutdown.load(Ordering::Relaxed)
    }

    /// Set the shutdown status
    pub(crate) fn set_shutdown(&self, shutdown: bool) {
        self.is_shutdown.store(shutdown, Ordering::Relaxed);
    }

    /// Check if thread is eligible to start (ported from Java)
    pub(crate) fn is_thread_eligible_to_start(&self, _group: &str, _task: &crate::models::task::structs::Task, _material: &str) -> bool {
        // Simplified implementation - in full version would check thread group rankings
        true
    }
}

impl Default for CutListOptimizerServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
