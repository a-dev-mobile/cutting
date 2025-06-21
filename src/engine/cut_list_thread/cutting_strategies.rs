//! Cutting strategy implementations for CutListThread
//! 
//! This module contains the various cutting algorithms and strategies for splitting tiles.

use crate::{
    models::TileNode,
    error::Result,
};

use super::structs::CutListThread;

impl CutListThread {
    /// Split using horizontal-then-vertical strategy
    pub fn split_hv(
        &self,
        node: &TileNode,
        tile_dimensions: &crate::models::TileDimensions,
        cut_thickness: i32,
    ) -> Result<Vec<crate::models::Cut>> {
        let mut cuts = Vec::new();
        let mut working_node = node.clone();
        
        if node.width() > tile_dimensions.width {
            let cut = self.split_horizontally_with_children(&mut working_node, tile_dimensions.width, cut_thickness)?;
            cuts.push(cut);
            
            if node.height() > tile_dimensions.height {
                // Split the left child (child1) vertically
                if let Some(child1) = working_node.child1() {
                    let mut child1_clone = child1.clone();
                    let vertical_cut = self.split_vertically_with_children(&mut child1_clone, tile_dimensions.height, cut_thickness)?;
                    cuts.push(vertical_cut);
                    
                    // Mark the final tile
                    if let Some(final_child) = child1_clone.child1() {
                        let mut final_tile = final_child.clone();
                        final_tile.set_final(true);
                        final_tile.set_rotated(tile_dimensions.is_rotated);
                        final_tile.set_external_id(Some(tile_dimensions.id));
                    }
                }
            } else {
                // Mark child1 as final
                if let Some(child1) = working_node.child1() {
                    let mut final_tile = child1.clone();
                    final_tile.set_final(true);
                    final_tile.set_rotated(tile_dimensions.is_rotated);
                    final_tile.set_external_id(Some(tile_dimensions.id));
                }
            }
        } else if node.height() > tile_dimensions.height {
            let cut = self.split_vertically_with_children(&mut working_node, tile_dimensions.height, cut_thickness)?;
            cuts.push(cut);
            
            // Mark child1 as final
            if let Some(child1) = working_node.child1() {
                let mut final_tile = child1.clone();
                final_tile.set_final(true);
                final_tile.set_rotated(tile_dimensions.is_rotated);
                final_tile.set_external_id(Some(tile_dimensions.id));
            }
        }
        
        Ok(cuts)
    }

    /// Split using vertical-then-horizontal strategy
    pub fn split_vh(
        &self,
        node: &TileNode,
        tile_dimensions: &crate::models::TileDimensions,
        cut_thickness: i32,
    ) -> Result<Vec<crate::models::Cut>> {
        let mut cuts = Vec::new();
        let mut working_node = node.clone();
        
        if node.height() > tile_dimensions.height {
            let cut = self.split_vertically_with_children(&mut working_node, tile_dimensions.height, cut_thickness)?;
            cuts.push(cut);
            
            if node.width() > tile_dimensions.width {
                // Split the top child (child1) horizontally
                if let Some(child1) = working_node.child1() {
                    let mut child1_clone = child1.clone();
                    let horizontal_cut = self.split_horizontally_with_children(&mut child1_clone, tile_dimensions.width, cut_thickness)?;
                    cuts.push(horizontal_cut);
                    
                    // Mark the final tile
                    if let Some(final_child) = child1_clone.child1() {
                        let mut final_tile = final_child.clone();
                        final_tile.set_final(true);
                        final_tile.set_rotated(tile_dimensions.is_rotated);
                        final_tile.set_external_id(Some(tile_dimensions.id));
                    }
                }
            } else {
                // Mark child1 as final
                if let Some(child1) = working_node.child1() {
                    let mut final_tile = child1.clone();
                    final_tile.set_final(true);
                    final_tile.set_rotated(tile_dimensions.is_rotated);
                    final_tile.set_external_id(Some(tile_dimensions.id));
                }
            }
        } else if node.width() > tile_dimensions.width {
            let cut = self.split_horizontally_with_children(&mut working_node, tile_dimensions.width, cut_thickness)?;
            cuts.push(cut);
            
            // Mark child1 as final
            if let Some(child1) = working_node.child1() {
                let mut final_tile = child1.clone();
                final_tile.set_final(true);
                final_tile.set_rotated(tile_dimensions.is_rotated);
                final_tile.set_external_id(Some(tile_dimensions.id));
            }
        }
        
        Ok(cuts)
    }

    /// Create a horizontal cut
    pub fn split_horizontally(
        &self,
        node: &TileNode,
        width: i32,
        _cut_thickness: i32,
        tile_id: i32,
    ) -> Result<crate::models::Cut> {
        use crate::models::Cut;
        
        Ok(Cut {
            x1: node.x1() + width,
            y1: node.y1(),
            x2: node.x1() + width,
            y2: node.y2(),
            original_width: node.width(),
            original_height: node.height(),
            is_horizontal: true,
            cut_coord: width,
            original_tile_id: node.id() as i32,
            child1_tile_id: tile_id,
            child2_tile_id: 0, // Placeholder for second child
        })
    }

    /// Create a vertical cut
    pub fn split_vertically(
        &self,
        node: &TileNode,
        height: i32,
        _cut_thickness: i32,
        tile_id: i32,
    ) -> Result<crate::models::Cut> {
        use crate::models::Cut;
        
        Ok(Cut {
            x1: node.x1(),
            y1: node.y1() + height,
            x2: node.x2(),
            y2: node.y1() + height,
            original_width: node.width(),
            original_height: node.height(),
            is_horizontal: false,
            cut_coord: height,
            original_tile_id: node.id() as i32,
            child1_tile_id: tile_id,
            child2_tile_id: 0, // Placeholder for second child
        })
    }

    /// Create a horizontal cut and set up child nodes
    pub fn split_horizontally_with_children(
        &self,
        node: &mut TileNode,
        width: i32,
        cut_thickness: i32,
    ) -> Result<crate::models::Cut> {
        use crate::models::Cut;
        
        let original_width = node.width();
        let original_height = node.height();
        
        // Create child1 (left part)
        let child1 = TileNode::new(
            node.x1(),
            node.x1() + width,
            node.y1(),
            node.y2(),
        );
        
        // Create child2 (right part)
        let child2 = TileNode::new(
            node.x1() + width + cut_thickness,
            node.x2(),
            node.y1(),
            node.y2(),
        );
        
        let child1_id = child1.id();
        let child2_id = child2.id();
        
        // Set children if they have positive area
        if child1.area() > 0 {
            node.set_child1(Some(child1));
        }
        if child2.area() > 0 {
            node.set_child2(Some(child2));
        }
        
        Ok(Cut {
            x1: node.x1() + width,
            y1: node.y1(),
            x2: node.x1() + width,
            y2: node.y2(),
            original_width,
            original_height,
            is_horizontal: true,
            cut_coord: width,
            original_tile_id: node.id() as i32,
            child1_tile_id: child1_id as i32,
            child2_tile_id: child2_id as i32,
        })
    }

    /// Create a vertical cut and set up child nodes
    pub fn split_vertically_with_children(
        &self,
        node: &mut TileNode,
        height: i32,
        cut_thickness: i32,
    ) -> Result<crate::models::Cut> {
        use crate::models::Cut;
        
        let original_width = node.width();
        let original_height = node.height();
        
        // Create child1 (top part)
        let child1 = TileNode::new(
            node.x1(),
            node.x2(),
            node.y1(),
            node.y1() + height,
        );
        
        // Create child2 (bottom part)
        let child2 = TileNode::new(
            node.x1(),
            node.x2(),
            node.y1() + height + cut_thickness,
            node.y2(),
        );
        
        let child1_id = child1.id();
        let child2_id = child2.id();
        
        // Set children if they have positive area
        if child1.area() > 0 {
            node.set_child1(Some(child1));
        }
        if child2.area() > 0 {
            node.set_child2(Some(child2));
        }
        
        Ok(Cut {
            x1: node.x1(),
            y1: node.y1() + height,
            x2: node.x2(),
            y2: node.y1() + height,
            original_width,
            original_height,
            is_horizontal: false,
            cut_coord: height,
            original_tile_id: node.id() as i32,
            child1_tile_id: child1_id as i32,
            child2_tile_id: child2_id as i32,
        })
    }
}
