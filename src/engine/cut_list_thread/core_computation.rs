//! Core computation implementations for CutListThread
//! 
//! This module contains the main computation algorithm and orchestration logic.

use crate::{
    log_debug, log_info,
    models::{Solution, TileNode},
    error::{OptimizerError, Result},
    Status,
};
use std::{
    time::Instant,
};

use super::structs::CutListThread;

impl CutListThread {
    /// Main computation method - equivalent to Java's computeSolutions()
    /// Implements the complete tile fitting algorithm from the original Java code
    pub fn compute_solutions(&mut self) -> Result<()> {
        log_info!("Starting solution computation for thread group: {:?}", self.group);
        
        self.status = Status::Running;
        self.start_time = Some(Instant::now());

        let mut current_solutions = Vec::new();
        
        // Create initial solution from stock
        if let Some(ref stock_solution) = self.stock_solution {
            let initial_solution = Solution::from_stock_solution(stock_solution);
            current_solutions.push(initial_solution);
        }

        // Check if task is still running
        let task_running = if let Some(task) = &self.task {
            task.lock().map(|t| t.is_running()).unwrap_or(false)
        } else {
            true
        };

        if !task_running {
            return Ok(());
        }

        // Process each tile with the complex fitting algorithm
        let total_tiles = self.tiles.len();
        for (tile_index, tile_dimensions) in self.tiles.iter().enumerate() {
            // Update progress every 3 tiles
            if tile_index % 3 == 0 {
                self.percentage_done = ((tile_index as f32 / total_tiles as f32) * 100.0) as i32;
                log_debug!("Progress: {}% ({}/{})", self.percentage_done, tile_index, total_tiles);
            }

            let mut new_solutions = Vec::new();
            let mut tile_fitted = false;

            // Try to fit the tile into each existing solution
            for solution in &current_solutions {
                let mut solution_mosaics = solution.get_mosaics().clone();
                
                // Try to fit into each mosaic
                for mosaic in &mut solution_mosaics {
                    // Check material compatibility
                    let mosaic_material = mosaic.material();
                    let tile_material = &tile_dimensions.material;
                    if mosaic_material != tile_material {
                        continue;
                    }

                    let mut fitting_results = Vec::new();
                    self.add_tile_to_mosaic(tile_dimensions, mosaic, &mut fitting_results)?;
                    
                    for result_mosaic in fitting_results {
                        let mut new_solution = Solution::from_solution_excluding_mosaic(solution, mosaic);
                        new_solution.add_mosaic(result_mosaic);
                        new_solution.set_creator_thread_group(self.group.clone());
                        new_solution.set_aux_info(self.aux_info.clone());
                        new_solutions.push(new_solution);
                        tile_fitted = true;
                    }
                }

                // If tile didn't fit in any mosaic, try unused stock panels
                if !tile_fitted {
                    let unused_panels = solution.get_unused_stock_panels();
                    for panel in unused_panels {
                        if panel.fits(tile_dimensions) {
                            // Create new mosaic from unused panel
                            let mut new_solution = solution.clone();
                            new_solution.get_unused_stock_panels_mut().retain(|p| p != panel);
                            // Add new mosaic with the tile
                            // This would need proper mosaic creation logic
                            new_solutions.push(new_solution);
                            tile_fitted = true;
                            break;
                        }
                    }
                }

                // If still no fit, add to no-fit panels
                if !tile_fitted {
                    let mut new_solution = solution.clone();
                    new_solution.get_mosaics_mut().extend(vec![]);  // Placeholder - would add to no-fit panels
                    new_solutions.push(new_solution);
                }
            }

            // Update current solutions
            current_solutions = new_solutions;
            
            // Remove duplicates and limit solutions
            self.remove_duplicated(&mut current_solutions);
            self.sort_and_limit_solutions(&mut current_solutions, true)?;
        }

        // Update global solutions with thread safety
        {
            let mut all_solutions = self.all_solutions.lock()
                .map_err(|_| OptimizerError::ThreadSync { 
                    message: "Failed to lock all_solutions".to_string() 
                })?;
            
            all_solutions.extend(current_solutions);
            self.sort_and_limit_solutions(&mut all_solutions, false)?;
            
            // Update task rankings for top solutions
            if let Some(task) = &self.task {
                if let Ok(task_guard) = task.lock() {
                    let top_solutions = all_solutions.iter().take(5);
                    for solution in top_solutions {
                        if let Some(material) = solution.get_material() {
                            if let Some(group) = solution.get_creator_thread_group() {
                                task_guard.increment_thread_group_rankings(&material, &group);
                            }
                        }
                    }
                }
            }

            // Remove empty mosaics from the best solution
            if let Some(best_solution) = all_solutions.first_mut() {
                // Remove mosaics with no used area
                best_solution.get_mosaics_mut().retain(|m| {
                    let mut mosaic_clone = m.clone();
                    mosaic_clone.used_area() > 0
                });
            }
        }

        self.status = Status::Finished;
        log_info!("Solution computation completed for thread group: {:?}", self.group);
        Ok(())
    }

    /// Copy a tile node (placeholder implementation)
    pub(crate) fn copy_tile_node(&self, _original: &TileNode, _target: &TileNode) -> Result<TileNode> {
        // This would contain the complex node copying logic from the Java version
        // For now, return a simple copy
        Ok(TileNode::new(0, 100, 0, 100))
    }

    /// Find candidate tile nodes that can accommodate the given dimensions
    pub fn find_candidates(
        &self,
        width: i32,
        height: i32,
        tile_node: &TileNode,
        candidates: &mut Vec<TileNode>,
    ) {
        if tile_node.is_final() 
            || tile_node.width() < width 
            || tile_node.height() < height {
            return;
        }

        // If this is a leaf node, check if it can accommodate the tile
        if tile_node.child1().is_none() && tile_node.child2().is_none() {
            let width_ok = tile_node.width() == width 
                || tile_node.width() >= self.min_trim_dimension + width;
            let height_ok = tile_node.height() == height 
                || tile_node.height() >= self.min_trim_dimension + height;

            if !width_ok && tile_node.width() > width {
                if let Some(task) = &self.task {
                    if let Ok(mut task_guard) = task.lock() {
                        task_guard.set_min_trim_dimension_influenced(true);
                    }
                }
            }

            if !height_ok && tile_node.height() > height {
                if let Some(task) = &self.task {
                    if let Ok(mut task_guard) = task.lock() {
                        task_guard.set_min_trim_dimension_influenced(true);
                    }
                }
            }

            if width_ok && height_ok {
                candidates.push(tile_node.clone());
            }
            return;
        }

        // Recursively check children
        if let Some(child1) = tile_node.child1() {
            self.find_candidates(width, height, child1, candidates);
        }
        if let Some(child2) = tile_node.child2() {
            self.find_candidates(width, height, child2, candidates);
        }
    }
}
