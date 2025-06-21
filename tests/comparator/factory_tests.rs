//! Tests for solution comparator and priority list factories
//! 
//! This module contains comprehensive tests for the factory classes that
//! convert Java-style string-based comparator selection to Rust's type-safe approach.

use cutlist_optimizer_cli::comparator::{
    SolutionComparatorFactory, 
    PriorityListFactory, 
    SolutionComparator,
    ComparatorFactoryError
};
use cutlist_optimizer_cli::models::configuration::Configuration;
use cutlist_optimizer_cli::models::enums::OptimizationPriority;
use cutlist_optimizer_cli::models::performance_thresholds::PerformanceThresholds;

/// Helper function to create a test configuration
fn create_test_configuration(optimization_priority: OptimizationPriority) -> Configuration {
    Configuration {
        cut_thickness: 3,
        min_trim_dimension: 10,
        consider_orientation: true,
        optimization_factor: 5,
        optimization_priority,
        use_single_stock_unit: false,
        units: "mm".to_string(),
        performance_thresholds: PerformanceThresholds::default(),
    }
}

#[cfg(test)]
mod solution_comparator_factory_tests {
    use super::*;

    #[test]
    fn test_get_solution_comparator_valid_priorities() {
        let test_cases = vec![
            ("MOST_TILES", true),
            ("LEAST_WASTED_AREA", true),
            ("LEAST_NBR_CUTS", true),
            ("MOST_HV_DISCREPANCY", true),
            ("BIGGEST_UNUSED_TILE_AREA", true),
            ("SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN", true),
            ("LEAST_NBR_MOSAICS", true),
            ("LEAST_NBR_UNUSED_TILES", true),
            ("MOST_UNUSED_PANEL_AREA", true),
        ];

        for (priority_str, should_succeed) in test_cases {
            let result = SolutionComparatorFactory::get_solution_comparator(Some(priority_str));
            
            if should_succeed {
                assert!(result.is_ok(), "Failed for priority: {}", priority_str);
                assert!(result.unwrap().is_some(), "Expected Some for priority: {}", priority_str);
            } else {
                assert!(result.is_err(), "Expected error for priority: {}", priority_str);
            }
        }
    }

    #[test]
    fn test_get_solution_comparator_case_insensitive() {
        let test_cases = vec![
            "most_tiles",
            "Most_Tiles", 
            "MOST_TILES",
            "least_wasted_area",
            "Least_Wasted_Area",
            "LEAST_WASTED_AREA",
        ];

        for priority_str in test_cases {
            let result = SolutionComparatorFactory::get_solution_comparator(Some(priority_str));
            assert!(result.is_ok(), "Failed for case variation: {}", priority_str);
            assert!(result.unwrap().is_some(), "Expected Some for case variation: {}", priority_str);
        }
    }

    #[test]
    fn test_get_solution_comparator_none_input() {
        let result = SolutionComparatorFactory::get_solution_comparator(None);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_get_solution_comparator_empty_string() {
        let result = SolutionComparatorFactory::get_solution_comparator(Some(""));
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_get_solution_comparator_whitespace_only() {
        let result = SolutionComparatorFactory::get_solution_comparator(Some("   "));
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_get_solution_comparator_invalid_priority() {
        let invalid_priorities = vec![
            "INVALID_PRIORITY",
            "UNKNOWN",
            "RANDOM_STRING",
            "123",
            "",
        ];

        for priority_str in invalid_priorities {
            if priority_str.is_empty() {
                // Empty string should return None, not error
                let result = SolutionComparatorFactory::get_solution_comparator(Some(priority_str));
                assert!(result.is_ok());
                assert!(result.unwrap().is_none());
            } else {
                let result = SolutionComparatorFactory::get_solution_comparator(Some(priority_str));
                assert!(result.is_err(), "Expected error for invalid priority: {}", priority_str);
                assert!(matches!(result.unwrap_err(), ComparatorFactoryError::UnknownPriority(_)));
            }
        }
    }

    #[test]
    fn test_get_solution_comparator_enum_valid() {
        let result = SolutionComparatorFactory::get_solution_comparator_enum(Some("MOST_TILES"));
        assert!(result.is_ok());
        let comparator = result.unwrap().unwrap();
        assert_eq!(comparator, SolutionComparator::MostNbrTiles);
    }

    #[test]
    fn test_get_solution_comparator_list_mixed_valid_invalid() {
        let priorities = vec![
            "MOST_TILES",
            "INVALID_PRIORITY", 
            "LEAST_WASTED_AREA",
            "ANOTHER_INVALID",
            "LEAST_NBR_CUTS"
        ];
        
        let comparators = SolutionComparatorFactory::get_solution_comparator_list(&priorities);
        assert_eq!(comparators.len(), 3); // Only valid ones should be included
    }

    #[test]
    fn test_get_solution_comparator_enum_list() {
        let priorities = vec!["MOST_TILES", "LEAST_WASTED_AREA"];
        let comparators = SolutionComparatorFactory::get_solution_comparator_enum_list(&priorities);
        
        assert_eq!(comparators.len(), 2);
        assert_eq!(comparators[0], SolutionComparator::MostNbrTiles);
        assert_eq!(comparators[1], SolutionComparator::LeastWastedArea);
    }

    #[test]
    fn test_optimization_priority_to_solution_comparator_conversion() {
        let test_cases = vec![
            (OptimizationPriority::MostTiles, SolutionComparator::MostNbrTiles),
            (OptimizationPriority::LeastWastedArea, SolutionComparator::LeastWastedArea),
            (OptimizationPriority::LeastNbrCuts, SolutionComparator::LeastNbrCuts),
            (OptimizationPriority::MostHvDiscrepancy, SolutionComparator::HvDiscrepancy),
            (OptimizationPriority::BiggestUnusedTileArea, SolutionComparator::BiggestUnusedTileArea),
            (OptimizationPriority::SmallestCenterOfMassDistToOrigin, SolutionComparator::SmallestCenterOfMassDistToOrigin),
            (OptimizationPriority::LeastNbrMosaics, SolutionComparator::LeastNbrMosaics),
            (OptimizationPriority::LeastNbrUnusedTiles, SolutionComparator::LeastNbrUnusedTiles),
            (OptimizationPriority::MostUnusedPanelArea, SolutionComparator::MostUnusedPanelArea),
        ];

        for (optimization_priority, expected_comparator) in test_cases {
            let comparator: SolutionComparator = optimization_priority.into();
            assert_eq!(comparator, expected_comparator);
        }
    }

    #[test]
    fn test_comparator_factory_error_display() {
        let error = ComparatorFactoryError::UnknownPriority("INVALID".to_string());
        let error_string = format!("{}", error);
        assert!(error_string.contains("Unknown optimization priority: INVALID"));

        let error = ComparatorFactoryError::InvalidInput;
        let error_string = format!("{}", error);
        assert!(error_string.contains("Invalid input: null or empty string"));
    }
}

#[cfg(test)]
mod priority_list_factory_tests {
    use super::*;

    #[test]
    fn test_most_tiles_priority_configuration() {
        let config = create_test_configuration(OptimizationPriority::MostTiles);
        let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config);
        
        assert_eq!(priorities.len(), 6);
        assert_eq!(priorities[0], "MOST_TILES");
        assert_eq!(priorities[1], "LEAST_WASTED_AREA");
        assert_eq!(priorities[2], "LEAST_NBR_CUTS");
        assert_eq!(priorities[3], "LEAST_NBR_MOSAICS");
        assert_eq!(priorities[4], "BIGGEST_UNUSED_TILE_AREA");
        assert_eq!(priorities[5], "MOST_HV_DISCREPANCY");
    }

    #[test]
    fn test_non_most_tiles_priority_configuration() {
        let test_priorities = vec![
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts,
            OptimizationPriority::MostHvDiscrepancy,
            OptimizationPriority::BiggestUnusedTileArea,
            OptimizationPriority::SmallestCenterOfMassDistToOrigin,
            OptimizationPriority::LeastNbrMosaics,
            OptimizationPriority::LeastNbrUnusedTiles,
            OptimizationPriority::MostUnusedPanelArea,
        ];

        for priority in test_priorities {
            let config = create_test_configuration(priority);
            let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config);
            
            assert_eq!(priorities.len(), 6);
            assert_eq!(priorities[0], "MOST_TILES");
            assert_eq!(priorities[1], "LEAST_NBR_CUTS");
            assert_eq!(priorities[2], "LEAST_WASTED_AREA");
            assert_eq!(priorities[3], "LEAST_NBR_MOSAICS");
            assert_eq!(priorities[4], "BIGGEST_UNUSED_TILE_AREA");
            assert_eq!(priorities[5], "MOST_HV_DISCREPANCY");
        }
    }

    #[test]
    fn test_enum_list_most_tiles() {
        let config = create_test_configuration(OptimizationPriority::MostTiles);
        let comparators = PriorityListFactory::get_final_solution_prioritized_comparator_enum_list(&config);
        
        assert_eq!(comparators.len(), 6);
        assert_eq!(comparators[0], SolutionComparator::MostNbrTiles);
        assert_eq!(comparators[1], SolutionComparator::LeastWastedArea);
        assert_eq!(comparators[2], SolutionComparator::LeastNbrCuts);
        assert_eq!(comparators[3], SolutionComparator::LeastNbrMosaics);
        assert_eq!(comparators[4], SolutionComparator::BiggestUnusedTileArea);
        assert_eq!(comparators[5], SolutionComparator::HvDiscrepancy);
    }

    #[test]
    fn test_enum_list_other_priority() {
        let config = create_test_configuration(OptimizationPriority::LeastNbrCuts);
        let comparators = PriorityListFactory::get_final_solution_prioritized_comparator_enum_list(&config);
        
        assert_eq!(comparators.len(), 6);
        assert_eq!(comparators[0], SolutionComparator::MostNbrTiles);
        assert_eq!(comparators[1], SolutionComparator::LeastNbrCuts);
        assert_eq!(comparators[2], SolutionComparator::LeastWastedArea);
        assert_eq!(comparators[3], SolutionComparator::LeastNbrMosaics);
        assert_eq!(comparators[4], SolutionComparator::BiggestUnusedTileArea);
        assert_eq!(comparators[5], SolutionComparator::HvDiscrepancy);
    }

    #[test]
    fn test_function_list_length_and_validity() {
        let config = create_test_configuration(OptimizationPriority::MostTiles);
        let functions = PriorityListFactory::get_final_solution_prioritized_comparator_functions(&config);
        
        assert_eq!(functions.len(), 6);
        // All functions should be valid (this test just ensures no panics)
        for _function in functions {
            // Functions exist and can be stored
        }
    }

    #[test]
    fn test_custom_priority_list_primary_first() {
        let test_priorities = vec![
            OptimizationPriority::BiggestUnusedTileArea,
            OptimizationPriority::SmallestCenterOfMassDistToOrigin,
            OptimizationPriority::LeastNbrUnusedTiles,
            OptimizationPriority::MostUnusedPanelArea,
        ];

        for primary_priority in test_priorities {
            let priorities = PriorityListFactory::create_custom_priority_list(primary_priority);
            
            assert!(!priorities.is_empty());
            assert_eq!(priorities[0], primary_priority.to_string());
            
            // Ensure no duplicates
            let primary_count = priorities.iter()
                .filter(|&p| p == &primary_priority.to_string())
                .count();
            assert_eq!(primary_count, 1);
        }
    }

    #[test]
    fn test_custom_priority_list_contains_standard_priorities() {
        let primary_priority = OptimizationPriority::BiggestUnusedTileArea;
        let priorities = PriorityListFactory::create_custom_priority_list(primary_priority);
        
        let expected_priorities = vec![
            "MOST_TILES",
            "LEAST_WASTED_AREA", 
            "LEAST_NBR_CUTS",
            "LEAST_NBR_MOSAICS",
            "MOST_HV_DISCREPANCY",
        ];

        for expected in expected_priorities {
            assert!(priorities.contains(&expected.to_string()), 
                   "Missing expected priority: {}", expected);
        }
    }

    #[test]
    fn test_custom_priority_list_no_duplicates_when_primary_is_standard() {
        let primary_priority = OptimizationPriority::MostTiles;
        let priorities = PriorityListFactory::create_custom_priority_list(primary_priority);
        
        // Count occurrences of the primary priority
        let count = priorities.iter()
            .filter(|&p| p == "MOST_TILES")
            .count();
        assert_eq!(count, 1, "Primary priority should appear exactly once");
    }

    #[test]
    fn test_priority_list_consistency_between_string_and_enum() {
        let config = create_test_configuration(OptimizationPriority::LeastWastedArea);
        
        let string_priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config);
        let enum_comparators = PriorityListFactory::get_final_solution_prioritized_comparator_enum_list(&config);
        
        assert_eq!(string_priorities.len(), enum_comparators.len());
        
        for (i, (string_priority, enum_comparator)) in string_priorities.iter()
            .zip(enum_comparators.iter())
            .enumerate() {
            
            // Convert enum back to string and compare
            let enum_as_string = match enum_comparator {
                SolutionComparator::MostNbrTiles => "MOST_TILES",
                SolutionComparator::LeastWastedArea => "LEAST_WASTED_AREA",
                SolutionComparator::LeastNbrCuts => "LEAST_NBR_CUTS",
                SolutionComparator::HvDiscrepancy => "MOST_HV_DISCREPANCY",
                SolutionComparator::BiggestUnusedTileArea => "BIGGEST_UNUSED_TILE_AREA",
                SolutionComparator::SmallestCenterOfMassDistToOrigin => "SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN",
                SolutionComparator::LeastNbrMosaics => "LEAST_NBR_MOSAICS",
                SolutionComparator::LeastNbrUnusedTiles => "LEAST_NBR_UNUSED_TILES",
                SolutionComparator::MostUnusedPanelArea => "MOST_UNUSED_PANEL_AREA",
            };
            
            assert_eq!(string_priority, enum_as_string, 
                      "Mismatch at position {}: string='{}', enum='{}'", 
                      i, string_priority, enum_as_string);
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_factory_integration_with_configuration() {
        let config = create_test_configuration(OptimizationPriority::MostTiles);
        
        // Get priority list from PriorityListFactory
        let priority_strings = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config);
        
        // Convert strings to comparators using SolutionComparatorFactory
        let comparators = SolutionComparatorFactory::get_solution_comparator_list(
            &priority_strings.iter().map(|s| s.as_str()).collect::<Vec<_>>()
        );
        
        assert_eq!(comparators.len(), priority_strings.len());
        assert_eq!(comparators.len(), 6);
    }

    #[test]
    fn test_round_trip_string_to_enum_conversion() {
        let original_strings = vec![
            "MOST_TILES",
            "LEAST_WASTED_AREA", 
            "LEAST_NBR_CUTS",
        ];
        
        // Convert strings to enums
        let enums = SolutionComparatorFactory::get_solution_comparator_enum_list(&original_strings);
        
        // Convert enums back to strings (via OptimizationPriority)
        let round_trip_strings: Vec<String> = enums.iter().map(|comparator| {
            match comparator {
                SolutionComparator::MostNbrTiles => OptimizationPriority::MostTiles.to_string(),
                SolutionComparator::LeastWastedArea => OptimizationPriority::LeastWastedArea.to_string(),
                SolutionComparator::LeastNbrCuts => OptimizationPriority::LeastNbrCuts.to_string(),
                SolutionComparator::HvDiscrepancy => OptimizationPriority::MostHvDiscrepancy.to_string(),
                SolutionComparator::BiggestUnusedTileArea => OptimizationPriority::BiggestUnusedTileArea.to_string(),
                SolutionComparator::SmallestCenterOfMassDistToOrigin => OptimizationPriority::SmallestCenterOfMassDistToOrigin.to_string(),
                SolutionComparator::LeastNbrMosaics => OptimizationPriority::LeastNbrMosaics.to_string(),
                SolutionComparator::LeastNbrUnusedTiles => OptimizationPriority::LeastNbrUnusedTiles.to_string(),
                SolutionComparator::MostUnusedPanelArea => OptimizationPriority::MostUnusedPanelArea.to_string(),
            }
        }).collect();
        
        assert_eq!(original_strings.len(), round_trip_strings.len());
        for (original, round_trip) in original_strings.iter().zip(round_trip_strings.iter()) {
            assert_eq!(original, round_trip);
        }
    }
}
