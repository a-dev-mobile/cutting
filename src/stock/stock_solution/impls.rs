use super::structs::StockSolution;
use crate::models::TileDimensions;
use std::collections::HashMap;

impl StockSolution {
    /// Create a new empty StockSolution
    pub fn new() -> Self {
        Self {
            stock_tile_dimensions: Vec::new(),
        }
    }

    /// Create a StockSolution from a vector of TileDimensions
    pub fn from_tiles(tiles: Vec<TileDimensions>) -> Self {
        Self {
            stock_tile_dimensions: tiles,
        }
    }

    /// Create a StockSolution from a slice of TileDimensions
    pub fn from_slice(tiles: &[TileDimensions]) -> Self {
        Self {
            stock_tile_dimensions: tiles.to_vec(),
        }
    }

    /// Add a stock tile to the solution
    pub fn add_stock_tile(&mut self, tile: TileDimensions) {
        self.stock_tile_dimensions.push(tile);
    }

    /// Get a reference to the stock tile dimensions
    pub fn get_stock_tile_dimensions(&self) -> &Vec<TileDimensions> {
        &self.stock_tile_dimensions
    }

    /// Get a mutable reference to the stock tile dimensions
    pub fn get_stock_tile_dimensions_mut(&mut self) -> &mut Vec<TileDimensions> {
        &mut self.stock_tile_dimensions
    }

    /// Set the stock tile dimensions
    pub fn set_stock_tile_dimensions(&mut self, tiles: Vec<TileDimensions>) {
        self.stock_tile_dimensions = tiles;
    }

    /// Sort panels in ascending order by area
    pub fn sort_panels_asc(&mut self) {
        self.stock_tile_dimensions.sort_by(|a, b| a.area().cmp(&b.area()));
    }

    /// Sort panels in descending order by area
    pub fn sort_panels_desc(&mut self) {
        self.stock_tile_dimensions.sort_by(|a, b| b.area().cmp(&a.area()));
    }

    /// Check if all panels have the same unique size
    pub fn has_unique_panel_size(&self) -> bool {
        if self.stock_tile_dimensions.is_empty() {
            return true;
        }

        let first = &self.stock_tile_dimensions[0];
        self.stock_tile_dimensions.iter().all(|tile| tile.has_same_dimensions(first))
    }

    /// Calculate the total area of all tiles
    pub fn get_total_area(&self) -> i64 {
        self.stock_tile_dimensions.iter().map(|tile| tile.area() as i64).sum()
    }

    /// Get a string representation of all tiles
    pub fn to_string(&self) -> String {
        self.stock_tile_dimensions
            .iter()
            .map(|tile| format!("[{}x{}]", tile.width, tile.height))
            .collect::<Vec<_>>()
            .join("")
    }

    /// Get a grouped string representation showing counts of each size
    pub fn to_string_grouped(&self) -> String {
        let mut counts: HashMap<String, i32> = HashMap::new();
        
        for tile in &self.stock_tile_dimensions {
            let key = format!("{}x{}", tile.width, tile.height);
            *counts.entry(key).or_insert(0) += 1;
        }

        let mut result: Vec<String> = counts
            .iter()
            .map(|(size, count)| format!("{}*{}", size, count))
            .collect();
        
        result.sort(); // For consistent output
        result.join(" ")
    }

    /// Get the number of tiles in the solution
    pub fn len(&self) -> usize {
        self.stock_tile_dimensions.len()
    }

    /// Check if the solution is empty
    pub fn is_empty(&self) -> bool {
        self.stock_tile_dimensions.is_empty()
    }

    /// Get an iterator over the tiles
    pub fn iter(&self) -> std::slice::Iter<TileDimensions> {
        self.stock_tile_dimensions.iter()
    }

    /// Get a mutable iterator over the tiles
    pub fn iter_mut(&mut self) -> std::slice::IterMut<TileDimensions> {
        self.stock_tile_dimensions.iter_mut()
    }
}

impl Default for StockSolution {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<TileDimensions>> for StockSolution {
    fn from(tiles: Vec<TileDimensions>) -> Self {
        Self::from_tiles(tiles)
    }
}

impl From<&[TileDimensions]> for StockSolution {
    fn from(tiles: &[TileDimensions]) -> Self {
        Self::from_slice(tiles)
    }
}

impl IntoIterator for StockSolution {
    type Item = TileDimensions;
    type IntoIter = std::vec::IntoIter<TileDimensions>;

    fn into_iter(self) -> Self::IntoIter {
        self.stock_tile_dimensions.into_iter()
    }
}

impl<'a> IntoIterator for &'a StockSolution {
    type Item = &'a TileDimensions;
    type IntoIter = std::slice::Iter<'a, TileDimensions>;

    fn into_iter(self) -> Self::IntoIter {
        self.stock_tile_dimensions.iter()
    }
}

impl<'a> IntoIterator for &'a mut StockSolution {
    type Item = &'a mut TileDimensions;
    type IntoIter = std::slice::IterMut<'a, TileDimensions>;

    fn into_iter(self) -> Self::IntoIter {
        self.stock_tile_dimensions.iter_mut()
    }
}

// Custom equality implementation that matches the Java logic
impl PartialEq for StockSolution {
    fn eq(&self, other: &Self) -> bool {
        if self.stock_tile_dimensions.len() != other.stock_tile_dimensions.len() {
            return false;
        }

        // Create a mutable copy of other's tiles for matching
        let mut other_tiles = other.stock_tile_dimensions.clone();

        // For each tile in self, try to find a matching tile in other
        for tile in &self.stock_tile_dimensions {
            let mut found = false;
            for (i, other_tile) in other_tiles.iter().enumerate() {
                if tile.has_same_dimensions(other_tile) {
                    other_tiles.remove(i);
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }

        true
    }
}

impl Eq for StockSolution {}

impl std::hash::Hash for StockSolution {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash based on sorted dimension pairs to ensure consistent hashing
        // regardless of order
        let mut dimension_pairs: Vec<(i32, i32)> = self.stock_tile_dimensions
            .iter()
            .map(|tile| {
                let (min, max) = if tile.width <= tile.height {
                    (tile.width, tile.height)
                } else {
                    (tile.height, tile.width)
                };
                (min, max)
            })
            .collect();
        
        dimension_pairs.sort();
        dimension_pairs.hash(state);
    }
}

impl std::fmt::Display for StockSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
