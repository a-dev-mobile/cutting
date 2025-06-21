use super::structs::Tile;
use crate::models::TileDimensions;

impl Tile {
    /// Create a new tile from TileDimensions, positioned at origin (0,0)
    pub fn from_dimensions(tile_dimensions: &TileDimensions) -> Self {
        Self {
            x1: 0,
            x2: tile_dimensions.width,
            y1: 0,
            y2: tile_dimensions.height,
        }
    }

    /// Create a new tile with explicit coordinates
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        Self { x1, x2, y1, y2 }
    }

    /// Create a copy of an existing tile
    pub fn from_tile(tile: &Tile) -> Self {
        Self {
            x1: tile.x1,
            x2: tile.x2,
            y1: tile.y1,
            y2: tile.y2,
        }
    }

    /// Get the x1 coordinate
    pub fn x1(&self) -> i32 {
        self.x1
    }

    /// Get the x2 coordinate
    pub fn x2(&self) -> i32 {
        self.x2
    }

    /// Get the y1 coordinate
    pub fn y1(&self) -> i32 {
        self.y1
    }

    /// Get the y2 coordinate
    pub fn y2(&self) -> i32 {
        self.y2
    }

    /// Calculate the width of the tile
    pub fn width(&self) -> i32 {
        self.x2 - self.x1
    }

    /// Calculate the height of the tile
    pub fn height(&self) -> i32 {
        self.y2 - self.y1
    }

    /// Calculate the area of the tile
    pub fn area(&self) -> i64 {
        (self.width() as i64) * (self.height() as i64)
    }

    /// Get the maximum side length (width or height)
    pub fn max_side(&self) -> i32 {
        self.width().max(self.height())
    }

    /// Check if the tile is horizontally oriented (width > height)
    pub fn is_horizontal(&self) -> bool {
        self.width() > self.height()
    }

    /// Check if the tile is vertically oriented (height > width)
    pub fn is_vertical(&self) -> bool {
        self.height() > self.width()
    }

    /// Check if the tile is square (width == height)
    pub fn is_square(&self) -> bool {
        self.width() == self.height()
    }

    /// Get the minimum side length (width or height)
    pub fn min_side(&self) -> i32 {
        self.width().min(self.height())
    }

    /// Check if this tile contains a point
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.x1 && x < self.x2 && y >= self.y1 && y < self.y2
    }

    /// Check if this tile overlaps with another tile
    pub fn overlaps_with(&self, other: &Tile) -> bool {
        !(self.x2 <= other.x1 || other.x2 <= self.x1 || self.y2 <= other.y1 || other.y2 <= self.y1)
    }

    /// Move the tile by the specified offset
    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x1 += dx;
        self.x2 += dx;
        self.y1 += dy;
        self.y2 += dy;
    }

    /// Create a new tile translated by the specified offset
    pub fn translated(&self, dx: i32, dy: i32) -> Self {
        Self {
            x1: self.x1 + dx,
            x2: self.x2 + dx,
            y1: self.y1 + dy,
            y2: self.y2 + dy,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            x1: 0,
            x2: 0,
            y1: 0,
            y2: 0,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tile[({}, {}) -> ({}, {}), {}x{}]",
            self.x1,
            self.y1,
            self.x2,
            self.y2,
            self.width(),
            self.height()
        )
    }
}
