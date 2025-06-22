//! Request validation utilities

use crate::models::{CalculationRequest, enums::StatusCode};
use super::core::{MAX_PANELS_LIMIT, MAX_STOCK_PANELS_LIMIT};

pub struct RequestValidator;

impl RequestValidator {
    /// Validate a calculation request (migrated from Java)
    pub async fn validate_request(request: &CalculationRequest) -> Option<StatusCode> {
        // Count valid panels
        let panel_count: usize = request.panels.iter()
            .filter(|p| p.is_valid().unwrap_or(false))
            .map(|p| p.count as usize)
            .sum();

        if panel_count == 0 {
            return Some(StatusCode::InvalidTiles);
        }

        if panel_count > MAX_PANELS_LIMIT {
            return Some(StatusCode::TooManyPanels);
        }

        // Count valid stock panels
        let stock_count: usize = request.stock_panels.iter()
            .filter(|p| p.is_valid().unwrap_or(false))
            .map(|p| p.count as usize)
            .sum();

        if stock_count == 0 {
            return Some(StatusCode::InvalidStockTiles);
        }

        if stock_count > MAX_STOCK_PANELS_LIMIT {
            return Some(StatusCode::TooManyStockPanels);
        }

        None // Request is valid
    }
}
