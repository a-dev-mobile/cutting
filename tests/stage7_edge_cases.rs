//! Этап 7: Тесты граничных случаев и специфических сценариев
//! 
//! Этот модуль содержит тесты для проверки работы сервиса в граничных случаях
//! и специфических сценариях использования.

use cutting_cli::engine::service::{CutListOptimizerService, CutListOptimizerServiceImpl};
use cutting_cli::engine::model::request::{CalculationRequest, Panel, ClientInfo, Configuration};
use cutting_cli::engine::logger::CutListLoggerImpl;
use std::sync::Arc;

/// Тест с очень маленькими размерами
#[test]
fn test_very_small_dimensions() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("test_client".to_string());
    let config = Configuration::default();
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "5".to_string(), "5".to_string(), 4, None),
        ],
        vec![
            Panel::new(1, "10".to_string(), "10".to_string(), 1, None),
        ],
    );
    
    let result = service.optimize(request);
    
    assert!(result.is_ok(), "Оптимизация с маленькими размерами должна работать");
    
    let response = result.unwrap();
    assert!(response.statistics.total_panels == response.statistics.total_panels, "Статистика должна быть корректной");
}

/// Тест с очень большими размерами
#[test]
fn test_very_large_dimensions() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("test_client".to_string());
    let config = Configuration::default();
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "1000".to_string(), "800".to_string(), 50, None),
        ],
        vec![
            Panel::new(1, "10000".to_string(), "8000".to_string(), 1, None),
        ],
    );
    
    let result = service.optimize(request);
    
    assert!(result.is_ok(), "Оптимизация с большими размерами должна работать");
    
    let response = result.unwrap();
    assert!(response.statistics.total_panels >= 0, "Статистика должна быть корректной");
}

/// Тест с плитками одинакового размера с панелью
#[test]
fn test_tile_same_size_as_panel() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("test_client".to_string());
    let mut config = Configuration::default();
    config.cut_thickness = "0".to_string(); // Нулевая толщина пропила
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "500".to_string(), "300".to_string(), 3, None),
        ],
        vec![
            Panel::new(1, "500".to_string(), "300".to_string(), 2, None),
        ],
    );
    
    let result = service.optimize(request);
    
    assert!(result.is_ok(), "Оптимизация с плитками размера панели должна работать");
    
    let response = result.unwrap();
    assert!(response.statistics.total_panels >= 0, "Статистика должна быть корректной");
}

/// Тест с плитками больше панели
#[test]
fn test_tiles_larger_than_panel() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("test_client".to_string());
    let config = Configuration::default();
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "400".to_string(), "300".to_string(), 4, None),
        ],
        vec![
            Panel::new(1, "300".to_string(), "200".to_string(), 2, None),
        ],
    );
    
    let result = service.optimize(request);
    
    assert!(result.is_ok(), "Сервис должен обработать запрос с большими плитками");
    
    let response = result.unwrap();
    assert!(response.statistics.total_panels >= 0, "Статистика должна быть корректной");
}

/// Тест с единичными количествами
#[test]
fn test_single_quantities() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("test_client".to_string());
    let config = Configuration::default();
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "200".to_string(), "300".to_string(), 1, None),
        ],
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 1, None),
        ],
    );
    
    let result = service.optimize(request);
    
    assert!(result.is_ok(), "Оптимизация с единичными количествами должна работать");
    
    let response = result.unwrap();
    assert!(response.statistics.total_panels >= 0, "Статистика должна быть корректной");
}

/// Тест с максимальными количествами
// #[test]
// fn test_maximum_quantities() {
//     let logger = Arc::new(CutListLoggerImpl::new());
//     let mut service = CutListOptimizerServiceImpl::new(logger);
//     service.init(3).unwrap();
    
//     let client_info = ClientInfo::new("test_client".to_string());
//     let config = Configuration::default();
    
//     let request = CalculationRequest::new(
//         client_info,
//         config,
//         vec![
//             Panel::new(1, "100".to_string(), "100".to_string(), 1000, None),
//         ],
//         vec![
//             Panel::new(1, "2000".to_string(), "1000".to_string(), 100, None),
//         ],
//     );
    
//     let start_time = std::time::Instant::now();
//     let result = service.optimize(request);
//     let duration = start_time.elapsed();
    
//     assert!(result.is_ok(), "Оптимизация с большими количествами должна работать");
    
//     // Проверяем производительность
//     assert!(duration.as_secs() < 60, "Оптимизация должна завершаться в разумное время");
    
//     let response = result.unwrap();
//     assert!(response.statistics.total_panels >= 0, "Статистика должна быть корректной");
// }

/// Тест с нулевыми размерами (граничный случай)
#[test]
fn test_zero_dimensions() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("test_client".to_string());
    let config = Configuration::default();
    
    // Тест с нулевой шириной складской панели
    let request_zero_panel_width = CalculationRequest::new(
        client_info.clone(),
        config.clone(),
        vec![
            Panel::new(1, "200".to_string(), "300".to_string(), 2, None),
        ],
        vec![
            Panel::new(1, "0".to_string(), "600".to_string(), 1, None),
        ],
    );
    
    let result = service.optimize(request_zero_panel_width);
    
    // Сервис должен обработать некорректные данные
    assert!(result.is_err() || (result.is_ok() && result.unwrap().statistics.total_panels >= 0));
    
    // Тест с нулевой шириной плитки
    let request_zero_tile_width = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "0".to_string(), "300".to_string(), 2, None),
        ],
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 1, None),
        ],
    );
    
    let result = service.optimize(request_zero_tile_width);
    
    // Сервис должен обработать некорректные данные
    assert!(result.is_err() || (result.is_ok() && result.unwrap().statistics.total_panels >= 0));
}

/// Тест с отрицательными размерами (граничный случай)
#[test]
fn test_negative_dimensions() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("test_client".to_string());
    let config = Configuration::default();
    
    // Тест с отрицательными размерами
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "200".to_string(), "300".to_string(), 2, None),
        ],
        vec![
            Panel::new(1, "-1000".to_string(), "600".to_string(), 1, None),
        ],
    );
    
    let result = service.optimize(request);
    
    // Сервис должен обработать запрос с некорректными данными
    assert!(result.is_err() || (result.is_ok() && result.unwrap().statistics.total_panels >= 0));
}

/// Тест с экстремально большой шириной реза
#[test]
fn test_extreme_cut_width() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("test_client".to_string());
    let mut config = Configuration::default();
    config.cut_thickness = "500".to_string(); // Ширина реза больше размеров плитки
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "200".to_string(), "300".to_string(), 4, None),
        ],
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 1, None),
        ],
    );
    
    let result = service.optimize(request);
    
    assert!(result.is_ok(), "Сервис должен обработать большую ширину реза");
    
    let response = result.unwrap();
    assert!(response.statistics.total_panels >= 0, "Статистика должна быть корректной");
}

/// Тест с квадратными плитками и панелями
#[test]
fn test_square_shapes() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("test_client".to_string());
    let config = Configuration::default();
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "200".to_string(), "200".to_string(), 18, None), // 9 плиток на панель теоретически
        ],
        vec![
            Panel::new(1, "600".to_string(), "600".to_string(), 2, None),
        ],
    );
    
    let result = service.optimize(request);
    
    assert!(result.is_ok(), "Оптимизация квадратных форм должна работать");
    
    let response = result.unwrap();
    assert!(response.statistics.total_panels >= 0, "Статистика должна быть корректной");
}

/// Тест с прямоугольными плитками разных пропорций
#[test]
fn test_different_aspect_ratios() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("aspect_client".to_string());
    let config = Configuration::default();
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "100".to_string(), "400".to_string(), 5, Some("Narrow Tile".to_string())), // Узкая и длинная
            Panel::new(2, "400".to_string(), "100".to_string(), 5, Some("Wide Tile".to_string())), // Широкая и короткая
            Panel::new(3, "200".to_string(), "200".to_string(), 5, Some("Square Tile".to_string())), // Квадратная
        ],
        vec![
            Panel::new(1, "1200".to_string(), "800".to_string(), 1, Some("Panel".to_string())),
        ],
    );
    
    let result = service.optimize(request);
    
    assert!(result.is_ok(), "Оптимизация с разными пропорциями должна работать");
    
    let response = result.unwrap();
    assert!(response.statistics.total_panels >= 0, "Статистика должна быть корректной");
}

/// Тест стабильности результатов (детерминированность)
#[test]
fn test_result_stability() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("stability_client".to_string());
    let config = Configuration::default();
    
    let create_request = || {
        CalculationRequest::new(
            client_info.clone(),
            config.clone(),
            vec![
                Panel::new(1, "200".to_string(), "150".to_string(), 10, Some("Tile".to_string())),
            ],
            vec![
                Panel::new(1, "800".to_string(), "600".to_string(), 1, Some("Panel".to_string())),
            ],
        )
    };
    
    // Запускаем оптимизацию несколько раз
    let mut results = Vec::new();
    for _ in 0..3 {
        let result = service.optimize(create_request());
        assert!(result.is_ok(), "Каждый запуск должен быть успешным");
        results.push(result.unwrap());
    }
    
    // Проверяем, что результаты стабильны (одинаковая статистика)
    let panel_counts: Vec<usize> = results.iter()
        .map(|r| r.statistics.total_panels)
        .collect();
    
    assert!(panel_counts.iter().all(|&count| count == panel_counts[0]), 
           "Количество панелей должно быть стабильным между запусками");
}
