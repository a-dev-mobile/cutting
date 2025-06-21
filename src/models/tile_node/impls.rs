use super::structs::{TileNode, NEXT_ID};
use crate::models::{Tile, TileDimensions};
use std::collections::HashSet;
use std::sync::atomic::Ordering;

impl TileNode {
    /// Create a new TileNode with explicit coordinates
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            external_id: None,
            tile: Tile::new(x1, x2, y1, y2),
            is_final: false,
            is_rotated: false,
            child1: None,
            child2: None,
            is_area_totally_used: false,
            totally_used_area: 0,
        }
    }

    /// Create a new TileNode from TileDimensions
    pub fn from_dimensions(tile_dimensions: &TileDimensions) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            external_id: None,
            tile: Tile::from_dimensions(tile_dimensions),
            is_final: false,
            is_rotated: false,
            child1: None,
            child2: None,
            is_area_totally_used: false,
            totally_used_area: 0,
        }
    }

    /// Create a copy of an existing TileNode (with same ID)
    pub fn from_tile_node(other: &TileNode) -> Self {
        Self {
            id: other.id,
            external_id: other.external_id,
            tile: other.tile.clone(),
            is_final: other.is_final,
            is_rotated: other.is_rotated,
            child1: other.child1.clone(),
            child2: other.child2.clone(),
            is_area_totally_used: false,
            totally_used_area: 0,
        }
    }

    /// Get the tile reference
    pub fn tile(&self) -> &Tile {
        &self.tile
    }

    /// Set the tile
    pub fn set_tile(&mut self, tile: Tile) {
        self.tile = tile;
    }

    /// Check if this node is final
    pub fn is_final(&self) -> bool {
        self.is_final
    }

    /// Set the final status
    pub fn set_final(&mut self, is_final: bool) {
        self.is_final = is_final;
    }

    /// Get the external ID
    pub fn external_id(&self) -> Option<i32> {
        self.external_id
    }

    /// Set the external ID
    pub fn set_external_id(&mut self, external_id: Option<i32>) {
        self.external_id = external_id;
    }

    /// Get the unique ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get reference to first child
    pub fn child1(&self) -> Option<&TileNode> {
        self.child1.as_deref()
    }

    /// Get mutable reference to first child
    pub fn child1_mut(&mut self) -> Option<&mut TileNode> {
        self.child1.as_deref_mut()
    }

    /// Set the first child
    pub fn set_child1(&mut self, child: Option<TileNode>) {
        self.child1 = child.map(Box::new);
    }

    /// Get reference to second child
    pub fn child2(&self) -> Option<&TileNode> {
        self.child2.as_deref()
    }

    /// Get mutable reference to second child
    pub fn child2_mut(&mut self) -> Option<&mut TileNode> {
        self.child2.as_deref_mut()
    }

    /// Set the second child
    pub fn set_child2(&mut self, child: Option<TileNode>) {
        self.child2 = child.map(Box::new);
    }

    /// Check if this node has children
    pub fn has_children(&self) -> bool {
        self.child1.is_some() || self.child2.is_some()
    }

    /// Check if this tile is rotated
    pub fn is_rotated(&self) -> bool {
        self.is_rotated
    }

    /// Set the rotation status
    pub fn set_rotated(&mut self, is_rotated: bool) {
        self.is_rotated = is_rotated;
    }

    /// Find a tile node in the tree
    pub fn find_tile(&self, target: &TileNode) -> Option<&TileNode> {
        if self == target {
            return Some(self);
        }

        if let Some(child1) = &self.child1 {
            if let Some(found) = child1.find_tile(target) {
                return Some(found);
            }
        }

        if let Some(child2) = &self.child2 {
            return child2.find_tile(target);
        }

        None
    }

    /// Replace a tile node in the tree
    pub fn replace_tile(&mut self, new_node: TileNode, target: &TileNode) -> Option<&TileNode> {
        if let Some(child1) = &self.child1 {
            if child1.find_tile(target).is_some() {
                self.child1 = Some(Box::new(new_node));
                return self.child1.as_deref();
            }
        }

        if let Some(child2) = &self.child2 {
            if child2.find_tile(target).is_some() {
                self.child2 = Some(Box::new(new_node));
                return self.child2.as_deref();
            }
        }

        None
    }

    /// Get the used area of this node and its children
    pub fn used_area(&mut self) -> i64 {
        if self.is_area_totally_used {
            return self.totally_used_area;
        }

        if self.is_final {
            return self.area();
        }

        let mut used_area = 0i64;

        if let Some(child1) = &mut self.child1 {
            used_area += child1.used_area();
        }

        if let Some(child2) = &mut self.child2 {
            used_area += child2.used_area();
        }

        if used_area == self.area() {
            self.is_area_totally_used = true;
            self.totally_used_area = self.area();
        }

        used_area
    }

    /// Get all unused tiles (leaf nodes that are not final)
    pub fn unused_tiles(&self) -> Vec<&TileNode> {
        let mut result = Vec::new();
        self.collect_unused_tiles(&mut result);
        result
    }

    /// Helper method to collect unused tiles
    fn collect_unused_tiles<'a>(&'a self, result: &mut Vec<&'a TileNode>) {
        if !self.is_final && self.child1.is_none() && self.child2.is_none() {
            result.push(self);
        }

        if let Some(child1) = &self.child1 {
            child1.collect_unused_tiles(result);
        }

        if let Some(child2) = &self.child2 {
            child2.collect_unused_tiles(result);
        }
    }

    /// Get all final tiles
    pub fn final_tiles(&self) -> Vec<&Tile> {
        let mut result = Vec::new();
        self.collect_final_tiles(&mut result);
        result
    }

    /// Helper method to collect final tiles
    fn collect_final_tiles<'a>(&'a self, result: &mut Vec<&'a Tile>) {
        if self.is_final {
            result.push(&self.tile);
        }

        if let Some(child1) = &self.child1 {
            child1.collect_final_tiles(result);
        }

        if let Some(child2) = &self.child2 {
            child2.collect_final_tiles(result);
        }
    }

    /// Get all final tile nodes
    pub fn final_tile_nodes(&self) -> Vec<&TileNode> {
        let mut result = Vec::new();
        self.collect_final_tile_nodes(&mut result);
        result
    }

    /// Helper method to collect final tile nodes
    fn collect_final_tile_nodes<'a>(&'a self, result: &mut Vec<&'a TileNode>) {
        if self.is_final {
            result.push(self);
        }

        if let Some(child1) = &self.child1 {
            child1.collect_final_tile_nodes(result);
        }

        if let Some(child2) = &self.child2 {
            child2.collect_final_tile_nodes(result);
        }
    }

    /// Get the unused area
    pub fn unused_area(&mut self) -> i64 {
        self.area() - self.used_area()
    }

    /// Get the ratio of used area to total area
    pub fn used_area_ratio(&mut self) -> f32 {
        let total_area = self.area();
        if total_area == 0 {
            0.0
        } else {
            self.used_area() as f32 / total_area as f32
        }
    }

    /// Check if this node or any child has final tiles
    pub fn has_final(&self) -> bool {
        if self.is_final {
            return true;
        }

        if let Some(child1) = &self.child1 {
            if child1.has_final() {
                return true;
            }
        }

        if let Some(child2) = &self.child2 {
            if child2.has_final() {
                return true;
            }
        }

        false
    }

    /// Count the number of unused tiles
    pub fn count_unused_tiles(&self) -> usize {
        let mut count = 0;

        if !self.is_final && self.child1.is_none() && self.child2.is_none() {
            count += 1;
        }

        if let Some(child1) = &self.child1 {
            count += child1.count_unused_tiles();
        }

        if let Some(child2) = &self.child2 {
            count += child2.count_unused_tiles();
        }

        count
    }

    /// Get the depth of the tree
    pub fn depth(&self) -> usize {
        let mut depth = 0;

        if let Some(child1) = &self.child1 {
            depth = depth.max(1 + child1.depth());
        }

        if let Some(child2) = &self.child2 {
            depth = depth.max(1 + child2.depth());
        }

        depth
    }

    /// Count the number of final tiles
    pub fn count_final_tiles(&self) -> usize {
        let mut count = if self.is_final { 1 } else { 0 };

        if let Some(child1) = &self.child1 {
            count += child1.count_final_tiles();
        }

        if let Some(child2) = &self.child2 {
            count += child2.count_final_tiles();
        }

        count
    }

    /// Get the biggest unused area
    pub fn biggest_area(&self) -> i64 {
        let mut area = if self.child1.is_none() && self.child2.is_none() && !self.is_final {
            self.area()
        } else {
            0
        };

        if let Some(child1) = &self.child1 {
            area = area.max(child1.biggest_area());
        }

        if let Some(child2) = &self.child2 {
            area = area.max(child2.biggest_area());
        }

        area
    }

    /// Count final horizontal tiles
    pub fn count_final_horizontal(&self) -> usize {
        let mut count = if self.is_final && self.is_horizontal() { 1 } else { 0 };

        if let Some(child1) = &self.child1 {
            count += child1.count_final_horizontal();
        }

        if let Some(child2) = &self.child2 {
            count += child2.count_final_horizontal();
        }

        count
    }

    /// Count final vertical tiles
    pub fn count_final_vertical(&self) -> usize {
        let mut count = if self.is_final && self.is_vertical() { 1 } else { 0 };

        if let Some(child1) = &self.child1 {
            count += child1.count_final_vertical();
        }

        if let Some(child2) = &self.child2 {
            count += child2.count_final_vertical();
        }

        count
    }

    /// Get distinct tile set using a hash-based approach
    pub fn distinct_tile_set(&self) -> HashSet<i32> {
        let mut set = HashSet::new();
        self.collect_distinct_tiles(&mut set);
        set
    }

    /// Helper method to collect distinct tiles
    fn collect_distinct_tiles(&self, set: &mut HashSet<i32>) {
        if self.is_final {
            let width = self.width();
            let height = self.height();
            let sum = width + height;
            let hash_value = ((sum * (sum + 1)) / 2) + height;
            set.insert(hash_value);
        } else {
            if let Some(child1) = &self.child1 {
                child1.collect_distinct_tiles(set);
            }

            if let Some(child2) = &self.child2 {
                child2.collect_distinct_tiles(set);
            }
        }
    }

    /// Convert to TileDimensions
    pub fn to_tile_dimensions(&self) -> TileDimensions {
        TileDimensions {
            id: self.external_id.unwrap_or(self.id as i32),
            width: self.width(),
            height: self.height(),
            label: None,
            material: String::from("default"),
            orientation: crate::models::Orientation::Any,
            is_rotated: self.is_rotated,
        }
    }

    /// Create a string identifier for this node
    pub fn string_identifier(&self) -> String {
        let mut result = String::new();
        self.append_string_identifier(&mut result);
        result
    }

    /// Helper method to append string identifier
    fn append_string_identifier(&self, result: &mut String) {
        result.push_str(&self.x1().to_string());
        result.push_str(&self.y1().to_string());
        result.push_str(&self.x2().to_string());
        result.push_str(&self.y2().to_string());
        result.push_str(&self.is_final.to_string());

        if let Some(child1) = &self.child1 {
            child1.append_string_identifier(result);
        }

        if let Some(child2) = &self.child2 {
            child2.append_string_identifier(result);
        }
    }

    /// Create a tree representation string
    pub fn tree_string(&self) -> String {
        self.append_tree_string("")
    }

    /// Helper method to append tree string with indentation
    fn append_tree_string(&self, indent: &str) -> String {
        let mut result = format!(
            "\n{}({}, {})({}, {})",
            indent,
            self.x1(),
            self.y1(),
            self.x2(),
            self.y2()
        );

        if self.is_final {
            result.push('*');
        }

        if let Some(child1) = &self.child1 {
            let new_indent = format!("{}    ", indent);
            result.push_str(&child1.append_tree_string(&new_indent));
        }

        if let Some(child2) = &self.child2 {
            let new_indent = format!("{}    ", indent);
            result.push_str(&child2.append_tree_string(&new_indent));
        }

        result
    }

    // Delegate methods to the contained Tile
    pub fn x1(&self) -> i32 {
        self.tile.x1()
    }

    pub fn x2(&self) -> i32 {
        self.tile.x2()
    }

    pub fn y1(&self) -> i32 {
        self.tile.y1()
    }

    pub fn y2(&self) -> i32 {
        self.tile.y2()
    }

    pub fn width(&self) -> i32 {
        self.tile.width()
    }

    pub fn height(&self) -> i32 {
        self.tile.height()
    }

    pub fn area(&self) -> i64 {
        self.tile.area()
    }

    pub fn max_side(&self) -> i32 {
        self.tile.max_side()
    }

    pub fn is_horizontal(&self) -> bool {
        self.tile.is_horizontal()
    }

    pub fn is_vertical(&self) -> bool {
        self.tile.is_vertical()
    }
}

impl Default for TileNode {
    fn default() -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            external_id: None,
            tile: Tile::default(),
            is_final: false,
            is_rotated: false,
            child1: None,
            child2: None,
            is_area_totally_used: false,
            totally_used_area: 0,
        }
    }
}

impl std::fmt::Display for TileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tree_string())
    }
}

// Custom equality implementation that matches the Java logic
impl PartialEq for TileNode {
    fn eq(&self, other: &Self) -> bool {
        // Check basic properties
        if self.id != other.id || self.tile != other.tile || self.is_final != other.is_final {
            return false;
        }

        // Check children combinations
        match (&self.child1, &other.child1, &self.child2, &other.child2) {
            // Both have no children
            (None, None, None, None) => true,
            // Both have only child1
            (Some(c1), Some(o1), None, None) => c1 == o1,
            // Both have only child2
            (None, None, Some(c2), Some(o2)) => c2 == o2,
            // Both have both children
            (Some(c1), Some(o1), Some(c2), Some(o2)) => c1 == o1 && c2 == o2,
            // Any other combination
            _ => false,
        }
    }
}

impl Eq for TileNode {}
