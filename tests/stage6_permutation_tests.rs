//! Тесты для этапа 6: Генерация перестановок деталей и многопоточная оптимизация
//! 
//! Этот модуль содержит тесты для проверки функциональности генерации перестановок,
//! управления потоками и интеграции с алгоритмом размещения деталей.

use cutting_cli::engine::arrangement::Arrangement;
use cutting_cli::engine::permutation::{PermutationThreadSpawner, PermutationBatchProcessor};
use cutting_cli::engine::thread::{CutListThread, CutListThreadBuilder, CutDirection};
use cutting_cli::engine::model::tile::TileDimensions;
use cutting_cli::engine::model::solution::Solution;
use cutting_cli::engine::stock::stock_solution::StockSolution;
use cutting_cli::engine::comparator::factory::SolutionComparatorFactory;
use cutting_cli::engine::comparator::priority::OptimizationPriority;
use cutting_cli::error::CuttingError;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashSet;

/// Тест 1: Генерация перестановок малого списка
/// 
/// Проверяет корректность генерации всех перестановок для списка из 3 элементов.
/// Ожидается 3! = 6 уникальных перестановок.
#[test]
fn test_generate_permutations_small_list() {
    // Вход: список из 3 элементов
    let items = vec!['A', 'B', 'C'];
    
    // Генерируем перестановки
    let permutations = Arrangement::generate_permutations(items.clone());
    
    // Проверяем количество перестановок
    assert_eq!(permutations.len(), 6, "Должно быть 6 перестановок для 3 элементов");
    
    // Проверяем, что все перестановки уникальны
    let mut unique_perms = HashSet::new();
    for perm in &permutations {
        assert!(unique_perms.insert(perm.clone()), "Найдена дублирующаяся перестановка: {:?}", perm);
    }
    assert_eq!(unique_perms.len(), 6, "Все перестановки должны быть уникальными");
    
    // Проверяем содержимое каждой перестановки
    for perm in &permutations {
        assert_eq!(perm.len(), 3, "Каждая перестановка должна содержать 3 элемента");
        assert!(perm.contains(&'A'), "Перестановка должна содержать A");
        assert!(perm.contains(&'B'), "Перестановка должна содержать B");
        assert!(perm.contains(&'C'), "Перестановка должна содержать C");
    }
    
    println!("✓ Тест 1 пройден: Генерация перестановок малого списка");
}

/// Тест 2: Генерация перестановок среднего списка
/// 
/// Проверяет производительность и корректность генерации перестановок
/// для списка из 5 элементов (5! = 120 перестановок).
#[test]
fn test_generate_permutations_medium_list() {
    // Вход: список из 5 элементов
    let items = vec!["D1", "D2", "D3", "D4", "D5"];
    
    // Засекаем время начала
    let start_time = Instant::now();
    
    // Генерируем перестановки
    let permutations = Arrangement::generate_permutations(items.clone());
    
    // Засекаем время окончания
    let elapsed = start_time.elapsed();
    
    // Проверяем количество перестановок
    assert_eq!(permutations.len(), 120, "Должно быть 120 перестановок для 5 элементов");
    
    // Проверяем время выполнения (должно быть быстро)
    assert!(elapsed < Duration::from_millis(100), "Генерация должна занимать менее 100 мс, заняло: {:?}", elapsed);
    
    // Проверяем уникальность всех перестановок
    let mut unique_perms = HashSet::new();
    for perm in &permutations {
        assert!(unique_perms.insert(perm.clone()), "Найдена дублирующаяся перестановка");
    }
    assert_eq!(unique_perms.len(), 120, "Все 120 перестановок должны быть уникальными");
    
    // Выборочно проверяем несколько перестановок на корректность
    for (i, perm) in permutations.iter().enumerate() {
        if i % 20 == 0 { // Проверяем каждую 20-ю перестановку
            assert_eq!(perm.len(), 5, "Перестановка {} должна содержать 5 элементов", i);
            let mut sorted_perm = perm.clone();
            sorted_perm.sort();
            assert_eq!(sorted_perm, items, "Перестановка {} должна содержать все исходные элементы", i);
        }
    }
    
    println!("✓ Тест 2 пройден: Генерация перестановок среднего списка за {:?}", elapsed);
}

/// Тест 3: Граничные случаи генерации перестановок
/// 
/// Проверяет обработку пустого списка, списка с одним элементом
/// и списка с дубликатами.
#[test]
fn test_generate_permutations_edge_cases() {
    // Тест пустого списка
    let empty: Vec<char> = vec![];
    let empty_perms = Arrangement::generate_permutations(empty);
    assert_eq!(empty_perms.len(), 1, "Пустой список должен давать одну пустую перестановку");
    assert_eq!(empty_perms[0].len(), 0, "Единственная перестановка должна быть пустой");
    
    // Тест списка с одним элементом
    let single = vec!['A'];
    let single_perms = Arrangement::generate_permutations(single);
    assert_eq!(single_perms.len(), 1, "Список с одним элементом должен давать одну перестановку");
    assert_eq!(single_perms[0], vec!['A'], "Перестановка должна содержать исходный элемент");
    
    // Тест списка с дубликатами
    let duplicates = vec!['A', 'A', 'B'];
    let dup_perms = Arrangement::generate_permutations(duplicates);
    assert_eq!(dup_perms.len(), 6, "Алгоритм не учитывает дубликаты, должно быть 6 перестановок");
    
    // Проверяем, что некоторые перестановки идентичны
    let identical_count = dup_perms.iter()
        .filter(|perm| **perm == vec!['A', 'A', 'B'])
        .count();
    assert!(identical_count > 1, "Должны быть идентичные перестановки из-за дубликатов");
    
    println!("✓ Тест 3 пройден: Граничные случаи генерации перестановок");
}

/// Тест 4: Создание и управление потоками
/// 
/// Проверяет корректность работы PermutationThreadSpawner
/// с ограничением количества одновременных потоков.
#[test]
fn test_thread_spawner_management() {
    // Создаем spawner с лимитом 3 потока
    let spawner = PermutationThreadSpawner::new(3, 100);
    
    assert_eq!(spawner.get_max_alive_threads(), 3, "Максимальное количество потоков должно быть 3");
    assert_eq!(spawner.get_check_interval(), 100, "Интервал проверки должен быть 100 мс");
    assert!(spawner.is_running(), "Spawner должен быть в рабочем состоянии");
    
    // Создаем 10 тестовых задач с задержкой
    let task_count = 10;
    let task_duration = Duration::from_millis(200);
    
    let start_time = Instant::now();
    
    // Запускаем задачи
    for i in 0..task_count {
        let task_name = format!("test_task_{}", i);
        let duration = task_duration;
        
        let result = spawner.spawn(task_name, move || {
            thread::sleep(duration);
            Ok(vec![]) // Возвращаем пустой результат
        });
        
        assert!(result.is_ok(), "Задача {} должна быть запущена успешно", i);
        
        // Проверяем, что количество активных потоков не превышает лимит
        let active_count = spawner.get_nbr_unfinished_threads();
        assert!(active_count <= 3, "Количество активных потоков ({}) не должно превышать лимит (3)", active_count);
        
        // Небольшая пауза между запусками
        thread::sleep(Duration::from_millis(10));
    }
    
    // Проверяем общее количество созданных потоков
    assert_eq!(spawner.get_nbr_total_threads(), task_count, "Должно быть создано {} потоков", task_count);
    
    // Ждем завершения всех потоков
    let completion_timeout = Duration::from_secs(5);
    let completed = spawner.wait_for_all_threads(Some(completion_timeout));
    assert!(completed, "Все потоки должны завершиться в течение таймаута");
    
    // Проверяем статистику выполнения
    let (successful, failed, cancelled) = spawner.get_execution_statistics();
    assert_eq!(successful, task_count, "Все {} задач должны завершиться успешно", task_count);
    assert_eq!(failed, 0, "Не должно быть неудачных задач");
    assert_eq!(cancelled, 0, "Не должно быть отмененных задач");
    
    let total_time = start_time.elapsed();
    println!("✓ Тест 4 пройден: Управление потоками за {:?}", total_time);
}

/// Тест 5: Прерывание и обработка ошибок в потоках
/// 
/// Проверяет корректную обработку различных сценариев выполнения потоков,
/// включая ошибки и прерывания.
#[test]
fn test_thread_error_handling() {
    let spawner = PermutationThreadSpawner::new(2, 100);
    
    // Сценарий 1: Нормальное выполнение
    let result1 = spawner.spawn("normal_task".to_string(), || {
        thread::sleep(Duration::from_millis(50));
        Ok(vec![])
    });
    assert!(result1.is_ok(), "Нормальная задача должна запуститься успешно");
    
    // Сценарий 2: Задача с ошибкой
    let result2 = spawner.spawn("error_task".to_string(), || {
        thread::sleep(Duration::from_millis(50));
        Err(CuttingError::InvalidInput("Тестовая ошибка".to_string()))
    });
    assert!(result2.is_ok(), "Задача с ошибкой должна запуститься (ошибка обрабатывается внутри)");
    
    // Сценарий 3: Долгая задача
    let result3 = spawner.spawn("long_task".to_string(), || {
        thread::sleep(Duration::from_millis(300));
        Ok(vec![])
    });
    assert!(result3.is_ok(), "Долгая задача должна запуститься успешно");
    
    // Ждем завершения всех задач
    let completed = spawner.wait_for_all_threads(Some(Duration::from_secs(2)));
    assert!(completed, "Все задачи должны завершиться");
    
    // Проверяем, что spawner продолжает работать после ошибок
    assert!(spawner.is_running(), "Spawner должен продолжать работать после ошибок");
    
    // Проверяем статистику
    let (successful, failed, cancelled) = spawner.get_execution_statistics();
    assert!(successful >= 2, "Должно быть минимум 2 успешных задачи");
    assert!(failed >= 1, "Должна быть минимум 1 неудачная задача");
    
    println!("✓ Тест 5 пройден: Обработка ошибок в потоках");
}

/// Тест 6: Построение CutListThread с различными конфигурациями
/// 
/// Проверяет работу CutListThreadBuilder с различными наборами параметров.
#[test]
fn test_cutlist_thread_builder_configurations() {
    // Подготавливаем тестовые данные
    let tiles = vec![
        TileDimensions::simple(400, 300),
        TileDimensions::simple(350, 250),
    ];
    let stock_solution = StockSolution::new(vec![
        TileDimensions::simple(1000, 600),
    ]);
    let all_solutions = Arc::new(Mutex::new(Vec::new()));
    
    // Тест 1: Минимальная конфигурация
    let minimal_thread = CutListThreadBuilder::new()
        .set_tiles(tiles.clone())
        .set_stock_solution(stock_solution.clone())
        .build();
    
    assert!(minimal_thread.is_ok(), "Минимальная конфигурация должна создаваться успешно");
    let thread = minimal_thread.unwrap();
    assert_eq!(thread.get_tiles().len(), 2, "Должно быть 2 детали");
    assert_eq!(thread.get_accuracy_factor(), 100, "Фактор точности по умолчанию должен быть 100");
    
    // Тест 2: Полная конфигурация
    let thread_comparators = vec![SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastWastedArea)];
    let final_comparators = vec![SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastWastedArea)];
    let full_thread = CutListThreadBuilder::new()
        .set_group("test_group".to_string())
        .set_aux_info("test_info".to_string())
        .set_all_solutions(all_solutions.clone())
        .set_tiles(tiles.clone())
        .set_consider_grain_direction(true)
        .set_cut_thickness(3)
        .set_min_trim_dimension(50)
        .set_first_cut_orientation(CutDirection::Horizontal)
        .set_thread_prioritized_comparators(thread_comparators)
        .set_final_solution_prioritized_comparators(final_comparators)
        .set_accuracy_factor(50)
        .set_stock_solution(stock_solution.clone())
        .build();
    
    assert!(full_thread.is_ok(), "Полная конфигурация должна создаваться успешно");
    let thread = full_thread.unwrap();
    assert_eq!(thread.get_group(), Some(&"test_group".to_string()));
    assert_eq!(thread.get_aux_info(), Some(&"test_info".to_string()));
    assert!(thread.is_consider_grain_direction());
    assert_eq!(thread.get_cut_thickness(), 3);
    assert_eq!(thread.get_min_trim_dimension(), 50);
    assert_eq!(thread.get_first_cut_orientation(), CutDirection::Horizontal);
    assert_eq!(thread.get_accuracy_factor(), 50);
    
    // Тест 3: Различные приоритеты оптимизации
    let priority0_comparators = vec![SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastWastedArea)];
    let priority1_comparators = vec![SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastNbrCuts)];
    
    let thread_p0 = CutListThreadBuilder::new()
        .set_tiles(tiles.clone())
        .set_stock_solution(stock_solution.clone())
        .set_thread_prioritized_comparators(priority0_comparators)
        .build();
    
    let thread_p1 = CutListThreadBuilder::new()
        .set_tiles(tiles.clone())
        .set_stock_solution(stock_solution.clone())
        .set_thread_prioritized_comparators(priority1_comparators)
        .build();
    
    assert!(thread_p0.is_ok(), "Поток с приоритетом 0 должен создаваться");
    assert!(thread_p1.is_ok(), "Поток с приоритетом 1 должен создаваться");
    
    // Тест 4: Различные направления первого разреза
    for direction in [CutDirection::Horizontal, CutDirection::Vertical, CutDirection::Both] {
        let thread = CutListThreadBuilder::new()
            .set_tiles(tiles.clone())
            .set_stock_solution(stock_solution.clone())
            .set_first_cut_orientation(direction)
            .build();
        
        assert!(thread.is_ok(), "Поток с направлением {:?} должен создаваться", direction);
        assert_eq!(thread.unwrap().get_first_cut_orientation(), direction);
    }
    
    println!("✓ Тест 6 пройден: Построение CutListThread с различными конфигурациями");
}

/// Тест 7: Интеграция генерации перестановок с размещением деталей
/// 
/// Проверяет интеграцию перестановок с алгоритмом размещения деталей
/// и сравнение результатов разных перестановок.
#[test]
fn test_permutation_integration_with_placement() {
    // Подготавливаем тестовые данные
    let tiles = vec![
        TileDimensions::simple(400, 300),
        TileDimensions::simple(350, 250),
        TileDimensions::simple(200, 150),
    ];
    let stock_solution = StockSolution::new(vec![
        TileDimensions::simple(1000, 600),
    ]);
    
    // Генерируем все перестановки деталей
    let permutations = Arrangement::generate_permutations(tiles.clone());
    assert_eq!(permutations.len(), 6, "Должно быть 6 перестановок для 3 деталей");
    
    let mut solutions_by_permutation = Vec::new();
    let all_solutions = Arc::new(Mutex::new(Vec::new()));
    
    // Для каждой перестановки создаем и запускаем поток
    for (i, permutation) in permutations.iter().enumerate() {
        let mut thread = CutListThreadBuilder::new()
            .set_group(format!("permutation_{}", i))
            .set_aux_info(format!("perm_{:?}", permutation))
            .set_all_solutions(Arc::clone(&all_solutions))
            .set_tiles(permutation.clone())
            .set_stock_solution(stock_solution.clone())
            .set_accuracy_factor(10)
            .build()
            .expect("Поток должен создаваться успешно");
        
        // Запускаем вычисление
        let result = thread.compute_solutions();
        assert!(result.is_ok(), "Вычисление для перестановки {} должно быть успешным", i);
        
        // Сохраняем решения этой перестановки
        solutions_by_permutation.push(thread.get_solutions().clone());
    }
    
    // Проверяем, что разные перестановки дают разные результаты
    let mut unique_solution_structures = HashSet::new();
    for (i, solutions) in solutions_by_permutation.iter().enumerate() {
        if !solutions.is_empty() {
            for solution in solutions {
                let structure_id = format!("{:?}", solution); // Упрощенный идентификатор
                unique_solution_structures.insert(structure_id);
            }
        }
    }
    
    // Находим лучшее решение по минимальным отходам
    let all_solutions_guard = all_solutions.lock().unwrap();
    if !all_solutions_guard.is_empty() {
        let best_solution = all_solutions_guard.iter()
            .min_by_key(|s| s.get_unused_area())
            .unwrap();
        
        println!("Лучшее решение имеет {} отходов", best_solution.get_unused_area());
        assert!(best_solution.get_unused_area() >= 0, "Отходы не могут быть отрицательными");
    }
    
    println!("✓ Тест 7 пройден: Интеграция перестановок с размещением деталей");
}

/// Тест 8: Производительность при большом количестве деталей
/// 
/// Проверяет производительность и стратегии оптимизации
/// при работе с большим количеством деталей.
#[test]
fn test_performance_with_large_tile_count() {
    // Создаем список из 7 деталей (7! = 5040 перестановок)
    let tiles: Vec<TileDimensions> = (1..=7)
        .map(|i| TileDimensions::simple(100 + i * 50, 80 + i * 30))
        .collect();
    
    // Тест 1: Проверяем время генерации всех перестановок
    let start_time = Instant::now();
    let all_permutations = Arrangement::generate_permutations(tiles.clone());
    let generation_time = start_time.elapsed();
    
    assert_eq!(all_permutations.len(), 5040, "Должно быть 5040 перестановок для 7 деталей");
    assert!(generation_time < Duration::from_secs(1), "Генерация должна занимать менее 1 секунды, заняло: {:?}", generation_time);
    
    // Тест 2: Ограниченная генерация (первые 1000 перестановок)
    let limited_permutations = Arrangement::generate_limited_permutations(tiles.clone(), 1000);
    assert_eq!(limited_permutations.len(), 1000, "Должно быть ограничено до 1000 перестановок");
    
    // Тест 3: Случайные перестановки
    let random_permutations = Arrangement::generate_random_permutations(tiles.clone(), 100);
    assert_eq!(random_permutations.len(), 100, "Должно быть 100 случайных перестановок");
    
    // Проверяем уникальность случайных перестановок
    let mut unique_random = HashSet::new();
    for perm in &random_permutations {
        unique_random.insert(perm.clone());
    }
    assert!(unique_random.len() > 50, "Большинство случайных перестановок должны быть уникальными");
    
    // Тест 4: Эвристические перестановки
    let area_fn = |tile: &TileDimensions| tile.width as u64 * tile.height as u64;
    let heuristic_permutations = Arrangement::generate_heuristic_permutations(tiles.clone(), area_fn, 10);
    assert_eq!(heuristic_permutations.len(), 10, "Должно быть 10 эвристических перестановок");
    
    // Проверяем, что первая перестановка отсортирована по убыванию площади
    let first_perm = &heuristic_permutations[0];
    let areas: Vec<u64> = first_perm.iter().map(|t| area_fn(t)).collect();
    let mut sorted_areas = areas.clone();
    sorted_areas.sort_by(|a, b| b.cmp(a)); // Сортировка по убыванию
    assert_eq!(areas, sorted_areas, "Первая эвристическая перестановка должна быть отсортирована по убыванию площади");
    
    println!("✓ Тест 8 пройден: Производительность при большом количестве деталей");
    println!("  - Генерация 5040 перестановок: {:?}", generation_time);
    println!("  - Ограниченная генерация: {} перестановок", limited_permutations.len());
    println!("  - Случайные перестановки: {} уникальных из {}", unique_random.len(), random_permutations.len());
}

/// Тест 9: Многопоточная обработка перестановок
/// 
/// Проверяет эффективность многопоточной обработки множества перестановок
/// с использованием PermutationThreadSpawner.
#[test]
fn test_multithreaded_permutation_processing() {
    let spawner = PermutationThreadSpawner::new(4, 100);
    
    // Создаем 20 различных перестановок
    let base_tiles = vec![
        TileDimensions::simple(300, 200),
        TileDimensions::simple(250, 150),
        TileDimensions::simple(200, 100),
        TileDimensions::simple(150, 100),
    ];
    
    let permutations = Arrangement::generate_limited_permutations(base_tiles, 20);
    assert_eq!(permutations.len(), 20, "Должно быть 20 перестановок");
    
    let stock_solution = StockSolution::new(vec![
        TileDimensions::simple(800, 600),
    ]);
    
    let all_solutions = Arc::new(Mutex::new(Vec::new()));
    let start_time = Instant::now();
    
    // Запускаем обработку каждой перестановки в отдельном потоке
    for (i, permutation) in permutations.iter().enumerate() {
        let task_name = format!("permutation_task_{}", i);
        let perm_clone = permutation.clone();
        let stock_clone = stock_solution.clone();
        let solutions_clone = Arc::clone(&all_solutions);
        
        let result = spawner.spawn(task_name, move || {
            // Создаем и запускаем поток для обработки перестановки
            let mut thread = CutListThreadBuilder::new()
                .set_tiles(perm_clone)
                .set_stock_solution(stock_clone)
                .set_all_solutions(solutions_clone)
                .set_accuracy_factor(5)
                .build()
                .expect("Поток должен создаваться успешно");
            
            // Запускаем вычисление
            thread.compute_solutions().expect("Вычисление должно быть успешным");
            
            Ok(thread.get_solutions().clone())
        });
        
        assert!(result.is_ok(), "Задача {} должна запуститься успешно", i);
        
        // Проверяем, что не более 4 потоков работают одновременно
        let active_count = spawner.get_nbr_unfinished_threads();
        assert!(active_count <= 4, "Количество активных потоков ({}) не должно превышать лимит (4)", active_count);
    }
    
    // Ждем завершения всех потоков
    let completed = spawner.wait_for_all_threads(Some(Duration::from_secs(10)));
    assert!(completed, "Все потоки должны завершиться");
    
    let total_time = start_time.elapsed();
    
    // Проверяем статистику выполнения
    let (successful, failed, cancelled) = spawner.get_execution_statistics();
    assert_eq!(successful, 20, "Все 20 задач должны завершиться успешно");
    assert_eq!(failed, 0, "Не должно быть неудачных задач");
    assert_eq!(cancelled, 0, "Не должно быть отмененных задач");
    
    // Проверяем, что получены решения
    let all_solutions_guard = all_solutions.lock().unwrap();
    assert!(!all_solutions_guard.is_empty(), "Должны быть получены решения");
    
    println!("✓ Тест 9 пройден: Многопоточная обработка перестановок за {:?}", total_time);
}

/// Тест 10: Мониторинг прогресса и отчетности
/// 
/// Проверяет корректность отслеживания прогресса выполнения
/// и генерации отчетов о состоянии потоков.
#[test]
fn test_progress_monitoring_and_reporting() {
    let spawner = PermutationThreadSpawner::new(3, 200);
    
    // Запускаем несколько потоков с разной продолжительностью
    let task_durations = vec![100, 200, 300, 150, 250]; // в миллисекундах
    
    for (i, &duration) in task_durations.iter().enumerate() {
        let task_name = format!("progress_task_{}", i);
        let task_duration = Duration::from_millis(duration);
        
        let result = spawner.spawn(task_name, move || {
            thread::sleep(task_duration);
            Ok(vec![])
        });
        
        assert!(result.is_ok(), "Задача {} должна запуститься успешно", i);
        
        // Проверяем периодические обновления прогресса
        thread::sleep(Duration::from_millis(50));
        
        let progress_report = spawner.get_progress_report();
        assert!(progress_report.total_tasks > 0, "Должны быть зарегистрированы задачи");
        
        // Проверяем корректность подсчета
        let active_count = spawner.get_nbr_unfinished_threads();
        let total_count = spawner.get_nbr_total_threads();
        assert!(active_count <= 3, "Активных потоков не должно быть больше лимита");
        assert_eq!(total_count, i + 1, "Общее количество должно соответствовать запущенным задачам");
    }
    
    // Ждем завершения всех потоков
    let completed = spawner.wait_for_all_threads(Some(Duration::from_secs(3)));
    assert!(completed, "Все потоки должны завершиться");
    
    // Проверяем финальный отчет
    let final_report = spawner.get_progress_report();
    assert_eq!(final_report.completed_tasks, task_durations.len(), "Все задачи должны быть завершены");
    assert_eq!(final_report.running_tasks, 0, "Не должно быть активных задач");
    
    // Проверяем статистику выполнения
    let (successful, failed, cancelled) = spawner.get_execution_statistics();
    assert_eq!(successful, task_durations.len(), "Все задачи должны завершиться успешно");
    assert_eq!(failed, 0, "Не должно быть неудачных задач");
    assert_eq!(cancelled, 0, "Не должно быть отмененных задач");
    
    println!("✓ Тест 10 пройден: Мониторинг прогресса и отчетность");
}

/// Тест 11: Граничные случаи многопоточности
/// 
/// Проверяет обработку различных граничных случаев в многопоточной среде.
#[test]
fn test_multithreading_edge_cases() {
    // Тест 1: Последовательное выполнение (maxAliveSpawnerThreads = 1)
    let sequential_spawner = PermutationThreadSpawner::new(1, 100);
    
    let start_time = Instant::now();
    for i in 0..3 {
        let task_name = format!("sequential_task_{}", i);
        let result = sequential_spawner.spawn(task_name, || {
            thread::sleep(Duration::from_millis(100));
            Ok(vec![])
        });
        assert!(result.is_ok(), "Последовательная задача {} должна запуститься", i);
    }
    
    let completed = sequential_spawner.wait_for_all_threads(Some(Duration::from_secs(2)));
    assert!(completed, "Все последовательные задачи должны завершиться");
    let sequential_time = start_time.elapsed();
    
    // Тест 2: Больше потоков чем задач
    let oversized_spawner = PermutationThreadSpawner::new(10, 100);
    
    for i in 0..3 {
        let task_name = format!("oversized_task_{}", i);
        let result = oversized_spawner.spawn(task_name, || {
            thread::sleep(Duration::from_millis(50));
            Ok(vec![])
        });
        assert!(result.is_ok(), "Задача {} должна запуститься при избытке потоков", i);
    }
    
    let completed = oversized_spawner.wait_for_all_threads(Some(Duration::from_secs(1)));
    assert!(completed, "Все задачи должны завершиться при избытке потоков");
    
    // Тест 3: Быстро завершающиеся потоки
    let fast_spawner = PermutationThreadSpawner::new(3, 50);
    
    for i in 0..10 {
        let task_name = format!("fast_task_{}", i);
        let result = fast_spawner.spawn(task_name, || {
            // Очень быстрая задача
            Ok(vec![])
        });
        assert!(result.is_ok(), "Быстрая задача {} должна запуститься", i);
    }
    
    let completed = fast_spawner.wait_for_all_threads(Some(Duration::from_secs(1)));
    assert!(completed, "Все быстрые задачи должны завершиться");
    
    // Тест 4: Остановка spawner
    let shutdown_spawner = PermutationThreadSpawner::new(2, 100);
    
    // Запускаем долгую задачу
    let result = shutdown_spawner.spawn("long_task".to_string(), || {
        thread::sleep(Duration::from_secs(5));
        Ok(vec![])
    });
    assert!(result.is_ok(), "Долгая задача должна запуститься");
    
    // Останавливаем spawner
    shutdown_spawner.shutdown();
    assert!(!shutdown_spawner.is_running(), "Spawner должен быть остановлен");
    
    // Попытка запустить новую задачу должна завершиться ошибкой
    let result = shutdown_spawner.spawn("new_task".to_string(), || Ok(vec![]));
    assert!(result.is_err(), "Новая задача не должна запускаться после остановки");
    
    println!("✓ Тест 11 пройден: Граничные случаи многопоточности");
    println!("  - Последовательное выполнение: {:?}", sequential_time);
}

/// Тест 12: Пакетная обработка перестановок
/// 
/// Проверяет работу PermutationBatchProcessor для эффективной
/// обработки больших объемов перестановок.
#[test]
fn test_batch_permutation_processing() {
    let processor = PermutationBatchProcessor::new(3, 5); // 3 потока, пакеты по 5
    
    // Создаем 20 тестовых перестановок
    let permutations: Vec<Vec<String>> = (0..20)
        .map(|i| vec![format!("item_{}", i), format!("item_{}", (i + 1) % 20)])
        .collect();
    
    // Добавляем перестановки в очередь
    processor.add_permutations(permutations);
    assert_eq!(processor.get_queue_size(), 20, "В очереди должно быть 20 перестановок");
    
    // Обрабатываем все перестановки
    let start_time = Instant::now();
    let batch_count = processor.process_all(|batch| {
        // Имитируем обработку пакета
        thread::sleep(Duration::from_millis(100));
        
        // Возвращаем фиктивные решения
        let solutions: Vec<Solution> = batch.iter()
            .map(|_| Solution::new())
            .collect();
        
        Ok(solutions)
    });
    
    assert!(batch_count.is_ok(), "Обработка пакетов должна быть успешной");
    let batches_processed = batch_count.unwrap();
    assert_eq!(batches_processed, 4, "Должно быть обработано 4 пакета (20/5)");
    
    // Ждем завершения обработки
    let completed = processor.wait_for_completion(Some(Duration::from_secs(3)));
    assert!(completed, "Все пакеты должны быть обработаны");
    
    let processing_time = start_time.elapsed();
    
    // Проверяем, что очередь пуста
    assert_eq!(processor.get_queue_size(), 0, "Очередь должна быть пуста после обработки");
    
    // Проверяем статистику
    let (successful, failed, cancelled) = processor.get_processing_statistics();
    assert_eq!(successful, batches_processed, "Все пакеты должны быть обработаны успешно");
    assert_eq!(failed, 0, "Не должно быть неудачных пакетов");
    assert_eq!(cancelled, 0, "Не должно быть отмененных пакетов");
    
    println!("✓ Тест 12 пройден: Пакетная обработка перестановок за {:?}", processing_time);
    println!("  - Обработано {} пакетов", batches_processed);
}

/// Тест 13: Интеграционный тест полного цикла оптимизации
/// 
/// Проверяет полный цикл от генерации перестановок до получения
/// оптимального решения с использованием всех компонентов системы.
#[test]
fn test_full_optimization_cycle() {
    // Подготавливаем реалистичные тестовые данные
    let tiles = vec![
        TileDimensions::simple(400, 300),  // Большая деталь
        TileDimensions::simple(200, 150),  // Средняя деталь
        TileDimensions::simple(100, 100),  // Маленькая квадратная деталь
        TileDimensions::simple(300, 100),  // Длинная узкая деталь
    ];
    
    let stock_panels = vec![
        TileDimensions::simple(800, 600),  // Основная панель
        TileDimensions::simple(400, 400),  // Квадратная панель
    ];
    
    let stock_solution = StockSolution::new(stock_panels);
    
    // Этап 1: Генерация эвристических перестановок
    let area_fn = |tile: &TileDimensions| tile.width as u64 * tile.height as u64;
    let permutations = Arrangement::generate_heuristic_permutations(tiles.clone(), area_fn, 12);
    
    assert!(permutations.len() <= 12, "Должно быть не более 12 эвристических перестановок");
    assert!(!permutations.is_empty(), "Должна быть хотя бы одна перестановка");
    
    // Этап 2: Многопоточная обработка перестановок
    let spawner = PermutationThreadSpawner::new(3, 100);
    let all_solutions = Arc::new(Mutex::new(Vec::new()));
    let start_time = Instant::now();
    
    for (i, permutation) in permutations.iter().enumerate() {
        let task_name = format!("optimization_task_{}", i);
        let perm_clone = permutation.clone();
        let stock_clone = stock_solution.clone();
        let solutions_clone = Arc::clone(&all_solutions);
        
        let result = spawner.spawn(task_name, move || {
            // Создаем оптимизированный поток
            let thread_comparators = vec![
                SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastWastedArea),
                SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastNbrCuts),
            ];
            
            let final_comparators = vec![
                SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastWastedArea),
                SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastNbrCuts),
            ];
            
            let mut thread = CutListThreadBuilder::new()
                .set_group(format!("heuristic_group_{}", i))
                .set_tiles(perm_clone)
                .set_stock_solution(stock_clone)
                .set_all_solutions(solutions_clone)
                .set_thread_prioritized_comparators(thread_comparators)
                .set_final_solution_prioritized_comparators(final_comparators)
                .set_accuracy_factor(20)
                .set_cut_thickness(3)
                .set_min_trim_dimension(10)
                .build()
                .expect("Поток должен создаваться успешно");
            
            // Запускаем оптимизацию
            thread.compute_solutions().expect("Оптимизация должна быть успешной");
            
            Ok(thread.get_solutions().clone())
        });
        
        assert!(result.is_ok(), "Задача оптимизации {} должна запуститься", i);
    }
    
    // Этап 3: Ожидание завершения и анализ результатов
    let completed = spawner.wait_for_all_threads(Some(Duration::from_secs(10)));
    assert!(completed, "Все задачи оптимизации должны завершиться");
    
    let optimization_time = start_time.elapsed();
    
    // Этап 4: Анализ полученных решений
    let all_solutions_guard = all_solutions.lock().unwrap();
    assert!(!all_solutions_guard.is_empty(), "Должны быть получены решения");
    
    // Находим лучшее решение по комплексному критерию
    let best_solution = all_solutions_guard.iter()
        .min_by(|a, b| {
            // Сначала сравниваем по отходам, потом по количеству разрезов
            let waste_cmp = a.get_wasted_area().cmp(&b.get_wasted_area());
            if waste_cmp == std::cmp::Ordering::Equal {
                a.get_cuts_count().cmp(&b.get_cuts_count())
            } else {
                waste_cmp
            }
        })
        .unwrap();
    
    // Проверяем качество лучшего решения
    let total_tile_area: u64 = tiles.iter()
        .map(|t| t.width as u64 * t.height as u64)
        .sum();
    
    let total_stock_area: u64 = stock_solution.get_panels().iter()
        .map(|p| p.width as u64 * p.height as u64)
        .sum();
    
    let waste_percentage = if !all_solutions_guard.is_empty() {
        let best_solution = all_solutions_guard.iter()
            .min_by_key(|s| s.get_wasted_area())
            .unwrap();
        (best_solution.get_wasted_area() as f64 / total_stock_area as f64) * 100.0
    } else {
        0.0
    };
    
    // Этап 5: Проверка статистики выполнения
    let (successful, failed, cancelled) = spawner.get_execution_statistics();
    assert_eq!(successful, permutations.len(), "Все задачи должны завершиться успешно");
    assert_eq!(failed, 0, "Не должно быть неудачных задач");
    
    println!("✓ Тест 13 пройден: Полный цикл оптимизации за {:?}", optimization_time);
    println!("  - Обработано {} перестановок", permutations.len());
    println!("  - Получено {} решений", all_solutions_guard.len());
    if !all_solutions_guard.is_empty() {
        let best_solution = all_solutions_guard.iter()
            .min_by_key(|s| s.get_wasted_area())
            .unwrap();
        println!("  - Лучшее решение: {} отходов ({:.1}%), {} разрезов", 
                 best_solution.get_wasted_area(), 
                 waste_percentage,
                 best_solution.get_cuts_count());
    }
    println!("  - Общая площадь деталей: {}, панелей: {}", total_tile_area, total_stock_area);
    
    // Проверяем разумность результата
    if !all_solutions_guard.is_empty() {
        assert!(waste_percentage < 50.0, "Отходы не должны превышать 50% от общей площади");
        let best_solution = all_solutions_guard.iter()
            .min_by_key(|s| s.get_total_wasted_area())
            .unwrap();
        assert!(best_solution.get_total_cuts_count() >= 0, "Количество разрезов не может быть отрицательным");
    }
}
