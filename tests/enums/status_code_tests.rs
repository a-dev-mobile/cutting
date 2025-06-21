use cutlist_optimizer_cli::StatusCode;

#[cfg(test)]
mod status_code_integration_tests {
    use super::*;

    #[test]
    fn test_status_code_descriptions() {
        assert_eq!(StatusCode::Ok.description(), "Operation completed successfully");
        assert_eq!(StatusCode::InvalidTiles.description(), "Invalid tiles provided");
        assert_eq!(StatusCode::InvalidStockTiles.description(), "Invalid stock tiles provided");
        assert_eq!(StatusCode::TaskAlreadyRunning.description(), "Task is already running");
        assert_eq!(StatusCode::ServerUnavailable.description(), "Server is unavailable");
        assert_eq!(StatusCode::TooManyPanels.description(), "Too many panels specified");
        assert_eq!(StatusCode::TooManyStockPanels.description(), "Too many stock panels specified");
    }

    #[test]
    fn test_status_code_is_ok_and_is_error() {
        // Тест успешного статуса
        assert!(StatusCode::Ok.is_ok());
        assert!(!StatusCode::Ok.is_error());

        // Тест статусов ошибок
        let error_statuses = vec![
            StatusCode::InvalidTiles,
            StatusCode::InvalidStockTiles,
            StatusCode::TaskAlreadyRunning,
            StatusCode::ServerUnavailable,
            StatusCode::TooManyPanels,
            StatusCode::TooManyStockPanels,
        ];

        for status in error_statuses {
            assert!(!status.is_ok());
            assert!(status.is_error());
        }
    }

    #[test]
    fn test_status_code_display_with_description() {
        assert_eq!(format!("{}", StatusCode::Ok), "0: Operation completed successfully");
        assert_eq!(format!("{}", StatusCode::InvalidTiles), "1: Invalid tiles provided");
        assert_eq!(format!("{}", StatusCode::ServerUnavailable), "4: Server is unavailable");
    }

    #[test]
    fn test_status_code_default() {
        let default_status = StatusCode::default();
        assert_eq!(default_status, StatusCode::Ok);
        assert!(default_status.is_ok());
    }

    #[test]
    fn test_status_code_conversions() {
        // Тест конверсии в u8
        let status = StatusCode::InvalidTiles;
        let value: u8 = status.into();
        assert_eq!(value, 1);

        // Тест конверсии из u8
        let back_status = StatusCode::try_from(value).unwrap();
        assert_eq!(back_status, status);

        // Тест неудачной конверсии
        let invalid_conversion = StatusCode::try_from(99u8);
        assert!(invalid_conversion.is_err());
    }

    #[test]
    fn test_status_code_all_variants_comprehensive() {
        let all_variants = vec![
            StatusCode::Ok,
            StatusCode::InvalidTiles,
            StatusCode::InvalidStockTiles,
            StatusCode::TaskAlreadyRunning,
            StatusCode::ServerUnavailable,
            StatusCode::TooManyPanels,
            StatusCode::TooManyStockPanels,
        ];

        // Проверяем, что все варианты имеют уникальные значения
        let mut values: Vec<u8> = all_variants.iter().map(|s| s.value()).collect();
        values.sort();
        values.dedup();
        assert_eq!(values.len(), all_variants.len());

        // Проверяем, что все значения находятся в ожидаемом диапазоне
        for variant in &all_variants {
            assert!(variant.value() <= 6);
        }

        // Проверяем, что каждый вариант имеет описание
        for variant in &all_variants {
            assert!(!variant.description().is_empty());
        }
    }

    #[test]
    fn test_status_code_boundary_values() {
        // Тестируем граничные случаи для u8
        assert_eq!(StatusCode::from_value(7), None);
        assert_eq!(StatusCode::from_value(255), None);
        assert_eq!(StatusCode::from_value(u8::MAX), None);
    }

    #[test]
    fn test_status_code_serialization() {
        // Тест сериализации в JSON
        let status = StatusCode::Ok;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"Ok\"");
        
        let deserialized: StatusCode = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);

        // Тест сериализации статуса ошибки
        let error_status = StatusCode::InvalidTiles;
        let error_json = serde_json::to_string(&error_status).unwrap();
        assert_eq!(error_json, "\"InvalidTiles\"");
        
        let error_deserialized: StatusCode = serde_json::from_str(&error_json).unwrap();
        assert_eq!(error_status, error_deserialized);
    }

    #[test]
    fn test_status_code_hash_and_equality() {
        use std::collections::HashMap;

        let mut status_map = HashMap::new();
        status_map.insert(StatusCode::Ok, "success");
        status_map.insert(StatusCode::InvalidTiles, "error");

        assert_eq!(status_map.get(&StatusCode::Ok), Some(&"success"));
        assert_eq!(status_map.get(&StatusCode::InvalidTiles), Some(&"error"));
        assert_eq!(status_map.get(&StatusCode::ServerUnavailable), None);
    }

    #[test]
    fn test_status_code_copy_semantics() {
        let status1 = StatusCode::Ok;
        let status2 = status1; // Copy, not move
        
        // Both should still be usable
        assert_eq!(status1, StatusCode::Ok);
        assert_eq!(status2, StatusCode::Ok);
        assert_eq!(status1, status2);
    }
}
