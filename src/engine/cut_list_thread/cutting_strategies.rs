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
        
        if node.width() > tile_dimensions.width {
            let cut = self.split_horizontally(node, tile_dimensions.width, cut_thickness, tile_dimensions.id)?;
            cuts.push(cut);
            
            if node.height() > tile_dimensions.height {
                // Would need to operate on child node - simplified for now
            }
        } else if node.height() > tile_dimensions.height {
            let cut = self.split_vertically(node, tile_dimensions.height, cut_thickness, tile_dimensions.id)?;
            cuts.push(cut);
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
        
        if node.height() > tile_dimensions.height {
            let cut = self.split_vertically(node, tile_dimensions.height, cut_thickness, tile_dimensions.id)?;
            cuts.push(cut);
            
            if node.width() > tile_dimensions.width {
                // Would need to operate on child node - simplified for now
            }
        } else if node.width() > tile_dimensions.width {
            let cut = self.split_horizontally(node, tile_dimensions.width, cut_thickness, tile_dimensions.id)?;
            cuts.push(cut);
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
}
