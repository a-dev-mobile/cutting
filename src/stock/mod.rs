
pub mod constants;
pub mod stock_solution;
pub mod stock_solution_generator;
pub mod stock_panel_picker;

// Export main types to avoid ambiguous glob re-exports
pub use constants::StockConstants;
pub use stock_solution::{StockSolution};
pub use stock_solution_generator::{StockSolutionGenerator};
pub use stock_panel_picker::{StockPanelPicker, StockPanelPickerBuilder, StockPanelPickerStats, SolutionSortConfig};
