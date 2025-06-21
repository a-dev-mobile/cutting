//! CalculationResponse structure definition

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::{CalculationRequest, FinalTile, NoFitTile, Mosaic};

/// Response structure containing the results of a cutting calculation
/// 
/// This structure represents the complete output of a cutting optimization process,
/// including timing information, calculated metrics, and the resulting layout solutions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResponse {
    /// Static version identifier for the calculation engine
    pub version: String,
    
    /// Edge band usage by type/material
    pub edge_bands: Option<HashMap<String, f64>>,
    
    /// Total elapsed time for the calculation in milliseconds
    pub elapsed_time: u64,
    
    /// Unique identifier for this calculation response
    pub id: Option<String>,
    
    /// List of panels in the final solution
    pub panels: Option<Vec<FinalTile>>,
    
    /// Reference to the original calculation request
    pub request: Option<CalculationRequest>,
    
    /// Time spent on solution calculation in milliseconds
    pub solution_elapsed_time: Option<u64>,
    
    /// Task identifier for tracking purposes
    pub task_id: Option<String>,
    
    /// Total length of all cuts made
    pub total_cut_length: f64,
    
    /// Total number of cuts performed
    pub total_nbr_cuts: u64,
    
    /// Total area of material used
    pub total_used_area: f64,
    
    /// Ratio of used area to total available area
    pub total_used_area_ratio: f64,
    
    /// Total area of material wasted
    pub total_wasted_area: f64,
    
    /// List of stock panels that were used in the solution
    pub used_stock_panels: Option<Vec<FinalTile>>,
    
    /// List of panels that could not be fit in the solution
    pub no_fit_panels: Vec<NoFitTile>,
    
    /// List of cutting mosaics representing the complete solutions
    pub mosaics: Vec<Mosaic>,
}

