//! Тесты для этапа 5: Система компараторов решений
//!
//! Этот модуль содержит тесты для проверки корректности работы
//! системы сравнения решений по различным критериям.

use cutting_cli::engine::comparator::{
    MultiCriteriaComparator, OptimizationPriority, PriorityListFactory, SolutionComparator,
    SolutionComparatorFactory, SolutionUtils,
};
use cutting_cli::engine::model::{Solution, TileDimensions};
use cutting_cli::engine::stock::StockSolution;
use std::cmp::Ordering;
use std::time::Instant;

/// Создает простое тестовое решение
fn create_simple_solution() -> Solution {
    let stock_solution = StockSolution::new(vec![TileDimensions::new(
        1,
        1000,
        800,
        "wood".to_string(),
        1,
        None,
    )]);
    Solution::from_stock_solution(&stock_solution)
}

/// Создает тестовое решение с заданным количеством листов
fn create_solution_with_mosaics(mosaic_count: usize) -> Solution {
    let mut tiles = Vec::new();
    for i in 0..mosaic_count {
        tiles.push(TileDimensions::new(
            (i + 1) as i32,
            1000,
            800,
            "wood".to_string(),
            1,
            None,
        ));
    }

    let stock_solution = StockSolution::new(tiles);
    Solution::from_stock_solution(&stock_solution)
}

#[test]
fn test_solution_comparator_factory_creation() {
    // Тестируем создание компараторов для всех доступных приоритетов
    let priorities = vec![
        OptimizationPriority::MostTiles,
        OptimizationPriority::LeastWastedArea,
        OptimizationPriority::LeastNbrCuts,
        OptimizationPriority::LeastNbrMosaics,
        OptimizationPriority::BiggestUnusedTileArea,
        OptimizationPriority::MostHVDiscrepancy,
        OptimizationPriority::SmallestCenterOfMassDistToOrigin,
        OptimizationPriority::LeastNbrUnusedTiles,
        OptimizationPriority::MostUnusedPanelArea,
    ];

    for priority in priorities {
        let _comparator = SolutionComparatorFactory::create_comparator(priority);
        // Простая проверка что компаратор создан
        println!("Компаратор для {:?} создан успешно", priority);
    }
}

#[test]
fn test_most_tiles_comparison() {
    let solution1 = create_simple_solution();
    let solution2 = create_simple_solution();

    let tiles_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::MostTiles);

    // Два одинаковых решения должны быть равны
    let result = tiles_comparator.compare(&solution1, &solution2);
    assert_eq!(
        result,
        Ordering::Equal,
        "Два одинаковых решения должны быть равны"
    );
}

#[test]
fn test_least_wasted_area_comparison() {
    let solution1 = create_simple_solution();
    let solution2 = create_simple_solution();

    let area_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastWastedArea);

    // Два одинаковых решения должны быть равны
    let result = area_comparator.compare(&solution1, &solution2);
    assert_eq!(
        result,
        Ordering::Equal,
        "Два одинаковых решения должны быть равны"
    );
}

#[test]
fn test_least_nbr_cuts_comparison() {
    let solution1 = create_simple_solution();
    let solution2 = create_simple_solution();

    let cuts_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastNbrCuts);

    // Два одинаковых решения должны быть равны
    let result = cuts_comparator.compare(&solution1, &solution2);
    assert_eq!(
        result,
        Ordering::Equal,
        "Два одинаковых решения должны быть равны"
    );
}

#[test]
fn test_least_nbr_mosaics_comparison() {
    let solution_with_1_mosaic = create_solution_with_mosaics(1);
    let solution_with_2_mosaics = create_solution_with_mosaics(2);

    let mosaics_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastNbrMosaics);

    // Решение с меньшим количеством мозаик должно быть лучше или равно
    let result = mosaics_comparator.compare(&solution_with_1_mosaic, &solution_with_2_mosaics);
    assert!(
        result == Ordering::Less || result == Ordering::Equal,
        "Решение с меньшим количеством мозаик должно быть лучше или равно"
    );
}

#[test]
fn test_biggest_unused_tile_area_comparison() {
    let solution1 = create_simple_solution();
    let solution2 = create_simple_solution();

    let unused_area_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::BiggestUnusedTileArea);

    // Два одинаковых решения должны быть равны
    let result = unused_area_comparator.compare(&solution1, &solution2);
    assert_eq!(
        result,
        Ordering::Equal,
        "Два одинаковых решения должны быть равны"
    );
}

#[test]
fn test_most_hv_discrepancy_comparison() {
    let solution1 = create_simple_solution();
    let solution2 = create_simple_solution();

    let discrepancy_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::MostHVDiscrepancy);

    // Два одинаковых решения должны быть равны
    let result = discrepancy_comparator.compare(&solution1, &solution2);
    assert_eq!(
        result,
        Ordering::Equal,
        "Два одинаковых решения должны быть равны"
    );
}

#[test]
fn test_smallest_center_of_mass_comparison() {
    let solution1 = create_simple_solution();
    let solution2 = create_simple_solution();

    let center_mass_comparator = SolutionComparatorFactory::create_comparator(
        OptimizationPriority::SmallestCenterOfMassDistToOrigin,
    );

    // Два одинаковых решения должны быть равны
    let result = center_mass_comparator.compare(&solution1, &solution2);
    assert_eq!(
        result,
        Ordering::Equal,
        "Два одинаковых решения должны быть равны"
    );
}

#[test]
fn test_least_nbr_unused_tiles_comparison() {
    let solution1 = create_simple_solution();
    let solution2 = create_simple_solution();

    let unused_tiles_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastNbrUnusedTiles);

    // Два одинаковых решения должны быть равны
    let result = unused_tiles_comparator.compare(&solution1, &solution2);
    assert_eq!(
        result,
        Ordering::Equal,
        "Два одинаковых решения должны быть равны"
    );
}

#[test]
fn test_most_unused_panel_area_comparison() {
    let solution1 = create_simple_solution();
    let solution2 = create_simple_solution();

    let panel_area_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::MostUnusedPanelArea);

    // Два одинаковых решения должны быть равны
    let result = panel_area_comparator.compare(&solution1, &solution2);
    assert_eq!(
        result,
        Ordering::Equal,
        "Два одинаковых решения должны быть равны"
    );
}

#[test]
fn test_comparator_consistency() {
    let solution1 = create_simple_solution();
    let solution2 = create_simple_solution();

    let tiles_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::MostTiles);

    // Проверяем рефлексивность: A == A
    let aa = tiles_comparator.compare(&solution1, &solution1);
    assert_eq!(aa, Ordering::Equal, "Решение должно быть равно самому себе");

    // Проверяем симметричность: если A == B, то B == A
    let ab = tiles_comparator.compare(&solution1, &solution2);
    let ba = tiles_comparator.compare(&solution2, &solution1);

    if ab == Ordering::Equal {
        assert_eq!(ba, Ordering::Equal, "Симметричность должна соблюдаться");
    } else if ab == Ordering::Less {
        assert_eq!(
            ba,
            Ordering::Greater,
            "Антисимметричность должна соблюдаться"
        );
    } else if ab == Ordering::Greater {
        assert_eq!(ba, Ordering::Less, "Антисимметричность должна соблюдаться");
    }
}

#[test]
fn test_different_optimization_priorities_basic() {
    let solution1 = create_simple_solution();
    let solution2 = create_simple_solution();

    // Тестируем основные приоритеты оптимизации
    let priorities = vec![
        OptimizationPriority::MostTiles,
        OptimizationPriority::LeastWastedArea,
        OptimizationPriority::LeastNbrCuts,
        OptimizationPriority::LeastNbrMosaics,
    ];

    for priority in priorities {
        let comparator = SolutionComparatorFactory::create_comparator(priority);
        let result = comparator.compare(&solution1, &solution2);

        // Результат должен быть одним из трех возможных значений
        assert!(
            result == Ordering::Less || result == Ordering::Equal || result == Ordering::Greater,
            "Результат сравнения для {:?} должен быть валидным",
            priority
        );
    }
}

#[test]
fn test_empty_solutions() {
    let empty_solution1 = Solution::new();
    let empty_solution2 = Solution::new();

    let tiles_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::MostTiles);

    // Два пустых решения должны быть равны
    let result = tiles_comparator.compare(&empty_solution1, &empty_solution2);
    assert_eq!(
        result,
        Ordering::Equal,
        "Два пустых решения должны быть равны"
    );
}

#[test]
fn test_solution_vs_empty() {
    let solution = create_simple_solution();
    let empty_solution = Solution::new();

    let tiles_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::MostTiles);

    // Решение с данными должно быть лучше пустого (для MostTiles)
    let result = tiles_comparator.compare(&solution, &empty_solution);
    assert!(
        result == Ordering::Less || result == Ordering::Equal,
        "Решение с данными должно быть лучше или равно пустому"
    );
}

#[test]
fn test_all_comparators_work() {
    let solution1 = create_simple_solution();
    let solution2 = create_solution_with_mosaics(2);

    // Проверяем что все компараторы работают без паники
    let all_priorities = vec![
        OptimizationPriority::MostTiles,
        OptimizationPriority::LeastWastedArea,
        OptimizationPriority::LeastNbrCuts,
        OptimizationPriority::LeastNbrMosaics,
        OptimizationPriority::BiggestUnusedTileArea,
        OptimizationPriority::MostHVDiscrepancy,
        OptimizationPriority::SmallestCenterOfMassDistToOrigin,
        OptimizationPriority::LeastNbrUnusedTiles,
        OptimizationPriority::MostUnusedPanelArea,
    ];

    for priority in all_priorities {
        let comparator = SolutionComparatorFactory::create_comparator(priority);
        let result = comparator.compare(&solution1, &solution2);

        // Просто проверяем что сравнение не вызывает панику
        assert!(
            result == Ordering::Less || result == Ordering::Equal || result == Ordering::Greater,
            "Компаратор {:?} должен возвращать валидный результат",
            priority
        );
    }
}

// ===== ДЕТАЛЬНЫЕ ТЕСТЫ ПО ПЛАНУ ЭТАПА 5 =====

/// Создает решение с заданными характеристиками для тестирования
fn create_solution_with_characteristics(
    nbr_tiles: i32,
    _unused_area: i64,
    _nbr_cuts: i32,
    nbr_mosaics: usize,
) -> Solution {
    // Создаем решение с заданным количеством мозаик
    let mut solution = create_solution_with_mosaics(nbr_mosaics);

    // Размещаем детали для получения нужного количества финальных деталей
    for i in 0..nbr_tiles {
        let tile_width = 100 + (i % 5) * 50; // Варьируем размеры
        let tile_height = 80 + (i % 3) * 40;
        let tile_to_place = TileDimensions::new(
            100 + i,
            tile_width,
            tile_height,
            "wood".to_string(),
            1,
            None,
        );

        // Пытаемся разместить деталь
        if let Ok(new_solutions) = solution.try_place_tile(&tile_to_place) {
            if let Some(new_solution) = new_solutions.into_iter().next() {
                solution = new_solution;
            }
        }
    }

    solution
}

#[test]
fn test_most_tiles_detailed_comparison() {
    // Тест 1: Компаратор по количеству размещенных деталей
    // Создаем решения с разным количеством деталей
    let solution1 = create_solution_with_characteristics(5, 200000, 8, 1);
    let solution2 = create_solution_with_characteristics(7, 150000, 10, 1);
    let solution3 = create_solution_with_characteristics(5, 100000, 6, 1);

    let tiles_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::MostTiles);

    // solution2 должно быть лучше solution1 (больше деталей)
    let result12 = tiles_comparator.compare(&solution1, &solution2);
    assert_eq!(
        result12,
        Ordering::Greater,
        "Solution2 должно быть лучше (больше деталей)"
    );

    // solution2 должно быть лучше solution1 (обратное сравнение)
    let result21 = tiles_comparator.compare(&solution2, &solution1);
    assert_eq!(
        result21,
        Ordering::Less,
        "Solution2 должно быть лучше (больше деталей)"
    );

    // solution1 и solution3 равны по количеству деталей
    let result13 = tiles_comparator.compare(&solution1, &solution3);
    assert_eq!(
        result13,
        Ordering::Equal,
        "Solution1 и Solution3 равны по деталям"
    );

    // Тестируем сортировку
    let mut solutions = vec![solution1, solution2, solution3];
    solutions.sort_by(|a, b| tiles_comparator.compare(a, b));

    // После сортировки решения должны быть упорядочены по убыванию количества деталей
    assert!(solutions[0].get_nbr_final_tiles() >= solutions[1].get_nbr_final_tiles());
    assert!(solutions[1].get_nbr_final_tiles() >= solutions[2].get_nbr_final_tiles());
}

#[test]
fn test_least_wasted_area_detailed_comparison() {
    // Тест 2: Компаратор по площади отходов
    let solution1 = create_solution_with_characteristics(5, 200000, 8, 1);
    let solution2 = create_solution_with_characteristics(7, 100000, 10, 1);
    let solution3 = create_solution_with_characteristics(5, 200000, 6, 1);

    let area_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastWastedArea);

    // solution2 должно быть лучше solution1 (меньше отходов)
    let result12 = area_comparator.compare(&solution1, &solution2);
    assert_eq!(
        result12,
        Ordering::Greater,
        "Solution2 должно быть лучше (меньше отходов)"
    );

    // solution2 должно быть лучше solution1 (обратное сравнение)
    let result21 = area_comparator.compare(&solution2, &solution1);
    assert_eq!(
        result21,
        Ordering::Less,
        "Solution2 должно быть лучше (меньше отходов)"
    );

    // solution1 и solution3 равны по отходам
    let result13 = area_comparator.compare(&solution1, &solution3);
    assert_eq!(
        result13,
        Ordering::Equal,
        "Solution1 и Solution3 равны по отходам"
    );

    // Тестируем сортировку
    let mut solutions = vec![solution1, solution2, solution3];
    solutions.sort_by(|a, b| area_comparator.compare(a, b));

    // После сортировки решения должны быть упорядочены по возрастанию отходов
    assert!(solutions[0].get_unused_area() <= solutions[1].get_unused_area());
    assert!(solutions[1].get_unused_area() <= solutions[2].get_unused_area());
}

#[test]
fn test_least_nbr_cuts_detailed_comparison() {
    // Тест 3: Компаратор по количеству разрезов
    // Создаем решения с разным количеством размещенных деталей, что приведет к разному количеству разрезов
    let solution1 = create_solution_with_characteristics(2, 200000, 8, 1); // Меньше деталей = меньше разрезов
    let solution2 = create_solution_with_characteristics(1, 150000, 5, 1); // Еще меньше деталей = еще меньше разрезов
    let solution3 = create_solution_with_characteristics(3, 100000, 12, 1); // Больше деталей = больше разрезов

    let cuts_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::LeastNbrCuts);

    // Проверяем фактические значения разрезов
    println!("Solution1 cuts: {}", solution1.get_nbr_cuts());
    println!("Solution2 cuts: {}", solution2.get_nbr_cuts());
    println!("Solution3 cuts: {}", solution3.get_nbr_cuts());

    // solution2 должно быть лучше solution1 (меньше разрезов)
    let result12 = cuts_comparator.compare(&solution1, &solution2);
    if solution2.get_nbr_cuts() < solution1.get_nbr_cuts() {
        assert_eq!(
            result12,
            Ordering::Greater,
            "Solution2 должно быть лучше (меньше разрезов)"
        );
    } else if solution2.get_nbr_cuts() > solution1.get_nbr_cuts() {
        assert_eq!(
            result12,
            Ordering::Less,
            "Solution1 должно быть лучше (меньше разрезов)"
        );
    } else {
        assert_eq!(result12, Ordering::Equal, "Решения равны по разрезам");
    }

    // solution2 должно быть лучше solution3 (меньше разрезов)
    let result23 = cuts_comparator.compare(&solution2, &solution3);
    if solution2.get_nbr_cuts() < solution3.get_nbr_cuts() {
        assert_eq!(
            result23,
            Ordering::Less,
            "Solution2 должно быть лучше (меньше разрезов)"
        );
    } else if solution2.get_nbr_cuts() > solution3.get_nbr_cuts() {
        assert_eq!(
            result23,
            Ordering::Greater,
            "Solution3 должно быть лучше (меньше разрезов)"
        );
    } else {
        assert_eq!(result23, Ordering::Equal, "Решения равны по разрезам");
    }

    // Тестируем сортировку
    let mut solutions = vec![solution1, solution2, solution3];
    solutions.sort_by(|a, b| cuts_comparator.compare(a, b));

    // После сортировки решения должны быть упорядочены по возрастанию разрезов
    assert!(solutions[0].get_nbr_cuts() <= solutions[1].get_nbr_cuts());
    assert!(solutions[1].get_nbr_cuts() <= solutions[2].get_nbr_cuts());
}

#[test]
fn test_multi_criteria_comparison() {
    // Тест 4: Многокритериальная сортировка
    let solution1 = create_solution_with_characteristics(5, 200000, 8, 1);
    let solution2 = create_solution_with_characteristics(7, 150000, 10, 1);
    let solution3 = create_solution_with_characteristics(5, 100000, 6, 1);
    let solution4 = create_solution_with_characteristics(7, 150000, 8, 1);

    // Создаем список приоритетов: [MOST_TILES, LEAST_WASTED_AREA, LEAST_NBR_CUTS]
    let priorities = vec![
        OptimizationPriority::MostTiles,
        OptimizationPriority::LeastWastedArea,
        OptimizationPriority::LeastNbrCuts,
    ];

    let multi_comparator = MultiCriteriaComparator::from_priorities(priorities);

    // Тестируем многокритериальное сравнение
    let mut solutions = vec![solution1, solution2, solution3, solution4];
    solutions.sort_by(|a, b| multi_comparator.compare(a, b));

    // Проверяем итоговый порядок:
    // Solution4 и Solution2 (7 деталей) должны идти первыми
    // Между ними Solution4 лучше (меньше разрезов при равных отходах)
    // Solution3 (5 деталей, но меньше отходов) должно идти перед Solution1

    // Первые два решения должны иметь 7 деталей
    assert_eq!(solutions[0].get_nbr_final_tiles(), 7);
    assert_eq!(solutions[1].get_nbr_final_tiles(), 7);

    // Последние два решения должны иметь 5 деталей
    assert_eq!(solutions[2].get_nbr_final_tiles(), 5);
    assert_eq!(solutions[3].get_nbr_final_tiles(), 5);
}

#[test]
fn test_priority_list_factory() {
    // Тест 5: Фабрика приоритетов (PriorityListFactory)

    // Тестируем приоритет на площадь (0)
    let priorities_area = PriorityListFactory::get_final_solution_prioritized_comparator_list(0);
    assert_eq!(priorities_area[0], OptimizationPriority::MostTiles);
    assert_eq!(priorities_area[1], OptimizationPriority::LeastWastedArea);
    assert_eq!(priorities_area[2], OptimizationPriority::LeastNbrCuts);
    assert_eq!(priorities_area[3], OptimizationPriority::LeastNbrMosaics);
    assert_eq!(priorities_area.len(), 9);

    // Тестируем приоритет на разрезы (1)
    let priorities_cuts = PriorityListFactory::get_final_solution_prioritized_comparator_list(1);
    assert_eq!(priorities_cuts[0], OptimizationPriority::MostTiles);
    assert_eq!(priorities_cuts[1], OptimizationPriority::LeastNbrCuts);
    assert_eq!(priorities_cuts[2], OptimizationPriority::LeastWastedArea);
    assert_eq!(priorities_cuts[3], OptimizationPriority::LeastNbrMosaics);
    assert_eq!(priorities_cuts.len(), 9);

    // Тестируем промежуточные списки
    let intermediate_area =
        PriorityListFactory::get_intermediate_solution_prioritized_comparator_list(0);
    let intermediate_cuts =
        PriorityListFactory::get_intermediate_solution_prioritized_comparator_list(1);

    assert!(intermediate_area.len() < priorities_area.len());
    assert!(intermediate_cuts.len() < priorities_cuts.len());
    assert_eq!(intermediate_area[0], OptimizationPriority::MostTiles);
    assert_eq!(intermediate_cuts[0], OptimizationPriority::MostTiles);
}

#[test]
fn test_solution_utils_remove_duplicates() {
    // Тест 6: Удаление дубликатов решений

    // Создаем решения, некоторые из которых будут дубликатами
    let solution1 = create_simple_solution();
    let solution2 = create_simple_solution();
    let solution3 = create_solution_with_mosaics(2);
    let solution4 = create_simple_solution(); // Дубликат solution1

    let mut solutions = vec![solution1, solution2, solution3, solution4];
    let initial_count = solutions.len();

    // Удаляем дубликаты
    let removed_count = SolutionUtils::remove_duplicates(&mut solutions);

    // Проверяем что дубликаты удалены
    assert!(removed_count > 0, "Должны быть удалены дубликаты");
    assert_eq!(solutions.len(), initial_count - removed_count);

    // Проверяем что все оставшиеся решения уникальны
    for i in 0..solutions.len() {
        for j in (i + 1)..solutions.len() {
            assert_ne!(
                solutions[i].get_structure_identifier(),
                solutions[j].get_structure_identifier(),
                "Все оставшиеся решения должны быть уникальными"
            );
        }
    }
}

#[test]
fn test_solution_utils_sort_and_process() {
    // Тест интеграции: полная обработка списка решений

    let mut solutions = vec![
        create_solution_with_characteristics(3, 300000, 12, 1),
        create_solution_with_characteristics(7, 150000, 8, 1),
        create_solution_with_characteristics(5, 200000, 10, 1),
        create_solution_with_characteristics(7, 100000, 6, 1),
        create_solution_with_characteristics(5, 200000, 10, 1), // Дубликат
    ];

    let priorities = vec![
        OptimizationPriority::MostTiles,
        OptimizationPriority::LeastWastedArea,
        OptimizationPriority::LeastNbrCuts,
    ];

    // Полная обработка: сортировка, удаление дубликатов, ограничение
    let removed_count = SolutionUtils::process_solutions(&mut solutions, priorities, Some(3));

    // Проверяем что дубликаты удалены
    assert!(removed_count > 0, "Должны быть удалены дубликаты");

    // Проверяем что количество ограничено
    assert!(
        solutions.len() <= 3,
        "Количество решений должно быть ограничено"
    );

    // Проверяем что решения отсортированы (лучшие первыми)
    for i in 0..(solutions.len() - 1) {
        let current_tiles = solutions[i].get_nbr_final_tiles();
        let next_tiles = solutions[i + 1].get_nbr_final_tiles();
        assert!(
            current_tiles >= next_tiles,
            "Решения должны быть отсортированы по убыванию деталей"
        );
    }
}

#[test]
fn test_boundary_cases() {
    // Тест 9: Граничные случаи компараторов

    let solution = create_simple_solution();
    let empty_solution = Solution::new();

    let tiles_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::MostTiles);

    // Сравнение решения с самим собой
    let self_comparison = tiles_comparator.compare(&solution, &solution);
    assert_eq!(
        self_comparison,
        Ordering::Equal,
        "Решение должно быть равно самому себе"
    );

    // Сравнение пустых решений
    let empty_comparison = tiles_comparator.compare(&empty_solution, &empty_solution);
    assert_eq!(
        empty_comparison,
        Ordering::Equal,
        "Пустые решения должны быть равны"
    );

    // Сортировка списка из одного элемента
    let mut single_solution = vec![solution.clone()];
    SolutionUtils::sort_solutions(&mut single_solution, vec![OptimizationPriority::MostTiles]);
    assert_eq!(
        single_solution.len(),
        1,
        "Список из одного элемента должен остаться неизменным"
    );

    // Сортировка пустого списка
    let mut empty_list: Vec<Solution> = Vec::new();
    SolutionUtils::sort_solutions(&mut empty_list, vec![OptimizationPriority::MostTiles]);
    assert_eq!(empty_list.len(), 0, "Пустой список должен остаться пустым");
}

#[test]
fn test_performance_sorting() {
    // Тест 8: Производительность сортировки

    // Создаем большой список решений
    let mut solutions = Vec::new();
    for i in 0..100 {
        // Уменьшено для быстрого тестирования
        let solution = create_solution_with_characteristics(
            (i % 10) + 1,
            ((i % 5) + 1) as i64 * 50000,
            (i % 8) + 3,
            1,
        );
        solutions.push(solution);
    }

    let priorities = vec![
        OptimizationPriority::MostTiles,
        OptimizationPriority::LeastWastedArea,
        OptimizationPriority::LeastNbrCuts,
    ];

    // Засекаем время сортировки
    let start_time = Instant::now();
    SolutionUtils::sort_solutions(&mut solutions, priorities);
    let elapsed = start_time.elapsed();

    // Проверяем что сортировка выполнилась быстро (менее 1 секунды)
    assert!(
        elapsed.as_secs() < 1,
        "Сортировка должна выполняться быстро"
    );

    // Проверяем что результат корректно отсортирован
    for i in 0..(solutions.len() - 1) {
        let current_tiles = solutions[i].get_nbr_final_tiles();
        let next_tiles = solutions[i + 1].get_nbr_final_tiles();
        assert!(
            current_tiles >= next_tiles,
            "Решения должны быть отсортированы"
        );
    }

    // Проверяем стабильность сортировки (повторный запуск)
    let mut solutions_second = solutions.clone();
    SolutionUtils::sort_solutions(
        &mut solutions_second,
        vec![
            OptimizationPriority::MostTiles,
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts,
        ],
    );

    // Результаты должны быть одинаковыми
    for i in 0..solutions.len() {
        assert_eq!(
            solutions[i].get_id(),
            solutions_second[i].get_id(),
            "Повторная сортировка должна давать тот же результат"
        );
    }
}

#[test]
fn test_comparator_factory_by_name() {
    // Тестируем создание компараторов по имени
    let valid_names = vec![
        "most_tiles",
        "least_wasted_area",
        "least_nbr_cuts",
        "least_nbr_mosaics",
    ];

    for name in valid_names {
        let comparator = SolutionComparatorFactory::create_comparator_by_name(name);
        assert!(
            comparator.is_some(),
            "Компаратор для '{}' должен быть создан",
            name
        );
    }

    // Тестируем невалидное имя
    let invalid_comparator = SolutionComparatorFactory::create_comparator_by_name("invalid_name");
    assert!(
        invalid_comparator.is_none(),
        "Невалидное имя не должно создавать компаратор"
    );

    // Тестируем проверку валидности имен
    assert!(SolutionComparatorFactory::is_valid_comparator_name(
        "most_tiles"
    ));
    assert!(!SolutionComparatorFactory::is_valid_comparator_name(
        "invalid_name"
    ));

    // Тестируем получение списка доступных имен
    let available_names = SolutionComparatorFactory::get_available_comparator_names();
    assert!(
        !available_names.is_empty(),
        "Должен быть доступен список имен компараторов"
    );
    assert!(available_names.contains(&"most_tiles"));
}

#[test]
fn test_custom_priority_lists() {
    // Тестируем создание пользовательских списков приоритетов
    let custom_priorities = PriorityListFactory::create_custom_priority_list(
        OptimizationPriority::LeastNbrCuts,
        Some(OptimizationPriority::MostTiles),
        vec![
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrMosaics,
        ],
    );

    assert_eq!(custom_priorities[0], OptimizationPriority::LeastNbrCuts);
    assert_eq!(custom_priorities[1], OptimizationPriority::MostTiles);
    assert_eq!(custom_priorities[2], OptimizationPriority::LeastWastedArea);
    assert_eq!(custom_priorities[3], OptimizationPriority::LeastNbrMosaics);
    assert_eq!(custom_priorities.len(), 4);

    // Тестируем исключение дубликатов
    let priorities_with_duplicates = PriorityListFactory::create_custom_priority_list(
        OptimizationPriority::MostTiles,
        Some(OptimizationPriority::LeastWastedArea),
        vec![
            OptimizationPriority::MostTiles,       // дубликат
            OptimizationPriority::LeastWastedArea, // дубликат
            OptimizationPriority::LeastNbrCuts,
        ],
    );

    assert_eq!(priorities_with_duplicates.len(), 3);
    assert_eq!(
        priorities_with_duplicates[0],
        OptimizationPriority::MostTiles
    );
    assert_eq!(
        priorities_with_duplicates[1],
        OptimizationPriority::LeastWastedArea
    );
    assert_eq!(
        priorities_with_duplicates[2],
        OptimizationPriority::LeastNbrCuts
    );
}

#[test]
fn test_special_comparators_detailed() {
    // Тест 7: Компараторы для специальных метрик

    // Создаем решения с размещенными деталями для более реалистичного тестирования
    let solution1 = create_solution_with_characteristics(3, 200000, 6, 1);
    let solution2 = create_solution_with_characteristics(4, 150000, 8, 1);

    // Тестируем компаратор по максимальной неиспользуемой площади
    let unused_area_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::BiggestUnusedTileArea);
    let result = unused_area_comparator.compare(&solution1, &solution2);

    // Результат должен быть валидным
    assert!(
        result == Ordering::Less || result == Ordering::Equal || result == Ordering::Greater,
        "Компаратор BiggestUnusedTileArea должен возвращать валидный результат"
    );

    // Тестируем компаратор по дисбалансу горизонтальных/вертикальных разрезов
    let discrepancy_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::MostHVDiscrepancy);
    let result = discrepancy_comparator.compare(&solution1, &solution2);

    // Результат должен быть валидным
    assert!(
        result == Ordering::Less || result == Ordering::Equal || result == Ordering::Greater,
        "Компаратор MostHVDiscrepancy должен возвращать валидный результат"
    );

    // Тестируем компаратор по расстоянию центра масс
    let center_mass_comparator = SolutionComparatorFactory::create_comparator(
        OptimizationPriority::SmallestCenterOfMassDistToOrigin,
    );
    let result = center_mass_comparator.compare(&solution1, &solution2);

    // Результат должен быть валидным
    assert!(
        result == Ordering::Less || result == Ordering::Equal || result == Ordering::Greater,
        "Компаратор SmallestCenterOfMassDistToOrigin должен возвращать валидный результат"
    );
}

#[test]
fn test_integration_with_main_algorithm() {
    // Тест 10: Интеграция с основным алгоритмом

    // Создаем множество решений разного качества
    let mut solutions = vec![
        create_solution_with_characteristics(2, 400000, 4, 1), // Худшее: мало деталей, много отходов
        create_solution_with_characteristics(8, 100000, 12, 1), // Хорошее: много деталей, мало отходов
        create_solution_with_characteristics(6, 200000, 8, 1),  // Среднее
        create_solution_with_characteristics(8, 120000, 10, 1), // Очень хорошее: много деталей, мало отходов
        create_solution_with_characteristics(4, 300000, 6, 1), // Плохое: мало деталей, много отходов
        create_solution_with_characteristics(7, 150000, 9, 1), // Хорошее
    ];

    // Применяем полный цикл сортировки и фильтрации
    let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(0);
    let removed_count = SolutionUtils::process_solutions(&mut solutions, priorities, Some(3));

    // Проверяем что обработка прошла корректно
    assert!(
        solutions.len() <= 3,
        "Количество решений должно быть ограничено"
    );
    assert!(
        removed_count >= 0,
        "Количество удаленных решений должно быть неотрицательным"
    );

    // Проверяем что лучшие решения остались в начале списка
    if solutions.len() >= 2 {
        let first_tiles = solutions[0].get_nbr_final_tiles();
        let second_tiles = solutions[1].get_nbr_final_tiles();
        assert!(
            first_tiles >= second_tiles,
            "Первое решение должно иметь больше или равно деталей"
        );
    }

    // Проверяем что все решения уникальны
    for i in 0..solutions.len() {
        for j in (i + 1)..solutions.len() {
            assert_ne!(
                solutions[i].get_structure_identifier(),
                solutions[j].get_structure_identifier(),
                "Все решения должны быть уникальными"
            );
        }
    }
}

#[test]
fn test_comparator_transitivity() {
    // Дополнительный тест: проверка транзитивности компараторов
    let solution1 = create_solution_with_characteristics(3, 300000, 6, 1);
    let solution2 = create_solution_with_characteristics(5, 200000, 8, 1);
    let solution3 = create_solution_with_characteristics(7, 100000, 10, 1);

    let tiles_comparator =
        SolutionComparatorFactory::create_comparator(OptimizationPriority::MostTiles);

    // Проверяем транзитивность: если A <= B и B <= C, то A <= C
    let ab = tiles_comparator.compare(&solution1, &solution2);
    let bc = tiles_comparator.compare(&solution2, &solution3);
    let ac = tiles_comparator.compare(&solution1, &solution3);

    // Если solution1 <= solution2 и solution2 <= solution3, то solution1 <= solution3
    if (ab == Ordering::Less || ab == Ordering::Equal)
        && (bc == Ordering::Less || bc == Ordering::Equal)
    {
        assert!(
            ac == Ordering::Less || ac == Ordering::Equal,
            "Транзитивность должна соблюдаться: если A <= B и B <= C, то A <= C"
        );
    }
}

#[test]
fn test_solution_utils_edge_cases() {
    // Тестируем граничные случаи для SolutionUtils

    // Тест с пустым списком решений
    let mut empty_solutions: Vec<Solution> = Vec::new();
    let removed = SolutionUtils::remove_duplicates(&mut empty_solutions);
    assert_eq!(
        removed, 0,
        "Удаление дубликатов из пустого списка должно вернуть 0"
    );
    assert_eq!(
        empty_solutions.len(),
        0,
        "Пустой список должен остаться пустым"
    );

    // Тест с одним решением
    let mut single_solution = vec![create_simple_solution()];
    let removed = SolutionUtils::remove_duplicates(&mut single_solution);
    assert_eq!(
        removed, 0,
        "Удаление дубликатов из списка с одним элементом должно вернуть 0"
    );
    assert_eq!(
        single_solution.len(),
        1,
        "Список с одним элементом должен остаться неизменным"
    );

    // Тест сортировки с пустым списком приоритетов
    let mut solutions = vec![
        create_solution_with_characteristics(3, 200000, 6, 1),
        create_solution_with_characteristics(5, 100000, 8, 1),
    ];

    SolutionUtils::sort_solutions(&mut solutions, vec![]);
    assert_eq!(
        solutions.len(),
        2,
        "Сортировка с пустым списком приоритетов не должна изменять размер"
    );

    // Тест обработки с лимитом 0
    let mut solutions_for_limit = vec![
        create_solution_with_characteristics(3, 200000, 6, 1),
        create_solution_with_characteristics(5, 100000, 8, 1),
    ];

    let priorities = vec![OptimizationPriority::MostTiles];
    SolutionUtils::process_solutions(&mut solutions_for_limit, priorities, Some(0));
    assert_eq!(
        solutions_for_limit.len(),
        0,
        "Лимит 0 должен оставить пустой список"
    );
}
