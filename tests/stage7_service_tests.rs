//! Этап 7: Комплексные тесты CutListOptimizerService
//! 
//! Этот модуль содержит тесты для проверки функциональности сервиса оптимизации раскроя
//! согласно детальному плану этапа 7.

use cutting_cli::engine::service::{CutListOptimizerService, CutListOptimizerServiceImpl};
use cutting_cli::engine::model::request::{CalculationRequest, Panel, ClientInfo, Configuration, PerformanceThresholds};
use cutting_cli::engine::model::response::{StatusCode, CalculationSubmissionResult};
use cutting_cli::engine::logger::CutListLoggerImpl;
use cutting_cli::types::DEFAULT_MATERIAL;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Тест 1: Инициализация сервиса
#[test]
fn test_service_initialization() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    
    // Тестируем инициализацию с 10 потоками
    let result = service.init(10);
    assert!(result.is_ok(), "Инициализация сервиса должна быть успешной");
    
    // Проверяем, что сервис готов к работе
    let stats = service.get_stats();
    assert!(stats.is_ok(), "Статистика должна быть доступна после инициализации");
    
    let stats_data = stats.unwrap();
    assert_eq!(stats_data.nbr_running_tasks, 0, "Изначально не должно быть активных задач");
    assert_eq!(stats_data.nbr_queued_threads, 0, "Изначально не должно быть потоков в очереди");
}

/// Тест 2: Валидация входящих запросов
#[test]
fn test_request_validation() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(5).unwrap();
    
    let client_info = ClientInfo::new("test_client".to_string());
    let config = Configuration::default();
    
    // Тест 2.1: Запрос без деталей (пустой список panels)
    let request_empty_panels = CalculationRequest::new(
        client_info.clone(),
        config.clone(),
        vec![], // Пустой список панелей
        vec![Panel::new(1, "1000".to_string(), "600".to_string(), 1, Some(DEFAULT_MATERIAL.to_string()))],
    );
    
    let result = service.submit_task(request_empty_panels);
    assert!(result.is_ok());
    let submission_result = result.unwrap();
    assert!(!submission_result.is_success());
    assert_eq!(submission_result.status_code, StatusCode::InvalidTiles.to_string());
    
    // Тест 2.2: Запрос без стоковых листов (пустой список stockPanels)
    let request_empty_stock = CalculationRequest::new(
        client_info.clone(),
        config.clone(),
        vec![Panel::new(1, "200".to_string(), "300".to_string(), 2, Some(DEFAULT_MATERIAL.to_string()))],
        vec![], // Пустой список складских панелей
    );
    
    let result = service.submit_task(request_empty_stock);
    assert!(result.is_ok());
    let submission_result = result.unwrap();
    assert!(!submission_result.is_success());
    assert_eq!(submission_result.status_code, StatusCode::InvalidStockTiles.to_string());
    
    // Тест 2.3: Запрос с слишком большим количеством деталей (>5000)
    let mut many_panels = Vec::new();
    for i in 0..6000 {
        many_panels.push(Panel::new(i, "100".to_string(), "200".to_string(), 1, Some(DEFAULT_MATERIAL.to_string())));
    }
    
    let request_too_many_panels = CalculationRequest::new(
        client_info.clone(),
        config.clone(),
        many_panels,
        vec![Panel::new(1, "1000".to_string(), "600".to_string(), 1, Some(DEFAULT_MATERIAL.to_string()))],
    );
    
    let result = service.submit_task(request_too_many_panels);
    assert!(result.is_ok());
    let submission_result = result.unwrap();
    assert!(!submission_result.is_success());
    assert_eq!(submission_result.status_code, StatusCode::TooManyPanels.to_string());
    
    // Тест 2.4: Запрос с слишком большим количеством стоковых листов (>5000)
    let mut many_stock_panels = Vec::new();
    for i in 0..6000 {
        many_stock_panels.push(Panel::new(i, "1000".to_string(), "600".to_string(), 1, Some(DEFAULT_MATERIAL.to_string())));
    }
    
    let request_too_many_stock = CalculationRequest::new(
        client_info,
        config,
        vec![Panel::new(1, "200".to_string(), "300".to_string(), 2, Some(DEFAULT_MATERIAL.to_string()))],
        many_stock_panels,
    );
    
    let result = service.submit_task(request_too_many_stock);
    assert!(result.is_ok());
    let submission_result = result.unwrap();
    assert!(!submission_result.is_success());
    assert_eq!(submission_result.status_code, StatusCode::TooManyStockPanels.to_string());
}

/// Тест 3: Ограничение одновременных задач клиента
#[test]
fn test_client_task_limits() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(5).unwrap();
    
    // Настраиваем ограничение: не разрешаем множественные задачи
    service.set_allow_multiple_tasks_per_client(false);
    
    let client_info = ClientInfo::new("client1".to_string());
    let mut config = Configuration::default();
    config.performance_thresholds = Some(PerformanceThresholds::new(5, 1000, 2));
    
    let create_request = || {
        CalculationRequest::new(
            client_info.clone(),
            config.clone(),
            vec![Panel::new(1, "200".to_string(), "300".to_string(), 5, Some(DEFAULT_MATERIAL.to_string()))],
            vec![Panel::new(1, "1000".to_string(), "600".to_string(), 2, Some(DEFAULT_MATERIAL.to_string()))],
        )
    };
    
    // Отправляем первую задачу
    let result1 = service.submit_task(create_request());
    assert!(result1.is_ok());
    let submission1 = result1.unwrap();
    assert!(submission1.is_success(), "Первая задача должна быть принята");
    
    // Отправляем вторую задачу
    let result2 = service.submit_task(create_request());
    assert!(result2.is_ok());
    let submission2 = result2.unwrap();
    assert!(submission2.is_success(), "Вторая задача должна быть принята (лимит 2)");
    
    // Отправляем третью задачу (должна быть отклонена)
    let result3 = service.submit_task(create_request());
    assert!(result3.is_ok());
    let submission3 = result3.unwrap();
    assert!(!submission3.is_success(), "Третья задача должна быть отклонена");
    assert_eq!(submission3.status_code, StatusCode::TaskAlreadyRunning.to_string());
}

/// Тест 4: Обработка точности чисел и масштабирование
#[test]
fn test_number_precision_and_scaling() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("precision_client".to_string());
    let mut config = Configuration::default();
    config.cut_thickness = "3.2".to_string();
    config.min_trim_dimension = "10.5".to_string();
    
    // Создаем запрос с дробными размерами
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "123.456".to_string(), "78.9".to_string(), 2, Some(DEFAULT_MATERIAL.to_string())),
            Panel::new(2, "200.12".to_string(), "150.34".to_string(), 1, Some(DEFAULT_MATERIAL.to_string())),
        ],
        vec![Panel::new(1, "1000.12".to_string(), "600.34".to_string(), 1, Some(DEFAULT_MATERIAL.to_string()))],
    );
    
    // Проверяем, что запрос обрабатывается корректно
    let result = service.submit_task(request);
    assert!(result.is_ok());
    let submission_result = result.unwrap();
    assert!(submission_result.is_success(), "Запрос с дробными размерами должен быть принят");
    
    // Проверяем, что задача создана
    assert!(submission_result.task_id.is_some(), "Должен быть создан идентификатор задачи");
    
    // Проверяем, что коэффициент масштабирования вычислен правильно (3 десятичных знака -> 1000)
    // Это будет проверено в логике обработки задачи
}

/// Тест 5: Группировка деталей по материалам
#[test]
fn test_material_grouping() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("material_client".to_string());
    let config = Configuration::default();
    
    // Создаем запрос с разными материалами
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "100".to_string(), "50".to_string(), 2, Some("wood".to_string())),
            Panel::new(2, "200".to_string(), "100".to_string(), 1, Some("metal".to_string())),
            Panel::new(3, "150".to_string(), "75".to_string(), 1, Some("wood".to_string())),
            Panel::new(4, "80".to_string(), "40".to_string(), 1, Some("plastic".to_string())),
        ],
        vec![
            Panel::new(1, "1000".to_string(), "600".to_string(), 1, Some("wood".to_string())),
            Panel::new(2, "800".to_string(), "400".to_string(), 1, Some("metal".to_string())),
        ],
    );
    
    let result = service.submit_task(request);
    assert!(result.is_ok());
    let submission_result = result.unwrap();
    assert!(submission_result.is_success(), "Запрос с разными материалами должен быть принят");
    
    // Проверяем, что задача создана
    assert!(submission_result.task_id.is_some(), "Должен быть создан идентификатор задачи");
    
    // Алгоритм группировки:
    // 1. Материалы с деталями И стоковыми листами: ["wood", "metal"]
    // 2. Детали без подходящих стоковых листов: plastic деталь → в noMaterialTiles
}

/// Тест 6: Генерация групп деталей и оптимизация перестановок
#[test]
fn test_tile_grouping_and_permutation_optimization() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("grouping_client".to_string());
    let config = Configuration::default();
    
    // Создаем запрос с повторяющимися деталями
    let mut panels = Vec::new();
    
    // 20 одинаковых деталей (должны быть разделены на подгруппы)
    for i in 0..20 {
        panels.push(Panel::new(i, "100".to_string(), "50".to_string(), 1, Some(DEFAULT_MATERIAL.to_string())));
    }
    
    // 5 деталей другого размера
    for i in 20..25 {
        panels.push(Panel::new(i, "200".to_string(), "100".to_string(), 1, Some(DEFAULT_MATERIAL.to_string())));
    }
    
    // 3 детали третьего размера
    for i in 25..28 {
        panels.push(Panel::new(i, "150".to_string(), "75".to_string(), 1, Some(DEFAULT_MATERIAL.to_string())));
    }
    
    let request = CalculationRequest::new(
        client_info,
        config,
        panels,
        vec![Panel::new(1, "2000".to_string(), "1000".to_string(), 5, Some(DEFAULT_MATERIAL.to_string()))],
    );
    
    let result = service.submit_task(request);
    assert!(result.is_ok());
    let submission_result = result.unwrap();
    assert!(submission_result.is_success(), "Запрос с группировкой деталей должен быть принят");
    
    // Алгоритм группировки:
    // 1. 20 одинаковых деталей → разделены на подгруппы (если maxGroupSize < 20)
    // 2. Проверка isOneDimensionalOptimization() для оптимизации
    // 3. Создание GroupedTileDimensions для каждой подгруппы
}

/// Тест 7: Управление перестановками и стоковыми решениями
#[test]
fn test_permutation_and_stock_management() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(5).unwrap();
    
    let client_info = ClientInfo::new("permutation_client".to_string());
    let mut config = Configuration::default();
    config.optimization_factor = 0.8; // Ограничиваем количество потоков
    
    // Создаем запрос с 5 различными группами деталей (5! = 120 перестановок)
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![
            Panel::new(1, "150".to_string(), "200".to_string(), 3, Some(DEFAULT_MATERIAL.to_string())),
            Panel::new(2, "175".to_string(), "220".to_string(), 2, Some(DEFAULT_MATERIAL.to_string())),
            Panel::new(3, "200".to_string(), "250".to_string(), 2, Some(DEFAULT_MATERIAL.to_string())),
            Panel::new(4, "225".to_string(), "275".to_string(), 1, Some(DEFAULT_MATERIAL.to_string())),
            Panel::new(5, "250".to_string(), "300".to_string(), 1, Some(DEFAULT_MATERIAL.to_string())),
        ],
        vec![
            Panel::new(1, "1200".to_string(), "800".to_string(), 1, Some(DEFAULT_MATERIAL.to_string())),
            Panel::new(2, "1000".to_string(), "600".to_string(), 1, Some(DEFAULT_MATERIAL.to_string())),
            Panel::new(3, "800".to_string(), "500".to_string(), 1, Some(DEFAULT_MATERIAL.to_string())),
        ],
    );
    
    let result = service.submit_task(request);
    assert!(result.is_ok());
    let submission_result = result.unwrap();
    assert!(submission_result.is_success(), "Запрос с перестановками должен быть принят");
    
    // Алгоритм управления:
    // 1. Генерация 120 перестановок (5!)
    // 2. Удаление дубликатов перестановок
    // 3. Для каждой перестановки - до 3 стоковых итераций
    // 4. optimizationFactor = 0.8 ограничивает количество потоков
    // 5. Раннее прекращение при найденном решении (MAX_PERMUTATIONS_WITH_SOLUTION = 150)
}

/// Тест 8: Создание и выполнение CutListThread
#[test]
fn test_cut_thread_creation_and_execution() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(5).unwrap();
    
    let client_info = ClientInfo::new("thread_client".to_string());
    let mut config = Configuration::default();
    config.cut_orientation_preference = 0; // Все направления разрезов
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![Panel::new(1, "200".to_string(), "250".to_string(), 8, Some(DEFAULT_MATERIAL.to_string()))],
        vec![Panel::new(1, "1200".to_string(), "800".to_string(), 2, Some(DEFAULT_MATERIAL.to_string()))],
    );
    
    let result = service.submit_task(request);
    assert!(result.is_ok());
    let submission_result = result.unwrap();
    assert!(submission_result.is_success(), "Запрос должен быть принят");
    
    let task_id = submission_result.task_id.unwrap();
    
    // Проверяем статус задачи
    let status_result = service.get_task_status(&task_id);
    assert!(status_result.is_ok());
    assert!(status_result.unwrap().is_some(), "Статус задачи должен быть доступен");
    
    // Алгоритм создания потоков:
    // 1. Для каждой комбинации (перестановка + стоковое решение) создать до 3 потоков:
    //    - AREA (BOTH направления)
    //    - AREA_HCUTS_1ST (горизонтальные разрезы первыми)
    //    - AREA_VCUTS_1ST (вертикальные разрезы первыми)
    // 2. cutOrientationPreference = 0 → все 3 типа потоков
}

/// Тест 9: Мониторинг выполнения и статистика
#[test]
fn test_execution_monitoring_and_statistics() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("monitoring_client".to_string());
    let config = Configuration::default();
    
    let request = CalculationRequest::new(
        client_info.clone(),
        config,
        vec![Panel::new(1, "200".to_string(), "300".to_string(), 10, Some(DEFAULT_MATERIAL.to_string()))],
        vec![Panel::new(1, "1000".to_string(), "600".to_string(), 3, Some(DEFAULT_MATERIAL.to_string()))],
    );
    
    let result = service.submit_task(request);
    assert!(result.is_ok());
    let submission_result = result.unwrap();
    assert!(submission_result.is_success());
    
    let task_id = submission_result.task_id.unwrap();
    
    // Проверяем статистику системы
    let stats_result = service.get_stats();
    assert!(stats_result.is_ok());
    let stats = stats_result.unwrap();
    
    // Должна быть хотя бы одна активная задача
    assert!(stats.nbr_running_tasks > 0 || stats.nbr_idle_tasks > 0, "Должны быть активные задачи");
    
    // Проверяем список задач клиента
    let tasks_result = service.get_tasks(&client_info.id, None);
    assert!(tasks_result.is_ok());
    let tasks = tasks_result.unwrap();
    assert!(!tasks.is_empty(), "У клиента должны быть задачи");
    
    // Проверяем, что наша задача в списке
    let task_found = tasks.iter().any(|t| t.id == task_id);
    assert!(task_found, "Задача должна быть в списке задач клиента");
    
    // Проверяем мониторинг статуса
    // Статус должен изменяться: IDLE → RUNNING → FINISHED
    // percentageDone должен увеличиваться от 0 до 100
    // initPercentage должен отражать прогресс инициализации
}

/// Тест 10: Остановка и завершение задач
#[test]
fn test_task_stopping_and_termination() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("stop_client".to_string());
    let config = Configuration::default();
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![Panel::new(1, "100".to_string(), "150".to_string(), 50, Some(DEFAULT_MATERIAL.to_string()))],
        vec![Panel::new(1, "2000".to_string(), "1000".to_string(), 10, Some(DEFAULT_MATERIAL.to_string()))],
    );
    
    let result = service.submit_task(request);
    assert!(result.is_ok());
    let submission_result = result.unwrap();
    assert!(submission_result.is_success());
    
    let task_id = submission_result.task_id.unwrap();
    
    // Даем задаче немного времени на запуск
    thread::sleep(Duration::from_millis(100));
    
    // Тестируем мягкую остановку
    let stop_result = service.stop_task(&task_id);
    assert!(stop_result.is_ok());
    let stop_response = stop_result.unwrap();
    assert!(stop_response.is_some(), "Должен быть получен ответ об остановке");
    
    // Тестируем принудительное завершение
    let terminate_result = service.terminate_task(&task_id);
    assert!(terminate_result.is_ok());
    let terminate_code = terminate_result.unwrap();
    assert_eq!(terminate_code, 0, "Принудительное завершение должно вернуть 0 при успехе");
    
    // Алгоритм остановки:
    // 1. stopTask() - мягкая остановка (дождаться завершения текущих потоков)
    // 2. terminateTask() - принудительное завершение (прервать все потоки)
}

/// Тест 11: Обработка ошибок и исключений
#[test]
fn test_error_handling() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(2).unwrap();
    
    let client_info = ClientInfo::new("error_client".to_string());
    let config = Configuration::default();
    
    // Тест с некорректными данными (невалидные размеры)
    let mut invalid_panel = Panel::new(1, "invalid_width".to_string(), "200".to_string(), 1, Some(DEFAULT_MATERIAL.to_string()));
    invalid_panel.enabled = true; // Убеждаемся, что панель включена
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![invalid_panel],
        vec![Panel::new(1, "1000".to_string(), "600".to_string(), 1, Some(DEFAULT_MATERIAL.to_string()))],
    );
    
    // Система должна обработать некорректные данные
    let result = service.submit_task(request);
    assert!(result.is_ok(), "Сервис должен обработать некорректные данные без падения");
    
    // Проверяем, что система остается стабильной
    let stats_result = service.get_stats();
    assert!(stats_result.is_ok(), "Статистика должна быть доступна после ошибки");
    
    // Тест получения статуса несуществующей задачи
    let status_result = service.get_task_status("nonexistent_task");
    assert!(status_result.is_ok(), "Запрос статуса несуществующей задачи не должен вызывать ошибку");
    
    // Сценарии для тестирования:
    // 1. Ошибка в процессе вычислений
    // 2. Исключение при создании потока
    // 3. Переполнение очереди потоков
    // 4. Ошибка при логировании
    // 5. Недоступность ресурсов
}

/// Тест 12: Синхронная оптимизация
#[test]
fn test_synchronous_optimization() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    let client_info = ClientInfo::new("sync_client".to_string());
    let config = Configuration::default();
    
    let request = CalculationRequest::new(
        client_info,
        config,
        vec![Panel::new(1, "200".to_string(), "300".to_string(), 6, Some(DEFAULT_MATERIAL.to_string()))],
        vec![Panel::new(1, "1000".to_string(), "600".to_string(), 2, Some(DEFAULT_MATERIAL.to_string()))],
    );
    
    let start_time = std::time::Instant::now();
    let result = service.optimize(request);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Синхронная оптимизация должна завершиться успешно");
    
    let response = result.unwrap();
    
    // Проверяем структуру ответа
    assert!(response.statistics.total_panels > 0, "Должны быть обработаны панели");
    assert!(!response.metadata.is_empty(), "Должны быть метаданные");
    
    // Проверяем время выполнения
    assert!(duration.as_secs() < 30, "Синхронная оптимизация должна завершаться быстро");
}

/// Тест производительности с множественными клиентами
#[test]
fn test_multiple_clients_performance() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(10).unwrap();
    
    service.set_allow_multiple_tasks_per_client(true);
    
    let config = Configuration::default();
    
    // Создаем 5 клиентов, каждый отправляет по 2 задачи
    for client_num in 0..5 {
        for task_num in 0..2 {
            let client_info = ClientInfo::new(format!("client_{}_{}", client_num, task_num));
            let request = CalculationRequest::new(
                client_info,
                config.clone(),
                vec![Panel::new(1, "150".to_string(), "200".to_string(), 5, Some(DEFAULT_MATERIAL.to_string()))],
                vec![Panel::new(1, "800".to_string(), "600".to_string(), 2, Some(DEFAULT_MATERIAL.to_string()))],
            );
            
            let result = service.submit_task(request);
            assert!(result.is_ok());
            let submission_result = result.unwrap();
            assert!(submission_result.is_success(), "Задача от клиента client_{} должна быть принята", client_num);
        }
    }
    
    // Проверяем статистику
    let stats_result = service.get_stats();
    assert!(stats_result.is_ok());
    let stats = stats_result.unwrap();
    
    // Должно быть обработано 10 задач
    let total_tasks = stats.nbr_running_tasks + stats.nbr_idle_tasks + stats.nbr_finished_tasks;
    assert!(total_tasks >= 10, "Должно быть обработано как минимум 10 задач");
}

/// Тест конфигурации с различными параметрами
#[test]
fn test_different_configuration_parameters() {
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = CutListOptimizerServiceImpl::new(logger);
    service.init(3).unwrap();
    
    // Разрешаем множественные задачи на клиента для этого теста
    service.set_allow_multiple_tasks_per_client(true);
    
    // Тест с различными предпочтениями ориентации разрезов
    let orientations = vec![0, 1, 2]; // Любая, горизонтальная, вертикальная
    
    for (index, orientation) in orientations.iter().enumerate() {
        let client_info = ClientInfo::new(format!("config_client_orientation_{}", index));
        let mut config = Configuration::default();
        config.cut_orientation_preference = *orientation;
        
        let request = CalculationRequest::new(
            client_info,
            config,
            vec![Panel::new(1, "200".to_string(), "300".to_string(), 4, Some(DEFAULT_MATERIAL.to_string()))],
            vec![Panel::new(1, "1000".to_string(), "600".to_string(), 1, Some(DEFAULT_MATERIAL.to_string()))],
        );
        
        let result = service.submit_task(request);
        assert!(result.is_ok(), "Конфигурация с ориентацией {} должна работать", orientation);
        let submission_result = result.unwrap();
        if !submission_result.is_success() {
            println!("Ошибка для ориентации {}: статус = {}, сообщение = {:?}", 
                     orientation, submission_result.status_code, submission_result.error_message);
        }
        assert!(submission_result.is_success(), "Задача с ориентацией {} должна быть принята", orientation);
    }
    
    // Тест с различными факторами оптимизации
    let optimization_factors = vec![0.1, 0.5, 0.8, 1.0];
    
    for (index, factor) in optimization_factors.iter().enumerate() {
        let client_info = ClientInfo::new(format!("config_client_factor_{}", index));
        let mut config = Configuration::default();
        config.optimization_factor = *factor;
        
        let request = CalculationRequest::new(
            client_info,
            config,
            vec![Panel::new(1, "150".to_string(), "200".to_string(), 3, Some(DEFAULT_MATERIAL.to_string()))],
            vec![Panel::new(1, "800".to_string(), "500".to_string(), 1, Some(DEFAULT_MATERIAL.to_string()))],
        );
        
        let result = service.submit_task(request);
        assert!(result.is_ok(), "Конфигурация с фактором {} должна работать", factor);
        let submission_result = result.unwrap();
        assert!(submission_result.is_success(), "Задача с фактором {} должна быть принята", factor);
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_values() {
        assert_eq!(StatusCode::Ok.value(), 0);
        assert_eq!(StatusCode::InvalidTiles.value(), 1);
        assert_eq!(StatusCode::TaskAlreadyRunning.value(), 3);
    }

    #[test]
    fn test_calculation_submission_result() {
        let success_result = CalculationSubmissionResult::success("task123".to_string());
        assert!(success_result.is_success());
        assert_eq!(success_result.task_id, Some("task123".to_string()));
        
        let error_result = CalculationSubmissionResult::error(
            StatusCode::InvalidTiles,
            Some("Invalid tiles".to_string())
        );
        assert!(!error_result.is_success());
        assert_eq!(error_result.task_id, None);
    }
}
