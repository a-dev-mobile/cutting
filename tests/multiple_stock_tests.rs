//! Тесты для проверки правильного распределения деталей между несколькими заготовками

use cutting_cli::engine::model::tile::TileDimensions;
use cutting_cli::engine::model::solution::Solution;
use cutting_cli::engine::stock::StockSolution;
use cutting_cli::engine::model::request::{CalculationRequest, Panel, ClientInfo, Configuration};
use cutting_cli::engine::service::{CutListOptimizerService, CutListOptimizerServiceImpl};
use cutting_cli::engine::logger::CutListLoggerImpl;
use std::sync::Arc;

#[cfg(test)]
mod basic_multiple_stock_tests {
    use super::*;

    #[test]
    fn test_single_stock_sufficient() {
        // Тест: все детали помещаются на одну заготовку
        let stock_tiles = vec![
            TileDimensions::simple(1000, 600), // Большая заготовка
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Маленькие детали, которые должны поместиться на одну заготовку
        let tiles = vec![
            TileDimensions::simple(200, 150),
            TileDimensions::simple(180, 120),
            TileDimensions::simple(150, 100),
        ];
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем результаты
        assert_eq!(placed_count, 3, "Все детали должны поместиться на одну заготовку");
        assert_eq!(solution.get_nbr_mosaics(), 1, "Должна использоваться только одна мозаика");
        assert_eq!(solution.get_nbr_final_tiles(), 3, "Должно быть размещено 3 детали");
        assert!(solution.get_no_fit_panels().is_empty(), "Не должно быть неразмещенных деталей");
        
        println!("✅ Тест одной заготовки: размещено {}/{} деталей на {} мозаиках", 
            placed_count, tiles.len(), solution.get_nbr_mosaics());
    }

    #[test]
    fn test_two_stocks_required() {
        // Тест: детали требуют две заготовки
        let stock_tiles = vec![
            TileDimensions::simple(500, 400), // Первая заготовка
            TileDimensions::simple(500, 400), // Вторая заготовка
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Детали, которые не поместятся все на одну заготовку
        let tiles = vec![
            TileDimensions::simple(450, 350), // Большая деталь - займет почти всю первую заготовку
            TileDimensions::simple(400, 300), // Большая деталь - потребует вторую заготовку
            TileDimensions::simple(200, 150), // Средняя деталь
        ];
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем результаты
        assert!(placed_count >= 2, "Должно быть размещено минимум 2 детали");
        assert!(solution.get_nbr_mosaics() >= 2, "Должно использоваться минимум 2 мозаики");
        
        println!("✅ Тест двух заготовок: размещено {}/{} деталей на {} мозаиках", 
            placed_count, tiles.len(), solution.get_nbr_mosaics());
    }

    #[test]
    fn test_three_stocks_required() {
        // Тест: детали требуют три заготовки
        let stock_tiles = vec![
            TileDimensions::simple(300, 200), // Маленькие заготовки
            TileDimensions::simple(300, 200),
            TileDimensions::simple(300, 200),
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Детали, каждая из которых займет почти всю заготовку
        let tiles = vec![
            TileDimensions::simple(280, 180), // Деталь 1
            TileDimensions::simple(270, 170), // Деталь 2
            TileDimensions::simple(260, 160), // Деталь 3
            TileDimensions::simple(100, 80),  // Маленькая деталь
        ];
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем результаты
        assert!(placed_count >= 3, "Должно быть размещено минимум 3 детали");
        assert!(solution.get_nbr_mosaics() >= 3, "Должно использоваться минимум 3 мозаики");
        
        println!("✅ Тест трех заготовок: размещено {}/{} деталей на {} мозаиках", 
            placed_count, tiles.len(), solution.get_nbr_mosaics());
    }
}

#[cfg(test)]
mod efficiency_tests {
    use super::*;

    #[test]
    fn test_optimal_stock_usage() {
        // Тест: проверка оптимального использования заготовок
        let stock_tiles = vec![
            TileDimensions::simple(600, 400), // Большая заготовка
            TileDimensions::simple(300, 200), // Маленькая заготовка
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Детали, которые лучше разместить на большой заготовке
        let tiles = vec![
            TileDimensions::simple(250, 180), // Поместится на обеих, но лучше на большой
            TileDimensions::simple(200, 150), // Поместится на обеих
        ];
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем, что используется минимальное количество заготовок
        assert_eq!(placed_count, 2, "Должны быть размещены обе детали");
        assert_eq!(solution.get_nbr_mosaics(), 1, "Должна использоваться только одна мозаика");
        
        // Проверяем эффективность
        let efficiency = solution.get_efficiency();
        assert!(efficiency > 30.0, "Эффективность должна быть разумной: {:.2}%", efficiency);
        
        println!("✅ Тест оптимального использования: эффективность {:.2}%", efficiency);
    }

    #[test]
    fn test_no_unnecessary_stocks() {
        // Тест: проверка что не создаются лишние заготовки
        let stock_tiles = vec![
            TileDimensions::simple(1000, 800), // Очень большая заготовка
            TileDimensions::simple(500, 400),  // Средняя заготовка
            TileDimensions::simple(300, 200),  // Маленькая заготовка
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Маленькие детали, которые все поместятся на первую заготовку
        let tiles = vec![
            TileDimensions::simple(200, 150),
            TileDimensions::simple(180, 120),
            TileDimensions::simple(160, 100),
            TileDimensions::simple(140, 80),
        ];
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем, что используется только одна заготовка
        assert_eq!(placed_count, 4, "Должны быть размещены все детали");
        assert_eq!(solution.get_nbr_mosaics(), 1, "Должна использоваться только одна мозаика");
        assert_eq!(solution.get_unused_stock_panels().len(), 2, "Должно остаться 2 неиспользованные заготовки");
        
        println!("✅ Тест отсутствия лишних заготовок: использовано {} из {} заготовок", 
            solution.get_nbr_mosaics(), 3);
    }

    #[test]
    fn test_fill_existing_before_new() {
        // Тест: проверка приоритета заполнения существующих заготовок
        let stock_tiles = vec![
            TileDimensions::simple(400, 300),
            TileDimensions::simple(400, 300),
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Сначала размещаем одну деталь
        let first_tile = TileDimensions::simple(200, 150);
        match solution.try_place_tile(&first_tile) {
            Ok(new_solutions) => {
                if !new_solutions.is_empty() {
                    solution = new_solutions[0].clone();
                }
            }
            Err(_) => {}
        }
        
        assert_eq!(solution.get_nbr_mosaics(), 1, "После первой детали должна быть одна мозаика");
        
        // Теперь размещаем вторую деталь, которая должна поместиться в ту же мозаику
        let second_tile = TileDimensions::simple(180, 120);
        match solution.try_place_tile(&second_tile) {
            Ok(new_solutions) => {
                if !new_solutions.is_empty() {
                    solution = new_solutions[0].clone();
                }
            }
            Err(_) => {}
        }
        
        // Проверяем, что вторая деталь размещена в той же мозаике
        assert_eq!(solution.get_nbr_mosaics(), 1, "Вторая деталь должна быть размещена в существующей мозаике");
        assert_eq!(solution.get_nbr_final_tiles(), 2, "Должно быть размещено 2 детали");
        
        println!("✅ Тест приоритета существующих заготовок: {} деталей на {} мозаиках", 
            solution.get_nbr_final_tiles(), solution.get_nbr_mosaics());
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_many_small_tiles() {
        // Тест: много мелких деталей на нескольких заготовках
        let stock_tiles = vec![
            TileDimensions::simple(500, 400),
            TileDimensions::simple(500, 400),
            TileDimensions::simple(500, 400),
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Создаем много мелких деталей
        let mut tiles = Vec::new();
        for i in 0..20 {
            tiles.push(TileDimensions::simple(80 + i * 5, 60 + i * 3));
        }
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем результаты
        assert!(placed_count >= 15, "Должно быть размещено минимум 15 из 20 деталей");
        assert!(solution.get_nbr_mosaics() <= 3, "Не должно использоваться больше 3 мозаик");
        
        let efficiency = solution.get_efficiency();
        println!("✅ Тест множества мелких деталей: размещено {}/{} деталей, эффективность {:.2}%", 
            placed_count, tiles.len(), efficiency);
    }

    #[test]
    fn test_large_tiles_separate_stocks() {
        // Тест: крупные детали, требующие отдельных заготовок
        let stock_tiles = vec![
            TileDimensions::simple(600, 400),
            TileDimensions::simple(600, 400),
            TileDimensions::simple(600, 400),
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Крупные детали, каждая займет почти всю заготовку
        let tiles = vec![
            TileDimensions::simple(580, 380), // Очень большая деталь 1
            TileDimensions::simple(570, 370), // Очень большая деталь 2
            TileDimensions::simple(560, 360), // Очень большая деталь 3
        ];
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем, что каждая деталь использует отдельную заготовку
        assert_eq!(placed_count, 3, "Должны быть размещены все 3 детали");
        assert_eq!(solution.get_nbr_mosaics(), 3, "Должно использоваться 3 мозаики");
        assert_eq!(solution.get_nbr_final_tiles(), 3, "Должно быть 3 финальных детали");
        
        println!("✅ Тест крупных деталей: {} деталей на {} отдельных заготовках", 
            placed_count, solution.get_nbr_mosaics());
    }

    #[test]
    fn test_mixed_sizes() {
        // Тест: смешанные размеры деталей
        let stock_tiles = vec![
            TileDimensions::simple(800, 600), // Большая заготовка
            TileDimensions::simple(400, 300), // Средняя заготовка
            TileDimensions::simple(200, 150), // Маленькая заготовка
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Смешанные размеры деталей
        let tiles = vec![
            TileDimensions::simple(350, 250), // Большая деталь
            TileDimensions::simple(180, 120), // Средняя деталь
            TileDimensions::simple(150, 100), // Средняя деталь
            TileDimensions::simple(80, 60),   // Маленькая деталь
            TileDimensions::simple(70, 50),   // Маленькая деталь
            TileDimensions::simple(60, 40),   // Маленькая деталь
        ];
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем результаты
        assert!(placed_count >= 5, "Должно быть размещено минимум 5 из 6 деталей");
        assert!(solution.get_nbr_mosaics() <= 3, "Не должно использоваться больше 3 мозаик");
        
        let efficiency = solution.get_efficiency();
        println!("✅ Тест смешанных размеров: размещено {}/{} деталей на {} заготовках, эффективность {:.2}%", 
            placed_count, tiles.len(), solution.get_nbr_mosaics(), efficiency);
    }

    #[test]
    fn test_insufficient_stock() {
        // Тест: недостаточно заготовок для всех деталей
        let stock_tiles = vec![
            TileDimensions::simple(300, 200), // Маленькая заготовка
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Детали, которые не все поместятся
        let tiles = vec![
            TileDimensions::simple(280, 180), // Займет почти всю заготовку
            TileDimensions::simple(250, 150), // Не поместится
            TileDimensions::simple(200, 120), // Не поместится
        ];
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем результаты
        assert!(placed_count >= 1, "Должна быть размещена минимум одна деталь");
        assert_eq!(solution.get_nbr_mosaics(), 1, "Должна использоваться одна мозаика");
        assert!(solution.get_no_fit_panels().len() <= 2, "Должно быть максимум 2 неразмещенные детали");
        
        println!("✅ Тест недостаточных заготовок: размещено {}/{} деталей, неразмещенных: {}", 
            placed_count, tiles.len(), solution.get_no_fit_panels().len());
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_all_tiles_accounted() {
        // Тест: проверка что все детали учтены (размещены или в no_fit_panels)
        let stock_tiles = vec![
            TileDimensions::simple(500, 400),
            TileDimensions::simple(300, 200),
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        let tiles = vec![
            TileDimensions::simple(450, 350), // Поместится на первой
            TileDimensions::simple(280, 180), // Поместится на второй
            TileDimensions::simple(600, 500), // Не поместится никуда
        ];
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем баланс
        let total_accounted = solution.get_nbr_final_tiles() as usize + solution.get_no_fit_panels().len();
        assert_eq!(total_accounted, tiles.len(), 
            "Все детали должны быть учтены: размещено {} + неразмещенных {} = {} из {}", 
            solution.get_nbr_final_tiles(), solution.get_no_fit_panels().len(), 
            total_accounted, tiles.len());
        
        println!("✅ Тест учета всех деталей: размещено {}, неразмещенных {}, всего {}", 
            solution.get_nbr_final_tiles(), solution.get_no_fit_panels().len(), total_accounted);
    }

    #[test]
    fn test_stock_usage_correctness() {
        // Тест: проверка корректности использования заготовок
        let stock_tiles = vec![
            TileDimensions::simple(600, 400),
            TileDimensions::simple(500, 300),
            TileDimensions::simple(400, 250),
        ];
        
        let initial_stock_count = stock_tiles.len();
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        let tiles = vec![
            TileDimensions::simple(550, 350), // Потребует первую заготовку
            TileDimensions::simple(450, 250), // Потребует вторую заготовку
        ];
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем баланс заготовок
        let used_stocks = solution.get_nbr_mosaics() as usize;
        let unused_stocks = solution.get_unused_stock_panels().len();
        let total_stocks = used_stocks + unused_stocks;
        
        assert_eq!(total_stocks, initial_stock_count, 
            "Общее количество заготовок должно сохраняться: использовано {} + неиспользованных {} = {} из {}", 
            used_stocks, unused_stocks, total_stocks, initial_stock_count);
        
        println!("✅ Тест корректности заготовок: использовано {}, неиспользованных {}, всего {}", 
            used_stocks, unused_stocks, total_stocks);
    }

    #[test]
    fn test_efficiency_metrics() {
        // Тест: проверка метрик эффективности
        let stock_tiles = vec![
            TileDimensions::simple(1000, 600),
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        let tiles = vec![
            TileDimensions::simple(400, 300), // 120,000
            TileDimensions::simple(300, 200), // 60,000
            TileDimensions::simple(200, 150), // 30,000
        ];
        
        let expected_used_area: i64 = tiles.iter().map(|t| t.get_area()).sum();
        
        let mut placed_count = 0;
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        // Проверяем метрики
        let total_area = solution.get_total_area();
        let used_area = solution.get_used_area();
        let efficiency = solution.get_efficiency();
        
        assert_eq!(placed_count, 3, "Должны быть размещены все детали");
        assert_eq!(used_area, expected_used_area, "Используемая площадь должна соответствовать сумме площадей деталей");
        assert_eq!(total_area, 600000, "Общая площадь должна равняться площади заготовки");
        assert!((efficiency - (used_area as f64 / total_area as f64) * 100.0).abs() < 0.01, 
            "Эффективность должна быть правильно рассчитана");
        
        println!("✅ Тест метрик эффективности: площадь {}/{}, эффективность {:.2}%", 
            used_area, total_area, efficiency);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_large_number_of_tiles() {
        // Тест производительности: большое количество деталей на множестве заготовок
        let stock_tiles = vec![
            TileDimensions::simple(800, 600),
            TileDimensions::simple(800, 600),
            TileDimensions::simple(800, 600),
            TileDimensions::simple(800, 600),
            TileDimensions::simple(800, 600),
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Создаем много деталей разных размеров
        let mut tiles = Vec::new();
        for i in 0..50 {
            let width = 100 + (i % 10) * 20;
            let height = 80 + (i % 8) * 15;
            tiles.push(TileDimensions::simple(width, height));
        }
        
        let start_time = std::time::Instant::now();
        let mut placed_count = 0;
        
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        let duration = start_time.elapsed();
        
        // Проверяем производительность
        assert!(duration.as_millis() < 5000, "Алгоритм должен работать быстро даже с большим количеством деталей");
        assert!(placed_count >= 30, "Должно быть размещено минимум 30 из 50 деталей");
        
        let efficiency = solution.get_efficiency();
        println!("✅ Тест производительности: размещено {}/{} деталей за {:?}, эффективность {:.2}%", 
            placed_count, tiles.len(), duration, efficiency);
    }

    #[test]
    fn test_complex_scenario() {
        // Тест: комплексный сценарий с реалистичными мебельными деталями
        let stock_tiles = vec![
            TileDimensions::simple(2440, 1220), // Стандартный лист ДСП
            TileDimensions::simple(2440, 1220), // Второй лист
            TileDimensions::simple(1830, 915),  // Меньший лист
        ];
        
        let stock_solution = StockSolution::new(stock_tiles);
        let mut solution = Solution::from_stock_solution(&stock_solution);
        
        // Реалистичные мебельные детали
        let tiles = vec![
            TileDimensions::simple(800, 600),  // Столешница
            TileDimensions::simple(700, 400),  // Полка большая
            TileDimensions::simple(600, 300),  // Полка средняя
            TileDimensions::simple(500, 300),  // Дверца
            TileDimensions::simple(400, 300),  // Дверца маленькая
            TileDimensions::simple(350, 250),  // Ящик передняя стенка
            TileDimensions::simple(300, 200),  // Ящик боковая стенка
            TileDimensions::simple(300, 200),  // Ящик боковая стенка
            TileDimensions::simple(280, 180),  // Ящик задняя стенка
            TileDimensions::simple(200, 150),  // Маленькая деталь
        ];
        
        let start_time = std::time::Instant::now();
        let mut placed_count = 0;
        
        for tile in &tiles {
            match solution.try_place_tile(tile) {
                Ok(new_solutions) => {
                    if !new_solutions.is_empty() {
                        solution = new_solutions[0].clone();
                        placed_count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        
        let duration = start_time.elapsed();
        
        // Проверяем результаты
        assert!(placed_count >= 8, "Должно быть размещено минимум 8 из 10 деталей");
        assert!(solution.get_nbr_mosaics() <= 3, "Не должно использоваться больше 3 заготовок");
        
        let efficiency = solution.get_efficiency();
        assert!(efficiency > 20.0, "Эффективность должна быть разумной для реалистичного сценария");
        
        println!("✅ Комплексный тест: размещено {}/{} деталей на {} заготовках за {:?}, эффективность {:.2}%", 
            placed_count, tiles.len(), solution.get_nbr_mosaics(), duration, efficiency);
    }
}
