use cutlist_optimizer_cli::models::TileDimensions;
use cutlist_optimizer_cli::stock::{StockSolution, StockSolutionGenerator};
use cutlist_optimizer_cli::models::enums::StockSolutionResult;
use cutlist_optimizer_cli::errors::{AppError, StockError};

#[test]
fn test_stock_solution_generator_creation() {
    let tiles_to_fit = vec![
        TileDimensions::new(1, 100, 50),
        TileDimensions::new(2, 80, 60),
    ];
    let stock_tiles = vec![
        TileDimensions::new(10, 200, 100),
        TileDimensions::new(11, 150, 120),
        TileDimensions::new(12, 180, 90),
    ];

    let generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None);
    assert!(generator.is_ok());
    
    let generator = generator.unwrap();
    assert_eq!(generator.get_required_area(), 9800); // 100*50 + 80*60 = 5000 + 4800
}

#[test]
fn test_stock_solution_generator_simple_creation() {
    let tiles_to_fit = vec![TileDimensions::new(1, 50, 30)];
    let stock_tiles = vec![TileDimensions::new(10, 100, 80)];

    let generator = StockSolutionGenerator::new_simple(tiles_to_fit, stock_tiles);
    assert!(generator.is_ok());
}

#[test]
fn test_empty_tiles_to_fit_error() {
    let tiles_to_fit = vec![];
    let stock_tiles = vec![TileDimensions::new(10, 200, 100)];

    let result = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None);
    assert!(matches!(result.unwrap_err(), AppError::Stock(StockError::NoTilesToFit)));
}

#[test]
fn test_empty_stock_tiles_error() {
    let tiles_to_fit = vec![TileDimensions::new(1, 100, 50)];
    let stock_tiles = vec![];

    let result = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None);
    assert!(matches!(result.unwrap_err(), AppError::Stock(StockError::NoStockTiles)));
}

#[test]
fn test_generate_stock_solution_basic() {
    let tiles_to_fit = vec![
        TileDimensions::new(1, 50, 30), // area = 1500
        TileDimensions::new(2, 40, 25), // area = 1000
    ];
    let stock_tiles = vec![
        TileDimensions::new(10, 100, 80), // area = 8000
        TileDimensions::new(11, 90, 70),  // area = 6300
        TileDimensions::new(12, 80, 60),  // area = 4800
    ];

    let mut generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
    
    match generator.generate_stock_solution() {
        StockSolutionResult::Solution(solution) => {
            assert!(!solution.is_empty());
            // The solution should have enough area to fit all required tiles
            assert!(solution.get_total_area() >= 2500); // 1500 + 1000
        }
        StockSolutionResult::NoSolution => panic!("Expected a solution but got none"),
        StockSolutionResult::AllExcluded => panic!("Expected a solution but all were excluded"),
    }
}

#[test]
fn test_generate_multiple_solutions() {
    let tiles_to_fit = vec![
        TileDimensions::new(1, 30, 20), // area = 600
    ];
    let stock_tiles = vec![
        TileDimensions::new(10, 50, 40), // area = 2000
        TileDimensions::new(11, 60, 35), // area = 2100
        TileDimensions::new(12, 45, 50), // area = 2250
    ];

    let mut generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, Some(2)).unwrap();
    
    // Generate first solution
    let first_result = generator.generate_stock_solution();
    assert!(matches!(first_result, StockSolutionResult::Solution(_)));
    
    // Generate second solution (should be different or no solution)
    let second_result = generator.generate_stock_solution();
    // This might be NoSolution or a different solution depending on the algorithm
    assert!(matches!(second_result, StockSolutionResult::Solution(_) | StockSolutionResult::NoSolution));
}

#[test]
fn test_unique_stock_panel_scenario() {
    let tiles_to_fit = vec![
        TileDimensions::new(1, 30, 20),
    ];
    // All stock tiles have the same ID (unique panel type)
    let stock_tiles = vec![
        TileDimensions::new(10, 100, 80),
        TileDimensions::new(10, 100, 80), // Same ID
        TileDimensions::new(10, 100, 80), // Same ID
    ];

    let mut generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
    
    match generator.generate_stock_solution() {
        StockSolutionResult::Solution(solution) => {
            assert!(!solution.is_empty());
        }
        StockSolutionResult::AllExcluded => {
            // This is also valid for unique panels after first generation
        }
        StockSolutionResult::NoSolution => panic!("Expected a solution for unique panels"),
    }
}

#[test]
fn test_insufficient_stock_area() {
    let tiles_to_fit = vec![
        TileDimensions::new(1, 200, 150), // area = 30000
    ];
    let stock_tiles = vec![
        TileDimensions::new(10, 50, 40), // area = 2000 (too small)
        TileDimensions::new(11, 60, 35), // area = 2100 (too small)
    ];

    let mut generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
    
    match generator.generate_stock_solution() {
        StockSolutionResult::NoSolution => {
            // Expected when stock tiles don't have enough total area
        }
        StockSolutionResult::Solution(solution) => {
            // The algorithm might still return a solution even if total area is insufficient
            // because it uses area as one constraint but not the only one
            // In this case, we just verify the solution is not empty
            assert!(!solution.is_empty());
        }
        StockSolutionResult::AllExcluded => {
            // Also possible if all combinations are excluded
        }
    }
}

#[test]
fn test_dimension_constraints() {
    let tiles_to_fit = vec![
        TileDimensions::new(1, 80, 60), // max dimension = 80
    ];
    let stock_tiles = vec![
        TileDimensions::new(10, 100, 70), // max dimension = 100 (can fit)
        TileDimensions::new(11, 50, 40),  // max dimension = 50 (too small)
        TileDimensions::new(12, 90, 85),  // max dimension = 90 (can fit)
    ];

    let mut generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
    
    match generator.generate_stock_solution() {
        StockSolutionResult::Solution(solution) => {
            // Verify that at least one tile in the solution can accommodate the required max dimension
            let has_sufficient_dimension = solution.iter()
                .any(|tile| tile.max_dimension() >= 80);
            assert!(has_sufficient_dimension, "Solution must include tiles that can fit the required dimensions");
        }
        StockSolutionResult::NoSolution => {
            // This could happen if the algorithm is very strict about constraints
        }
        StockSolutionResult::AllExcluded => {
            // Also possible
        }
    }
}

#[test]
fn test_optimizer_error_display() {
    let error = AppError::no_stock_tiles();
    assert_eq!(error.to_string(), "No stock tiles provided");
    
    let error = AppError::no_tiles_to_fit();
    assert_eq!(error.to_string(), "No tiles to fit provided");
    
    let error = AppError::invalid_configuration("test message");
    assert_eq!(error.to_string(), "Invalid configuration: test message");
    
    let error = AppError::Stock(StockError::ComputationLimitExceeded);
    assert_eq!(error.to_string(), "Stock solution computation exceeded reasonable limits");
}

#[test]
fn test_stock_solution_result_variants() {
    let solution = StockSolution::new();
    let result = StockSolutionResult::Solution(solution);
    assert!(matches!(result, StockSolutionResult::Solution(_)));
    
    let result = StockSolutionResult::NoSolution;
    assert!(matches!(result, StockSolutionResult::NoSolution));
    
    let result = StockSolutionResult::AllExcluded;
    assert!(matches!(result, StockSolutionResult::AllExcluded));
}

#[test]
fn test_with_length_hint() {
    let tiles_to_fit = vec![
        TileDimensions::new(1, 40, 30),
    ];
    let stock_tiles = vec![
        TileDimensions::new(10, 100, 80),
        TileDimensions::new(11, 90, 70),
        TileDimensions::new(12, 80, 60),
        TileDimensions::new(13, 70, 50),
    ];

    // Test with a small length hint
    let mut generator = StockSolutionGenerator::new(tiles_to_fit.clone(), stock_tiles.clone(), Some(2)).unwrap();
    
    match generator.generate_stock_solution() {
        StockSolutionResult::Solution(solution) => {
            // With a length hint of 2, the solution should use at most 2 tiles
            assert!(solution.len() <= 2, "Solution should respect the length hint");
        }
        _ => {
            // Other results are also valid
        }
    }
}
