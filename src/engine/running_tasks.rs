//! Running tasks management system
//!
//! This module provides thread-safe management of active optimization tasks,
//! including task storage, status tracking, and statistics collection.

use crate::{
    errors::Result,
    models::{
        task::Task,
        enums::Status,
        stats::{Stats, TaskReport},
    },
};
use dashmap::DashMap;
use parking_lot::RwLock;
use std::{
    sync::{
        atomic::{AtomicI32, AtomicU64, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{debug, warn};

/// Thread-safe container for managing running optimization tasks
/// 
/// This structure provides concurrent access to task storage and maintains
/// real-time statistics about task execution states.
#[derive(Debug)]
pub struct RunningTasks {
    /// Map of task_id -> Task for fast lookup
    tasks: DashMap<String, Arc<RwLock<Task>>>,
    
    /// Atomic counters for different task states
    nbr_idle_tasks: AtomicI32,
    nbr_running_tasks: AtomicI32,
    nbr_finished_tasks: AtomicI32,
    nbr_stopped_tasks: AtomicI32,
    nbr_terminated_tasks: AtomicI32,
    nbr_error_tasks: AtomicI32,
    
    /// Thread execution statistics
    nbr_finished_threads: AtomicU64,
    
    /// Service start time for uptime calculation
    start_time: SystemTime,
}

impl RunningTasks {
    /// Create a new RunningTasks instance
    pub fn new() -> Self {
        Self {
            tasks: DashMap::new(),
            nbr_idle_tasks: AtomicI32::new(0),
            nbr_running_tasks: AtomicI32::new(0),
            nbr_finished_tasks: AtomicI32::new(0),
            nbr_stopped_tasks: AtomicI32::new(0),
            nbr_terminated_tasks: AtomicI32::new(0),
            nbr_error_tasks: AtomicI32::new(0),
            nbr_finished_threads: AtomicU64::new(0),
            start_time: SystemTime::now(),
        }
    }
    
    /// Get singleton instance (thread-safe lazy initialization)
    pub fn get_instance() -> &'static Arc<Self> {
        use std::sync::OnceLock;
        static INSTANCE: OnceLock<Arc<RunningTasks>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::new()))
    }
    
    /// Add a new task to the running tasks collection
    pub fn add_task(&self, task: Task) -> Result<()> {
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
    pub fn remove_task(&self, task_id: &str) -> Result<Option<Arc<RwLock<Task>>>> {
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
    pub fn get_task(&self, task_id: &str) -> Option<Arc<RwLock<Task>>> {
        self.tasks.get(task_id).map(|entry| entry.value().clone())
    }
    
    /// Get all tasks as a vector
    pub fn get_tasks(&self) -> Vec<Arc<RwLock<Task>>> {
        self.tasks.iter().map(|entry| entry.value().clone()).collect()
    }
    
    /// Update task status and adjust counters accordingly
    pub fn update_task_status(&self, task_id: &str, old_status: Status, new_status: Status) -> Result<()> {
        if old_status != new_status {
            debug!("Updating task {} status from {:?} to {:?}", task_id, old_status, new_status);
            
            self.decrement_status_counter(old_status);
            self.increment_status_counter(new_status);
        }
        
        Ok(())
    }
    
    /// Get current statistics
    pub fn get_stats(&self) -> Stats {
        let mut task_reports = Vec::new();
        
        // Collect task reports
        for entry in self.tasks.iter() {
            let task_id = entry.key().clone();
            let task = entry.value().read();
            
            let status = *task.status.read().unwrap();
            let start_time = task.start_time;
            let running_time_ms = start_time
                .elapsed()
                .unwrap_or_default()
                .as_millis() as i64;
            
            let client_id = "unknown".to_string(); // Client info removed as requested
            
            let progress_percentage = task.percentage_done();
            
            let report = TaskReport {
                task_id: task_id.clone(),
                client_id,
                status: format!("{:?}", status),
                progress_percentage,
                start_time: start_time
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64,
                duration_ms: Some(running_time_ms),
                error_message: None,
            };
            
            task_reports.push(report);
        }
        
        Stats {
            nbr_idle_tasks: self.nbr_idle_tasks.load(Ordering::Relaxed),
            nbr_running_tasks: self.nbr_running_tasks.load(Ordering::Relaxed),
            nbr_finished_tasks: self.nbr_finished_tasks.load(Ordering::Relaxed),
            nbr_stopped_tasks: self.nbr_stopped_tasks.load(Ordering::Relaxed),
            nbr_terminated_tasks: self.nbr_terminated_tasks.load(Ordering::Relaxed),
            nbr_error_tasks: self.nbr_error_tasks.load(Ordering::Relaxed),
            nbr_running_threads: 0, // Will be set by service implementation
            nbr_queued_threads: 0,  // Will be set by service implementation
            nbr_finished_threads: self.nbr_finished_threads.load(Ordering::Relaxed) as i64,
            task_reports,
        }
    }
    
    /// Get tasks with given status (client info functionality removed)
    pub fn get_tasks_with_status(&self, status: Status) -> Vec<String> {
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
    
    /// Clean up old finished tasks (older than specified duration)
    pub fn cleanup_old_tasks(&self, max_age_seconds: u64) -> Result<usize> {
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .saturating_sub(max_age_seconds);
        
        let mut removed_count = 0;
        let mut tasks_to_remove = Vec::new();
        
        // Collect tasks to remove
        for entry in self.tasks.iter() {
            let task = entry.value().read();
            let status = *task.status.read().unwrap();
            
            // Only remove finished, terminated, or error tasks
            if matches!(status, Status::Finished | Status::Terminated | Status::Error) {
                let task_time = task.start_time
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                if task_time < cutoff_time {
                    tasks_to_remove.push(entry.key().clone());
                }
            }
        }
        
        // Remove old tasks
        for task_id in tasks_to_remove {
            if self.remove_task(&task_id)?.is_some() {
                removed_count += 1;
                debug!("Cleaned up old task: {}", task_id);
            }
        }
        
        if removed_count > 0 {
            debug!("Cleaned up {} old tasks", removed_count);
        }
        
        Ok(removed_count)
    }
    
    /// Get service uptime in seconds
    pub fn get_uptime_seconds(&self) -> u64 {
        self.start_time
            .elapsed()
            .unwrap_or_default()
            .as_secs()
    }
    
    /// Increment finished thread counter
    pub fn increment_finished_threads(&self) {
        self.nbr_finished_threads.fetch_add(1, Ordering::Relaxed);
    }
    
    // Private helper methods
    
    fn increment_status_counter(&self, status: Status) {
        match status {
            Status::Queued => self.nbr_idle_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Running => self.nbr_running_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Finished => self.nbr_finished_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Terminated => self.nbr_terminated_tasks.fetch_add(1, Ordering::Relaxed),
            Status::Error => self.nbr_error_tasks.fetch_add(1, Ordering::Relaxed),
        };
    }
    
    fn decrement_status_counter(&self, status: Status) {
        match status {
            Status::Queued => self.nbr_idle_tasks.fetch_sub(1, Ordering::Relaxed),
            Status::Running => self.nbr_running_tasks.fetch_sub(1, Ordering::Relaxed),
            Status::Finished => self.nbr_finished_tasks.fetch_sub(1, Ordering::Relaxed),
            Status::Terminated => self.nbr_terminated_tasks.fetch_sub(1, Ordering::Relaxed),
            Status::Error => self.nbr_error_tasks.fetch_sub(1, Ordering::Relaxed),
        };
    }
}

impl Default for RunningTasks {
    fn default() -> Self {
        Self::new()
    }
}
