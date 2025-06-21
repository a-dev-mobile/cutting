use super::structs::Mosaic;
use crate::{models::{ Cut, TileDimensions, TileNode}, Orientation, };
use std::collections::HashSet;

impl Mosaic {
    // Getter and setter methods (following Rust conventions)
    
    /// Get a reference to the root tile node
    pub fn root_tile_node(&self) -> &TileNode {
        &self.root_tile_node
    }

    /// Get a mutable reference to the root tile node
    pub fn root_tile_node_mut(&mut self) -> &mut TileNode {
        &mut self.root_tile_node
    }

    /// Set the root tile node
    pub fn set_root_tile_node(&mut self, tile_node: TileNode) {
        self.root_tile_node = tile_node;
    }

    /// Get a reference to the cuts vector
    pub fn cuts(&self) -> &Vec<Cut> {
        &self.cuts
    }

    /// Get a mutable reference to the cuts vector
    pub fn cuts_mut(&mut self) -> &mut Vec<Cut> {
        &mut self.cuts
    }

    /// Set the cuts vector
    pub fn set_cuts(&mut self, cuts: Vec<Cut>) {
        self.cuts = cuts;
    }

    /// Get the number of cuts
    pub fn nbr_cuts(&self) -> usize {
        self.cuts.len()
    }

    /// Get the stock ID
    pub fn stock_id(&self) -> i32 {
        self.stock_id
    }

    /// Set the stock ID
    pub fn set_stock_id(&mut self, stock_id: i32) {
        self.stock_id = stock_id;
    }

    /// Get a reference to the material string
    pub fn material(&self) -> &str {
        &self.material
    }

    /// Set the material
    pub fn set_material(&mut self, material: String) {
        self.material = material;
    }

    /// Get the orientation
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Set the orientation
    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    // Delegation methods to root_tile_node
    
    /// Get all final tile nodes from the root tile node
    pub fn final_tile_nodes(&self) -> Vec<&TileNode> {
        self.root_tile_node.final_tile_nodes()
    }

    /// Get the horizontal/vertical difference
    /// Returns the absolute difference between horizontal and vertical final tiles
    pub fn hv_diff(&self) -> f32 {
        let horizontal_count = self.root_tile_node.count_final_horizontal() as f32;
        let vertical_count = self.root_tile_node.count_final_vertical() as f32;
        (horizontal_count - vertical_count).abs()
    }

    /// Get the distinct tile set
    pub fn distinct_tile_set(&self) -> HashSet<i32> {
        self.root_tile_node.distinct_tile_set()
    }

    /// Get the used area
    pub fn used_area(&mut self) -> i64 {
        self.root_tile_node.used_area()
    }

    /// Get the unused area
    pub fn unused_area(&mut self) -> i64 {
        self.root_tile_node.unused_area()
    }

    /// Get the depth of the cutting tree
    pub fn depth(&self) -> usize {
        self.root_tile_node.depth()
    }

    /// Get the biggest unused tile
    /// Returns None if no unused tiles exist
    pub fn biggest_unused_tile(&self) -> Option<&TileNode> {
        let unused_tiles = self.root_tile_node.unused_tiles();
        unused_tiles.iter()
            .max_by_key(|tile| tile.area())
            .copied()
    }

    /// Calculate the center of mass distance to origin
    /// Returns a normalized distance from 0.0 to 1.0
    pub fn center_of_mass_distance_to_origin(&self) -> f32 {
        let used_area = {
            let mut mosaic_copy = self.clone();
            mosaic_copy.used_area()
        };
        
        if used_area == 0 {
            return 0.0;
        }

        let mut weighted_x = 0.0f32;
        let mut weighted_y = 0.0f32;

        for tile_node in self.final_tile_nodes() {
            let area = tile_node.area() as f32;
            let center_x = tile_node.x1() as f32 + (tile_node.width() as f32 * 0.5);
            let center_y = tile_node.y1() as f32 + (tile_node.height() as f32 * 0.5);
            
            weighted_x += area * center_x;
            weighted_y += area * center_y;
        }

        let used_area_f32 = used_area as f32;
        let center_of_mass_x = weighted_x / used_area_f32;
        let center_of_mass_y = weighted_y / used_area_f32;

        // Calculate distance from origin
        let distance = (center_of_mass_x.powi(2) + center_of_mass_y.powi(2)).sqrt();
        
        // Normalize by the diagonal of the root tile
        let root_width = self.root_tile_node.width() as f32;
        let root_height = self.root_tile_node.height() as f32;
        let diagonal = (root_width.powi(2) + root_height.powi(2)).sqrt();
        
        if diagonal == 0.0 {
            0.0
        } else {
            distance / diagonal
        }
    }

    /// Get the biggest area among unused tiles
    pub fn biggest_area(&self) -> i64 {
        self.root_tile_node.biggest_area()
    }

    /// Add a cut to the mosaic
    pub fn add_cut(&mut self, cut: Cut) {
        self.cuts.push(cut);
    }

    /// Remove all cuts
    pub fn clear_cuts(&mut self) {
        self.cuts.clear();
    }

    /// Check if the mosaic has any cuts
    pub fn has_cuts(&self) -> bool {
        !self.cuts.is_empty()
    }

    /// Get the total area of the root tile
    pub fn total_area(&self) -> i64 {
        self.root_tile_node.area()
    }

    /// Get the efficiency ratio (used area / total area)
    pub fn efficiency(&mut self) -> f32 {
        let total = self.total_area();
        if total == 0 {
            0.0
        } else {
            self.used_area() as f32 / total as f32
        }
    }

    /// Get the waste ratio (unused area / total area)
    pub fn waste_ratio(&mut self) -> f32 {
        1.0 - self.efficiency()
    }

    /// Count the number of final tiles
    pub fn final_tile_count(&self) -> usize {
        self.root_tile_node.count_final_tiles()
    }

    /// Count the number of unused tiles
    pub fn unused_tile_count(&self) -> usize {
        self.root_tile_node.count_unused_tiles()
    }

    /// Check if the mosaic has any final tiles
    pub fn has_final_tiles(&self) -> bool {
        self.root_tile_node.has_final()
    }

    /// Get the width of the root tile
    pub fn width(&self) -> i32 {
        self.root_tile_node.width()
    }

    /// Get the height of the root tile
    pub fn height(&self) -> i32 {
        self.root_tile_node.height()
    }

    /// Convert to TileDimensions
    pub fn to_tile_dimensions(&self) -> TileDimensions {
        TileDimensions {
            id: self.stock_id,
            width: self.width(),
            height: self.height(),
            label: None,
            material: self.material.clone(),
            orientation: self.orientation,
            is_rotated: false,
        }
    }
}

impl std::fmt::Display for Mosaic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mosaic[stock_id={}, material={}, cuts={}, area={}]", 
               self.stock_id, self.material, self.cuts.len(), self.root_tile_node.area())
    }
}
