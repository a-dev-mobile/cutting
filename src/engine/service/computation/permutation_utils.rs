//! Permutation utilities for the CutList Optimizer Service
//!
//! This module provides utilities for working with permutations and duplicates
//! in tile arrangements and groupings.

use crate::models::tile_dimensions::structs::TileDimensions;

/// Utility struct for permutation-related operations
pub struct PermutationUtils;

impl PermutationUtils {
    /// Removes duplicated permutations from a collection of tiles
    /// 
    /// This method groups tiles that are equivalent (same dimensions, material, orientation)
    /// and removes duplicates while preserving one representative from each group.
    /// 
    /// # Arguments
    /// * `tiles` - Vector of tiles to process
    /// 
    /// # Returns
    /// Vector of unique tiles with duplicates removed
    pub fn remove_duplicated_permutations(tiles: Vec<TileDimensions>) -> Vec<TileDimensions> {
        let mut unique_tiles = Vec::new();
        
        for tile in tiles {
            let mut found_duplicate = false;
            
            for existing_tile in &unique_tiles {
                if Self::are_equivalent_tiles(&tile, existing_tile) {
                    found_duplicate = true;
                    break;
                }
            }
            
            if !found_duplicate {
                unique_tiles.push(tile);
            }
        }
        
        unique_tiles
    }
    
    /// Generates a unique key for a tile based on its properties
    /// 
    /// # Arguments
    /// * `tile` - Reference to the tile
    /// 
    /// # Returns
    /// String key representing the tile's unique properties
    pub fn generate_tile_key(tile: &TileDimensions) -> String {
        format!(
            "{}x{}_{}_{:?}",
            tile.width,
            tile.height,
            tile.material,
            tile.orientation
        )
    }
    
    /// Groups equivalent tiles together
    /// 
    /// # Arguments
    /// * `tiles` - Vector of tiles to group
    /// 
    /// # Returns
    /// Vector of groups, where each group contains equivalent tiles
    pub fn group_equivalent_tiles(tiles: Vec<TileDimensions>) -> Vec<Vec<TileDimensions>> {
        let mut groups: Vec<Vec<TileDimensions>> = Vec::new();
        
        'outer: for tile in tiles {
            for group in &mut groups {
                if let Some(first_tile) = group.first() {
                    if Self::are_equivalent_tiles(&tile, first_tile) {
                        group.push(tile);
                        continue 'outer;
                    }
                }
            }
            
            // If we reach here, tile wasn't added to any existing group
            groups.push(vec![tile]);
        }
        
        groups
    }
    
    /// Checks if two tiles are equivalent (same dimensions, material, orientation)
    /// 
    /// # Arguments
    /// * `tile1` - First tile to compare
    /// * `tile2` - Second tile to compare
    /// 
    /// # Returns
    /// True if tiles are equivalent, false otherwise
    pub fn are_equivalent_tiles(tile1: &TileDimensions, tile2: &TileDimensions) -> bool {
        tile1.width == tile2.width
            && tile1.height == tile2.height
            && tile1.material == tile2.material
            && tile1.orientation == tile2.orientation
    }
    
    /// Calculates the total number of tiles in a group
    /// 
    /// # Arguments
    /// * `tiles` - Reference to vector of tiles
    /// 
    /// # Returns
    /// Total count of tiles (for now, just the length since TileDimensions doesn't have count field)
    pub fn calculate_group_total_count(tiles: &[TileDimensions]) -> usize {
        tiles.len()
    }
    
    /// Gets the maximum count from a group of tiles
    /// 
    /// # Arguments
    /// * `tiles` - Reference to vector of tiles
    /// 
    /// # Returns
    /// Maximum count (for now, returns 1 since TileDimensions doesn't have count field)
    pub fn get_max_count_from_group(tiles: &[TileDimensions]) -> usize {
        if tiles.is_empty() { 0 } else { 1 }
    }
    
    /// Creates a representative tile from a group with updated count
    /// 
    /// # Arguments
    /// * `groups` - Mutable reference to vector of tile groups
    /// 
    /// # Returns
    /// Vector of representative tiles
    pub fn create_representatives_from_groups(groups: &[Vec<TileDimensions>]) -> Vec<TileDimensions> {
        groups.iter()
            .filter_map(|group| group.first().cloned())
            .collect()
    }
    
    /// Validates that permutation removal was successful
    /// 
    /// # Arguments
    /// * `original_tiles` - Original tiles before processing
    /// * `processed_tiles` - Tiles after permutation removal
    /// 
    /// # Returns
    /// True if validation passes, false otherwise
    pub fn validate_permutation_removal(
        original_tiles: &[TileDimensions],
        processed_tiles: &[TileDimensions]
    ) -> bool {
        // Basic validation: processed should have <= original count
        processed_tiles.len() <= original_tiles.len()
    }
}
