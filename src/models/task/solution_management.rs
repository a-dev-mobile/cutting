//! Solution management for Task struct
//! 
//! This module contains methods for managing task solutions and building final results.

use std::collections::HashMap;
use tracing::{debug, info, warn};
use crate::models::{CalculationResponse, FinalTile, NoFitTile, Mosaic, Solution, TileNode};
use super::Task;

impl Task {
    // ===== Solution Management =====

    /// Check if the task has a solution
    pub fn has_solution(&self) -> bool {
        self.solution.read().unwrap().is_some()
    }

    /// Check if the solution has all panels fitting
    pub fn has_solution_all_fit(&self) -> bool {
        self.has_solution() && 
        self.solution
            .read()
            .unwrap()
            .as_ref()
            .map(|s| s.no_fit_panels.is_empty())
            .unwrap_or(false)
    }

    /// Build the final solution from all thread solutions
    /// Returns the built solution or None if no calculation request exists
    pub fn build_solution(&self) -> Option<CalculationResponse> {
        let request = self.calculation_request.as_ref()?;
        
        debug!("Building solution for task {} with {} materials", 
               self.id, self.solutions.lock().unwrap().len());
        
        // Collect all solutions from all materials
        let all_solutions = self.collect_all_solutions();
        
        if all_solutions.is_empty() {
            warn!("No solutions found for task {}", self.id);
            return self.build_empty_solution(request);
        }
        
        // Find the best solution using optimization criteria
        let best_solution = self.select_best_solution(&all_solutions);
        
        // Build the final response from the best solution
        self.build_response_from_solution(request, &best_solution)
    }

    /// Collect all solutions from all materials
    fn collect_all_solutions(&self) -> Vec<Solution> {
        let solutions_map = self.solutions.lock().unwrap();
        let mut all_solutions = Vec::new();
        
        for (material, material_solutions) in solutions_map.iter() {
            debug!("Material '{}' has {} solutions", material, material_solutions.len());
            all_solutions.extend(material_solutions.iter().cloned());
        }
        
        info!("Collected {} total solutions for task {}", all_solutions.len(), self.id);
        all_solutions
    }

    /// Select the best solution based on optimization criteria
    fn select_best_solution(&self, solutions: &[Solution]) -> Solution {
        if solutions.is_empty() {
            panic!("Cannot select best solution from empty list");
        }
        
        // Find solution with minimum waste (best area utilization)
        let best = solutions
            .iter()
            .min_by(|a, b| {
                let waste_a = self.calculate_solution_waste(a);
                let waste_b = self.calculate_solution_waste(b);
                waste_a.partial_cmp(&waste_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap();
        
        debug!("Selected best solution {} with waste area: {:.2}", 
               best.id, self.calculate_solution_waste(best));
        
        best.clone()
    }

    /// Calculate total waste area for a solution
    fn calculate_solution_waste(&self, solution: &Solution) -> f64 {
        solution.mosaics.iter()
            .map(|mosaic| {
                // Calculate waste as stock area minus used area
                let stock_area = mosaic.root_tile_node.tile.width() as f64 * mosaic.root_tile_node.tile.height() as f64;
                let used_area = self.calculate_mosaic_used_area(mosaic);
                stock_area - used_area
            })
            .sum()
    }

    /// Calculate used area in a mosaic by traversing the tile tree
    fn calculate_mosaic_used_area(&self, mosaic: &Mosaic) -> f64 {
        self.calculate_node_used_area(&mosaic.root_tile_node)
    }

    /// Recursively calculate used area in a tile node
    fn calculate_node_used_area(&self, node: &TileNode) -> f64 {
        if node.is_final {
            // This is a final tile, count its area
            node.tile.width() as f64 * node.tile.height() as f64
        } else if let (Some(child1), Some(child2)) = (&node.child1, &node.child2) {
            // This node has children, sum their used areas
            self.calculate_node_used_area(child1) + self.calculate_node_used_area(child2)
        } else {
            // Empty node
            0.0
        }
    }

    /// Extract final tiles from a mosaic by traversing the tile tree
    fn extract_final_tiles(&self, mosaic: &Mosaic) -> Vec<FinalTile> {
        let mut tiles = Vec::new();
        self.extract_tiles_from_node(&mosaic.root_tile_node, &mut tiles);
        tiles
    }

    /// Recursively extract final tiles from a tile node
    fn extract_tiles_from_node(&self, node: &TileNode, tiles: &mut Vec<FinalTile>) {
        if node.is_final {
            // This is a final tile
            let final_tile = FinalTile {
                request_obj_id: node.external_id.unwrap_or(node.id as i32),
                width: node.tile.width() as f64,
                height: node.tile.height() as f64,
                label: Some(format!("tile_{}", node.id)), // Generate label since Tile doesn't have one
                count: 1, // Each node represents one tile
            };
            tiles.push(final_tile);
        } else if let (Some(child1), Some(child2)) = (&node.child1, &node.child2) {
            // This node has children, recurse into them
            self.extract_tiles_from_node(child1, tiles);
            self.extract_tiles_from_node(child2, tiles);
        }
    }

    /// Build response from the selected best solution
    fn build_response_from_solution(&self, request: &crate::models::CalculationRequest, solution: &Solution) -> Option<CalculationResponse> {
        let elapsed_time = self.elapsed_time();
        
        // Extract final tiles from all mosaics
        let mut panels = Vec::new();
        let mut total_cut_length = 0.0;
        let mut total_cuts = 0u64;
        let mut total_used_area = 0.0;
        let mut total_stock_area = 0.0;
        
        for mosaic in &solution.mosaics {
            // Extract tiles from this mosaic
            let mosaic_tiles = self.extract_final_tiles(mosaic);
            panels.extend(mosaic_tiles);
            
            // Calculate statistics
            total_cuts += mosaic.cuts.len() as u64;
            total_cut_length += self.estimate_cut_length(mosaic);
            total_used_area += self.calculate_mosaic_used_area(mosaic);
            total_stock_area += mosaic.root_tile_node.tile.width() as f64 * mosaic.root_tile_node.tile.height() as f64;
        }
        
        // Convert no-fit panels
        let no_fit_panels: Vec<NoFitTile> = solution.no_fit_panels.iter()
            .chain(self.no_material_tiles.iter())
            .map(|tile| NoFitTile {
                id: tile.id,
                width: tile.width as f64,
                height: tile.height as f64,
                count: 1,
                label: tile.label.clone(),
                material: Some(tile.material.clone()),
            })
            .collect();
        
        // Calculate ratios and waste
        let total_used_area_ratio = if total_stock_area > 0.0 {
            total_used_area / total_stock_area
        } else {
            0.0
        };
        let total_wasted_area = total_stock_area - total_used_area;
        
        info!("Built solution for task {}: {} panels, {:.1}% efficiency, {} no-fit panels", 
              self.id, panels.len(), total_used_area_ratio * 100.0, no_fit_panels.len());
        
        Some(CalculationResponse {
            version: "1.0.0".to_string(),
            edge_bands: self.calculate_edge_bands(&solution.mosaics),
            elapsed_time,
            id: Some(self.id.clone()),
            panels: Some(panels),
            request: Some(request.clone()),
            solution_elapsed_time: Some(elapsed_time),
            task_id: Some(self.id.clone()),
            total_cut_length,
            total_nbr_cuts: total_cuts,
            total_used_area,
            total_used_area_ratio,
            total_wasted_area,
            used_stock_panels: None, // Could be populated if needed
            no_fit_panels,
            mosaics: solution.mosaics.clone(),
        })
    }

    /// Build an empty solution when no solutions are available
    fn build_empty_solution(&self, request: &crate::models::CalculationRequest) -> Option<CalculationResponse> {
        let elapsed_time = self.elapsed_time();
        
        // All panels become no-fit panels
        let no_fit_panels: Vec<NoFitTile> = self.no_material_tiles.iter()
            .map(|tile| NoFitTile {
                id: tile.id,
                width: tile.width as f64,
                height: tile.height as f64,
                count: 1,
                label: tile.label.clone(),
                material: Some(tile.material.clone()),
            })
            .collect();
        
        warn!("Built empty solution for task {} with {} no-fit panels", 
              self.id, no_fit_panels.len());
        
        Some(CalculationResponse {
            version: "1.0.0".to_string(),
            edge_bands: None,
            elapsed_time,
            id: Some(self.id.clone()),
            panels: Some(Vec::new()),
            request: Some(request.clone()),
            solution_elapsed_time: Some(elapsed_time),
            task_id: Some(self.id.clone()),
            total_cut_length: 0.0,
            total_nbr_cuts: 0,
            total_used_area: 0.0,
            total_used_area_ratio: 0.0,
            total_wasted_area: 0.0,
            used_stock_panels: None,
            no_fit_panels,
            mosaics: Vec::new(),
        })
    }

    /// Estimate cut length for a mosaic based on the number of cuts
    fn estimate_cut_length(&self, mosaic: &Mosaic) -> f64 {
        // Simplified: assume each cut has an average length based on stock dimensions
        let avg_dimension = (mosaic.root_tile_node.tile.width() + mosaic.root_tile_node.tile.height()) as f64 / 2.0;
        mosaic.cuts.len() as f64 * avg_dimension * 0.5 // Rough estimate
    }

    /// Calculate edge band usage by material (simplified implementation)
    fn calculate_edge_bands(&self, mosaics: &[Mosaic]) -> Option<HashMap<String, f64>> {
        let mut edge_bands = HashMap::new();
        
        for mosaic in mosaics {
            // Simplified: calculate perimeter of all final tiles
            let final_tiles = self.extract_final_tiles(mosaic);
            let total_perimeter: f64 = final_tiles.iter()
                .map(|tile| 2.0 * (tile.width + tile.height))
                .sum();
            
            *edge_bands.entry(mosaic.material.clone()).or_insert(0.0) += total_perimeter;
        }
        
        if edge_bands.is_empty() {
            None
        } else {
            Some(edge_bands)
        }
    }

    /// Build and set the solution for this task
    pub fn build_and_set_solution(&self) {
        if let Some(solution) = self.build_solution() {
            *self.solution.write().unwrap() = Some(solution);
            info!("Solution built and set for task {}", self.id);
        } else {
            warn!("Failed to build solution for task {}", self.id);
        }
    }

    /// Add a solution for a specific material
    pub fn add_solution(&self, material: &str, solution: Solution) {
        let solution_id = solution.id.clone();
        let mut solutions = self.solutions.lock().unwrap();
        solutions.entry(material.to_string())
            .or_insert_with(Vec::new)
            .push(solution);
        
        debug!("Added solution {} for material '{}' in task {}", 
               solution_id, material, self.id);
    }

    /// Get the number of solutions for a specific material
    pub fn solution_count(&self, material: &str) -> usize {
        self.solutions.lock().unwrap()
            .get(material)
            .map(|sols| sols.len())
            .unwrap_or(0)
    }

    /// Get total number of solutions across all materials
    pub fn total_solution_count(&self) -> usize {
        self.solutions.lock().unwrap()
            .values()
            .map(|sols| sols.len())
            .sum()
    }
}
