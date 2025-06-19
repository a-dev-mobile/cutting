//! Этап 7: Тесты производительности и бенчмарки
//! 
//! Этот модуль содержит тесты для проверки производительности сервиса оптимизации
//! в различных сценариях нагрузки.

use cutting_cli::engine::service::{CutListOptimizerService, CutListOptimizerServiceImpl};
use cutting_cli::engine::model::request::{CalculationRequest, Panel, ClientInfo, Configuration};
use cutting_cli::engine::logger::CutListLoggerImpl;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Базовый тест производительности
#[test]
fn test_basic_performance() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("perf_client".to_string());
    let config = Configuration::default();
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "200".to_string(), "300".to_string(), 25, Some("Standard Tile".to_string())),
        ],
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 5, Some("Standard Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Базовая оптимизация должна работать");
    assert!(duration.as_millis() < 5000, "Базовая оптимизация должна завершаться быстро (< 5 сек)");
    
    println!("Базовая производительность: {:?}", duration);
}

/// Тест производительности с увеличенным количеством панелей
#[test]
fn test_performance_many_panels() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(5).unwrap();
    
    let client_info = ClientInfo::new("many_panels_client".to_string());
    let config = Configuration::default();
    
    let mut panels = Vec::new();
    for i in 1..=20 {
        panels.push(Panel::new(
            i, 
            (800 + (i * 10)).to_string(), 
            "600".to_string(), 
            2, 
            Some(format!("Panel {}", i))
        ));
    }
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "200".to_string(), "300".to_string(), 30, Some("Standard Tile".to_string())),
        ],
        panels,
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Оптимизация с множеством панелей должна работать");
    assert!(duration.as_secs() < 15, "Оптимизация с множеством панелей должна завершаться в разумное время");
    
    println!("Производительность с множеством панелей: {:?}", duration);
}

/// Тест производительности с увеличенным количеством типов плиток
#[test]
fn test_performance_many_tile_types() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(5).unwrap();
    
    let client_info = ClientInfo::new("many_tiles_client".to_string());
    let config = Configuration::default();
    
    let mut tiles = Vec::new();
    for i in 1..=15 {
        tiles.push(Panel::new(
            i, 
            (100 + (i * 20)).to_string(), 
            (150 + (i * 15)).to_string(), 
            5, 
            Some(format!("Tile Type {}", i))
        ));
    }
    
    let request = CalculationRequest::new(
        client_info,
        config,
        tiles,
        vec![
            Panel::new(1, "2000".to_string(), "1200".to_string(), 3, Some("Large Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Оптимизация с множеством типов плиток должна работать");
    assert!(duration.as_secs() < 20, "Оптимизация с множеством типов плиток должна завершаться в разумное время");
    
    println!("Производительность с множеством типов плиток: {:?}", duration);
}

/// Тест производительности с большим количеством плиток одного типа
#[test]
fn test_performance_many_tiles_same_type() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(5).unwrap();
    
    let client_info = ClientInfo::new("mass_tiles_client".to_string());
    let config = Configuration::default();
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "150".to_string(), "200".to_string(), 100, Some("Mass Tile".to_string())), // Много плиток одного типа
        ],
        vec![
            Panel::new(1, "3000".to_string(), "2000".to_string(), 5, Some("Extra Large Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Оптимизация с большим количеством плиток должна работать");
    assert!(duration.as_secs() < 30, "Оптимизация с большим количеством плиток должна завершаться в разумное время");
    
    println!("Производительность с большим количеством плиток: {:?}", duration);
}

/// Тест производительности с минимальной шириной реза
#[test]
fn test_performance_minimal_cut_width() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("minimal_cut_client".to_string());
    let mut config = Configuration::default();
    config.cut_thickness = "1".to_string(); // Минимальная ширина реза
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "100".to_string(), "120".to_string(), 50, Some("Small Tile".to_string())), // Много маленьких плиток
        ],
        vec![
            Panel::new(1, "1000".to_string(), "800".to_string(), 3, Some("Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Оптимизация с минимальной шириной реза должна работать");
    assert!(duration.as_secs() < 20, "Оптимизация с минимальной шириной реза должна завершаться в разумное время");
    
    println!("Производительность с минимальной шириной реза: {:?}", duration);
}

/// Тест производительности с большой шириной реза
#[test]
fn test_performance_large_cut_width() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("large_cut_client".to_string());
    let mut config = Configuration::default();
    config.cut_thickness = "20".to_string(); // Большая ширина реза
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "300".to_string(), "400".to_string(), 15, Some("Large Tile".to_string())),
        ],
        vec![
            Panel::new(1, "2000".to_string(), "1500".to_string(), 2, Some("Large Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Оптимизация с большой шириной реза должна работать");
    assert!(duration.as_secs() < 15, "Оптимизация с большой шириной реза должна завершаться быстро");
    
    println!("Производительность с большой шириной реза: {:?}", duration);
}

/// Стресс-тест с комплексным сценарием
#[test]
fn test_stress_complex_scenario() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(8).unwrap();
    
    let client_info = ClientInfo::new("stress_client".to_string());
    let mut config = Configuration::default();
    config.cut_thickness = "5".to_string();
    
    // Создаем комплексный сценарий с множеством панелей и плиток
    let mut tiles = Vec::new();
    for i in 1..=8 {
        tiles.push(Panel::new(
            i, 
            (150 + (i * 25)).to_string(), 
            (200 + (i * 20)).to_string(), 
            8, 
            Some(format!("Tile Type {}", i))
        ));
    }
    
    let mut panels = Vec::new();
    for i in 1..=10 {
        panels.push(Panel::new(
            i, 
            (800 + (i * 50)).to_string(), 
            (600 + (i * 30)).to_string(), 
            2, 
            Some(format!("Panel Type {}", i))
        ));
    }
    
    let request = CalculationRequest::new(
        client_info,
        config,
        tiles,
        panels,
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Стресс-тест должен завершиться успешно");
    assert!(duration.as_secs() < 45, "Стресс-тест должен завершаться в разумное время");
    
    let response = result.unwrap();
    assert!(response.statistics.total_panels >= 0, "Стресс-тест должен иметь корректную статистику");
    
    println!("Производительность стресс-теста: {:?}", duration);
    println!("Статистика стресс-теста: {} панелей", response.statistics.total_panels);
}

/// Тест производительности с повторными запусками
#[test]
fn test_performance_multiple_runs() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("multiple_runs_client".to_string());
    let config = Configuration::default();
    
    let create_request = || {
        CalculationRequest::new(
            client_info.clone(),
            config.clone(),
            vec![
                Panel::new(1, "200".to_string(), "250".to_string(), 18, Some("Tile".to_string())),
            ],
            vec![
                Panel::new(1, "1200".to_string(), "800".to_string(), 3, Some("Panel".to_string())),
            ],
        )
    };
    
    let mut durations = Vec::new();
    let runs = 5;
    
    for run in 1..=runs {
        let start_time = Instant::now();
        let result = service.optimize(create_request());
        let duration = start_time.elapsed();
        
        assert!(result.is_ok(), "Запуск {} должен быть успешным", run);
        durations.push(duration);
        
        println!("Запуск {}: {:?}", run, duration);
    }
    
    // Вычисляем статистику
    let total_duration: Duration = durations.iter().sum();
    let avg_duration = total_duration / runs as u32;
    let max_duration = durations.iter().max().unwrap();
    let min_duration = durations.iter().min().unwrap();
    
    println!("Статистика производительности:");
    println!("  Среднее время: {:?}", avg_duration);
    println!("  Максимальное время: {:?}", max_duration);
    println!("  Минимальное время: {:?}", min_duration);
    
    // Проверяем стабильность производительности
    assert!(max_duration.as_secs() < 10, "Максимальное время должно быть приемлемым");
    assert!(avg_duration.as_secs() < 8, "Среднее время должно быть приемлемым");
    
    // Проверяем, что разброс времени не слишком большой
    let time_variance = max_duration.as_millis() - min_duration.as_millis();
    assert!(time_variance < 5000, "Разброс времени выполнения должен быть небольшим");
}

/// Тест производительности памяти (косвенная проверка)
#[test]
fn test_memory_performance() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("memory_client".to_string());
    let mut config = Configuration::default();
    config.cut_thickness = "2".to_string();
    
    // Создаем запрос, который может потреблять много памяти
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "100".to_string(), "150".to_string(), 200, Some("Tiny Tile".to_string())), // Много маленьких плиток
        ],
        vec![
            Panel::new(1, "5000".to_string(), "3000".to_string(), 10, Some("Huge Panel".to_string())),
        ],
    );
    
    let start_time = Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Тест производительности памяти должен завершиться успешно");
    assert!(duration.as_secs() < 60, "Тест производительности памяти должен завершаться в разумное время");
    
    println!("Производительность с большой нагрузкой на память: {:?}", duration);
}

/// Бенчмарк для сравнения производительности с разными конфигурациями
#[test]
fn benchmark_different_configurations() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(5).unwrap();
    
    let client_info = ClientInfo::new("benchmark_client".to_string());
    
    let cut_thicknesses = vec!["1", "3", "5", "10"];
    
    println!("\n=== Бенчмарк разных конфигураций ===");
    
    for cut_thickness in cut_thicknesses {
        let mut config = Configuration::default();
        config.cut_thickness = cut_thickness.to_string();
        
        let request = CalculationRequest::new(
            client_info.clone(),
            config,
            vec![
                Panel::new(1, "200".to_string(), "250".to_string(), 25, Some("Benchmark Tile".to_string())),
            ],
            vec![
                Panel::new(1, "1000".to_string(), "800".to_string(), 5, Some("Benchmark Panel".to_string())),
            ],
        );
        
        let mut durations = Vec::new();
        let runs = 3;
        
        for _ in 0..runs {
            let start_time = Instant::now();
            let result = service.optimize(request.clone());
            let duration = start_time.elapsed();
            
            assert!(result.is_ok(), "Бенчмарк для толщины реза '{}' должен работать", cut_thickness);
            durations.push(duration);
        }
        
        let avg_duration: Duration = durations.iter().sum::<Duration>() / runs as u32;
        println!("Толщина реза '{}': среднее время {:?}", cut_thickness, avg_duration);
    }
}
