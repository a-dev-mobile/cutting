use serde::{Deserialize, Serialize};
use crate::models::{Configuration, Panel};

/// Request structure for cutting calculations containing configuration and panel data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationRequest {
    /// Configuration parameters for the optimization process
    pub configuration: Option<Configuration>,
    
    /// List of panels to be cut
    pub panels: Vec<Panel>,
    
    /// List of available stock panels
    pub stock_panels: Vec<Panel>,
}

