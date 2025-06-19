//! Этап 7: Тесты производительности и бенчмарки
//! 
//! Этот модуль содержит тесты для проверки производительности сервиса оптимизации
//! в различных сценариях нагрузки, основанные на логике Java CutListOptimizerServiceImpl.

use cutting_cli::engine::service::{CutListOptimizerService, CutListOptimizerServiceImpl};
use cutting_cli::engine::model::request::{CalculationRequest, Panel, ClientInfo, Configuration, PerformanceThresholds};
use cutting_cli::engine::logger::CutListLoggerImpl;
use std::sync::Arc;
use std::time::Instant;

/// Базовый тест производительности - простой случай
#[test]
fn test_basic_performance() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(1).unwrap(); // Используем только 1 поток для простого теста
    
    let client_info = ClientInfo::new("perf_client".to_string());
    let config = Configuration::default();
    
    // Простой случай: 5 небольших деталей на 1 большой панели
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "200".to_string(), "300".to_string(), 5, Some("Small Tile".to_string())),
        ],
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 1, Some("Large Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Базовая оптимизация должна работать");
    assert!(duration.as_millis() < 5000, "Базовая оптимизация должна завершаться быстро (< 5 сек), фактически: {:?}", duration);
    
    println!("Базовая производительность: {:?}", duration);
    
    // Проверяем результат - используем правильные имена полей
    let response = result.unwrap();
    assert!(response.statistics.placed_panels > 0, "Должны быть размещены панели");
    println!("Размещено панелей: {}", response.statistics.placed_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
}

/// Тест производительности с несколькими деталями
#[test]
fn test_performance_multiple_tiles() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(2).unwrap();
    
    let client_info = ClientInfo::new("multi_tiles_client".to_string());
    let config = Configuration::default();
    
    // Средний случай: 10 деталей на 2 панели
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "150".to_string(), "200".to_string(), 5, Some("Tile A".to_string())),
            Panel::new(2, "100".to_string(), "250".to_string(), 5, Some("Tile B".to_string())),
        ],
        vec![
            Panel::new(1, "800".to_string(), "600".to_string(), 2, Some("Panel A".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Оптимизация с несколькими деталями должна работать");
    assert!(duration.as_millis() < 8000, "Оптимизация с несколькими деталями должна завершаться быстро (< 8 сек), фактически: {:?}", duration);
    
    println!("Производительность с несколькими деталями: {:?}", duration);
    
    // Проверяем результат - используем правильные имена полей
    let response = result.unwrap();
    assert!(response.statistics.placed_panels > 0, "Должны быть размещены панели");
    println!("Размещено панелей: {}", response.statistics.placed_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
}

/// Тест производительности с увеличенным количеством панелей
#[test]
fn test_performance_many_panels() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("many_panels_client".to_string());
    let config = Configuration::default();
    
    // Более сложный случай: 15 деталей на 3 панели
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "200".to_string(), "300".to_string(), 15, Some("Standard Tile".to_string())),
        ],
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 3, Some("Standard Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Оптимизация с множеством панелей должна работать");
    assert!(duration.as_secs() < 15, "Оптимизация с множеством панелей должна завершаться в разумное время (< 15 сек), фактически: {:?}", duration);
    
    println!("Производительность с множеством панелей: {:?}", duration);
    
    // Проверяем результат - используем правильные имена полей
    let response = result.unwrap();
    assert!(response.statistics.placed_panels > 0, "Должны быть размещены панели");
    println!("Размещено панелей: {}", response.statistics.placed_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
}

/// Тест производительности с оптимальным случаем
#[test]
fn test_performance_optimal_case() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(1).unwrap();
    
    let client_info = ClientInfo::new("optimal_client".to_string());
    let config = Configuration::default();
    
    // Оптимальный случай: детали точно помещаются в панель
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "500".to_string(), "300".to_string(), 2, Some("Perfect Fit".to_string())),
        ],
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 1, Some("Perfect Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Оптимальная оптимизация должна работать");
    assert!(duration.as_millis() < 3000, "Оптимальная оптимизация должна завершаться очень быстро (< 3 сек), фактически: {:?}", duration);
    
    println!("Оптимальная производительность: {:?}", duration);
    
    // Проверяем результат - используем правильные имена полей
    let response = result.unwrap();
    assert!(response.statistics.placed_panels > 0, "Должны быть размещены панели");
    // Для оптимального случая ожидаем хорошую эффективность, но не требуем 100%
    assert!(response.statistics.efficiency_percentage > 30.0, "Эффективность должна быть разумной для оптимального случая");
    println!("Размещено панелей: {}/{}", response.statistics.placed_panels, response.statistics.total_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
}

/// Тест производительности с реалистичным сценарием
#[test]
fn test_performance_realistic_scenario() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(2).unwrap();
    
    let client_info = ClientInfo::new("realistic_client".to_string());
    let config = Configuration::default();
    
    // Реалистичный случай: различные размеры деталей
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "300".to_string(), "200".to_string(), 3, Some("Medium A".to_string())),
            Panel::new(2, "150".to_string(), "100".to_string(), 4, Some("Small B".to_string())),
            Panel::new(3, "400".to_string(), "250".to_string(), 2, Some("Large C".to_string())),
        ],
        vec![
            Panel::new(1, "1200".to_string(), "800".to_string(), 2, Some("Stock Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Реалистичная оптимизация должна работать");
    assert!(duration.as_secs() < 10, "Реалистичная оптимизация должна завершаться в разумное время (< 10 сек), фактически: {:?}", duration);
    
    println!("Реалистичная производительность: {:?}", duration);
    
    // Проверяем результат
    let response = result.unwrap();
    assert!(response.statistics.placed_panels > 0, "Должны быть размещены панели");
    println!("Размещено панелей: {}/{}", response.statistics.placed_panels, response.statistics.total_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
    println!("Использованная площадь: {:.0} из {:.0}", response.statistics.used_area, response.statistics.total_area);
}

/// Тест производительности с малыми деталями
#[test]
fn test_performance_small_tiles() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(1).unwrap();
    
    let client_info = ClientInfo::new("small_tiles_client".to_string());
    let config = Configuration::default();
    
    // Много маленьких деталей
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "50".to_string(), "50".to_string(), 8, Some("Tiny".to_string())),
            Panel::new(2, "75".to_string(), "60".to_string(), 6, Some("Small".to_string())),
        ],
        vec![
            Panel::new(1, "600".to_string(), "400".to_string(), 1, Some("Medium Stock".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Оптимизация малых деталей должна работать");
    assert!(duration.as_millis() < 4000, "Оптимизация малых деталей должна быть быстрой (< 4 сек), фактически: {:?}", duration);
    
    println!("Производительность с малыми деталями: {:?}", duration);
    
    // Проверяем результат
    let response = result.unwrap();
    assert!(response.statistics.placed_panels > 0, "Должны быть размещены панели");
    println!("Размещено панелей: {}/{}", response.statistics.placed_panels, response.statistics.total_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
    
    // Для малых деталей ожидаем разумную эффективность
    assert!(response.statistics.efficiency_percentage > 15.0, "Эффективность должна быть разумной для малых деталей");
}

/// Тест производительности: детали точно помещаются в панель (идеальное размещение)
#[test]
fn test_performance_perfect_fit_single() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(1).unwrap();
    
    let client_info = ClientInfo::new("perfect_fit_client".to_string());
    let config = Configuration::default();
    
    // Идеальный случай: одна деталь точно помещается в панель
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 1, Some("Perfect Match".to_string())),
        ],
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 1, Some("Exact Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Идеальное размещение должно работать");
    assert!(duration.as_millis() < 2000, "Идеальное размещение должно быть очень быстрым (< 2 сек), фактически: {:?}", duration);
    
    println!("Производительность идеального размещения: {:?}", duration);
    
    let response = result.unwrap();
    assert_eq!(response.statistics.placed_panels, 1, "Должна быть размещена 1 панель");
    assert_eq!(response.statistics.total_panels, 1, "Общее количество панелей должно быть 1");
    println!("Размещено панелей: {}/{}", response.statistics.placed_panels, response.statistics.total_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
    
    // Для идеального размещения ожидаем высокую эффективность
    assert!(response.statistics.efficiency_percentage > 90.0, "Эффективность должна быть очень высокой для идеального размещения");
}

/// Тест производительности: несколько деталей точно помещаются в панель
#[test]
fn test_performance_perfect_fit_multiple() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(1).unwrap();
    
    let client_info = ClientInfo::new("perfect_multiple_client".to_string());
    let config = Configuration::default();
    
    // Идеальный случай: 4 детали точно помещаются в панель (2x2)
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "500".to_string(), "300".to_string(), 4, Some("Quarter Panel".to_string())),
        ],
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 1, Some("Full Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Идеальное размещение нескольких деталей должно работать");
    assert!(duration.as_millis() < 3000, "Идеальное размещение нескольких деталей должно быть быстрым (< 3 сек), фактически: {:?}", duration);
    
    println!("Производительность идеального размещения нескольких деталей: {:?}", duration);
    
    let response = result.unwrap();
    assert!(response.statistics.placed_panels >= 3, "Должно быть размещено минимум 3 панели из 4");
    assert_eq!(response.statistics.total_panels, 4, "Общее количество панелей должно быть 4");
    println!("Размещено панелей: {}/{}", response.statistics.placed_panels, response.statistics.total_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
    
    // Для идеального размещения ожидаем высокую эффективность
    assert!(response.statistics.efficiency_percentage > 80.0, "Эффективность должна быть высокой для идеального размещения нескольких деталей");
}

/// Тест производительности: детали помещаются в несколько панелей точно
#[test]
fn test_performance_perfect_fit_grid() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(2).unwrap();
    
    let client_info = ClientInfo::new("perfect_grid_client".to_string());
    let config = Configuration::default();
    
    // Идеальный случай: детали образуют сетку 3x2 на панели 600x400
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "200".to_string(), "200".to_string(), 6, Some("Grid Cell".to_string())),
        ],
        vec![
            Panel::new(1, "600".to_string(), "400".to_string(), 1, Some("Grid Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Идеальное размещение сетки должно работать");
    assert!(duration.as_millis() < 4000, "Идеальное размещение сетки должно быть быстрым (< 4 сек), фактически: {:?}", duration);
    
    println!("Производительность идеального размещения сетки: {:?}", duration);
    
    let response = result.unwrap();
    assert!(response.statistics.placed_panels >= 5, "Должно быть размещено минимум 5 панелей из 6");
    assert_eq!(response.statistics.total_panels, 6, "Общее количество панелей должно быть 6");
    println!("Размещено панелей: {}/{}", response.statistics.placed_panels, response.statistics.total_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
    
    // Для идеального размещения сетки ожидаем очень высокую эффективность
    assert!(response.statistics.efficiency_percentage > 85.0, "Эффективность должна быть очень высокой для идеального размещения сетки");
}

/// Тест производительности: полосы точно помещаются в панель
#[test]
fn test_performance_perfect_fit_strips() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(1).unwrap();
    
    let client_info = ClientInfo::new("perfect_strips_client".to_string());
    let config = Configuration::default();
    
    // Идеальный случай: 5 полос точно помещаются в панель
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "200".to_string(), "600".to_string(), 5, Some("Strip".to_string())),
        ],
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 1, Some("Strip Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Идеальное размещение полос должно работать");
    assert!(duration.as_millis() < 3000, "Идеальное размещение полос должно быть быстрым (< 3 сек), фактически: {:?}", duration);
    
    println!("Производительность идеального размещения полос: {:?}", duration);
    
    let response = result.unwrap();
    assert!(response.statistics.placed_panels >= 4, "Должно быть размещено минимум 4 полосы из 5");
    assert_eq!(response.statistics.total_panels, 5, "Общее количество панелей должно быть 5");
    println!("Размещено панелей: {}/{}", response.statistics.placed_panels, response.statistics.total_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
    
    // Для идеального размещения полос ожидаем высокую эффективность
    assert!(response.statistics.efficiency_percentage > 75.0, "Эффективность должна быть высокой для идеального размещения полос");
}

/// Тест производительности: смешанные размеры с точным размещением
#[test]
fn test_performance_perfect_fit_mixed() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(2).unwrap();
    
    let client_info = ClientInfo::new("perfect_mixed_client".to_string());
    let config = Configuration::default();
    
    // Идеальный случай: смешанные размеры точно помещаются
    // Панель 1200x800: 2x(600x400) + 4x(300x200) + 8x(150x100)
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "600".to_string(), "400".to_string(), 2, Some("Large".to_string())),
            Panel::new(2, "300".to_string(), "200".to_string(), 4, Some("Medium".to_string())),
            Panel::new(3, "150".to_string(), "100".to_string(), 8, Some("Small".to_string())),
        ],
        vec![
            Panel::new(1, "1200".to_string(), "800".to_string(), 1, Some("Mixed Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Идеальное размещение смешанных размеров должно работать");
    assert!(duration.as_millis() < 5000, "Идеальное размещение смешанных размеров должно быть быстрым (< 5 сек), фактически: {:?}", duration);
    
    println!("Производительность идеального размещения смешанных размеров: {:?}", duration);
    
    let response = result.unwrap();
    assert!(response.statistics.placed_panels >= 10, "Должно быть размещено минимум 10 панелей из 14");
    assert_eq!(response.statistics.total_panels, 14, "Общее количество панелей должно быть 14");
    println!("Размещено панелей: {}/{}", response.statistics.placed_panels, response.statistics.total_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
    
    // Для идеального размещения смешанных размеров ожидаем хорошую эффективность
    assert!(response.statistics.efficiency_percentage > 60.0, "Эффективность должна быть хорошей для идеального размещения смешанных размеров");
}

/// Тест производительности: квадратные детали в квадратной панели
#[test]
fn test_performance_perfect_fit_squares() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(1).unwrap();
    
    let client_info = ClientInfo::new("perfect_squares_client".to_string());
    let config = Configuration::default();
    
    // Идеальный случай: 9 квадратных деталей в квадратной панели (3x3)
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "200".to_string(), "200".to_string(), 9, Some("Square".to_string())),
        ],
        vec![
            Panel::new(1, "600".to_string(), "600".to_string(), 1, Some("Square Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Идеальное размещение квадратов должно работать");
    assert!(duration.as_millis() < 3000, "Идеальное размещение квадратов должно быть быстрым (< 3 сек), фактически: {:?}", duration);
    
    println!("Производительность идеального размещения квадратов: {:?}", duration);
    
    let response = result.unwrap();
    assert!(response.statistics.placed_panels >= 8, "Должно быть размещено минимум 8 квадратов из 9");
    assert_eq!(response.statistics.total_panels, 9, "Общее количество панелей должно быть 9");
    println!("Размещено панелей: {}/{}", response.statistics.placed_panels, response.statistics.total_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
    
    // Для идеального размещения квадратов ожидаем очень высокую эффективность
    assert!(response.statistics.efficiency_percentage > 85.0, "Эффективность должна быть очень высокой для идеального размещения квадратов");
}

/// Тест производительности: одномерная оптимизация (как в Java isOneDimensionalOptimization)
#[test]
fn test_performance_one_dimensional_optimization() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(1).unwrap();
    
    let client_info = ClientInfo::new("one_dim_client".to_string());
    let config = Configuration::default();
    
    // Одномерная оптимизация: все детали имеют одинаковую ширину
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "100".to_string(), "200".to_string(), 3, Some("Same Width A".to_string())),
            Panel::new(2, "100".to_string(), "150".to_string(), 4, Some("Same Width B".to_string())),
            Panel::new(3, "100".to_string(), "100".to_string(), 5, Some("Same Width C".to_string())),
        ],
        vec![
            Panel::new(1, "100".to_string(), "1000".to_string(), 2, Some("Long Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Одномерная оптимизация должна работать");
    assert!(duration.as_millis() < 2000, "Одномерная оптимизация должна быть очень быстрой (< 2 сек), фактически: {:?}", duration);
    
    println!("Производительность одномерной оптимизации: {:?}", duration);
    
    let response = result.unwrap();
    assert!(response.statistics.placed_panels >= 10, "Должно быть размещено минимум 10 панелей из 12");
    assert_eq!(response.statistics.total_panels, 12, "Общее количество панелей должно быть 12");
    println!("Размещено панелей: {}/{}", response.statistics.placed_panels, response.statistics.total_panels);
    println!("Эффективность: {:.2}%", response.statistics.efficiency_percentage);
    
    // Для одномерной оптимизации ожидаем очень высокую эффективность
    assert!(response.statistics.efficiency_percentage > 80.0, "Эффективность должна быть очень высокой для одномерной оптимизации");
}
