//! Core service implementation structure
//!
//! This module contains the main service struct and its core functionality,
//! including initialization, configuration, and basic state management.

use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
};

use uuid::Uuid;

use crate::errors::Result;

/// Main implementation of the CutList Optimizer Service
/// 
/// This struct provides the concrete implementation of task management,
/// optimization execution, and service lifecycle operations.
#[derive(Debug)]
pub struct CutListOptimizerServiceImpl {
    /// Whether multiple tasks per client are allowed
    allow_multiple_tasks_per_client: AtomicBool,
    /// Task ID counter for generating unique task IDs
    task_id_counter: AtomicU64,
    /// Service initialization status
    is_initialized: AtomicBool,
    /// Service shutdown status
    is_shutdown: AtomicBool,
}

impl CutListOptimizerServiceImpl {
    /// Create a new service instance
    pub fn new() -> Self {
        Self {
            allow_multiple_tasks_per_client: AtomicBool::new(false),
            task_id_counter: AtomicU64::new(0),
            is_initialized: AtomicBool::new(false),
            is_shutdown: AtomicBool::new(false),
        }
    }

    /// Generate a unique task ID
    pub(crate) fn generate_task_id(&self) -> String {
        let counter = self.task_id_counter.fetch_add(1, Ordering::Relaxed);
        format!("task-{}-{}", Uuid::new_v4().simple(), counter)
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
}

impl Default for CutListOptimizerServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
