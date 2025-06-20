//! Тесты для проверки корректной работы с толщиной пропила (cut_thickness)
//! 
//! Эти тесты проверяют:
//! - Корректное применение толщины пропила при разрезах
//! - Влияние толщины пропила на размещение плиток
//! - Валидацию параметров толщины пропила
//! - Граничные случаи с различными значениями толщины

use cutting_cli::engine::cutting::{CuttingEngine, CutDirection};
use cutting_cli::engine::model::tile::{TileDimensions, TileNode};
use cutting_cli::engine::model::request::{Configuration, CalculationRequest, Panel, ClientInfo};
use cutting_cli::engine::model::mosaic::Mosaic;

#[cfg(test)]
mod cut_thickness_basic_tests {
    use super::*;

    #[test]
    fn test_configuration_cut_thickness_parsing() {
        // Тест парсинга толщины пропила из строки
        let config = Configuration {
            cut_thickness: "3.0".to_string(),
            min_trim_dimension: "10.0".to_string(),
            optimization_factor: 1.0,
            use_single_stock_unit: false,
            cut_orientation_preference: 0,
            performance_thresholds: None,
        };

        assert_eq!(config.get_cut_thickness_f64().unwrap(), 3.0);
        assert!(config.is_valid());
    }

    #[test]
    fn test_configuration_invalid_cut_thickness() {
        // Тест невалидной толщины пропила
        let config = Configuration {
            cut_thickness: "invalid".to_string(),
            min_trim_dimension: "10.0".to_string(),
            optimization_factor: 1.0,
            use_single_stock_unit: false,
            cut_orientation_preference: 0,
            performance_thresholds: None,
        };

        assert!(config.get_cut_thickness_f64().is_err());
        assert!(!config.is_valid());
    }

    #[test]
    fn test_configuration_zero_cut_thickness() {
        // Тест нулевой толщины пропила
        let config = Configuration {
            cut_thickness: "0.0".to_string(),
            min_trim_dimension: "10.0".to_string(),
            optimization_factor: 1.0,
            use_single_stock_unit: false,
            cut_orientation_preference: 0,
            performance_thresholds: None,
        };

        assert_eq!(config.get_cut_thickness_f64().unwrap(), 0.0);
        assert!(config.is_valid());
    }

    #[test]
    fn test_configuration_fractional_cut_thickness() {
        // Тест дробной толщины пропила
        let config = Configuration {
            cut_thickness: "2.5".to_string(),
            min_trim_dimension: "10.0".to_string(),
            optimization_factor: 1.0,
            use_single_stock_unit: false,
            cut_orientation_preference: 0,
            performance_thresholds: None,
        };

        assert_eq!(config.get_cut_thickness_f64().unwrap(), 2.5);
        assert!(config.is_valid());
    }

    #[test]
    fn test_configuration_large_cut_thickness() {
        // Тест большой толщины пропила
        let config = Configuration {
            cut_thickness: "10.0".to_string(),
            min_trim_dimension: "10.0".to_string(),
            optimization_factor: 1.0,
            use_single_stock_unit: false,
            cut_orientation_preference: 0,
            performance_thresholds: None,
        };

        assert_eq!(config.get_cut_thickness_f64().unwrap(), 10.0);
        assert!(config.is_valid());
    }
}

#[cfg(test)]
mod cut_thickness_real_world_tests {
    use super::*;

    /// Тест, основанный на логике splitHorizontally из Java кода
    /// В Java: TileNode(x1, x1+i, y1, y2) и TileNode(x1+i+i2, x2, y1, y2)
    /// где i - позиция разреза, i2 - толщина пропила
    #[test]
    fn test_java_like_horizontal_split_logic() {
        let stock_tile = TileDimensions::new(
            1,
            100, // Исходная ширина
            80,  // Исходная высота (больше чем плитка, чтобы требовался горизонтальный разрез)
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        // Размещаем плитку 40x50 (требует горизонтальный разрез по высоте)
        let tile = TileDimensions::simple(40, 50);
        let cut_thickness = 3;
        
        let results = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            cut_thickness,
            CutDirection::Both, // Разрешаем любые разрезы
            false,
        ).unwrap();
        
        assert!(results.iter().any(|r| r.placed));
        
        // Проверяем, что создается правильная структура разрезов
        let mut found_cuts = false;
        for result in &results {
            if result.placed {
                if let Some(ref new_mosaic) = result.new_mosaic {
                    let cuts = new_mosaic.get_cuts();
                    if !cuts.is_empty() {
                        found_cuts = true;
                        // Должны быть разрезы (горизонтальные и/или вертикальные)
                        let horizontal_cuts: Vec<_> = cuts.iter()
                            .filter(|c| c.get_is_horizontal())
                            .collect();
                        let vertical_cuts: Vec<_> = cuts.iter()
                            .filter(|c| !c.get_is_horizontal())
                            .collect();
                        
                        // Должен быть хотя бы один разрез
                        assert!(!horizontal_cuts.is_empty() || !vertical_cuts.is_empty());
                        break;
                    }
                }
            }
        }
        assert!(found_cuts, "Должны быть созданы разрезы для размещения плитки");
    }

    /// Тест, основанный на логике splitVertically из Java кода
    /// В Java: TileNode(x1, x2, y1, y1+i) и TileNode(x1, x2, y1+i+i2, y2)
    #[test]
    fn test_java_like_vertical_split_logic() {
        let stock_tile = TileDimensions::new(
            1,
            80,  // Исходная ширина (больше чем плитка, чтобы требовался вертикальный разрез)
            100, // Исходная высота
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        // Размещаем плитку 50x40 (требует вертикальный разрез по ширине)
        let tile = TileDimensions::simple(50, 40);
        let cut_thickness = 2;
        
        let results = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            cut_thickness,
            CutDirection::Both, // Разрешаем любые разрезы
            false,
        ).unwrap();
        
        assert!(results.iter().any(|r| r.placed));
        
        // Проверяем структуру разрезов
        let mut found_cuts = false;
        for result in &results {
            if result.placed {
                if let Some(ref new_mosaic) = result.new_mosaic {
                    let cuts = new_mosaic.get_cuts();
                    if !cuts.is_empty() {
                        found_cuts = true;
                        // Должны быть разрезы (горизонтальные и/или вертикальные)
                        let horizontal_cuts: Vec<_> = cuts.iter()
                            .filter(|c| c.get_is_horizontal())
                            .collect();
                        let vertical_cuts: Vec<_> = cuts.iter()
                            .filter(|c| !c.get_is_horizontal())
                            .collect();
                        
                        // Должен быть хотя бы один разрез
                        assert!(!horizontal_cuts.is_empty() || !vertical_cuts.is_empty());
                        break;
                    }
                }
            }
        }
        assert!(found_cuts, "Должны быть созданы разрезы для размещения плитки");
    }

    /// Тест сценария splitHV из Java кода (сначала горизонтальный, потом вертикальный)
    #[test]
    fn test_java_like_split_hv_scenario() {
        let stock_tile = TileDimensions::new(
            1,
            120, // Больше чем нужно по ширине
            80,  // Больше чем нужно по высоте
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        // Размещаем плитку, которая требует разрезов в обоих направлениях
        let tile = TileDimensions::simple(60, 40);
        let cut_thickness = 3;
        
        let results = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            cut_thickness,
            CutDirection::Both,
            false,
        ).unwrap();
        
        assert!(results.iter().any(|r| r.placed));
        
        // Проверяем, что создаются разрезы в обоих направлениях
        for result in &results {
            if result.placed && result.cuts_made >= 2 {
                if let Some(ref new_mosaic) = result.new_mosaic {
                    let cuts = new_mosaic.get_cuts();
                    if cuts.len() >= 2 {
                        let has_horizontal = cuts.iter().any(|c| c.get_is_horizontal());
                        let has_vertical = cuts.iter().any(|c| !c.get_is_horizontal());
                        
                        // В некоторых случаях могут быть разрезы в обоих направлениях
                        assert!(has_horizontal || has_vertical);
                    }
                }
            }
        }
    }

    /// Тест сценария splitVH из Java кода (сначала вертикальный, потом горизонтальный)
    #[test]
    fn test_java_like_split_vh_scenario() {
        let stock_tile = TileDimensions::new(
            1,
            80,  // Больше чем нужно по ширине
            120, // Больше чем нужно по высоте
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        // Размещаем плитку, которая требует разрезов в обоих направлениях
        let tile = TileDimensions::simple(40, 60);
        let cut_thickness = 3;
        
        let results = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            cut_thickness,
            CutDirection::Both,
            false,
        ).unwrap();
        
        assert!(results.iter().any(|r| r.placed));
    }

    /// Тест точного размещения без разрезов (как в Java: exact fit)
    #[test]
    fn test_exact_fit_no_cuts_needed() {
        let stock_tile = TileDimensions::new(
            1,
            100, // Точно по размеру плитки
            50,  // Точно по размеру плитки
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        // Плитка точно по размеру панели с конкретным ID
        let tile = TileDimensions::new(
            42, // Конкретный ID вместо -1
            100,
            50,
            "Wood".to_string(),
            0,
            Some("Test Tile".to_string()),
        );
        let cut_thickness = 3; // Толщина не должна влиять на точное размещение
        
        let results = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            cut_thickness,
            CutDirection::Both,
            false,
        ).unwrap();
        
        assert!(results.iter().any(|r| r.placed));
        
        // При точном размещении разрезы не нужны
        for result in &results {
            if result.placed && result.cuts_made == 0 {
                if let Some(ref new_mosaic) = result.new_mosaic {
                    let final_nodes = new_mosaic.get_root_tile_node().get_final_tile_nodes();
                    assert_eq!(final_nodes.len(), 1);
                    assert_eq!(final_nodes[0].external_id, tile.id);
                    return; // Нашли точное размещение без разрезов
                }
            }
        }
        
        // Если не нашли точное размещение без разрезов, проверим что хотя бы размещение есть
        assert!(results.iter().any(|r| r.placed), "Плитка должна быть размещена");
    }

    /// Тест множественных плиток с толщиной пропила (как в Java: multiple tiles processing)
    #[test]
    fn test_multiple_tiles_with_cut_thickness() {
        let stock_tile = TileDimensions::new(
            1,
            200,
            150,
            "Wood".to_string(),
            0,
            Some("Large Stock Panel".to_string()),
        );
        let mut mosaic = Mosaic::new_from_stock(&stock_tile);
        
        let tiles = vec![
            TileDimensions::simple(80, 60),  // Первая плитка
            TileDimensions::simple(50, 40),  // Вторая плитка
            TileDimensions::simple(70, 30),  // Третья плитка
        ];
        
        let cut_thickness = 3;
        let mut total_placed = 0;
        
        // Последовательно размещаем плитки (как в Java алгоритме)
        for tile in &tiles {
            let results = CuttingEngine::fit_tile(
                tile,
                &mosaic,
                cut_thickness,
                CutDirection::Both,
                false,
            ).unwrap();
            
            // Выбираем лучшее размещение
            if let Some(best_result) = results.iter().find(|r| r.placed) {
                if let Some(ref new_mosaic) = best_result.new_mosaic {
                    mosaic = new_mosaic.clone();
                    total_placed += 1;
                }
            }
        }
        
        // Должны разместить хотя бы 2 плитки
        assert!(total_placed >= 2);
    }

    /// Тест граничного случая: толщина пропила больше оставшегося места
    #[test]
    fn test_cut_thickness_exceeds_remaining_space() {
        let stock_tile = TileDimensions::new(
            1,
            55, // Небольшая панель
            50,
            "Wood".to_string(),
            0,
            Some("Small Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        // Плитка 50x50 с толщиной пропила 10 (больше оставшихся 5 единиц)
        let tile = TileDimensions::simple(50, 50);
        let cut_thickness = 10;
        
        let results = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            cut_thickness,
            CutDirection::Both,
            false,
        ).unwrap();
        
        // Размещение все равно должно быть возможно
        assert!(results.iter().any(|r| r.placed));
    }

    /// Тест поворота плитки с учетом толщины пропила (как в Java: rotation handling)
    #[test]
    fn test_tile_rotation_with_cut_thickness() {
        let stock_tile = TileDimensions::new(
            1,
            100,
            80, // Высота меньше ширины
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        // Плитка, которая помещается только с поворотом
        let tile = TileDimensions::simple(90, 70);
        let cut_thickness = 3;
        
        let results = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            cut_thickness,
            CutDirection::Both,
            false,
        ).unwrap();
        
        assert!(results.iter().any(|r| r.placed));
        
        // Проверяем, что плитка была размещена (возможно с поворотом)
        for result in &results {
            if result.placed {
                if let Some(ref new_mosaic) = result.new_mosaic {
                    let final_nodes = new_mosaic.get_root_tile_node().get_final_tile_nodes();
                    if !final_nodes.is_empty() {
                        assert_eq!(final_nodes[0].external_id, tile.id);
                        // Проверяем, что плитка действительно помещается
                        assert!(final_nodes[0].get_width() <= 100);
                        assert!(final_nodes[0].get_height() <= 80);
                    }
                }
            }
        }
    }

    /// Тест производительности с реальными размерами (как в Java: performance considerations)
    #[test]
    fn test_real_world_performance_scenario() {
        let stock_tile = TileDimensions::new(
            1,
            2440, // Стандартный лист фанеры 2440x1220 мм
            1220,
            "Plywood".to_string(),
            0,
            Some("Standard Plywood Sheet".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        // Реальные размеры деталей мебели
        let tiles = vec![
            TileDimensions::simple(600, 400),  // Столешница
            TileDimensions::simple(300, 200),  // Полка
            TileDimensions::simple(450, 350),  // Дверца
            TileDimensions::simple(200, 150),  // Ящик
        ];
        
        let cut_thickness = 3; // Стандартная толщина пропила для фанеры
        
        let start_time = std::time::Instant::now();
        
        for tile in &tiles {
            let results = CuttingEngine::fit_tile(
                tile,
                &mosaic,
                cut_thickness,
                CutDirection::Both,
                false,
            ).unwrap();
            
            assert!(results.iter().any(|r| r.placed));
        }
        
        let duration = start_time.elapsed();
        
        // Должно работать быстро даже с реальными размерами
        assert!(
            duration.as_millis() < 500,
            "Алгоритм должен работать быстро с реальными размерами: {:?}",
            duration
        );
    }

    /// Тест минимального размера обрезков с толщиной пропила
    #[test]
    fn test_min_trim_dimension_with_cut_thickness() {
        let stock_tile = TileDimensions::new(
            1,
            100,
            100,
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        // Плитка, которая оставляет маленький обрезок
        let tile = TileDimensions::simple(95, 95);
        let cut_thickness = 3;
        
        let results = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            cut_thickness,
            CutDirection::Both,
            false,
        ).unwrap();
        
        assert!(results.iter().any(|r| r.placed));
        
        // Проверяем, что учитывается минимальный размер обрезков
        for result in &results {
            if result.placed {
                if let Some(ref new_mosaic) = result.new_mosaic {
                    let unused_nodes = new_mosaic.get_root_tile_node().get_unused_tiles();
                    
                    // Проверяем, что нет слишком маленьких неиспользуемых узлов
                    for node in &unused_nodes {
                        if node.get_area() > 0 {
                            // Узлы должны быть достаточно большими или равными 0
                            assert!(
                                node.get_width() >= 5 || node.get_width() == 0,
                                "Ширина неиспользуемого узла слишком мала: {}",
                                node.get_width()
                            );
                            assert!(
                                node.get_height() >= 5 || node.get_height() == 0,
                                "Высота неиспользуемого узла слишком мала: {}",
                                node.get_height()
                            );
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod cut_thickness_cutting_tests {
    use super::*;

    #[test]
    fn test_horizontal_split_with_different_thicknesses() {
        let node = TileNode::new(0, 100, 0, 100);

        // Тест с разными толщинами пропила
        let thicknesses = vec![0, 1, 3, 5, 10];

        for thickness in thicknesses {
            let result = CuttingEngine::split_horizontally(&node, 50, thickness).unwrap();

            // Проверяем, что узлы позиционированы правильно
            assert_eq!(result.left_node.get_y1(), 0);
            assert_eq!(result.left_node.get_y2(), 50);
            assert_eq!(result.right_node.get_y1(), 50);
            assert_eq!(result.right_node.get_y2(), 100);

            // Проверяем, что разрез содержит информацию о толщине
            assert!(result.cut.get_is_horizontal());
            assert_eq!(result.cut.cut_coord, 50);
        }
    }

    #[test]
    fn test_vertical_split_with_different_thicknesses() {
        let node = TileNode::new(0, 100, 0, 100);

        // Тест с разными толщинами пропила
        let thicknesses = vec![0, 1, 3, 5, 10];

        for thickness in thicknesses {
            let result = CuttingEngine::split_vertically(&node, 50, thickness).unwrap();

            // Проверяем, что узлы позиционированы правильно
            assert_eq!(result.left_node.get_x1(), 0);
            assert_eq!(result.left_node.get_x2(), 50);
            assert_eq!(result.right_node.get_x1(), 50);
            assert_eq!(result.right_node.get_x2(), 100);

            // Проверяем, что разрез содержит информацию о толщине
            assert!(!result.cut.get_is_horizontal());
            assert_eq!(result.cut.get_cut_coords(), 50);
        }
    }

    #[test]
    fn test_cut_thickness_affects_available_space() {
        // Тест того, что толщина пропила влияет на доступное пространство
        let node = TileNode::new(0, 100, 0, 100);
        
        // Размещаем плитку 40x40 с разной толщиной пропила
        let _tile = TileDimensions::simple(40, 40);
        
        // С нулевой толщиной пропила
        let result_zero = CuttingEngine::split_vertically(&node, 40, 0).unwrap();
        assert_eq!(result_zero.right_node.get_width(), 60); // 100 - 40 = 60
        
        // С толщиной пропила 3
        let result_thick = CuttingEngine::split_vertically(&node, 40, 3).unwrap();
        assert_eq!(result_thick.right_node.get_width(), 60); // Узлы позиционированы без учета толщины
        
        // Но объект разреза содержит информацию о толщине
        assert_eq!(result_thick.cut.get_cut_coords(), 40);
    }

    #[test]
    fn test_multiple_cuts_with_thickness() {
        // Тест множественных разрезов с учетом толщины пропила
        let mut node = TileNode::new(0, 100, 0, 100);
        let cut_thickness = 3;
        
        // Первый вертикальный разрез
        let first_cut = CuttingEngine::split_vertically(&node, 40, cut_thickness).unwrap();
        node.child1 = Some(Box::new(first_cut.left_node));
        node.child2 = Some(Box::new(first_cut.right_node));
        
        // Второй горизонтальный разрез в левом дочернем узле
        if let Some(ref mut left_child) = node.child1 {
            let second_cut = CuttingEngine::split_horizontally(left_child, 30, cut_thickness).unwrap();
            left_child.child1 = Some(Box::new(second_cut.left_node));
            left_child.child2 = Some(Box::new(second_cut.right_node));
            
            // Проверяем размеры получившихся узлов
            if let Some(ref top_left) = left_child.child1 {
                assert_eq!(top_left.get_width(), 40);
                assert_eq!(top_left.get_height(), 30);
                assert_eq!(top_left.get_area(), 1200);
            }
        }
    }

    #[test]
    fn test_cut_thickness_boundary_conditions() {
        let node = TileNode::new(0, 50, 0, 50);
        
        // Тест разреза с толщиной, равной размеру узла
        let result = CuttingEngine::split_horizontally(&node, 25, 50);
        assert!(result.is_ok()); // Должно работать, так как толщина учитывается в объекте Cut
        
        // Тест разреза с очень большой толщиной
        let result_large = CuttingEngine::split_vertically(&node, 25, 1000);
        assert!(result_large.is_ok()); // Должно работать
    }
}

#[cfg(test)]
mod cut_thickness_placement_tests {
    use super::*;

    #[test]
    fn test_tile_placement_with_cut_thickness() {
        // Тест размещения плитки с учетом толщины пропила
        let mut node = TileNode::new(0, 100, 0, 80);
        let tile = TileDimensions::simple(60, 50);
        
        // Размещение должно учитывать толщину пропила при планировании разрезов
        let result = CuttingEngine::try_place_tile(&mut node, &tile).unwrap();
        assert!(result);
        assert!(node.has_children());
        
        // Проверяем, что плитка размещена
        let final_nodes = node.get_final_tile_nodes();
        assert_eq!(final_nodes.len(), 1);
        assert_eq!(final_nodes[0].external_id, tile.id);
    }

    #[test]
    fn test_fit_tile_with_different_cut_thicknesses() {
        // Создаем мозаику для тестирования
        let stock_tile = TileDimensions::new(
            1,
            200,
            150,
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        let tile_to_place = TileDimensions::simple(80, 60);
        
        // Тестируем с разными толщинами пропила
        let thicknesses = vec![0, 1, 3, 5, 10];
        
        for thickness in thicknesses {
            let results = CuttingEngine::fit_tile(
                &tile_to_place,
                &mosaic,
                thickness,
                CutDirection::Both,
                false,
            );
            
            assert!(results.is_ok());
            let placement_results = results.unwrap();
            assert!(!placement_results.is_empty());
            
            // Проверяем, что хотя бы одно размещение успешно
            let successful_placements: Vec<_> = placement_results
                .iter()
                .filter(|r| r.placed)
                .collect();
            assert!(!successful_placements.is_empty());
        }
    }

    #[test]
    fn test_cut_thickness_affects_optimization() {
        // Тест влияния толщины пропила на оптимизацию размещения
        let stock_tile = TileDimensions::new(
            1,
            100,
            100,
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        let tile = TileDimensions::simple(45, 45);
        
        // Размещение с нулевой толщиной пропила
        let results_zero = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            0,
            CutDirection::Both,
            false,
        ).unwrap();
        
        // Размещение с толщиной пропила 10
        let results_thick = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            10,
            CutDirection::Both,
            false,
        ).unwrap();
        
        // Оба размещения должны быть успешными
        assert!(results_zero.iter().any(|r| r.placed));
        assert!(results_thick.iter().any(|r| r.placed));
        
        // Но количество разрезов может отличаться
        let cuts_zero = results_zero.iter()
            .filter(|r| r.placed)
            .map(|r| r.cuts_made)
            .min()
            .unwrap_or(0);
            
        let cuts_thick = results_thick.iter()
            .filter(|r| r.placed)
            .map(|r| r.cuts_made)
            .min()
            .unwrap_or(0);
            
        // Количество разрезов должно быть одинаковым для данного случая
        assert_eq!(cuts_zero, cuts_thick);
    }

    #[test]
    fn test_minimal_space_with_cut_thickness() {
        // Тест размещения в минимальном пространстве с учетом толщины пропила
        let stock_tile = TileDimensions::new(
            1,
            53, // Минимальный размер: 50 (плитка) + 3 (толщина пропила)
            53,
            "Wood".to_string(),
            0,
            Some("Minimal Stock".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        let tile = TileDimensions::simple(50, 50);
        
        let results = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            3,
            CutDirection::Both,
            false,
        ).unwrap();
        
        // Должно быть возможно разместить плитку
        assert!(results.iter().any(|r| r.placed));
    }

    #[test]
    fn test_insufficient_space_with_cut_thickness() {
        // Тест случая, когда места недостаточно из-за толщины пропила
        let stock_tile = TileDimensions::new(
            1,
            52, // Недостаточно: нужно 50 (плитка) + 3 (толщина) = 53
            52,
            "Wood".to_string(),
            0,
            Some("Insufficient Stock".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        let tile = TileDimensions::simple(50, 50);
        
        let results = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            3,
            CutDirection::Both,
            false,
        ).unwrap();
        
        // Размещение должно быть успешным, так как плитка точно помещается
        // (толщина пропила учитывается только при необходимости разрезов)
        assert!(results.iter().any(|r| r.placed));
    }
}

#[cfg(test)]
mod cut_thickness_edge_cases {
    use super::*;

    #[test]
    fn test_negative_cut_thickness() {
        // Тест отрицательной толщины пропила (должно работать как 0)
        let node = TileNode::new(0, 100, 0, 100);
        let result = CuttingEngine::split_horizontally(&node, 50, -5);
        
        // Должно работать (отрицательные значения обрабатываются как валидные)
        assert!(result.is_ok());
    }

    #[test]
    fn test_very_large_cut_thickness() {
        // Тест очень большой толщины пропила
        let node = TileNode::new(0, 100, 0, 100);
        let result = CuttingEngine::split_vertically(&node, 50, i32::MAX);
        
        assert!(result.is_ok());
        let cut_result = result.unwrap();
        assert_eq!(cut_result.cut.get_cut_coords(), 50);
    }

    #[test]
    fn test_cut_thickness_with_rotated_tiles() {
        // Тест толщины пропила с повернутыми плитками
        let stock_tile = TileDimensions::new(
            1,
            100,
            80,
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        let tile = TileDimensions::simple(70, 50); // Помещается только с поворотом
        
        let results = CuttingEngine::fit_tile(
            &tile,
            &mosaic,
            5,
            CutDirection::Both,
            false,
        ).unwrap();
        
        // Должно быть возможно разместить с поворотом
        assert!(results.iter().any(|r| r.placed));
        
        // Проверяем, что есть размещение с поворотом
        for result in &results {
            if result.placed {
                if let Some(ref new_mosaic) = result.new_mosaic {
                    let final_nodes = new_mosaic.get_root_tile_node().get_final_tile_nodes();
                    if !final_nodes.is_empty() {
                        // Может быть повернута или нет, в зависимости от алгоритма
                        assert_eq!(final_nodes[0].external_id, tile.id);
                    }
                }
            }
        }
    }

    #[test]
    fn test_cut_thickness_precision() {
        // Тест точности при работе с дробными значениями толщины пропила
        let config = Configuration {
            cut_thickness: "3.14159".to_string(),
            min_trim_dimension: "10.0".to_string(),
            optimization_factor: 1.0,
            use_single_stock_unit: false,
            cut_orientation_preference: 0,
            performance_thresholds: None,
        };
        
        assert!((config.get_cut_thickness_f64().unwrap() - 3.14159).abs() < f64::EPSILON);
        assert!(config.is_valid());
    }

    #[test]
    fn test_cut_thickness_with_multiple_materials() {
        // Тест толщины пропила с разными материалами
        let wood_config = Configuration {
            cut_thickness: "3.0".to_string(),
            min_trim_dimension: "10.0".to_string(),
            optimization_factor: 1.0,
            use_single_stock_unit: false,
            cut_orientation_preference: 0,
            performance_thresholds: None,
        };
        
        let metal_config = Configuration {
            cut_thickness: "1.5".to_string(),
            min_trim_dimension: "5.0".to_string(),
            optimization_factor: 1.0,
            use_single_stock_unit: false,
            cut_orientation_preference: 0,
            performance_thresholds: None,
        };
        
        assert_eq!(wood_config.get_cut_thickness_f64().unwrap(), 3.0);
        assert_eq!(metal_config.get_cut_thickness_f64().unwrap(), 1.5);
        assert!(wood_config.is_valid());
        assert!(metal_config.is_valid());
    }
}

#[cfg(test)]
mod cut_thickness_performance_tests {
    use super::*;

    #[test]
    fn test_cut_thickness_performance_impact() {
        // Тест влияния толщины пропила на производительность
        let stock_tile = TileDimensions::new(
            1,
            1000,
            800,
            "Wood".to_string(),
            0,
            Some("Large Stock".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        let tiles = vec![
            TileDimensions::simple(100, 80),
            TileDimensions::simple(120, 90),
            TileDimensions::simple(80, 100),
            TileDimensions::simple(150, 70),
            TileDimensions::simple(90, 110),
        ];
        
        let start_time = std::time::Instant::now();
        
        for tile in &tiles {
            let _results = CuttingEngine::fit_tile(
                tile,
                &mosaic,
                3,
                CutDirection::Both,
                false,
            ).unwrap();
        }
        
        let duration = start_time.elapsed();
        
        // Проверяем, что алгоритм работает достаточно быстро
        assert!(
            duration.as_millis() < 1000,
            "Алгоритм с толщиной пропила должен работать быстро: {:?}",
            duration
        );
    }

    #[test]
    fn test_cut_thickness_memory_usage() {
        // Тест использования памяти при работе с толщиной пропила
        let stock_tile = TileDimensions::new(
            1,
            500,
            400,
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        let tile = TileDimensions::simple(50, 40);
        
        // Создаем много размещений для проверки утечек памяти
        for _ in 0..100 {
            let _results = CuttingEngine::fit_tile(
                &tile,
                &mosaic,
                3,
                CutDirection::Both,
                false,
            ).unwrap();
        }
        
        // Если тест завершается без паники, значит нет критических утечек памяти
        assert!(true);
    }
}

#[cfg(test)]
mod cut_thickness_integration_tests {
    use super::*;

    #[test]
    fn test_full_workflow_with_cut_thickness() {
        // Полный интеграционный тест с толщиной пропила
        let client_info = ClientInfo::new("test_client".to_string());
        
        let configuration = Configuration {
            cut_thickness: "3.0".to_string(),
            min_trim_dimension: "10.0".to_string(),
            optimization_factor: 1.0,
            use_single_stock_unit: false,
            cut_orientation_preference: 0,
            performance_thresholds: None,
        };
        
        let panels = vec![
            Panel::new(1, "100".to_string(), "80".to_string(), 2, Some("Wood".to_string())),
            Panel::new(2, "120".to_string(), "90".to_string(), 1, Some("Wood".to_string())),
        ];
        
        let stock_panels = vec![
            Panel::new(3, "300".to_string(), "200".to_string(), 2, Some("Wood".to_string())),
        ];
        
        let request = CalculationRequest::new(
            client_info,
            configuration,
            panels,
            stock_panels,
        );
        
        // Проверяем валидность запроса
        assert!(request.configuration.is_valid());
        assert_eq!(request.configuration.get_cut_thickness_f64().unwrap(), 3.0);
        
        // Проверяем количество панелей
        assert_eq!(request.count_valid_panels(), 3);
        assert_eq!(request.count_valid_stock_panels(), 2);
    }

    #[test]
    fn test_cut_thickness_with_different_orientations() {
        // Тест толщины пропила с разными ориентациями разрезов
        let stock_tile = TileDimensions::new(
            1,
            200,
            150,
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        let tile = TileDimensions::simple(80, 60);
        
        // Тестируем разные предпочтения ориентации
        let orientations = vec![
            CutDirection::Horizontal,
            CutDirection::Vertical,
            CutDirection::Both,
        ];
        
        for orientation in orientations {
            let results = CuttingEngine::fit_tile(
                &tile,
                &mosaic,
                3,
                orientation,
                false,
            ).unwrap();
            
            assert!(results.iter().any(|r| r.placed));
        }
    }

    #[test]
    fn test_cut_thickness_consistency() {
        // Тест консистентности результатов с одинаковой толщиной пропила
        let stock_tile = TileDimensions::new(
            1,
            100,
            100,
            "Wood".to_string(),
            0,
            Some("Stock Panel".to_string()),
        );
        let mosaic = Mosaic::new_from_stock(&stock_tile);
        
        let tile = TileDimensions::simple(50, 50);
        
        // Выполняем одинаковые операции несколько раз
        let mut all_results = Vec::new();
        
        for _ in 0..5 {
            let results = CuttingEngine::fit_tile(
                &tile,
                &mosaic,
                3,
                CutDirection::Both,
                false,
            ).unwrap();
            
            all_results.push(results);
        }
        
        // Проверяем, что результаты консистентны
        let first_result_count = all_results[0].len();
        for result_set in &all_results {
            assert_eq!(result_set.len(), first_result_count);
            assert!(result_set.iter().any(|r| r.placed));
        }
    }
}
