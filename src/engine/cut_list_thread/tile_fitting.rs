//! Tile fitting implementations for CutListThread
//! 
//! This module contains the logic for fitting tiles into mosaics and handling placement strategies.

use crate::{
    models::TileNode,
    error::Result,
};

use super::structs::CutListThread;

impl CutListThread {
    /// Add a tile to a mosaic, generating all possible fitting results
    pub(crate) fn add_tile_to_mosaic(
        &self,
        tile_dimensions: &crate::models::TileDimensions,
        mosaic: &crate::models::Mosaic,
        results: &mut Vec<crate::models::Mosaic>,
    ) -> Result<()> {
        // Check grain direction compatibility
        if !self.consider_grain_direction 
            || mosaic.orientation() == crate::Orientation::Any 
            || tile_dimensions.orientation == crate::Orientation::Any {
            self.fit_tile(tile_dimensions, mosaic, results, self.cut_thickness)?;
            
            if !tile_dimensions.is_square() {
                let mut rotated_tile = tile_dimensions.clone();
                rotated_tile.rotate_90();
                self.fit_tile(&rotated_tile, mosaic, results, self.cut_thickness)?;
            }
        } else {
            let tile_to_use = if mosaic.orientation() != tile_dimensions.orientation {
                let mut rotated = tile_dimensions.clone();
                rotated.rotate_90();
                rotated
            } else {
                tile_dimensions.clone()
            };
            self.fit_tile(&tile_to_use, mosaic, results, self.cut_thickness)?;
        }
        
        Ok(())
    }

    /// Fit a tile into a mosaic using various cutting strategies
    pub(crate) fn fit_tile(
        &self,
        tile_dimensions: &crate::models::TileDimensions,
        mosaic: &crate::models::Mosaic,
        results: &mut Vec<crate::models::Mosaic>,
        cut_thickness: i32,
    ) -> Result<()> {
        let mut candidates = Vec::new();
        self.find_candidates(
            tile_dimensions.width,
            tile_dimensions.height,
            &mosaic.root_tile_node(),
            &mut candidates,
        );

        for candidate in candidates {
            if candidate.width() == tile_dimensions.width 
                && candidate.height() == tile_dimensions.height {
                // Exact fit - copy the mosaic and mark the node as final
                let root_copy = self.copy_tile_node(&mosaic.root_tile_node(), &candidate)?;
                
                // Find the corresponding node in the copy and mark it as final
                if let Some(mut target_node) = self.find_corresponding_node(&root_copy, &candidate) {
                    target_node.set_external_id(Some(tile_dimensions.id));
                    target_node.set_final(true);
                    target_node.set_rotated(tile_dimensions.is_rotated);
                    
                    let mut new_mosaic = mosaic.clone();
                    new_mosaic.set_root_tile_node(root_copy);
                    new_mosaic.set_stock_id(mosaic.stock_id());
                    new_mosaic.set_orientation(mosaic.orientation());
                    results.push(new_mosaic);
                }
            } else {
                // Need to cut - try both cutting strategies if orientation allows
                self.fit_tile_with_cuts(tile_dimensions, mosaic, &candidate, results, cut_thickness)?;
            }
        }

        Ok(())
    }

    /// Fit a tile using cutting strategies
    pub(crate) fn fit_tile_with_cuts(
        &self,
        tile_dimensions: &crate::models::TileDimensions,
        mosaic: &crate::models::Mosaic,
        candidate: &TileNode,
        results: &mut Vec<crate::models::Mosaic>,
        cut_thickness: i32,
    ) -> Result<()> {
        use crate::models::enums::cut_direction::CutDirection;
        
        match self.first_cut_orientation {
            CutDirection::Both => {
                self.try_horizontal_first_cut(tile_dimensions, mosaic, candidate, results, cut_thickness)?;
                self.try_vertical_first_cut(tile_dimensions, mosaic, candidate, results, cut_thickness)?;
            },
            CutDirection::Horizontal => {
                self.try_horizontal_first_cut(tile_dimensions, mosaic, candidate, results, cut_thickness)?;
            },
            CutDirection::Vertical => {
                self.try_vertical_first_cut(tile_dimensions, mosaic, candidate, results, cut_thickness)?;
            },
        }
        Ok(())
    }

    /// Try horizontal-first cutting strategy
    pub(crate) fn try_horizontal_first_cut(
        &self,
        tile_dimensions: &crate::models::TileDimensions,
        mosaic: &crate::models::Mosaic,
        candidate: &TileNode,
        results: &mut Vec<crate::models::Mosaic>,
        cut_thickness: i32,
    ) -> Result<()> {
        let mut new_mosaic = mosaic.clone();
        let cuts = self.split_hv(candidate, tile_dimensions, cut_thickness)?;
        
        for cut in cuts {
            new_mosaic.add_cut(cut);
        }
        
        results.push(new_mosaic);
        Ok(())
    }

    /// Try vertical-first cutting strategy
    pub(crate) fn try_vertical_first_cut(
        &self,
        tile_dimensions: &crate::models::TileDimensions,
        mosaic: &crate::models::Mosaic,
        candidate: &TileNode,
        results: &mut Vec<crate::models::Mosaic>,
        cut_thickness: i32,
    ) -> Result<()> {
        let mut new_mosaic = mosaic.clone();
        let cuts = self.split_vh(candidate, tile_dimensions, cut_thickness)?;
        
        for cut in cuts {
            new_mosaic.add_cut(cut);
        }
        
        results.push(new_mosaic);
        Ok(())
    }

    /// Find the corresponding node in a copied tree structure
    pub(crate) fn find_corresponding_node(
        &self,
        root_copy: &TileNode,
        original_target: &TileNode,
    ) -> Option<TileNode> {
        self.find_node_by_coordinates(
            root_copy,
            original_target.x1(),
            original_target.y1(),
            original_target.x2(),
            original_target.y2(),
        )
    }

    /// Find a node by its coordinates in the tree
    fn find_node_by_coordinates(
        &self,
        node: &TileNode,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> Option<TileNode> {
        if node.x1() == x1 && node.y1() == y1 && node.x2() == x2 && node.y2() == y2 {
            return Some(node.clone());
        }

        // Search in children
        if let Some(child1) = node.child1() {
            if let Some(found) = self.find_node_by_coordinates(child1, x1, y1, x2, y2) {
                return Some(found);
            }
        }

        if let Some(child2) = node.child2() {
            if let Some(found) = self.find_node_by_coordinates(child2, x1, y1, x2, y2) {
                return Some(found);
            }
        }

        None
    }
}
