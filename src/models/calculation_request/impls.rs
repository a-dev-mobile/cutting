use super::structs::CalculationRequest;
use crate::models::{Configuration, Panel};

impl CalculationRequest {
    fn default() -> Self {
        Self {
            configuration: None,
            panels: Vec::new(),
            stock_panels: Vec::new(),
        }
    }

    /// Creates a new empty CalculationRequest
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new CalculationRequest with the given configuration
    pub fn with_configuration(configuration: Configuration) -> Self {
        Self {
            configuration: Some(configuration),
            panels: Vec::new(),
            stock_panels: Vec::new(),
        }
    }

    /// Gets a reference to the configuration
    pub fn configuration(&self) -> Option<&Configuration> {
        self.configuration.as_ref()
    }

    /// Sets the configuration
    pub fn set_configuration(&mut self, configuration: Configuration) {
        self.configuration = Some(configuration);
    }

    /// Takes ownership of the configuration, leaving None in its place
    pub fn take_configuration(&mut self) -> Option<Configuration> {
        self.configuration.take()
    }

    /// Gets a reference to the panels list
    pub fn panels(&self) -> &[Panel] {
        &self.panels
    }

    /// Gets a mutable reference to the panels list
    pub fn panels_mut(&mut self) -> &mut Vec<Panel> {
        &mut self.panels
    }

    /// Sets the panels list
    pub fn set_panels(&mut self, panels: Vec<Panel>) {
        self.panels = panels;
    }

    /// Adds a panel to the panels list
    pub fn add_panel(&mut self, panel: Panel) {
        self.panels.push(panel);
    }

    /// Gets a reference to the stock panels list
    pub fn stock_panels(&self) -> &[Panel] {
        &self.stock_panels
    }

    /// Gets a mutable reference to the stock panels list
    pub fn stock_panels_mut(&mut self) -> &mut Vec<Panel> {
        &mut self.stock_panels
    }

    /// Sets the stock panels list
    pub fn set_stock_panels(&mut self, stock_panels: Vec<Panel>) {
        self.stock_panels = stock_panels;
    }

    /// Adds a stock panel to the stock panels list
    pub fn add_stock_panel(&mut self, panel: Panel) {
        self.stock_panels.push(panel);
    }

    /// Converts panels with count > 0 to a string representation
    pub fn tiles_to_string(&self) -> String {
        self.panels
            .iter()
            .filter(|panel| panel.count > 0)
            .map(|panel| format!(" {}", panel))
            .collect::<String>()
    }

    /// Converts stock panels with count > 0 to a string representation
    pub fn base_tiles_to_string(&self) -> String {
        self.stock_panels
            .iter()
            .filter(|panel| panel.count > 0)
            .map(|panel| format!(" {}", panel))
            .collect::<String>()
    }
}
