use super::structs::{Solution, ID_COUNTER};
use crate::models::{Mosaic, TileDimensions, TileNode, Tile};
use crate::engine::stock::StockSolution;
use std::sync::atomic::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

impl Solution {
    /// Create a new empty solution
    pub fn new() -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            mosaics: Vec::new(),
            no_fit_panels: Vec::new(),
            unused_stock_panels: std::collections::VecDeque::new(),
            aux_info: None,
            creator_thread_group: None,
        }
    }
    
    /// Create a new solution by copying from another solution
    pub fn from_solution(other: &Solution) -> Self {
        let mut new_solution = Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            mosaics: Vec::new(),
            no_fit_panels: other.no_fit_panels.clone(),
            unused_stock_panels: other.unused_stock_panels.clone(),
            aux_info: other.aux_info.clone(),
            creator_thread_group: other.creator_thread_group.clone(),
        };
        
        // Deep copy mosaics
        for mosaic in &other.mosaics {
            new_solution.mosaics.push(Mosaic::from_mosaic(mosaic));
        }
        
        new_solution.sort_mosaics();
        new_solution
    }
    
    /// Create a new solution with a single mosaic from tile dimensions
    pub fn from_tile_dimensions(tile_dimensions: &TileDimensions) -> Self {
        let mut solution = Self::new();
        solution.add_mosaic(Mosaic::from_tile_dimensions(tile_dimensions));
        solution
    }
    
    /// Create a new solution from a stock solution
    pub fn from_stock_solution(stock_solution: &StockSolution) -> Self {
        let mut solution = Self::new();
        
        // Add all stock tile dimensions to unused stock panels
        for tile_dim in stock_solution.get_stock_tile_dimensions() {
            solution.unused_stock_panels.push_back(tile_dim.clone());
        }
        
        // Create first mosaic from the first unused stock panel
        if let Some(first_panel) = solution.unused_stock_panels.pop_front() {
            solution.add_mosaic(Mosaic::from_tile_dimensions(&first_panel));
        }
        
        solution
    }
    
    /// Create a new solution by copying another solution but excluding a specific mosaic
    pub fn from_solution_excluding_mosaic(solution: &Solution, excluded_mosaic: &Mosaic) -> Self {
        let mut new_solution = Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            mosaics: Vec::new(),
            no_fit_panels: solution.no_fit_panels.clone(),
            unused_stock_panels: solution.unused_stock_panels.clone(),
            aux_info: solution.aux_info.clone(),
            creator_thread_group: solution.creator_thread_group.clone(),
        };
        
        // Copy all mosaics except the excluded one
        for mosaic in &solution.mosaics {
            if mosaic != excluded_mosaic {
                new_solution.mosaics.push(Mosaic::from_mosaic(mosaic));
            }
        }
        
        new_solution
    }
    
    /// Sort mosaics by unused area (ascending order)
    fn sort_mosaics(&mut self) {
        self.mosaics.sort_by(|a, b| {
            let mut a_clone = a.clone();
            let mut b_clone = b.clone();
            a_clone.unused_area().cmp(&b_clone.unused_area())
        });
    }
    
    /// Add a mosaic to the solution and maintain sorted order
    pub fn add_mosaic(&mut self, mosaic: Mosaic) {
        self.mosaics.push(mosaic);
        self.sort_mosaics();
    }
    
    /// Add multiple mosaics to the solution and maintain sorted order
    pub fn add_all_mosaics(&mut self, mosaics: Vec<Mosaic>) {
        self.mosaics.extend(mosaics);
        self.sort_mosaics();
    }
    
    /// Remove a mosaic from the solution
    pub fn remove_mosaic(&mut self, mosaic: &Mosaic) {
        self.mosaics.retain(|m| m != mosaic);
    }
    
    /// Add panels that couldn't be fit
    pub fn add_all_no_fit_panels(&mut self, panels: Vec<TileDimensions>) {
        self.no_fit_panels.extend(panels);
    }
    
    /// Get the used area ratio across all mosaics
    pub fn get_used_area_ratio(&self) -> f32 {
        if self.mosaics.is_empty() {
            return 0.0;
        }
        
        let mut total_ratio = 0.0f32;
        for mosaic in &self.mosaics {
            let mut mosaic_clone = mosaic.clone();
            total_ratio += mosaic_clone.efficiency();
        }
            
        total_ratio / self.mosaics.len() as f32
    }
    
    /// Check if there's an unused base tile
    pub fn has_unused_base_tile(&self) -> bool {
        self.mosaics
            .first()
            .map(|m| !m.root_tile_node().has_final())
            .unwrap_or(false)
    }
    
    /// Get the number of unused tiles across all mosaics
    pub fn get_nbr_unused_tiles(&self) -> i32 {
        self.mosaics
            .iter()
            .map(|m| m.unused_tile_count() as i32)
            .sum()
    }
    
    /// Get a string representation of all base dimensions
    pub fn get_bases_as_string(&self) -> String {
        self.mosaics
            .iter()
            .map(|m| {
                format!("[{}x{}]", m.width(), m.height())
            })
            .collect::<Vec<_>>()
            .join("")
    }
    
    /// Get the number of horizontal cuts across all mosaics
    pub fn get_nbr_horizontal(&self) -> i32 {
        self.mosaics
            .iter()
            .map(|m| m.root_tile_node().count_final_horizontal() as i32)
            .sum()
    }
    
    /// Get all final tile nodes from all mosaics
    pub fn get_final_tile_nodes(&self) -> Vec<TileNode> {
        self.mosaics
            .iter()
            .flat_map(|m| m.final_tile_nodes().into_iter().cloned())
            .collect()
    }
    
    /// Get all final tiles from all mosaics
    pub fn get_final_tiles(&self) -> Vec<Tile> {
        self.mosaics
            .iter()
            .flat_map(|m| m.root_tile_node().final_tiles().into_iter().cloned())
            .collect()
    }
    
    /// Get the number of final tiles across all mosaics
    pub fn get_nbr_final_tiles(&self) -> i32 {
        self.mosaics
            .iter()
            .map(|m| m.final_tile_count() as i32)
            .sum()
    }
    
    /// Get the horizontal/vertical difference across all mosaics
    pub fn get_hv_diff(&self) -> f32 {
        if self.mosaics.is_empty() {
            return 0.0;
        }
        
        let total_diff: f32 = self.mosaics
            .iter()
            .map(|m| m.hv_diff())
            .sum();
            
        total_diff / self.mosaics.len() as f32
    }
    
    /// Get the total area across all mosaics
    pub fn get_total_area(&self) -> i64 {
        self.mosaics
            .iter()
            .map(|m| m.total_area())
            .sum()
    }
    
    /// Get the used area across all mosaics
    pub fn get_used_area(&self) -> i64 {
        let mut total_used = 0i64;
        for mosaic in &self.mosaics {
            let mut mosaic_clone = mosaic.clone();
            total_used += mosaic_clone.used_area();
        }
        total_used
    }
    
    /// Get the unused area across all mosaics
    pub fn get_unused_area(&self) -> i64 {
        let mut total_unused = 0i64;
        for mosaic in &self.mosaics {
            let mut mosaic_clone = mosaic.clone();
            total_unused += mosaic_clone.unused_area();
        }
        total_unused
    }
    
    /// Get the maximum depth across all mosaics
    pub fn get_max_depth(&self) -> i32 {
        self.mosaics
            .iter()
            .map(|m| m.depth() as i32)
            .max()
            .unwrap_or(0)
    }
    
    /// Get the number of cuts across all mosaics
    pub fn get_nbr_cuts(&self) -> i32 {
        self.mosaics
            .iter()
            .map(|m| m.nbr_cuts() as i32)
            .sum()
    }
    
    /// Get the distinct tile set size (maximum across all mosaics)
    pub fn get_distinct_tile_set(&self) -> usize {
        self.mosaics
            .iter()
            .map(|m| m.distinct_tile_set().len())
            .max()
            .unwrap_or(0)
    }
    
    /// Get the number of mosaics
    pub fn get_nbr_mosaics(&self) -> usize {
        self.mosaics.len()
    }
    
    /// Get stock tile dimensions from all mosaics
    pub fn get_stock_tiles_dimensions(&self) -> Vec<TileDimensions> {
        self.mosaics
            .iter()
            .map(|m| m.to_tile_dimensions())
            .collect()
    }
    
    /// Get the area of the mosaic with the most unused area
    pub fn get_most_unused_panel_area(&self) -> i64 {
        let mut max_unused = 0i64;
        for mosaic in &self.mosaics {
            let mut mosaic_clone = mosaic.clone();
            let unused = mosaic_clone.unused_area();
            if unused > max_unused {
                max_unused = unused;
            }
        }
        max_unused
    }
    
    /// Get the average center of mass distance to origin
    pub fn get_center_of_mass_distance_to_origin(&self) -> f32 {
        if self.mosaics.is_empty() {
            return 0.0;
        }
        
        let total_distance: f32 = self.mosaics
            .iter()
            .map(|m| m.center_of_mass_distance_to_origin())
            .sum();
            
        total_distance / self.get_nbr_mosaics() as f32
    }
    
    /// Get the biggest area across all mosaics
    pub fn get_biggest_area(&self) -> i64 {
        self.mosaics
            .iter()
            .map(|m| m.biggest_area())
            .max()
            .unwrap_or(0)
    }
    
    /// Get the material from the first mosaic (if any)
    pub fn get_material(&self) -> Option<&str> {
        self.mosaics
            .first()
            .map(|m| m.material())
    }

    /// Get a reference to the unused stock panels
    pub fn get_unused_stock_panels(&self) -> &std::collections::VecDeque<TileDimensions> {
        &self.unused_stock_panels
    }
    
    /// Get a mutable reference to the unused stock panels
    pub fn get_unused_stock_panels_mut(&mut self) -> &mut std::collections::VecDeque<TileDimensions> {
        &mut self.unused_stock_panels
    }
    
    /// Get the creator thread group
    pub fn get_creator_thread_group(&self) -> Option<&str> {
        self.creator_thread_group.as_deref()
    }
    
    /// Set the creator thread group
    pub fn set_creator_thread_group(&mut self, thread_group: Option<String>) {
        self.creator_thread_group = thread_group;
    }
    
    /// Get the auxiliary information
    pub fn get_aux_info(&self) -> Option<&str> {
        self.aux_info.as_deref()
    }
    
    /// Set the auxiliary information
    pub fn set_aux_info(&mut self, aux_info: Option<String>) {
        self.aux_info = aux_info;
    }
    
    /// Get the solution ID
    pub fn get_id(&self) -> u32 {
        self.id
    }
    
    /// Get a reference to the mosaics
    pub fn get_mosaics(&self) -> &Vec<Mosaic> {
        &self.mosaics
    }
    
    /// Get a mutable reference to the mosaics
    pub fn get_mosaics_mut(&mut self) -> &mut Vec<Mosaic> {
        &mut self.mosaics
    }
    
    /// Get a reference to the no-fit panels
    pub fn get_no_fit_panels(&self) -> &Vec<TileDimensions> {
        &self.no_fit_panels
    }
    
    /// Set the no-fit panels
    pub fn set_no_fit_panels(&mut self, panels: Vec<TileDimensions>) {
        self.no_fit_panels = panels;
    }
    
    /// Get the timestamp
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

// Additional helper implementations that might be needed for the cutting optimization
impl Solution {
    /// Check if the solution is empty (no mosaics)
    pub fn is_empty(&self) -> bool {
        self.mosaics.is_empty()
    }
    
    /// Get the total number of panels (mosaics + no-fit + unused stock)
    pub fn get_total_panel_count(&self) -> usize {
        self.mosaics.len() + self.no_fit_panels.len() + self.unused_stock_panels.len()
    }
    
    /// Calculate efficiency as used area / total area
    pub fn get_efficiency(&self) -> f32 {
        let total_area = self.get_total_area();
        if total_area == 0 {
            return 0.0;
        }
        
        let used_area = self.get_used_area();
        used_area as f32 / total_area as f32
    }
    
    /// Get waste percentage
    pub fn get_waste_percentage(&self) -> f32 {
        (1.0 - self.get_efficiency()) * 100.0
    }
    
    /// Check if all panels have been processed (no unused stock panels)
    pub fn is_complete(&self) -> bool {
        self.unused_stock_panels.is_empty()
    }
    
    /// Get a summary string of the solution
    pub fn get_summary(&self) -> String {
        format!(
            "Solution #{}: {} mosaics, {} no-fit panels, {} unused stock panels, {:.2}% efficiency",
            self.id,
            self.mosaics.len(),
            self.no_fit_panels.len(),
            self.unused_stock_panels.len(),
            self.get_efficiency() * 100.0
        )
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}
