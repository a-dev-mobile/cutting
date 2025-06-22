//! Constructors and builder pattern implementation for StockPanelPicker

use std::sync::{Arc, Mutex};
use crate::models::{task::Task, TileDimensions};
use super::super::StockSolutionGenerator;
use crate::errors::{AppError, Result};
use super::{StockPanelPicker, StockPanelPickerBuilder};

impl StockPanelPicker {
    /// Create a new StockPanelPicker with tiles to fit, stock tiles, task, and optional max length hint
    /// 
    /// This corresponds to the Java constructor:
    /// `StockPanelPicker(List<TileDimensions> list, List<TileDimensions> list2, Task task, Integer num)`
    pub fn new(
        tiles_to_fit: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
        task: Arc<Task>,
        max_stock_solution_length_hint: Option<usize>,
    ) -> Result<Self> {
        let stock_solution_generator = StockSolutionGenerator::new(
            tiles_to_fit,
            stock_tiles,
            max_stock_solution_length_hint,
        )?;

        Ok(Self {
            stock_solution_generator,
            task,
            stock_solutions: Arc::new(Mutex::new(Vec::new())),
            max_retrieved_idx: Arc::new(Mutex::new(0)),
            generation_thread: Arc::new(Mutex::new(None)),
            shutdown_sender: Arc::new(Mutex::new(None)),
        })
    }

    /// Create a new StockPanelPicker without max length hint
    /// 
    /// This corresponds to the Java constructor:
    /// `StockPanelPicker(List<TileDimensions> list, List<TileDimensions> list2, Task task)`
    pub fn new_without_hint(
        tiles_to_fit: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
        task: Arc<Task>,
    ) -> Result<Self> {
        Self::new(tiles_to_fit, stock_tiles, task, None)
    }

    /// Create a builder for StockPanelPicker
    pub fn builder() -> StockPanelPickerBuilder {
        StockPanelPickerBuilder::new()
    }
}

impl StockPanelPickerBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            tiles_to_fit: None,
            stock_tiles: None,
            task: None,
            max_stock_solution_length_hint: None,
        }
    }

    /// Set the tiles to fit
    pub fn tiles_to_fit(mut self, tiles: Vec<TileDimensions>) -> Self {
        self.tiles_to_fit = Some(tiles);
        self
    }

    /// Set the stock tiles
    pub fn stock_tiles(mut self, tiles: Vec<TileDimensions>) -> Self {
        self.stock_tiles = Some(tiles);
        self
    }

    /// Set the task
    pub fn task(mut self, task: Arc<Task>) -> Self {
        self.task = Some(task);
        self
    }

    /// Set the maximum stock solution length hint
    pub fn max_stock_solution_length_hint(mut self, hint: usize) -> Self {
        self.max_stock_solution_length_hint = Some(hint);
        self
    }

    /// Build the StockPanelPicker
    pub fn build(self) -> Result<StockPanelPicker> {
        let tiles_to_fit = self.tiles_to_fit.ok_or_else(|| {
            AppError::invalid_input("tiles_to_fit is required")
        })?;

        let stock_tiles = self.stock_tiles.ok_or_else(|| {
            AppError::invalid_input("stock_tiles is required")
        })?;

        let task = self.task.ok_or_else(|| {
            AppError::invalid_input("task is required")
        })?;

        StockPanelPicker::new(
            tiles_to_fit,
            stock_tiles,
            task,
            self.max_stock_solution_length_hint,
        )
    }
}
