//! Material management for Task struct
//! 
//! This module contains methods for managing materials, their progress, and solutions.

use std::collections::HashMap;
use crate::models::Solution;
use super::Task;

impl Task {
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
}
