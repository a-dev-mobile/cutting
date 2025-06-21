//! Core computation implementations for CutListThread
//! 
//! This module contains the main computation algorithm and orchestration logic.

use crate::{
    log_debug, log_info,
    models::{Solution, TileNode},
    error::{AppError, Result},
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

            // Try to fit the tile into each existing solution
            for solution in &current_solutions {
                let mut tile_fitted_in_solution = false;
                
                // Try to fit into each mosaic in the solution
                let mosaics = solution.get_mosaics();
                for mosaic in mosaics.iter() {
                    // Check material compatibility
                    let mosaic_material = mosaic.material();
                    if mosaic_material != tile_dimensions.material {
                        continue;
                    }

                    let mut fitting_results = Vec::new();
                    self.add_tile_to_mosaic(tile_dimensions, mosaic, &mut fitting_results)?;
                    
                    // Create new solutions for each fitting result
                    for result_mosaic in fitting_results {
                        let mut new_solution = Solution::from_solution_excluding_mosaic(solution, mosaic);
                        new_solution.add_mosaic(result_mosaic);
                        new_solution.set_creator_thread_group(self.group.clone());
                        new_solution.set_aux_info(self.aux_info.clone());
                        new_solutions.push(new_solution);
                        tile_fitted_in_solution = true;
                    }
                    
                    // If we found a fit, break to avoid multiple fits in same solution
                    if tile_fitted_in_solution {
                        break;
                    }
                }

                // If tile didn't fit in any existing mosaic, try unused stock panels
                if !tile_fitted_in_solution {
                    let unused_panels: Vec<_> = solution.get_unused_stock_panels().iter().cloned().collect();
                    for panel in unused_panels {
                        if panel.fits(tile_dimensions) {
                            // Create new solution with new mosaic from unused panel
                            let mut new_solution = solution.clone();
                            new_solution.get_unused_stock_panels_mut().retain(|p| p != &panel);
                            
                            // Create new mosaic from the panel and add the tile
                            let new_mosaic = crate::models::Mosaic::from_tile_dimensions(&panel);
                            let mut fitting_results = Vec::new();
                            self.add_tile_to_mosaic(tile_dimensions, &new_mosaic, &mut fitting_results)?;
                            
                            for result_mosaic in fitting_results {
                                let mut solution_with_new_mosaic = new_solution.clone();
                                solution_with_new_mosaic.add_mosaic(result_mosaic);
                                solution_with_new_mosaic.set_creator_thread_group(self.group.clone());
                                solution_with_new_mosaic.set_aux_info(self.aux_info.clone());
                                new_solutions.push(solution_with_new_mosaic);
                                tile_fitted_in_solution = true;
                            }
                            break;
                        }
                    }
                }

                // If still no fit, add to no-fit panels
                if !tile_fitted_in_solution {
                    let mut new_solution = solution.clone();
                    new_solution.add_no_fit_panel(tile_dimensions.clone());
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
                .map_err(|_| AppError::ThreadSync { 
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
                                task_guard.increment_thread_group_rankings(material, group);
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

    /// Copy a tile node tree, stopping at the target node
    pub(crate) fn copy_tile_node(&self, original: &TileNode, target: &TileNode) -> Result<TileNode> {
        let mut root_copy = TileNode::new(original.x1(), original.x2(), original.y1(), original.y2());
        self.copy_children(original, &mut root_copy, target)?;
        Ok(root_copy)
    }

    /// Recursively copy children of a tile node
    fn copy_children(&self, original: &TileNode, copy: &mut TileNode, target: &TileNode) -> Result<()> {
        // If we've reached the target node, stop copying
        if std::ptr::eq(original, target) {
            return Ok(());
        }

        // Copy child1 if it exists
        if let Some(child1) = original.child1() {
            let mut child1_copy = TileNode::new(child1.x1(), child1.x2(), child1.y1(), child1.y2());
            child1_copy.set_external_id(child1.external_id());
            child1_copy.set_final(child1.is_final());
            child1_copy.set_rotated(child1.is_rotated());
            copy.set_child1(Some(child1_copy.clone()));
            self.copy_children(child1, &mut child1_copy, target)?;
        }

        // Copy child2 if it exists
        if let Some(child2) = original.child2() {
            let mut child2_copy = TileNode::new(child2.x1(), child2.x2(), child2.y1(), child2.y2());
            child2_copy.set_external_id(child2.external_id());
            child2_copy.set_final(child2.is_final());
            child2_copy.set_rotated(child2.is_rotated());
            copy.set_child2(Some(child2_copy.clone()));
            self.copy_children(child2, &mut child2_copy, target)?;
        }

        Ok(())
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
