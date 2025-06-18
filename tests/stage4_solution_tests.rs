use cutting_cli::engine::model::{Solution, TileDimensions, Mosaic};
use cutting_cli::engine::stock::StockSolution;

#[cfg(test)]
mod stage4_tests {
    use super::*;

    /// Тест 1: Создание решения из стокового листа
    #[test]
    fn test_solution_creation_from_stock_solution() {
        // Вход: StockSolution с одним листом
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 1000, 600, "wood".to_string(), 1, None),
        ]);

        // Создать Solution(stockSolution)
        let solution = Solution::from_stock_solution(&stock_solution);

        // Проверки согласно плану
        assert_eq!(solution.get_mosaics().len(), 1, "Должна быть создана одна мозаика");
        assert_eq!(solution.get_unused_stock_panels().len(), 0, "Лист должен быть использован");
        assert!(solution.get_no_fit_panels().is_empty(), "Не должно быть неподходящих деталей");
        assert_eq!(solution.get_total_area(), 600000, "Общая площадь должна быть 600000");
        assert_eq!(solution.get_used_area(), 0, "Детали еще не размещены");
        assert_eq!(solution.get_unused_area(), 600000, "Вся площадь неиспользована");
        assert_eq!(solution.get_material(), Some("wood".to_string()), "Материал должен быть wood");
    }

    /// Тест 2: Размещение первой детали
    #[test]
    fn test_place_first_tile() {
        // Решение с одной мозаикой 1000x600
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 1000, 600, "wood".to_string(), 1, None),
        ]);
        let mut solution = Solution::from_stock_solution(&stock_solution);

        // Деталь для размещения
        let tile_to_place = TileDimensions::new(1, 400, 300, "wood".to_string(), 1, None);

        // Размещение детали
        let new_solutions = solution.try_place_tile(&tile_to_place).unwrap();

        // Проверки
        assert!(!new_solutions.is_empty(), "Должны быть созданы новые решения");
        let new_solution = &new_solutions[0];
        assert_eq!(new_solution.get_nbr_final_tiles(), 1, "Должна быть размещена одна деталь");
        assert_eq!(new_solution.get_used_area(), 120000, "Используемая площадь = 400*300");
        assert!(new_solution.get_no_fit_panels().is_empty(), "Деталь должна поместиться");
    }

    /// Тест 3: Размещение второй детали в том же листе
    #[test]
    fn test_place_second_tile_same_sheet() {
        // Создаем решение с уже размещенной деталью
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 1000, 600, "wood".to_string(), 1, None),
        ]);
        let mut solution = Solution::from_stock_solution(&stock_solution);

        // Размещаем первую деталь
        let tile1 = TileDimensions::new(1, 400, 300, "wood".to_string(), 1, None);
        let solutions_after_first = solution.try_place_tile(&tile1).unwrap();
        let mut solution_with_first = solutions_after_first[0].clone();

        // Размещаем вторую деталь
        let tile2 = TileDimensions::new(2, 350, 250, "wood".to_string(), 1, None);
        let solutions_after_second = solution_with_first.try_place_tile(&tile2).unwrap();

        // Проверки
        assert!(!solutions_after_second.is_empty(), "Должно быть место для второй детали");
        let final_solution = &solutions_after_second[0];
        assert_eq!(final_solution.get_nbr_final_tiles(), 2, "Должны быть размещены две детали");
        assert_eq!(final_solution.get_used_area(), 207500, "120000 + 87500 = 207500");
        assert!(final_solution.get_nbr_cuts() > 0, "Должны быть созданы разрезы");
    }

    /// Тест 4: Размещение детали, требующей новый лист
    #[test]
    fn test_place_tile_requiring_new_sheet() {
        // Создаем решение с маленьким листом
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 500, 400, "wood".to_string(), 1, None),
            TileDimensions::new(11, 1000, 600, "wood".to_string(), 1, None), // Дополнительный лист
        ]);
        let mut solution = Solution::from_stock_solution(&stock_solution);

        // Заполняем первый лист
        let small_tile = TileDimensions::new(1, 450, 350, "wood".to_string(), 1, None);
        let solutions_after_small = solution.try_place_tile(&small_tile).unwrap();
        let mut solution_with_small = solutions_after_small[0].clone();

        // Пытаемся разместить большую деталь
        let large_tile = TileDimensions::new(2, 800, 500, "wood".to_string(), 1, None);
        let initial_unused_count = solution_with_small.get_unused_stock_panels().len();
        let initial_mosaics_count = solution_with_small.get_mosaics().len();

        let solutions_after_large = solution_with_small.try_place_tile(&large_tile).unwrap();

        // Проверки
        assert!(!solutions_after_large.is_empty(), "Должно быть создано решение");
        let final_solution = &solutions_after_large[0];
        
        // Проверяем, что использован дополнительный лист или деталь в noFitPanels
        let used_additional_sheet = final_solution.get_unused_stock_panels().len() < initial_unused_count;
        let added_new_mosaic = final_solution.get_mosaics().len() > initial_mosaics_count;
        let added_to_no_fit = !final_solution.get_no_fit_panels().is_empty();

        assert!(used_additional_sheet || added_to_no_fit, "Должен быть использован дополнительный лист или деталь добавлена в noFitPanels");
        
        if used_additional_sheet && added_new_mosaic {
            assert_eq!(final_solution.get_nbr_final_tiles(), 2, "Должны быть размещены обе детали");
        }
    }

    /// Тест 5: Обработка неподходящих деталей
    #[test]
    fn test_handle_no_fit_panels() {
        // Решение с ограниченными стоковыми листами
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 100, 100, "wood".to_string(), 1, None), // Маленький лист
        ]);
        let mut solution = Solution::from_stock_solution(&stock_solution);

        // Деталь, которая не помещается
        let large_tile = TileDimensions::new(1, 200, 200, "wood".to_string(), 1, None);

        let new_solutions = solution.try_place_tile(&large_tile).unwrap();

        // Проверки
        assert!(!new_solutions.is_empty(), "Должно быть создано решение");
        let final_solution = &new_solutions[0];
        assert!(final_solution.get_no_fit_panels().contains(&large_tile), "Деталь должна быть в noFitPanels");
        assert_eq!(final_solution.get_nbr_final_tiles(), 0, "Количество размещенных деталей не должно измениться");
        assert_eq!(final_solution.get_used_area(), 0, "Используемая площадь не должна измениться");
    }

    /// Тест 6: Последовательное размещение нескольких деталей
    #[test]
    fn test_sequential_placement_multiple_tiles() {
        // Пустое решение с одним листом 1000x600
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 1000, 600, "wood".to_string(), 1, None),
        ]);
        let mut current_solution = Solution::from_stock_solution(&stock_solution);

        // Список деталей для размещения
        let tiles_to_place = vec![
            TileDimensions::new(1, 400, 300, "wood".to_string(), 1, None),
            TileDimensions::new(2, 350, 250, "wood".to_string(), 1, None),
            TileDimensions::new(3, 200, 150, "wood".to_string(), 1, None),
            TileDimensions::new(4, 180, 120, "wood".to_string(), 1, None),
        ];

        let mut expected_used_area = 0i64;

        // Последовательно размещаем все детали
        for (i, tile) in tiles_to_place.iter().enumerate() {
            let new_solutions = current_solution.try_place_tile(tile).unwrap();
            assert!(!new_solutions.is_empty(), "Должно быть создано решение для детали {}", i + 1);
            
            current_solution = new_solutions[0].clone();
            expected_used_area += tile.get_area();

            // Проверки после каждого размещения
            assert_eq!(current_solution.get_nbr_final_tiles(), (i + 1) as i32, 
                      "Количество размещенных деталей должно увеличиться на 1");
            
            if current_solution.get_no_fit_panels().is_empty() {
                assert_eq!(current_solution.get_used_area(), expected_used_area,
                          "Используемая площадь должна увеличиться на площадь детали");
            }
        }

        // Финальная проверка
        let successfully_placed = tiles_to_place.len() as i32 - current_solution.get_no_fit_panels().len() as i32;
        assert_eq!(current_solution.get_nbr_final_tiles(), successfully_placed, 
                  "Все подходящие детали должны быть размещены");
        assert!(current_solution.get_nbr_cuts() > 0, "Должны быть созданы разрезы");
    }

    /// Тест 7: Копирование решения
    #[test]
    fn test_solution_copying() {
        // Создаем решение с несколькими мозаиками
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 1000, 600, "wood".to_string(), 1, None),
            TileDimensions::new(11, 800, 500, "wood".to_string(), 1, None),
        ]);
        let mut original_solution = Solution::from_stock_solution(&stock_solution);

        // Добавляем деталь в noFitPanels
        let no_fit_tile = TileDimensions::new(99, 2000, 2000, "wood".to_string(), 1, None);
        original_solution.get_no_fit_panels_mut().push(no_fit_tile);

        // Размещаем деталь
        let tile = TileDimensions::new(1, 400, 300, "wood".to_string(), 1, None);
        let solutions_with_tile = original_solution.try_place_tile(&tile).unwrap();
        let original_with_tile = &solutions_with_tile[0];

        // Создаем копию
        let copied_solution = Solution::copy(original_with_tile);

        // Проверки
        assert_ne!(original_with_tile.get_id(), copied_solution.get_id(), "ID должны быть разными");
        assert_eq!(original_with_tile.get_mosaics().len(), copied_solution.get_mosaics().len(), 
                  "Количество мозаик должно совпадать");
        assert_eq!(original_with_tile.get_no_fit_panels().len(), copied_solution.get_no_fit_panels().len(),
                  "Количество неподходящих деталей должно совпадать");
        assert_eq!(original_with_tile.get_unused_stock_panels().len(), copied_solution.get_unused_stock_panels().len(),
                  "Количество неиспользованных панелей должно совпадать");
        
        // Проверяем, что метрики идентичны
        assert_eq!(original_with_tile.get_total_area(), copied_solution.get_total_area());
        assert_eq!(original_with_tile.get_used_area(), copied_solution.get_used_area());
        assert_eq!(original_with_tile.get_nbr_cuts(), copied_solution.get_nbr_cuts());
    }

    /// Тест 8: Создание решения исключающего мозаику
    #[test]
    fn test_solution_excluding_mosaic() {
        // Создаем решение с несколькими мозаиками
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 1000, 600, "wood".to_string(), 1, None),
            TileDimensions::new(11, 800, 500, "wood".to_string(), 1, None),
            TileDimensions::new(12, 600, 400, "wood".to_string(), 1, None),
        ]);
        let mut original_solution = Solution::from_stock_solution(&stock_solution);

        // Добавляем дополнительные мозаики
        let additional_mosaic1 = Mosaic::new(&TileDimensions::new(13, 500, 300, "wood".to_string(), 1, None));
        let additional_mosaic2 = Mosaic::new(&TileDimensions::new(14, 400, 200, "wood".to_string(), 1, None));
        original_solution.add_mosaic(additional_mosaic1.clone());
        original_solution.add_mosaic(additional_mosaic2);

        let original_mosaic_count = original_solution.get_mosaics().len();
        let mosaic_to_exclude = &additional_mosaic1;

        // Создаем решение исключающее мозаику
        let new_solution = Solution::copy_excluding_mosaic(&original_solution, mosaic_to_exclude);

        // Проверки
        assert_eq!(new_solution.get_mosaics().len(), original_mosaic_count - 1,
                  "Количество мозаик должно уменьшиться на 1");
        
        // Проверяем, что исключенная мозаика отсутствует
        assert!(!new_solution.get_mosaics().contains(mosaic_to_exclude),
               "Исключенная мозаика не должна присутствовать");
        
        // Проверяем, что остальные данные скопированы
        assert_eq!(new_solution.get_no_fit_panels().len(), original_solution.get_no_fit_panels().len());
        assert_eq!(new_solution.get_unused_stock_panels().len(), original_solution.get_unused_stock_panels().len());
    }

    /// Тест 9: Расчет метрик решения
    #[test]
    fn test_solution_metrics_calculation() {
        // Создаем решение с размещенными деталями
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 1000, 600, "wood".to_string(), 1, None),
            TileDimensions::new(11, 800, 500, "wood".to_string(), 1, None),
        ]);
        let mut solution = Solution::from_stock_solution(&stock_solution);

        // Размещаем несколько деталей
        let tiles = vec![
            TileDimensions::new(1, 400, 300, "wood".to_string(), 1, None),
            TileDimensions::new(2, 350, 250, "wood".to_string(), 1, None),
        ];

        for tile in &tiles {
            let new_solutions = solution.try_place_tile(tile).unwrap();
            if !new_solutions.is_empty() {
                solution = new_solutions[0].clone();
            }
        }

        // Добавляем неподходящую деталь
        let no_fit_tile = TileDimensions::new(99, 2000, 2000, "wood".to_string(), 1, None);
        solution.get_no_fit_panels_mut().push(no_fit_tile);

        // Проверяем метрики
        let total_area = solution.get_total_area();
        let used_area = solution.get_used_area();
        let unused_area = solution.get_unused_area();

        assert_eq!(total_area, used_area + unused_area, "Общая площадь = используемая + неиспользуемая");
        assert!(solution.get_nbr_cuts() >= 0, "Количество разрезов должно быть неотрицательным");
        assert!(solution.get_nbr_final_tiles() >= 0, "Количество финальных плиток должно быть неотрицательным");
        assert!(solution.get_used_area_ratio() >= 0.0, "Коэффициент использования должен быть неотрицательным");
        assert!(solution.get_used_area_ratio() <= 1.0, "Коэффициент использования не должен превышать 1.0");
        
        let biggest_area = solution.get_biggest_area();
        assert!(biggest_area >= 0, "Наибольшая площадь должна быть неотрицательной");
        
        let most_unused_panel_area = solution.get_most_unused_panel_area();
        assert!(most_unused_panel_area >= 0, "Наибольшая неиспользуемая площадь панели должна быть неотрицательной");
    }

    /// Тест 10: Сортировка мозаик
    #[test]
    fn test_mosaic_sorting() {
        let mut solution = Solution::new();

        // Создаем мозаики с разной степенью заполнения
        let mosaic1 = Mosaic::new(&TileDimensions::new(1, 1000, 600, "wood".to_string(), 1, None)); // Большая
        let mosaic2 = Mosaic::new(&TileDimensions::new(2, 500, 400, "wood".to_string(), 1, None));  // Средняя
        let mosaic3 = Mosaic::new(&TileDimensions::new(3, 300, 200, "wood".to_string(), 1, None));  // Маленькая

        // Добавляем в произвольном порядке
        solution.add_mosaic(mosaic1);
        solution.add_mosaic(mosaic3.clone());
        solution.add_mosaic(mosaic2);

        // Проверяем автоматическую сортировку
        let mosaics = solution.get_mosaics();
        assert_eq!(mosaics.len(), 3);

        // Мозаики должны быть отсортированы по неиспользуемой площади (по возрастанию)
        for i in 0..mosaics.len() - 1 {
            let current_unused = mosaics[i].get_unused_area_immutable();
            let next_unused = mosaics[i + 1].get_unused_area_immutable();
            assert!(current_unused <= next_unused, 
                   "Мозаики должны быть отсортированы по неиспользуемой площади");
        }

        // Первая мозаика должна иметь наименьшую свободную площадь
        let first_mosaic_area = mosaics[0].get_unused_area_immutable();
        let smallest_expected = mosaic3.get_unused_area_immutable();
        assert_eq!(first_mosaic_area, smallest_expected, 
                  "Первая мозаика должна иметь наименьшую свободную площадь");
    }

    /// Тест 11: Граничные случаи
    #[test]
    fn test_edge_cases() {
        // Создание решения без стоковых листов
        let empty_stock = StockSolution::new(vec![]);
        let empty_solution = Solution::from_stock_solution(&empty_stock);
        assert_eq!(empty_solution.get_mosaics().len(), 0, "Не должно быть мозаик");
        assert_eq!(empty_solution.get_total_area(), 0, "Общая площадь должна быть 0");

        // Размещение деталей в полностью заполненном решении
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 100, 100, "wood".to_string(), 1, None),
        ]);
        let mut solution = Solution::from_stock_solution(&stock_solution);

        // Заполняем полностью
        let exact_tile = TileDimensions::new(1, 100, 100, "wood".to_string(), 1, None);
        let solutions_after_exact = solution.try_place_tile(&exact_tile).unwrap();
        let mut filled_solution = solutions_after_exact[0].clone();

        // Пытаемся добавить еще одну деталь
        let another_tile = TileDimensions::new(2, 50, 50, "wood".to_string(), 1, None);
        let solutions_after_another = filled_solution.try_place_tile(&another_tile).unwrap();
        let final_solution = &solutions_after_another[0];

        // Деталь должна попасть в noFitPanels или использовать дополнительный лист
        let has_no_fit = !final_solution.get_no_fit_panels().is_empty();
        let used_additional = final_solution.get_mosaics().len() > 1;
        assert!(has_no_fit || used_additional, "Деталь должна быть обработана корректно");

        // Работа с пустыми списками
        let empty_solution = Solution::new();
        assert!(empty_solution.get_no_fit_panels().is_empty());
        assert!(empty_solution.get_unused_stock_panels().is_empty());
        assert!(empty_solution.get_mosaics().is_empty());

        // Копирование пустого решения
        let empty_copy = Solution::copy(&empty_solution);
        assert_ne!(empty_solution.get_id(), empty_copy.get_id());
        assert_eq!(empty_solution.get_mosaics().len(), empty_copy.get_mosaics().len());
    }

    /// Дополнительный тест: Проверка материалов
    #[test]
    fn test_material_handling() {
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 1000, 600, "wood".to_string(), 1, None),
        ]);
        let mut solution = Solution::from_stock_solution(&stock_solution);

        // Размещение детали с тем же материалом
        let wood_tile = TileDimensions::new(1, 400, 300, "wood".to_string(), 1, None);
        let solutions_wood = solution.try_place_tile(&wood_tile).unwrap();
        assert!(!solutions_wood.is_empty(), "Деталь с подходящим материалом должна размещаться");

        // Проверяем материал решения
        assert_eq!(solution.get_material(), Some("wood".to_string()));
    }

    /// Дополнительный тест: Производительность
    #[test]
    fn test_performance_multiple_placements() {
        let stock_solution = StockSolution::new(vec![
            TileDimensions::new(10, 2000, 1500, "wood".to_string(), 1, None),
        ]);
        let mut solution = Solution::from_stock_solution(&stock_solution);

        let start_time = std::time::Instant::now();

        // Размещаем много маленьких деталей
        for i in 1..=20 {
            let tile = TileDimensions::new(i, 100, 80, "wood".to_string(), 1, None);
            if let Ok(new_solutions) = solution.try_place_tile(&tile) {
                if !new_solutions.is_empty() {
                    solution = new_solutions[0].clone();
                }
            }
        }

        let duration = start_time.elapsed();
        assert!(duration.as_millis() < 5000, "Операции должны выполняться за разумное время");
        
        println!("Размещено {} деталей за {:?}", solution.get_nbr_final_tiles(), duration);
    }
}
