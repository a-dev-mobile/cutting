//! Task management operations
//!
//! This module provides functionality for adding, removing, and retrieving tasks
//! from the running tasks collection.

use crate::{
    errors::Result,
    models::{
        task::Task,
        enums::Status,
    },
};
use parking_lot::RwLock;
use std::sync::Arc;
use crate::logging::{debug, warn};

use super::structs::RunningTasks;

/// Trait for task management operations
pub trait TaskManager {
    /// Add a new task to the running tasks collection
    fn add_task(&self, task: Task) -> Result<()>;
    
    /// Remove a task from the running tasks collection
    fn remove_task(&self, task_id: &str) -> Result<Option<Arc<RwLock<Task>>>>;
    
    /// Get a task by ID
    fn get_task(&self, task_id: &str) -> Option<Arc<RwLock<Task>>>;
    
    /// Get all tasks as a vector
    fn get_tasks(&self) -> Vec<Arc<RwLock<Task>>>;
    
    /// Get tasks with given status
    fn get_tasks_with_status(&self, status: Status) -> Vec<String>;
}

impl TaskManager for RunningTasks {
    /// Add a new task to the running tasks collection
    fn add_task(&self, task: Task) -> Result<()> {
        let task_id = task.id.clone();
        let status = *task.status.read().unwrap();
        
        debug!("Adding task {} with status {:?}", task_id, status);
        
        // Insert task into collection
        let task_arc = Arc::new(RwLock::new(task));
        if self.tasks.insert(task_id.clone(), task_arc).is_some() {
            warn!("Task {} was already present, replacing", task_id);
        }
        
        // Update counters based on initial status
        self.increment_status_counter(status);
        
        Ok(())
    }
    
    /// Remove a task from the running tasks collection
    fn remove_task(&self, task_id: &str) -> Result<Option<Arc<RwLock<Task>>>> {
        debug!("Removing task {}", task_id);
        
        if let Some((_, task_arc)) = self.tasks.remove(task_id) {
            let status = *task_arc.read().status.read().unwrap();
            self.decrement_status_counter(status);
            Ok(Some(task_arc))
        } else {
            debug!("Task {} not found for removal", task_id);
            Ok(None)
        }
    }
    
    /// Get a task by ID
    fn get_task(&self, task_id: &str) -> Option<Arc<RwLock<Task>>> {
        self.tasks.get(task_id).map(|entry| entry.value().clone())
    }
    
    /// Get all tasks as a vector
    fn get_tasks(&self) -> Vec<Arc<RwLock<Task>>> {
        self.tasks.iter().map(|entry| entry.value().clone()).collect()
    }
    
    /// Get tasks with given status
    fn get_tasks_with_status(&self, status: Status) -> Vec<String> {
        self.tasks
            .iter()
            .filter_map(|entry| {
                let task = entry.value().read();
                let task_status = *task.status.read().unwrap();
                
                if task_status == status {
                    Some(entry.key().clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl RunningTasks {
    /// Helper method to increment status counter
    pub(crate) fn increment_status_counter(&self, status: Status) {
        use std::sync::atomic::Ordering;
        
        match status {
            Status::Queued => self.nbr_idle_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Running => self.nbr_running_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Finished => self.nbr_finished_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Stopped => self.nbr_stopped_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Terminated => self.nbr_terminated_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Error => self.nbr_error_tasks.fetch_add(1, Ordering::Relaxed),
        };
    }
    
    /// Helper method to decrement status counter
    pub(crate) fn decrement_status_counter(&self, status: Status) {
        use std::sync::atomic::Ordering;
        
        match status {
            Status::Queued => self.nbr_idle_tasks.fetch_sub(1, Ordering::Relaxed),
            Status::Running => self.nbr_running_tasks.fetch_sub(1, Ordering::Relaxed),
            Status::Finished => self.nbr_finished_tasks.fetch_sub(1, Ordering::Relaxed),
            Status::Stopped => self.nbr_stopped_tasks.fetch_sub(1, Ordering::Relaxed),
            Status::Terminated => self.nbr_terminated_tasks.fetch_sub(1, Ordering::Relaxed),
            Status::Error => self.nbr_error_tasks.fetch_sub(1, Ordering::Relaxed),
        };
    }
}
