use super::structs::{Cut, CutBuilder};

impl Cut {
    /// Create a new Cut with all parameters
    pub fn new(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        original_width: i32,
        original_height: i32,
        is_horizontal: bool,
        cut_coord: i32,
        original_tile_id: i32,
        child1_tile_id: i32,
        child2_tile_id: i32,
    ) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            original_width,
            original_height,
            is_horizontal,
            cut_coord,
            original_tile_id,
            child1_tile_id,
            child2_tile_id,
        }
    }

    /// Create a copy of an existing Cut
    pub fn from_cut(cut: &Cut) -> Self {
        Self {
            x1: cut.x1,
            y1: cut.y1,
            x2: cut.x2,
            y2: cut.y2,
            original_width: cut.original_width,
            original_height: cut.original_height,
            is_horizontal: cut.is_horizontal,
            cut_coord: cut.cut_coord,
            original_tile_id: cut.original_tile_id,
            child1_tile_id: cut.child1_tile_id,
            child2_tile_id: cut.child2_tile_id,
        }
    }

    /// Create a Cut from a CutBuilder
    pub fn from_builder(builder: CutBuilder) -> Self {
        Self {
            x1: builder.x1,
            y1: builder.y1,
            x2: builder.x2,
            y2: builder.y2,
            original_width: builder.original_width,
            original_height: builder.original_height,
            is_horizontal: builder.is_horizontal,
            cut_coord: builder.cut_coord,
            original_tile_id: builder.original_tile_id,
            child1_tile_id: builder.child1_tile_id,
            child2_tile_id: builder.child2_tile_id,
        }
    }

    /// Get the x1 coordinate
    pub fn x1(&self) -> i32 {
        self.x1
    }

    /// Get the y1 coordinate
    pub fn y1(&self) -> i32 {
        self.y1
    }

    /// Get the x2 coordinate
    pub fn x2(&self) -> i32 {
        self.x2
    }

    /// Get the y2 coordinate
    pub fn y2(&self) -> i32 {
        self.y2
    }

    /// Get the original tile ID
    pub fn original_tile_id(&self) -> i32 {
        self.original_tile_id
    }

    /// Get the first child tile ID
    pub fn child1_tile_id(&self) -> i32 {
        self.child1_tile_id
    }

    /// Get the second child tile ID
    pub fn child2_tile_id(&self) -> i32 {
        self.child2_tile_id
    }

    /// Get the original width
    pub fn original_width(&self) -> i32 {
        self.original_width
    }

    /// Get the original height
    pub fn original_height(&self) -> i32 {
        self.original_height
    }

    /// Check if the cut is horizontal
    pub fn is_horizontal(&self) -> bool {
        self.is_horizontal
    }

    /// Get the cut coordinate
    pub fn cut_coord(&self) -> i32 {
        self.cut_coord
    }

    /// Calculate the length of the cut (fixed typo from original Java "getLenght")
    pub fn length(&self) -> i64 {
        ((self.x2 - self.x1).abs() + (self.y2 - self.y1).abs()) as i64
    }

    /// Create a new CutBuilder
    pub fn builder() -> CutBuilder {
        CutBuilder::default()
    }
}

impl CutBuilder {
    /// Create a new CutBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the x1 coordinate
    pub fn x1(&self) -> i32 {
        self.x1
    }

    /// Set the x1 coordinate
    pub fn set_x1(mut self, x1: i32) -> Self {
        self.x1 = x1;
        self
    }

    /// Get the y1 coordinate
    pub fn y1(&self) -> i32 {
        self.y1
    }

    /// Set the y1 coordinate
    pub fn set_y1(mut self, y1: i32) -> Self {
        self.y1 = y1;
        self
    }

    /// Get the x2 coordinate
    pub fn x2(&self) -> i32 {
        self.x2
    }

    /// Set the x2 coordinate
    pub fn set_x2(mut self, x2: i32) -> Self {
        self.x2 = x2;
        self
    }

    /// Get the y2 coordinate
    pub fn y2(&self) -> i32 {
        self.y2
    }

    /// Set the y2 coordinate
    pub fn set_y2(mut self, y2: i32) -> Self {
        self.y2 = y2;
        self
    }

    /// Get the original width
    pub fn original_width(&self) -> i32 {
        self.original_width
    }

    /// Set the original width
    pub fn set_original_width(mut self, original_width: i32) -> Self {
        self.original_width = original_width;
        self
    }

    /// Get the original height
    pub fn original_height(&self) -> i32 {
        self.original_height
    }

    /// Set the original height
    pub fn set_original_height(mut self, original_height: i32) -> Self {
        self.original_height = original_height;
        self
    }

    /// Check if the cut is horizontal
    pub fn is_horizontal(&self) -> bool {
        self.is_horizontal
    }

    /// Set whether the cut is horizontal
    pub fn set_horizontal(mut self, is_horizontal: bool) -> Self {
        self.is_horizontal = is_horizontal;
        self
    }

    /// Get the cut coordinate
    pub fn cut_coord(&self) -> i32 {
        self.cut_coord
    }

    /// Set the cut coordinate (fixed method name from original Java "setCutCoords")
    pub fn set_cut_coord(mut self, cut_coord: i32) -> Self {
        self.cut_coord = cut_coord;
        self
    }

    /// Get the original tile ID
    pub fn original_tile_id(&self) -> i32 {
        self.original_tile_id
    }

    /// Set the original tile ID
    pub fn set_original_tile_id(mut self, original_tile_id: i32) -> Self {
        self.original_tile_id = original_tile_id;
        self
    }

    /// Get the first child tile ID
    pub fn child1_tile_id(&self) -> i32 {
        self.child1_tile_id
    }

    /// Set the first child tile ID
    pub fn set_child1_tile_id(mut self, child1_tile_id: i32) -> Self {
        self.child1_tile_id = child1_tile_id;
        self
    }

    /// Get the second child tile ID
    pub fn child2_tile_id(&self) -> i32 {
        self.child2_tile_id
    }

    /// Set the second child tile ID
    pub fn set_child2_tile_id(mut self, child2_tile_id: i32) -> Self {
        self.child2_tile_id = child2_tile_id;
        self
    }

    /// Build the Cut instance
    pub fn build(self) -> Cut {
        Cut::from_builder(self)
    }
}

impl Default for Cut {
    fn default() -> Self {
        Self {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            original_width: 0,
            original_height: 0,
            is_horizontal: false,
            cut_coord: 0,
            original_tile_id: 0,
            child1_tile_id: 0,
            child2_tile_id: 0,
        }
    }
}

impl std::fmt::Display for Cut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cut[({}, {}) -> ({}, {}), {}x{}, {} cut at {}, tile {} -> [{}, {}]]",
            self.x1,
            self.y1,
            self.x2,
            self.y2,
            self.original_width,
            self.original_height,
            if self.is_horizontal { "horizontal" } else { "vertical" },
            self.cut_coord,
            self.original_tile_id,
            self.child1_tile_id,
            self.child2_tile_id
        )
    }
}
