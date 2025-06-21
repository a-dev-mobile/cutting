//! CalculationResponse implementation methods

use super::CalculationResponse;
use crate::models::{CalculationRequest, FinalTile, Mosaic, NoFitTile};
use std::collections::HashMap;

impl CalculationResponse {
    pub fn default() -> Self {
        Self {
            version: "1.2".to_string(),
            edge_bands: None,
            elapsed_time: 0,
            id: None,
            panels: None,
            request: None,
            solution_elapsed_time: None,
            task_id: None,
            total_cut_length: 0.0,
            total_nbr_cuts: 0,
            total_used_area: 0.0,
            total_used_area_ratio: 0.0,
            total_wasted_area: 0.0,
            used_stock_panels: None,
            no_fit_panels: Vec::new(),
            mosaics: Vec::new(),
        }
    }

    /// Create a new CalculationResponse with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new CalculationResponse with a specific ID
    pub fn with_id(id: String) -> Self {
        Self {
            id: Some(id),
            ..Self::default()
        }
    }

    /// Get the version string
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the calculation ID
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Set the calculation ID
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Get the task ID
    pub fn task_id(&self) -> Option<&str> {
        self.task_id.as_deref()
    }

    /// Set the task ID
    pub fn set_task_id(&mut self, task_id: String) {
        self.task_id = Some(task_id);
    }

    /// Get the elapsed time in milliseconds
    pub fn elapsed_time(&self) -> u64 {
        self.elapsed_time
    }

    /// Set the elapsed time in milliseconds
    pub fn set_elapsed_time(&mut self, elapsed_time: u64) {
        self.elapsed_time = elapsed_time;
    }

    /// Get the solution elapsed time in milliseconds
    pub fn solution_elapsed_time(&self) -> Option<u64> {
        self.solution_elapsed_time
    }

    /// Set the solution elapsed time in milliseconds
    pub fn set_solution_elapsed_time(&mut self, solution_elapsed_time: u64) {
        self.solution_elapsed_time = Some(solution_elapsed_time);
    }

    /// Get the total used area
    pub fn total_used_area(&self) -> f64 {
        self.total_used_area
    }

    /// Set the total used area
    pub fn set_total_used_area(&mut self, total_used_area: f64) {
        self.total_used_area = total_used_area;
    }

    /// Get the total wasted area
    pub fn total_wasted_area(&self) -> f64 {
        self.total_wasted_area
    }

    /// Set the total wasted area
    pub fn set_total_wasted_area(&mut self, total_wasted_area: f64) {
        self.total_wasted_area = total_wasted_area;
    }

    /// Get the total used area ratio
    pub fn total_used_area_ratio(&self) -> f64 {
        self.total_used_area_ratio
    }

    /// Set the total used area ratio
    pub fn set_total_used_area_ratio(&mut self, total_used_area_ratio: f64) {
        self.total_used_area_ratio = total_used_area_ratio;
    }

    /// Get the total number of cuts
    pub fn total_nbr_cuts(&self) -> u64 {
        self.total_nbr_cuts
    }

    /// Set the total number of cuts
    pub fn set_total_nbr_cuts(&mut self, total_nbr_cuts: u64) {
        self.total_nbr_cuts = total_nbr_cuts;
    }

    /// Get the total cut length
    pub fn total_cut_length(&self) -> f64 {
        self.total_cut_length
    }

    /// Set the total cut length
    pub fn set_total_cut_length(&mut self, total_cut_length: f64) {
        self.total_cut_length = total_cut_length;
    }

    /// Get a reference to the calculation request
    pub fn request(&self) -> Option<&CalculationRequest> {
        self.request.as_ref()
    }

    /// Set the calculation request
    pub fn set_request(&mut self, request: CalculationRequest) {
        self.request = Some(request);
    }

    /// Get a reference to the panels list
    pub fn panels(&self) -> Option<&Vec<FinalTile>> {
        self.panels.as_ref()
    }

    /// Get a mutable reference to the panels list
    pub fn panels_mut(&mut self) -> &mut Option<Vec<FinalTile>> {
        &mut self.panels
    }

    /// Set the panels list
    pub fn set_panels(&mut self, panels: Vec<FinalTile>) {
        self.panels = Some(panels);
    }

    /// Get a reference to the used stock panels list
    pub fn used_stock_panels(&self) -> Option<&Vec<FinalTile>> {
        self.used_stock_panels.as_ref()
    }

    /// Get a mutable reference to the used stock panels list
    pub fn used_stock_panels_mut(&mut self) -> &mut Option<Vec<FinalTile>> {
        &mut self.used_stock_panels
    }

    /// Set the used stock panels list
    pub fn set_used_stock_panels(&mut self, used_stock_panels: Vec<FinalTile>) {
        self.used_stock_panels = Some(used_stock_panels);
    }

    /// Get a reference to the edge bands map
    pub fn edge_bands(&self) -> Option<&HashMap<String, f64>> {
        self.edge_bands.as_ref()
    }

    /// Get a mutable reference to the edge bands map
    pub fn edge_bands_mut(&mut self) -> &mut Option<HashMap<String, f64>> {
        &mut self.edge_bands
    }

    /// Set the edge bands map
    pub fn set_edge_bands(&mut self, edge_bands: HashMap<String, f64>) {
        self.edge_bands = Some(edge_bands);
    }

    /// Get a reference to the no-fit panels list
    pub fn no_fit_panels(&self) -> &Vec<NoFitTile> {
        &self.no_fit_panels
    }

    /// Get a mutable reference to the no-fit panels list
    pub fn no_fit_panels_mut(&mut self) -> &mut Vec<NoFitTile> {
        &mut self.no_fit_panels
    }

    /// Set the no-fit panels list
    pub fn set_no_fit_panels(&mut self, no_fit_panels: Vec<NoFitTile>) {
        self.no_fit_panels = no_fit_panels;
    }

    /// Get a reference to the mosaics list
    pub fn mosaics(&self) -> &Vec<Mosaic> {
        &self.mosaics
    }

    /// Get a mutable reference to the mosaics list
    pub fn mosaics_mut(&mut self) -> &mut Vec<Mosaic> {
        &mut self.mosaics
    }

    /// Set the mosaics list
    pub fn set_mosaics(&mut self, mosaics: Vec<Mosaic>) {
        self.mosaics = mosaics;
    }

    /// Add a panel to the no-fit panels list
    pub fn add_no_fit_panel(&mut self, panel: NoFitTile) {
        self.no_fit_panels.push(panel);
    }

    /// Add a mosaic to the mosaics list
    pub fn add_mosaic(&mut self, mosaic: Mosaic) {
        self.mosaics.push(mosaic);
    }

    /// Clear all no-fit panels
    pub fn clear_no_fit_panels(&mut self) {
        self.no_fit_panels.clear();
    }

    /// Clear all mosaics
    pub fn clear_mosaics(&mut self) {
        self.mosaics.clear();
    }
}
