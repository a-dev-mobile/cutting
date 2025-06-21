use super::structs::Solution;
use crate::models::{Mosaic, TileDimensions};
use crate::engine::stock::StockSolution;

impl Solution {
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
