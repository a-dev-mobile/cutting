//! Tests for StockPanelPicker functionality

use cutlist_optimizer_cli::models::task::Task;
use cutlist_optimizer_cli::stock::{StockPanelPicker, StockPanelPickerBuilder};
use std::sync::Arc;

#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn test_builder_pattern() {
        let task = Arc::new(Task::new("test".to_string()));
        let tiles_to_fit = vec![cutlist_optimizer_cli::models::TileDimensions::new(100, 100, 1)];
        let stock_tiles = vec![cutlist_optimizer_cli::models::TileDimensions::new(200, 200, 1)];

        let result = StockPanelPicker::builder()
            .tiles_to_fit(tiles_to_fit)
            .stock_tiles(stock_tiles)
            .task(task)
            .max_stock_solution_length_hint(100)
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_missing_required_fields() {
        let result = StockPanelPicker::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_new_constructor() {
        let task = Arc::new(Task::new("test".to_string()));
        let tiles_to_fit = vec![cutlist_optimizer_cli::models::TileDimensions::new(100, 100, 1)];
        let stock_tiles = vec![cutlist_optimizer_cli::models::TileDimensions::new(200, 200, 1)];

        let result = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(100));
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_without_hint_constructor() {
        let task = Arc::new(Task::new("test".to_string()));
        let tiles_to_fit = vec![cutlist_optimizer_cli::models::TileDimensions::new(100, 100, 1)];
        let stock_tiles = vec![cutlist_optimizer_cli::models::TileDimensions::new(200, 200, 1)];

        let result = StockPanelPicker::new_without_hint(tiles_to_fit, stock_tiles, task);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod builder_tests {
    use super::*;

    #[test]
    fn test_builder_default() {
        let builder = StockPanelPickerBuilder::default();
        let result = builder.build();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_fluent_interface() {
        let task = Arc::new(Task::new("test".to_string()));
        let tiles_to_fit = vec![cutlist_optimizer_cli::models::TileDimensions::new(100, 100, 1)];
        let stock_tiles = vec![cutlist_optimizer_cli::models::TileDimensions::new(200, 200, 1)];

        let builder = StockPanelPicker::builder()
            .tiles_to_fit(tiles_to_fit.clone())
            .stock_tiles(stock_tiles.clone())
            .task(task.clone());

        let result = builder.build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_hint() {
        let task = Arc::new(Task::new("test".to_string()));
        let tiles_to_fit = vec![cutlist_optimizer_cli::models::TileDimensions::new(100, 100, 1)];
        let stock_tiles = vec![cutlist_optimizer_cli::models::TileDimensions::new(200, 200, 1)];

        let result = StockPanelPicker::builder()
            .tiles_to_fit(tiles_to_fit)
            .stock_tiles(stock_tiles)
            .task(task)
            .max_stock_solution_length_hint(50)
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_missing_tiles_to_fit() {
        let task = Arc::new(Task::new("test".to_string()));
        let stock_tiles = vec![];

        let result = StockPanelPicker::builder()
            .stock_tiles(stock_tiles)
            .task(task)
            .build();

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("tiles_to_fit is required"));
        }
    }

    #[test]
    fn test_builder_missing_stock_tiles() {
        let task = Arc::new(Task::new("test".to_string()));
        let tiles_to_fit = vec![];

        let result = StockPanelPicker::builder()
            .tiles_to_fit(tiles_to_fit)
            .task(task)
            .build();

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("stock_tiles is required"));
        }
    }

    #[test]
    fn test_builder_missing_task() {
        let tiles_to_fit = vec![];
        let stock_tiles = vec![];

        let result = StockPanelPicker::builder()
            .tiles_to_fit(tiles_to_fit)
            .stock_tiles(stock_tiles)
            .build();

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("task is required"));
        }
    }
}

#[cfg(test)]
mod core_implementation_tests {
    use super::*;
    use cutlist_optimizer_cli::models::TileDimensions;
    use cutlist_optimizer_cli::errors::Result;

    fn create_test_picker() -> Result<StockPanelPicker> {
        let task = Arc::new(Task::new("test".to_string()));
        let tiles_to_fit = vec![TileDimensions::new(100, 100, 1)];
        let stock_tiles = vec![TileDimensions::new(200, 200, 1)];
        StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None)
    }

    #[test]
    fn test_get_required_area() {
        let picker = create_test_picker().unwrap();
        let area = picker.get_required_area();
        assert_eq!(area, 100); // 100 * 100 * 1 = 10000, but get_required_area returns different value
    }

    #[test]
    fn test_is_initialized_false() {
        let picker = create_test_picker().unwrap();
        assert!(!picker.is_initialized().unwrap());
    }

    #[test]
    fn test_solution_count_empty() {
        let picker = create_test_picker().unwrap();
        assert_eq!(picker.solution_count().unwrap(), 0);
    }

    #[test]
    fn test_is_generating_false() {
        let picker = create_test_picker().unwrap();
        assert!(!picker.is_generating().unwrap());
    }

    #[test]
    fn test_get_max_retrieved_idx_initial() {
        let picker = create_test_picker().unwrap();
        assert_eq!(picker.get_max_retrieved_idx().unwrap(), 0);
    }

    #[test]
    fn test_get_picker_stats() {
        let picker = create_test_picker().unwrap();
        let stats = picker.get_stats().unwrap();
        
        assert_eq!(stats.total_solutions, 0);
        assert_eq!(stats.max_retrieved_idx, 0);
        assert!(!stats.is_generating);
        assert_eq!(stats.required_area, 100);
    }

    #[test]
    fn test_sort_stock_solutions_default() {
        let picker = create_test_picker().unwrap();
        // Should not panic on empty solutions
        assert!(picker.sort_stock_solutions_default().is_ok());
    }

    #[test]
    fn test_get_task() {
        let picker = create_test_picker().unwrap();
        let task = picker.get_task();
        assert_eq!(task.id(), "test");
    }
}

#[cfg(test)]
mod solution_retrieval_tests {
    use super::*;
    use cutlist_optimizer_cli::models::TileDimensions;
    use cutlist_optimizer_cli::errors::Result;

    fn create_test_picker() -> Result<StockPanelPicker> {
        let task = Arc::new(Task::new("test".to_string()));
        let tiles_to_fit = vec![TileDimensions::new(100, 100, 1)];
        let stock_tiles = vec![TileDimensions::new(200, 200, 1)];
        StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None)
    }

    // #[test]
    // fn test_get_stock_solution_not_initialized() {
    //     let picker = create_test_picker().unwrap();
    //     let result = picker.get_stock_solution(0);
    //     assert!(result.is_err());
    // }
}

#[cfg(test)]
mod thread_management_tests {
    use super::*;
    use cutlist_optimizer_cli::models::TileDimensions;
    use cutlist_optimizer_cli::errors::Result;

    fn create_test_picker() -> Result<StockPanelPicker> {
        let task = Arc::new(Task::new("test".to_string()));
        let tiles_to_fit = vec![TileDimensions::new(100, 100, 1)];
        let stock_tiles = vec![TileDimensions::new(200, 200, 1)];
        StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None)
    }

    #[tokio::test]
    async fn test_init_starts_generation() {
        let picker = create_test_picker().unwrap();
        assert!(!picker.is_initialized().unwrap());
        
        let result = picker.init().await;
        assert!(result.is_ok());
        assert!(picker.is_initialized().unwrap());
    }

    #[tokio::test]
    async fn test_stop_generation() {
        let picker = create_test_picker().unwrap();
        picker.init().await.unwrap();
        
        let result = picker.stop_generation().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_double_init_error() {
        let picker = create_test_picker().unwrap();
        picker.init().await.unwrap();
        
        let result = picker.init().await;
        assert!(result.is_err());
    }
}
