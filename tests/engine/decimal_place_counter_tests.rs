//! Tests for DimensionUtils struct
//!
//! These tests verify the DimensionUtils utility functions for counting
//! decimal and integer places, including validation and error handling.

use cutlist_optimizer_cli::engine::service::computation::dimension_utils::DimensionUtils;
use cutlist_optimizer_cli::models::panel::structs::Panel;

#[cfg(test)]
mod decimal_place_counter_tests {
    use super::*;

    fn create_panel(width: &str, height: &str, enabled: bool) -> Panel {
        Panel {
            id: 1,
            width: Some(width.to_string()),
            height: Some(height.to_string()),
            count: 1,
            material: "wood".to_string(),
            enabled,
            orientation: 0,
            label: None,
            edge: None,
        }
    }

    #[test]
    fn test_decimal_places_edge_cases() {
        assert_eq!(DimensionUtils::get_nbr_decimal_places(""), 0);
        assert_eq!(DimensionUtils::get_nbr_decimal_places("."), 0);
        assert_eq!(DimensionUtils::get_nbr_decimal_places("123."), 0);
        assert_eq!(DimensionUtils::get_nbr_decimal_places(".5"), 1);
    }

    #[test]
    fn test_integer_places_edge_cases() {
        assert_eq!(DimensionUtils::get_nbr_integer_places(""), 0);
        assert_eq!(DimensionUtils::get_nbr_integer_places(".5"), 0);
        assert_eq!(DimensionUtils::get_nbr_integer_places("123."), 3);
    }

    #[test]
    fn test_validate_and_count_places() {
        assert!(DimensionUtils::validate_and_count_places("").is_err());
        assert!(DimensionUtils::validate_and_count_places("abc").is_err());
        
        let result = DimensionUtils::validate_and_count_places("123.45").unwrap();
        assert_eq!(result, (3, 2));
    }

    #[test]
    fn test_validate_digit_limits() {
        let panels = vec![
            create_panel("123.45", "67.89", true),
            create_panel("1.0", "2.123", true),
        ];

        assert!(DimensionUtils::validate_digit_limits(&panels, 6).is_ok());
        assert!(DimensionUtils::validate_digit_limits(&panels, 4).is_err());
    }

    #[test]
    fn test_none_values_handling() {
        let mut panel = create_panel("123", "456", true);
        panel.width = None;
        panel.height = None;
        
        let panels = vec![panel];
        
        assert_eq!(DimensionUtils::get_max_nbr_decimal_places(&panels), 0);
        assert_eq!(DimensionUtils::get_max_nbr_integer_places(&panels), 1);
    }

    #[test]
    fn test_decimal_places_basic() {
        assert_eq!(DimensionUtils::get_nbr_decimal_places("123.45"), 2);
        assert_eq!(DimensionUtils::get_nbr_decimal_places("0.123"), 3);
        assert_eq!(DimensionUtils::get_nbr_decimal_places("1.0"), 1);
        assert_eq!(DimensionUtils::get_nbr_decimal_places("999.9999"), 4);
        assert_eq!(DimensionUtils::get_nbr_decimal_places("123"), 0);
    }

    #[test]
    fn test_integer_places_basic() {
        assert_eq!(DimensionUtils::get_nbr_integer_places("123.45"), 3);
        assert_eq!(DimensionUtils::get_nbr_integer_places("0.123"), 1);
        assert_eq!(DimensionUtils::get_nbr_integer_places("1.0"), 1);
        assert_eq!(DimensionUtils::get_nbr_integer_places("999.9999"), 3);
        assert_eq!(DimensionUtils::get_nbr_integer_places("123"), 3);
    }

    #[test]
    fn test_max_decimal_places_single_panel() {
        let panels = vec![
            create_panel("123.45", "67.89", true),
        ];
        
        assert_eq!(DimensionUtils::get_max_nbr_decimal_places(&panels), 2);
    }

    #[test]
    fn test_max_decimal_places_multiple_panels() {
        let panels = vec![
            create_panel("123.45", "67.89", true),    // 2 decimal places
            create_panel("1.0", "2.123", true),       // 3 decimal places (max)
            create_panel("999", "888.1", true),       // 1 decimal place
        ];
        
        assert_eq!(DimensionUtils::get_max_nbr_decimal_places(&panels), 3);
    }

    #[test]
    fn test_max_decimal_places_with_disabled_panels() {
        let panels = vec![
            create_panel("123.45", "67.89", true),      // 2 decimal places
            create_panel("1.12345", "2.123", false),    // 5 decimal places but disabled
            create_panel("999", "888.1", true),         // 1 decimal place
        ];
        
        // Should ignore disabled panels
        assert_eq!(DimensionUtils::get_max_nbr_decimal_places(&panels), 2);
    }

    #[test]
    fn test_max_integer_places_single_panel() {
        let panels = vec![
            create_panel("123.45", "67.89", true),
        ];
        
        assert_eq!(DimensionUtils::get_max_nbr_integer_places(&panels), 3);
    }

    #[test]
    fn test_max_integer_places_multiple_panels() {
        let panels = vec![
            create_panel("123.45", "67.89", true),    // 3 integer places
            create_panel("1.0", "2.123", true),       // 1 integer place
            create_panel("99", "8888.1", true),       // 4 integer places (max)
        ];
        
        assert_eq!(DimensionUtils::get_max_nbr_integer_places(&panels), 4);
    }

    #[test]
    fn test_max_integer_places_with_disabled_panels() {
        let panels = vec![
            create_panel("123.45", "67.89", true),      // 3 integer places
            create_panel("12345.1", "2.123", false),    // 5 integer places but disabled
            create_panel("99", "888.1", true),          // 3 integer places
        ];
        
        // Should ignore disabled panels
        assert_eq!(DimensionUtils::get_max_nbr_integer_places(&panels), 3);
    }

    #[test]
    fn test_validate_and_count_places_valid_inputs() {
        let result = DimensionUtils::validate_and_count_places("123.45").unwrap();
        assert_eq!(result, (3, 2));

        let result = DimensionUtils::validate_and_count_places("0.123").unwrap();
        assert_eq!(result, (1, 3));

        let result = DimensionUtils::validate_and_count_places("999").unwrap();
        assert_eq!(result, (3, 0));
    }

    #[test]
    fn test_validate_and_count_places_invalid_inputs() {
        assert!(DimensionUtils::validate_and_count_places("").is_err());
        assert!(DimensionUtils::validate_and_count_places("abc").is_err());
        assert!(DimensionUtils::validate_and_count_places("12.34.56").is_err());
        assert!(DimensionUtils::validate_and_count_places("not_a_number").is_err());
    }

    #[test]
    fn test_validate_digit_limits_success() {
        let panels = vec![
            create_panel("123.45", "67.89", true),  // max 3 int + 2 dec from this panel
            create_panel("1.0", "2.123", true),     // max 1 int + 3 dec from this panel
        ];
        // Overall max: 3 int + 3 dec = 6 total

        // Should pass with limit of 7 or 6
        assert!(DimensionUtils::validate_digit_limits(&panels, 7).is_ok());
        assert!(DimensionUtils::validate_digit_limits(&panels, 6).is_ok());
    }

    #[test]
    fn test_validate_digit_limits_failure() {
        let panels = vec![
            create_panel("123.45", "67.89", true),  // max 3 int + 2 dec from this panel
            create_panel("1.0", "2.123", true),     // max 1 int + 3 dec from this panel
        ];
        // Overall max: 3 int + 3 dec = 6 total

        // Should fail with limit of 5 or less
        assert!(DimensionUtils::validate_digit_limits(&panels, 5).is_err());
        assert!(DimensionUtils::validate_digit_limits(&panels, 4).is_err());
    }

    #[test]
    fn test_empty_panels_collections() {
        let panels: Vec<Panel> = vec![];
        
        assert_eq!(DimensionUtils::get_max_nbr_decimal_places(&panels), 0);
        assert_eq!(DimensionUtils::get_max_nbr_integer_places(&panels), 0);
        assert!(DimensionUtils::validate_digit_limits(&panels, 1).is_ok());
    }
}
