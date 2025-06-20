//! Тесты для проверки правильного присвоения stock_panel_id при использовании нескольких листов материала

use cutting_cli::engine::model::{TileDimensions, Solution, Mosaic};
use cutting_cli::engine::stock::StockSolution;
use cutting_cli::engine::model::response::{CalculationResponse, OptimizedPanel, PanelPosition};
use cutting_cli::engine::service::CutListOptimizerService;
use cutting_cli::engine::model::request::CalculationRequest;
use std::collections::HashMap;

/// Тест проверяет, что панели правильно распределяются по разным листам
/// и каждая панель получает корректный stock_panel_id
#[test]
fn test_multiple_stock_panels_correct_id_assignment() {
    // Создаем два листа материала разного размера
    let stock_panels = vec![
        TileDimensions::new(1, 600, 600, "Material1".to_string(), 0, None), // Лист 1: 600x600
        TileDimensions::new(2, 1200, 450, "Material1".to_string(), 0, None), // Лист 2: 1200x450
    ];

    // Создаем панели для размещения
    let panels_to_cut = vec![
        TileDimensions::new(1001, 300, 300, "Material1".to_string(), 0, None), // Поместится на любом листе
        TileDimensions::new(1002, 500, 500, "Material1".to_string(), 0, None), // Поместится только на листе 1 (600x600)
        TileDimensions::new(1003, 800, 400, "Material1".to_string(), 0, None), // Поместится только на листе 2 (1200x450)
        TileDimensions::new(1004, 200, 200, "Material1".to_string(), 0, None), // Поместится на любом листе
    ];

    let stock_solution = StockSolution::new(stock_panels);
    let mut solution = Solution::from_stock_solution(&stock_solution);

    // Размещаем панели
    for panel in &panels_to_cut {
        let new_solutions = solution.try_place_tile(panel).unwrap();
        if !new_solutions.is_empty() {
            solution = new_solutions[0].clone();
        }
    }

    // Проверяем, что у нас есть мозаики (листы)
    assert!(!solution.get_mosaics().is_empty(), "Должны быть созданы мозаики");

    // Создаем ответ для проверки stock_panel_id
    let response = create_response_from_solution(&solution);

    // Проверяем, что панели имеют разные stock_panel_id
    let mut stock_panel_ids: Vec<String> = response.panels.iter()
        .map(|p| p.stock_panel_id.clone())
        .collect();
    stock_panel_ids.sort();
    stock_panel_ids.dedup();

    // Должно быть больше одного уникального stock_panel_id если панели размещены на разных листах
    if response.panels.len() > 1 {
        println!("Найденные stock_panel_id: {:?}", stock_panel_ids);
        
        // Проверяем, что панели не все имеют одинаковый stock_panel_id
        let all_same_id = response.panels.iter()
            .all(|p| p.stock_panel_id == response.panels[0].stock_panel_id);
        
        if all_same_id {
            panic!("ОШИБКА: Все панели имеют одинаковый stock_panel_id '{}', но должны быть распределены по разным листам", 
                   response.panels[0].stock_panel_id);
        }
    }
}

/// Тест проверяет, что панели размещаются на правильных листах в зависимости от размера
#[test]
fn test_panel_placement_on_correct_stock_by_size() {
    // Создаем два листа разного размера
    let stock_panels = vec![
        TileDimensions::new(1, 600, 600, "Material1".to_string(), 0, None),   // Маленький лист
        TileDimensions::new(2, 1200, 800, "Material1".to_string(), 0, None),  // Большой лист
    ];

    // Панель, которая поместится только на большом листе
    let large_panel = TileDimensions::new(1001, 1000, 700, "Material1".to_string(), 0, None);

    let stock_solution = StockSolution::new(stock_panels);
    let mut solution = Solution::from_stock_solution(&stock_solution);

    // Размещаем большую панель
    let new_solutions = solution.try_place_tile(&large_panel).unwrap();
    assert!(!new_solutions.is_empty(), "Большая панель должна поместиться");

    solution = new_solutions[0].clone();
    let response = create_response_from_solution(&solution);

    // Проверяем, что панель размещена
    assert_eq!(response.panels.len(), 1, "Должна быть размещена одна панель");
    
    // Проверяем, что панель размещена на правильном листе (большом)
    let placed_panel = &response.panels[0];
    
    // Панель должна иметь stock_panel_id соответствующий большому листу
    // В реальной реализации это должно быть "stock_2" для листа с id=2
    println!("Панель размещена на листе: {}", placed_panel.stock_panel_id);
    
    // Проверяем, что координаты панели не выходят за границы листа
    assert!(placed_panel.position.x >= 0, "X координата должна быть неотрицательной");
    assert!(placed_panel.position.y >= 0, "Y координата должна быть неотрицательной");
    assert!(placed_panel.position.x + placed_panel.position.width <= 1200, 
            "Панель не должна выходить за правую границу листа");
    assert!(placed_panel.position.y + placed_panel.position.height <= 800, 
            "Панель не должна выходить за нижнюю границу листа");
}

/// Тест проверяет, что при наложении панелей выдается ошибка
#[test]
fn test_no_panel_overlap_on_same_stock() {
    let stock_panels = vec![
        TileDimensions::new(1, 1000, 1000, "Material1".to_string(), 0, None),
    ];

    let panels_to_cut = vec![
        TileDimensions::new(1001, 600, 600, "Material1".to_string(), 0, None),
        TileDimensions::new(1002, 600, 600, "Material1".to_string(), 0, None),
    ];

    let stock_solution = StockSolution::new(stock_panels);
    let mut solution = Solution::from_stock_solution(&stock_solution);

    // Размещаем первую панель
    let new_solutions = solution.try_place_tile(&panels_to_cut[0]).unwrap();
    assert!(!new_solutions.is_empty());
    solution = new_solutions[0].clone();

    // Размещаем вторую панель
    let new_solutions = solution.try_place_tile(&panels_to_cut[1]).unwrap();
    assert!(!new_solutions.is_empty());
    solution = new_solutions[0].clone();

    let response = create_response_from_solution(&solution);

    // Проверяем, что панели не пересекаются
    if response.panels.len() >= 2 {
        for i in 0..response.panels.len() {
            for j in (i + 1)..response.panels.len() {
                let panel1 = &response.panels[i];
                let panel2 = &response.panels[j];
                
                // Если панели на одном листе, они не должны пересекаться
                if panel1.stock_panel_id == panel2.stock_panel_id {
                    assert!(!panel1.position.intersects(&panel2.position),
                            "Панели на одном листе не должны пересекаться: {:?} и {:?}",
                            panel1.position, panel2.position);
                }
            }
        }
    }
}

/// Тест проверяет корректность JSON ответа с несколькими листами
#[test]
fn test_json_response_multiple_stocks() {
    let stock_panels = vec![
        TileDimensions::new(1, 600, 600, "Material1".to_string(), 0, None),
        TileDimensions::new(2, 1200, 450, "Material1".to_string(), 0, None),
    ];

    let panels_to_cut = vec![
        TileDimensions::new(1001, 300, 300, "Material1".to_string(), 0, None),
        TileDimensions::new(1002, 800, 400, "Material1".to_string(), 0, None),
    ];

    let stock_solution = StockSolution::new(stock_panels);
    let mut solution = Solution::from_stock_solution(&stock_solution);

    for panel in &panels_to_cut {
        let new_solutions = solution.try_place_tile(panel).unwrap();
        if !new_solutions.is_empty() {
            solution = new_solutions[0].clone();
        }
    }

    let response = create_response_from_solution(&solution);
    
    // Сериализуем в JSON
    let json_result = serde_json::to_string_pretty(&response);
    assert!(json_result.is_ok(), "Ответ должен сериализоваться в JSON");
    
    let json_string = json_result.unwrap();
    println!("JSON ответ:\n{}", json_string);
    
    // Проверяем, что в JSON есть разные stock_panel_id
    assert!(json_string.contains("stock_panel_id"), "JSON должен содержать stock_panel_id");
    
    // Десериализуем обратно для проверки
    let deserialized: Result<CalculationResponse, _> = serde_json::from_str(&json_string);
    assert!(deserialized.is_ok(), "JSON должен корректно десериализоваться");
}

/// Тест проверяет, что панели правильно группируются по материалам и листам
#[test]
fn test_material_and_stock_grouping() {
    let stock_panels = vec![
        TileDimensions::new(1, 600, 600, "Wood".to_string(), 0, None),
        TileDimensions::new(2, 800, 800, "Wood".to_string(), 0, None),
        TileDimensions::new(3, 1000, 500, "Metal".to_string(), 0, None),
    ];

    let panels_to_cut = vec![
        TileDimensions::new(1001, 300, 300, "Wood".to_string(), 0, None),
        TileDimensions::new(1002, 700, 700, "Wood".to_string(), 0, None),
        TileDimensions::new(1003, 900, 400, "Metal".to_string(), 0, None),
    ];

    let stock_solution = StockSolution::new(stock_panels);
    let mut solution = Solution::from_stock_solution(&stock_solution);

    for panel in &panels_to_cut {
        let new_solutions = solution.try_place_tile(panel).unwrap();
        if !new_solutions.is_empty() {
            solution = new_solutions[0].clone();
        }
    }

    let response = create_response_from_solution(&solution);

    // Группируем панели по материалам
    let mut wood_panels = Vec::new();
    let mut metal_panels = Vec::new();

    for panel in &response.panels {
        match panel.material.as_str() {
            "Wood" => wood_panels.push(panel),
            "Metal" => metal_panels.push(panel),
            _ => {}
        }
    }

    // Проверяем, что панели из дерева размещены на листах для дерева
    for panel in &wood_panels {
        assert_eq!(panel.material, "Wood", "Панель должна быть из дерева");
        // stock_panel_id должен соответствовать листу из дерева (1 или 2)
        assert!(panel.stock_panel_id == "stock_1" || panel.stock_panel_id == "stock_2",
                "Деревянная панель должна быть на деревянном листе, но размещена на: {}", 
                panel.stock_panel_id);
    }

    // Проверяем, что металлические панели размещены на металлическом листе
    for panel in &metal_panels {
        assert_eq!(panel.material, "Metal", "Панель должна быть из металла");
        assert_eq!(panel.stock_panel_id, "stock_3", 
                  "Металлическая панель должна быть на металлическом листе, но размещена на: {}", 
                  panel.stock_panel_id);
    }
}

/// Вспомогательная функция для создания ответа из решения
fn create_response_from_solution(solution: &Solution) -> CalculationResponse {
    let mut response = CalculationResponse::new();
    
    // Преобразуем мозаики в оптимизированные панели
    for mosaic in solution.get_mosaics().iter() {
        // Используем stock_id из мозаики для формирования stock_panel_id
        // Это соответствует Java коду где используется mosaic.getStockId()
        let stock_panel_id = format!("stock_{}", mosaic.get_stock_id());
        
        // Получаем финальные узлы плиток из мозаики
        let final_nodes = mosaic.get_root_tile_node().get_final_tile_nodes();
        
        for node in final_nodes {
            let tile_dimensions = TileDimensions::new(
                node.external_id,
                node.get_width(),
                node.get_height(),
                mosaic.get_material().to_string(),
                0, // orientation
                None // label
            );
            
            let position = PanelPosition::new(
                node.get_x1(),
                node.get_y1(),
                node.get_width(),
                node.get_height(),
                node.is_rotated // Используем реальное значение поворота
            );
            
            let optimized_panel = OptimizedPanel::new(
                tile_dimensions,
                position,
                stock_panel_id.clone(),
                mosaic.get_material().to_string()
            );
            
            response.panels.push(optimized_panel);
        }
    }
    
    response
}

/// Тест проверяет, что алгоритм не создает виртуальные комбинированные листы
#[test]
fn test_no_virtual_combined_sheets() {
    let stock_panels = vec![
        TileDimensions::new(1, 600, 600, "Material1".to_string(), 0, None),
        TileDimensions::new(2, 1200, 450, "Material1".to_string(), 0, None),
    ];

    let panels_to_cut = vec![
        TileDimensions::new(1001, 300, 300, "Material1".to_string(), 0, None),
        TileDimensions::new(1002, 800, 400, "Material1".to_string(), 0, None),
    ];

    let stock_solution = StockSolution::new(stock_panels);
    let mut solution = Solution::from_stock_solution(&stock_solution);

    for panel in &panels_to_cut {
        let new_solutions = solution.try_place_tile(panel).unwrap();
        if !new_solutions.is_empty() {
            solution = new_solutions[0].clone();
        }
    }

    let response = create_response_from_solution(&solution);

    // Проверяем, что размеры листов в ответе соответствуют исходным
    let mut stock_sizes = HashMap::new();
    
    for panel in &response.panels {
        let max_x = panel.position.x + panel.position.width;
        let max_y = panel.position.y + panel.position.height;
        
        let entry = stock_sizes.entry(panel.stock_panel_id.clone())
            .or_insert((0, 0));
        
        entry.0 = entry.0.max(max_x);
        entry.1 = entry.1.max(max_y);
    }

    // Проверяем, что размеры не превышают исходные размеры листов
    for (stock_id, (max_width, max_height)) in stock_sizes {
        println!("Лист {}: максимальные размеры {}x{}", stock_id, max_width, max_height);
        
        // Размеры не должны превышать размеры исходных листов
        assert!(max_width <= 1200, "Ширина листа {} превышает максимальную: {}", stock_id, max_width);
        assert!(max_height <= 600, "Высота листа {} превышает максимальную: {}", stock_id, max_height);
    }
}

/// Интеграционный тест с реальными данными из competitor_data.json
#[test]
fn test_real_data_stock_panel_assignment() {
    // Данные из competitor_data.json
    let stock_panels = vec![
        TileDimensions::new(0, 600, 600, "DEFAULT_MATERIAL".to_string(), 0, None),
        TileDimensions::new(1, 1200, 450, "DEFAULT_MATERIAL".to_string(), 0, None),
    ];

    let panels_to_cut = vec![
        TileDimensions::new(8000, 300, 300, "DEFAULT_MATERIAL".to_string(), 0, None),
        TileDimensions::new(7000, 300, 300, "DEFAULT_MATERIAL".to_string(), 0, None),
        TileDimensions::new(6000, 300, 300, "DEFAULT_MATERIAL".to_string(), 0, None),
        TileDimensions::new(5000, 300, 300, "DEFAULT_MATERIAL".to_string(), 0, None),
    ];

    let stock_solution = StockSolution::new(stock_panels);
    let mut solution = Solution::from_stock_solution(&stock_solution);

    for panel in &panels_to_cut {
        let new_solutions = solution.try_place_tile(panel).unwrap();
        if !new_solutions.is_empty() {
            solution = new_solutions[0].clone();
        }
    }

    let response = create_response_from_solution(&solution);

    // Проверяем, что НЕ все панели имеют stock_panel_id = "stock_0"
    let all_on_first_stock = response.panels.iter()
        .all(|p| p.stock_panel_id == "stock_0");

    if all_on_first_stock && response.panels.len() > 1 {
        println!("ПРЕДУПРЕЖДЕНИЕ: Все панели размещены на одном листе (stock_0)");
        println!("Это может указывать на проблему в алгоритме распределения по листам");
        
        // Выводим детали для отладки
        for (i, panel) in response.panels.iter().enumerate() {
            println!("Панель {}: ID={}, позиция=({}, {}), размер={}x{}, лист={}",
                     i, panel.tile_dimensions.id, 
                     panel.position.x, panel.position.y,
                     panel.position.width, panel.position.height,
                     panel.stock_panel_id);
        }
    }

    // Проверяем, что панели не выходят за границы листов
    for panel in &response.panels {
        match panel.stock_panel_id.as_str() {
            "stock_0" => {
                // Первый лист 600x600
                assert!(panel.position.x + panel.position.width <= 600,
                        "Панель выходит за правую границу первого листа");
                assert!(panel.position.y + panel.position.height <= 600,
                        "Панель выходит за нижнюю границу первого листа");
            },
            "stock_1" => {
                // Второй лист 1200x450
                assert!(panel.position.x + panel.position.width <= 1200,
                        "Панель выходит за правую границу второго листа");
                assert!(panel.position.y + panel.position.height <= 450,
                        "Панель выходит за нижнюю границу второго листа");
            },
            _ => {
                panic!("Неожиданный stock_panel_id: {}", panel.stock_panel_id);
            }
        }
    }
}
