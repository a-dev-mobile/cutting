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
                // Exact fit
                let new_mosaic = mosaic.clone();
                // Set tile properties on the node
                results.push(new_mosaic);
            } else {
                // Need to cut
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
}
