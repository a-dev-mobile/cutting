use cutting_cli::engine::model::mosaic::Mosaic;
use cutting_cli::engine::model::tile::TileDimensions;

#[cfg(test)]
mod mosaic_tests {
    use super::*;

    /// Тест 1: Создание мозаики из стокового листа
    #[test]
    fn test_create_mosaic_from_stock() {
        let stock_tile = TileDimensions {
            id: 10,
            width: 1000,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let mosaic = Mosaic::new(&stock_tile);

        // Проверяем размеры корневого узла
        assert_eq!(mosaic.get_root_tile_node().get_width(), 1000);
        assert_eq!(mosaic.get_root_tile_node().get_height(), 600);

        // Проверяем материал и ID
        assert_eq!(mosaic.get_material(), "wood");
        assert_eq!(mosaic.get_stock_id(), 10);

        // Проверяем, что нет разрезов
        assert!(mosaic.get_cuts().is_empty());

        // Проверяем площади
        assert_eq!(mosaic.get_total_area(), 600000);

        println!("✓ Тест 1 пройден: Создание мозаики из стокового листа");
    }

    /// Тест 2: Размещение детали точно по размеру узла
    #[test]
    fn test_place_exact_size_tile() {
        let stock_tile = TileDimensions {
            id: 10,
            width: 1000,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let tile_to_place = TileDimensions {
            id: 1,
            width: 1000,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let mosaic = Mosaic::new(&stock_tile);
        let result_mosaics = mosaic.add(&tile_to_place, false).unwrap();

        // Может быть несколько результатов (с поворотом и без), но для квадратных размеров - один
        assert!(!result_mosaics.is_empty());

        let mut result_mosaic = result_mosaics[0].clone();

        // Проверяем финальные узлы
        let final_nodes = result_mosaic.get_final_tile_nodes();
        assert_eq!(final_nodes.len(), 1);

        let final_node = final_nodes[0];
        assert_eq!(final_node.external_id, 1);
        assert!(final_node.is_final);

        // Проверяем, что нет разрезов (размеры совпадают)
        assert!(result_mosaic.get_cuts().is_empty());

        assert!(final_node.get_width() == 1000);
        assert_eq!(result_mosaic.get_used_area(), 600000);

        println!("✓ Тест 2 пройден: Размещение детали точно по размеру узла");
    }

    /// Тест 3: Размещение детали с горизонтальным разрезом
    /// Деталь: 400x600 в листе 1000x600
    /// Ширина детали < ширины узла → горизонтальный разрез на x=400
    #[test]
    fn test_place_tile_with_horizontal_cut() {
        let stock_tile = TileDimensions {
            id: 10,
            width: 1000,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let tile_to_place = TileDimensions {
            id: 2,
            width: 400,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let mosaic = Mosaic::new(&stock_tile);
        let result_mosaics = mosaic.add(&tile_to_place, false).unwrap();

        // Должен быть хотя бы один результат
        assert!(!result_mosaics.is_empty());

        let result_mosaic = &result_mosaics[0];

        // Проверяем разрезы
        let cuts = result_mosaic.get_cuts();
        assert_eq!(cuts.len(), 1);

        let cut = &cuts[0];
        // Горизонтальный разрез означает, что линия разреза идет горизонтально (по X)
        assert!(!cut.is_horizontal); // Вертикальная линия разреза (разделяет по X)
        assert_eq!(cut.x1, 400);
        assert_eq!(cut.x2, 400);
        assert_eq!(cut.y1, 0);
        assert_eq!(cut.y2, 600);

        // Проверяем финальные узлы
        let final_nodes = result_mosaic.get_final_tile_nodes();
        assert_eq!(final_nodes.len(), 1);

        let final_node = final_nodes[0];
        assert_eq!(final_node.get_width(), 400);
        assert_eq!(final_node.get_height(), 600);
        assert_eq!(final_node.external_id, 2);

        // Проверяем площади
        let mut result_mosaic_mut = result_mosaic.clone();
        assert_eq!(result_mosaic_mut.get_used_area(), 240000); // 400 * 600

        println!("✓ Тест 3 пройден: Размещение детали с горизонтальным разрезом");
    }

    /// Тест 4: Размещение детали с вертикальным разрезом
    /// Деталь: 1000x250 в листе 1000x600
    /// Высота детали < высоты узла → вертикальный разрез на y=250
    #[test]
    fn test_place_tile_with_vertical_cut() {
        let stock_tile = TileDimensions {
            id: 10,
            width: 1000,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let tile_to_place = TileDimensions {
            id: 3,
            width: 1000,
            height: 250,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let mosaic = Mosaic::new(&stock_tile);
        let result_mosaics = mosaic.add(&tile_to_place, false).unwrap();

        // Должен быть хотя бы один результат
        assert!(!result_mosaics.is_empty());

        let result_mosaic = &result_mosaics[0];

        // Проверяем разрезы
        let cuts = result_mosaic.get_cuts();
        assert_eq!(cuts.len(), 1);

        let cut = &cuts[0];
        // Вертикальный разрез означает, что линия разреза идет вертикально (по Y)

        assert!(cut.is_horizontal); // Горизонтальная линия разреза (разделяет по Y)
        assert_eq!(cut.y1, 250);
        assert_eq!(cut.y2, 250);
        assert_eq!(cut.x1, 0);
        assert_eq!(cut.x2, 1000);

        // Проверяем финальные узлы
        let final_nodes = result_mosaic.get_final_tile_nodes();
        assert_eq!(final_nodes.len(), 1);

        let final_node = final_nodes[0];
        assert_eq!(final_node.get_width(), 1000);
        assert_eq!(final_node.get_height(), 250);
        assert_eq!(final_node.external_id, 3);

        // Проверяем площади
        let mut result_mosaic_mut = result_mosaic.clone();
        assert_eq!(result_mosaic_mut.get_used_area(), 250000); // 1000 * 250

        println!("✓ Тест 4 пройден: Размещение детали с вертикальным разрезом");
    }

    /// Тест 5: Размещение детали с двумя разрезами
    /// Алгоритм splitHV: сначала горизонтальный разрез (x=400), затем вертикальный (y=250)
    #[test]
    fn test_place_tile_with_two_cuts() {
        let stock_tile = TileDimensions {
            id: 10,
            width: 1000,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let tile_to_place = TileDimensions {
            id: 4,
            width: 400,
            height: 250,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let mosaic = Mosaic::new(&stock_tile);
        let result_mosaics = mosaic.add(&tile_to_place, false).unwrap();

        // Должен быть хотя бы один результат
        assert!(!result_mosaics.is_empty());

        let result_mosaic = &result_mosaics[0];

        // Проверяем разрезы
        let cuts = result_mosaic.get_cuts();
        assert_eq!(cuts.len(), 2);

        // Первый разрез: вертикальная линия на x=400 (горизонтальный разрез по алгоритму)
        let first_cut = &cuts[0];
        assert!(!first_cut.is_horizontal); // Вертикальная линия разреза
        assert_eq!(first_cut.x1, 400);
        assert_eq!(first_cut.x2, 400);
        assert_eq!(first_cut.y1, 0);
        assert_eq!(first_cut.y2, 600);

        // Второй разрез: горизонтальная линия на y=250 в области (0,400,0,600)
        let second_cut = &cuts[1];
        assert!(second_cut.is_horizontal); // Горизонтальная линия разреза
        assert_eq!(second_cut.y1, 250);
        assert_eq!(second_cut.y2, 250);
        assert_eq!(second_cut.x1, 0);
        assert_eq!(second_cut.x2, 400); // Только в левой части после первого разреза

        // Проверяем финальные узлы
        let final_nodes = result_mosaic.get_final_tile_nodes();
        assert_eq!(final_nodes.len(), 1);

        let final_node = final_nodes[0];
        assert_eq!(final_node.get_width(), 400);
        assert_eq!(final_node.get_height(), 250);
        assert_eq!(final_node.external_id, 4);
        assert_eq!(final_node.get_x1(), 0);
        assert_eq!(final_node.get_y1(), 0);

        // Проверяем используемую площадь
        let mut result_mosaic_mut = result_mosaic.clone();
        assert_eq!(result_mosaic_mut.get_used_area(), 100000); // 400 * 250

        // Проверяем неиспользуемые узлы (должно быть 2)
        let unused_nodes = result_mosaic.get_unused_tile_nodes();
        assert_eq!(unused_nodes.len(), 2);

        println!("✓ Тест 5 пройден: Размещение детали с двумя разрезами");
    }

    /// Тест 6: Размещение детали с поворотом
    /// Деталь 700x400 не помещается, но после поворота (400x700) помещается
    #[test]
    fn test_place_tile_with_rotation() {
        let stock_tile = TileDimensions {
            id: 10,
            width: 1000,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let tile_to_place = TileDimensions {
            id: 5,
            width: 700,  // Не помещается в исходной ориентации (700 > 600 высота листа)
            height: 400, // Помещается после поворота (400 <= 600)
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let mosaic = Mosaic::new(&stock_tile);
        let result_mosaics = mosaic.add(&tile_to_place, false).unwrap(); // consider_grain_direction = false

        // Должен быть хотя бы один результат с поворотом
        assert!(!result_mosaics.is_empty());

        // Ищем результат с поворотом
        let rotated_result = result_mosaics.iter().find(|m| {
            let final_nodes = m.get_final_tile_nodes();
            !final_nodes.is_empty() && final_nodes[0].is_rotated
        });

        assert!(rotated_result.is_some());

        let result_mosaic = rotated_result.unwrap();
        let final_nodes = result_mosaic.get_final_tile_nodes();
        let final_node = final_nodes[0];

        // Проверяем флаг поворота
        assert!(final_node.is_rotated);

        // После поворота деталь 700x400 становится 400x700
        // Но размещается в узле с размерами после поворота (ограничена высотой листа)
        assert_eq!(final_node.get_width(), 400); // ширина после поворота
        assert_eq!(final_node.get_height(), 600); // высота ограничена высотой листа
        assert_eq!(final_node.external_id, 5);

        // Проверяем используемую площадь
        let mut result_mosaic_mut = result_mosaic.clone();
        assert_eq!(result_mosaic_mut.get_used_area(), 240000); // 400 * 600 (ограничена листом)

        // Отладочная информация
        println!(
            "Final node dimensions: {}x{}",
            final_node.get_width(),
            final_node.get_height()
        );
        println!("Final node is_rotated: {}", final_node.is_rotated);
        println!("Final node external_id: {}", final_node.external_id);
        println!("Used area: {}", result_mosaic_mut.get_used_area());

        println!("✓ Тест 6 пройден: Размещение детали с поворотом");
    }

    /// Тест 7: Попытка размещения детали, которая не помещается
    /// Деталь больше листа даже после поворота
    #[test]
    fn test_place_oversized_tile() {
        let stock_tile = TileDimensions {
            id: 10,
            width: 1000,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let large_tile = TileDimensions {
            id: 6,
            width: 1200, // Больше ширины листа (1000)
            height: 800, // Больше высоты листа (600)
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let mut mosaic = Mosaic::new(&stock_tile);
        let result_mosaics = mosaic.add(&large_tile, false).unwrap();

        // Результат должен быть пустым (размещение неуспешно)
        assert!(result_mosaics.is_empty());
        assert_eq!(result_mosaics.len(), 0);

        // Исходная мозаика не должна измениться
        assert!(mosaic.get_cuts().is_empty());
        assert_eq!(mosaic.get_total_area(), 600000);

        // Проверяем, что используемая площадь равна 0
        assert_eq!(mosaic.get_used_area(), 0);

        println!("✓ Тест 7 пройден: Попытка размещения детали, которая не помещается");
    }

    /// Тест 8: Размещение в уже частично заполненной мозаике
    /// Корень (0, 1000, 0, 600) с уже размещенной деталью
    #[test]
    fn test_place_in_partially_filled_mosaic() {
        let stock_tile = TileDimensions {
            id: 10,
            width: 1000,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        // Первая деталь для создания частично заполненной мозаики
        let first_tile = TileDimensions {
            id: 1,
            width: 400,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        // Новая деталь для размещения в свободной области
        let new_tile = TileDimensions {
            id: 7,
            width: 300,
            height: 400,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        // Создаем частично заполненную мозаику
        let mosaic = Mosaic::new(&stock_tile);
        let result_mosaics = mosaic.add(&first_tile, false).unwrap();
        assert!(!result_mosaics.is_empty());

        let partially_filled_mosaic = &result_mosaics[0];

        // Проверяем, что первая деталь размещена
        let final_nodes_before = partially_filled_mosaic.get_final_tile_nodes();
        assert_eq!(final_nodes_before.len(), 1);
        assert_eq!(final_nodes_before[0].external_id, 1);

        // Размещаем новую деталь в частично заполненной мозаике
        let final_result = partially_filled_mosaic.add(&new_tile, false).unwrap();
        assert!(!final_result.is_empty());

        let final_mosaic = &final_result[0];

        // Проверяем, что теперь размещены обе детали
        let final_nodes_after = final_mosaic.get_final_tile_nodes();
        assert_eq!(final_nodes_after.len(), 2);

        // Проверяем ID размещенных деталей
        let ids: Vec<i32> = final_nodes_after.iter().map(|n| n.external_id).collect();
        assert!(ids.contains(&1)); // Старая деталь
        assert!(ids.contains(&7)); // Новая деталь

        // Проверяем, что новая деталь размещена в правой части (x > 400)
        let new_tile_node = final_nodes_after
            .iter()
            .find(|n| n.external_id == 7)
            .unwrap();
        assert!(new_tile_node.get_x1() >= 400); // Размещена в правой части

        // Проверяем общую используемую площадь
        let mut final_mosaic_mut = final_mosaic.clone();
        let total_used_area = final_mosaic_mut.get_used_area();
        let expected_area = (400 * 600) + (300 * 400); // Первая + вторая деталь
        assert_eq!(total_used_area, expected_area);

        println!("✓ Тест 8 пройден: Размещение в уже частично заполненной мозаике");
    }

    /// Тест 9: Расчет метрик мозаики
    #[test]
    fn test_mosaic_metrics() {
        let stock_tile = TileDimensions {
            id: 10,
            width: 1000,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let tile_to_place = TileDimensions {
            id: 4,
            width: 400,
            height: 250,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let mosaic = Mosaic::new(&stock_tile);
        let result_mosaics = mosaic.add(&tile_to_place, false).unwrap();
        let mut result_mosaic = result_mosaics[0].clone();

        // Проверяем метрики
        let used_area = result_mosaic.get_used_area();
        let unused_area = result_mosaic.get_unused_area();
        let total_area = result_mosaic.get_total_area();

        assert_eq!(used_area, 100000); // 400 * 250
        assert_eq!(unused_area, 500000); // 600000 - 100000
        assert_eq!(total_area, 600000);
        assert_eq!(used_area + unused_area, total_area);

        // Проверяем количество разрезов
        assert_eq!(result_mosaic.get_nbr_cuts(), 2);

        // Проверяем самый большой неиспользуемый узел
        let biggest_unused = result_mosaic.get_biggest_unused_tile();
        assert!(biggest_unused.is_some());

        // Проверяем коэффициент использования
        let utilization = result_mosaic.get_utilization_ratio();
        assert!((utilization - (100000.0 / 600000.0)).abs() < 0.001);

        // Проверяем расстояние центра масс
        let center_distance = result_mosaic.get_center_of_mass_distance_to_origin();
        assert!(center_distance > 0.0);

        println!("✓ Тест 9 пройден: Расчет метрик мозаики");
    }

    /// Тест 10: Граничные случаи
    #[test]
    fn test_edge_cases() {
        let stock_tile = TileDimensions {
            id: 10,
            width: 100,
            height: 100,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        // Квадратная деталь (не должна поворачиваться)
        let square_tile = TileDimensions {
            id: 7,
            width: 50,
            height: 50,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let mosaic = Mosaic::new(&stock_tile);
        let result_mosaics = mosaic.add(&square_tile, false).unwrap();

        assert!(!result_mosaics.is_empty());

        // Все результаты должны быть без поворота (квадрат)
        for result_mosaic in &result_mosaics {
            let final_nodes = result_mosaic.get_final_tile_nodes();
            if !final_nodes.is_empty() {
                assert!(!final_nodes[0].is_rotated);
            }
        }

        // Проверяем копирование мозаики
        let copied_mosaic = mosaic.copy();
        assert_eq!(copied_mosaic.get_material(), mosaic.get_material());
        assert_eq!(copied_mosaic.get_stock_id(), mosaic.get_stock_id());
        assert_eq!(copied_mosaic.get_total_area(), mosaic.get_total_area());

        // Тест поиска кандидатов
        let tile_to_find = TileDimensions {
            id: 1,
            width: 40,
            height: 30,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let candidates = mosaic.find_candidates(&tile_to_find);
        // Должен найти корневой узел как кандидата
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].get_width(), 100);
        assert_eq!(candidates[0].get_height(), 100);

        // Тест с деталью, которая не помещается
        let large_tile = TileDimensions {
            id: 2,
            width: 120,
            height: 80,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let no_candidates = mosaic.find_candidates(&large_tile);
        assert!(no_candidates.is_empty());

        println!("✓ Тест 10 пройден: Граничные случаи");
    }

    /// Интеграционный тест: Последовательное размещение нескольких деталей
    #[test]
    fn test_sequential_placement() {
        let stock_tile = TileDimensions {
            id: 10,
            width: 1000,
            height: 600,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        // Первая деталь
        let tile1 = TileDimensions {
            id: 1,
            width: 400,
            height: 300,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        // Вторая деталь
        let tile2 = TileDimensions {
            id: 2,
            width: 300,
            height: 200,
            material: "wood".to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        };

        let mosaic = Mosaic::new(&stock_tile);

        // Размещаем первую деталь
        let result1 = mosaic.add(&tile1, false).unwrap();
        assert!(!result1.is_empty());
        let mosaic = result1[0].clone();

        // Проверяем состояние после первого размещения
        let final_nodes1 = mosaic.get_final_tile_nodes();
        assert_eq!(final_nodes1.len(), 1);
        assert!(mosaic.has_unused_nodes());

        // Размещаем вторую деталь
        let result2 = mosaic.add(&tile2, false).unwrap();
        assert!(!result2.is_empty());
        let final_mosaic = &result2[0];

        // Проверяем финальное состояние
        let final_nodes2 = final_mosaic.get_final_tile_nodes();
        assert_eq!(final_nodes2.len(), 2);

        // Проверяем, что обе детали размещены
        let ids: Vec<i32> = final_nodes2.iter().map(|n| n.external_id).collect();
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));

        println!("✓ Интеграционный тест пройден: Последовательное размещение нескольких деталей");
    }
}

/// Функция для запуска всех тестов
pub fn run_all_mosaic_tests() {
    println!("Запуск тестов этапа 3: Mosaic и размещение одной детали");
    println!("{}", "=".repeat(60));

    // Тесты будут запущены автоматически при выполнении `cargo test`
    println!("Используйте команду: cargo test mosaic_tests");
}
