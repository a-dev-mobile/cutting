//! Tests for decimal and integer place counting methods
//!
//! These tests verify the correct conversion of Java methods:
//! - getNbrDecimalPlaces -> get_nbr_decimal_places
//! - getNbrIntegerPlaces -> get_nbr_integer_places  
//! - getMaxNbrDecimalPlaces -> get_max_nbr_decimal_places
//! - getMaxNbrIntegerPlaces -> get_max_nbr_integer_places

use cutlist_optimizer_cli::engine::service::core::CutListOptimizerServiceImpl;
use cutlist_optimizer_cli::models::panel::structs::Panel;

#[cfg(test)]
mod decimal_places_tests {
    use super::*;

    fn create_service() -> CutListOptimizerServiceImpl {
        CutListOptimizerServiceImpl::new()
    }

    #[test]
    fn test_get_nbr_decimal_places_with_decimals() {
        let service = create_service();
        
        // Test cases with decimal places
        assert_eq!(service.get_nbr_decimal_places("123.45"), 2);
        assert_eq!(service.get_nbr_decimal_places("0.123"), 3);
        assert_eq!(service.get_nbr_decimal_places("1.0"), 1);
        assert_eq!(service.get_nbr_decimal_places("999.9999"), 4);
        assert_eq!(service.get_nbr_decimal_places("0.1"), 1);
    }

    #[test]
    fn test_get_nbr_decimal_places_without_decimals() {
        let service = create_service();
        
        // Test cases without decimal places
        assert_eq!(service.get_nbr_decimal_places("123"), 0);
        assert_eq!(service.get_nbr_decimal_places("0"), 0);
        assert_eq!(service.get_nbr_decimal_places("999"), 0);
    }

    #[test]
    fn test_get_nbr_decimal_places_edge_cases() {
        let service = create_service();
        
        // Edge cases
        assert_eq!(service.get_nbr_decimal_places(""), 0);
        assert_eq!(service.get_nbr_decimal_places("."), 0);
        assert_eq!(service.get_nbr_decimal_places(".5"), 1);
        assert_eq!(service.get_nbr_decimal_places("123."), 0);
    }

    #[test]
    fn test_get_nbr_integer_places_basic() {
        let service = create_service();
        
        // Test cases with integers
        assert_eq!(service.get_nbr_integer_places("123"), 3);
        assert_eq!(service.get_nbr_integer_places("0"), 1);
        assert_eq!(service.get_nbr_integer_places("999"), 3);
        assert_eq!(service.get_nbr_integer_places("1"), 1);
    }

    #[test]
    fn test_get_nbr_integer_places_with_decimals() {
        let service = create_service();
        
        // Test cases with decimal points
        assert_eq!(service.get_nbr_integer_places("123.45"), 3);
        assert_eq!(service.get_nbr_integer_places("0.123"), 1);
        assert_eq!(service.get_nbr_integer_places("1.0"), 1);
        assert_eq!(service.get_nbr_integer_places("999.9999"), 3);
    }

    #[test]
    fn test_get_nbr_integer_places_edge_cases() {
        let service = create_service();
        
        // Edge cases
        assert_eq!(service.get_nbr_integer_places(""), 0);
        assert_eq!(service.get_nbr_integer_places(".5"), 0);
        assert_eq!(service.get_nbr_integer_places("123."), 3);
    }

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
    fn test_get_max_nbr_decimal_places_single_panel() {
        let service = create_service();
        
        let panels = vec![
            create_panel("123.45", "67.89", true),
        ];
        
        assert_eq!(service.get_max_nbr_decimal_places(&panels), 2);
    }

    #[test]
    fn test_get_max_nbr_decimal_places_multiple_panels() {
        let service = create_service();
        
        let panels = vec![
            create_panel("123.45", "67.89", true),    // 2 decimal places
            create_panel("1.0", "2.123", true),       // 3 decimal places (max)
            create_panel("999", "888.1", true),       // 1 decimal place
        ];
        
        assert_eq!(service.get_max_nbr_decimal_places(&panels), 3);
    }

    #[test]
    fn test_get_max_nbr_decimal_places_with_disabled_panels() {
        let service = create_service();
        
        let panels = vec![
            create_panel("123.45", "67.89", true),      // 2 decimal places
            create_panel("1.12345", "2.123", false),    // 5 decimal places but disabled
            create_panel("999", "888.1", true),         // 1 decimal place
        ];
        
        // Should ignore disabled panels
        assert_eq!(service.get_max_nbr_decimal_places(&panels), 2);
    }

    #[test]
    fn test_get_max_nbr_decimal_places_no_decimals() {
        let service = create_service();
        
        let panels = vec![
            create_panel("123", "67", true),
            create_panel("1", "2", true),
            create_panel("999", "888", true),
        ];
        
        assert_eq!(service.get_max_nbr_decimal_places(&panels), 0);
    }

    #[test]
    fn test_get_max_nbr_decimal_places_empty_panels() {
        let service = create_service();
        let panels = vec![];
        
        assert_eq!(service.get_max_nbr_decimal_places(&panels), 0);
    }

    #[test]
    fn test_get_max_nbr_integer_places_single_panel() {
        let service = create_service();
        
        let panels = vec![
            create_panel("123.45", "67.89", true),
        ];
        
        assert_eq!(service.get_max_nbr_integer_places(&panels), 3);
    }

    #[test]
    fn test_get_max_nbr_integer_places_multiple_panels() {
        let service = create_service();
        
        let panels = vec![
            create_panel("123.45", "67.89", true),    // 3 integer places (max)
            create_panel("1.0", "2.123", true),       // 1 integer place
            create_panel("99", "8888.1", true),       // 4 integer places (max)
        ];
        
        assert_eq!(service.get_max_nbr_integer_places(&panels), 4);
    }

    #[test]
    fn test_get_max_nbr_integer_places_with_disabled_panels() {
        let service = create_service();
        
        let panels = vec![
            create_panel("123.45", "67.89", true),      // 3 integer places
            create_panel("12345.1", "2.123", false),    // 5 integer places but disabled
            create_panel("99", "888.1", true),          // 3 integer places
        ];
        
        // Should ignore disabled panels
        assert_eq!(service.get_max_nbr_integer_places(&panels), 3);
    }

    #[test]
    fn test_get_max_nbr_integer_places_empty_panels() {
        let service = create_service();
        let panels = vec![];
        
        assert_eq!(service.get_max_nbr_integer_places(&panels), 0);
    }

    #[test]
    fn test_get_max_nbr_integer_places_with_none_values() {
        let service = create_service();
        
        let mut panel = create_panel("123", "456", true);
        panel.width = None;
        panel.height = None;
        
        let panels = vec![panel];
        
        // Should handle None values gracefully by using "0" as default
        assert_eq!(service.get_max_nbr_integer_places(&panels), 1);
    }

    #[test]
    fn test_comprehensive_decimal_and_integer_places() {
        let service = create_service();
        
        let panels = vec![
            create_panel("1234.567", "89.12", true),     // 4 int, 3 dec
            create_panel("12.3456", "789.1", true),      // 3 int, 4 dec
            create_panel("56789", "1.23456", true),      // 5 int, 5 dec
            create_panel("1.2", "34.56", false),         // disabled - should be ignored
        ];
        
        assert_eq!(service.get_max_nbr_decimal_places(&panels), 5);
        assert_eq!(service.get_max_nbr_integer_places(&panels), 5);
    }

    #[test]
    fn test_java_compatibility_edge_cases() {
        let service = create_service();
        
        // Test cases that match Java behavior exactly
        assert_eq!(service.get_nbr_decimal_places("46"), 0);  // Java: str.indexOf(46) where 46 is '.'
        assert_eq!(service.get_nbr_integer_places("46"), 2);
        
        // Test with actual decimal point
        assert_eq!(service.get_nbr_decimal_places("12.34"), 2);
        assert_eq!(service.get_nbr_integer_places("12.34"), 2);
        
        // Test edge case from Java: if indexOf returns -1
        assert_eq!(service.get_nbr_decimal_places("123"), 0);
        assert_eq!(service.get_nbr_integer_places("123"), 3);
    }
}
