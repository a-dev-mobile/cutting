use serde::{Deserialize, Serialize};
use std::fmt;
use crate::models::{edge::Edge, DEFAULT_MATERIAL};


/// Represents a panel with dimensions, material, and configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Panel {
    pub id: i32,
    pub width: Option<String>,
    pub height: Option<String>,
    pub count: i32,
    pub material: String,
    pub enabled: bool,
    pub orientation: i32,
    pub label: Option<String>,
    pub edge: Option<Edge>,
}

impl Default for Panel {
    fn default() -> Self {
        Self {
            id: 0,
            width: None,
            height: None,
            count: 0,
            material: DEFAULT_MATERIAL.to_string(), // Using same default as TileDimensions
            enabled: false,
            orientation: 0,
            label: None,
            edge: None,
        }
    }
}

impl fmt::Display for Panel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.width.as_deref().unwrap_or("?");
        let height = self.height.as_deref().unwrap_or("?");
        let disabled_suffix = if self.enabled { "" } else { "-disabled" };
        
        write!(f, "[{}x{}]*{}{}", width, height, self.count, disabled_suffix)
    }
}
