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
    
    /// Add a single panel that couldn't be fit
    pub fn add_no_fit_panel(&mut self, panel: TileDimensions) {
        self.no_fit_panels.push(panel);
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
    
    /// Get stock tile dimensions from all mosaics
    pub fn get_stock_tiles_dimensions(&self) -> Vec<TileDimensions> {
        self.mosaics
            .iter()
            .map(|m| m.to_tile_dimensions())
            .collect()
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}
