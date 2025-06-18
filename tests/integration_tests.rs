//! Интеграционные тесты для Этапа 1 и Этапа 2

use cutting_cli::engine::model::tile::{TileNode, TileDimensions};
use cutting_cli::engine::cutting::CuttingEngine;
use cutting_cli::error::CuttingError;

#[cfg(test)]
mod stage1_tests {
    use super::*;

    #[test]
    fn test_tile_dimensions_creation() {
        let tile = TileDimensions::simple(100, 50);
        assert_eq!(tile.width, 100);
        assert_eq!(tile.height, 50);
        assert!(!tile.is_square());
        assert!(tile.is_horizontal());
    }

    #[test]
    fn test_tile_dimensions_with_full_params() {
        let tile = TileDimensions::new(
            1,
            100,
            50,
            "Wood".to_string(),
            1,
            Some("Test Tile".to_string()),
        );
        assert_eq!(tile.id, 1);
        assert_eq!(tile.width, 100);
        assert_eq!(tile.height, 50);
        assert_eq!(tile.material, "Wood");
        assert_eq!(tile.label, Some("Test Tile".to_string()));
    }

    #[test]
    fn test_tile_dimensions_rotation() {
        let tile = TileDimensions::simple(100, 50);
        let rotated = tile.rotate90();
        
        assert_eq!(rotated.width, 50);
        assert_eq!(rotated.height, 100);
        assert!(rotated.is_rotated);
        assert_eq!(rotated.orientation, 1);
    }

    #[test]
    fn test_tile_dimensions_square() {
        let square_tile = TileDimensions::simple(50, 50);
        assert!(square_tile.is_square());
        assert!(!square_tile.is_horizontal());
    }

    #[test]
    fn test_tile_dimensions_fits() {
        let container = TileDimensions::simple(100, 80);
        let small_tile = TileDimensions::simple(50, 30);
        let large_tile = TileDimensions::simple(120, 90);
        let rotatable_tile = TileDimensions::simple(90, 70);

        assert!(container.fits(&small_tile));
        assert!(!container.fits(&large_tile));
        assert!(container.fits(&rotatable_tile)); // Помещается с поворотом
    }

    #[test]
    fn test_tile_node_creation() {
        let node = TileNode::new(0, 100, 0, 50);
        assert_eq!(node.get_width(), 100);
        assert_eq!(node.get_height(), 50);
        assert_eq!(node.get_area(), 5000);
        assert!(!node.is_final);
        assert!(!node.has_children());
    }

    #[test]
    fn test_tile_node_from_dimensions() {
        let dimensions = TileDimensions::simple(100, 50);
        let node = TileNode::from_dimensions(&dimensions);
        assert_eq!(node.get_width(), 100);
        assert_eq!(node.get_height(), 50);
    }

    #[test]
    fn test_tile_node_area_calculations() {
        let mut node = TileNode::new(0, 100, 0, 100);
        
        // Изначально неиспользуемая площадь равна общей площади
        assert_eq!(node.get_unused_area(), 10000);
        assert_eq!(node.get_used_area_ratio(), 0.0);
        
        // Помечаем как финальный
        node.is_final = true;
        assert_eq!(node.get_used_area(), 10000);
        assert_eq!(node.get_used_area_ratio(), 1.0);
    }

    #[test]
    fn test_tile_node_children_management() {
        let mut parent = TileNode::new(0, 100, 0, 100);
        let child1 = TileNode::new(0, 50, 0, 100);
        let child2 = TileNode::new(50, 100, 0, 100);
        
        parent.child1 = Some(Box::new(child1));
        parent.child2 = Some(Box::new(child2));
        
        assert!(parent.has_children());
        assert_eq!(parent.get_nbr_unused_tiles(), 2); // Два неиспользуемых дочерних узла
    }

    #[test]
    fn test_tile_node_final_nodes() {
        let mut parent = TileNode::new(0, 100, 0, 100);
        let mut child1 = TileNode::new(0, 50, 0, 100);
        let mut child2 = TileNode::new(50, 100, 0, 100);
        
        child1.is_final = true;
        child2.is_final = true;
        
        parent.child1 = Some(Box::new(child1));
        parent.child2 = Some(Box::new(child2));
        
        assert!(parent.has_final());
        assert_eq!(parent.get_nbr_final_tiles(), 2);
        
        let final_nodes = parent.get_final_tile_nodes();
        assert_eq!(final_nodes.len(), 2);
    }

    #[test]
    fn test_tile_node_orientation_stats() {
        let mut parent = TileNode::new(0, 100, 0, 100);
        let mut horizontal_child = TileNode::new(0, 80, 0, 40); // 80x40 - горизонтальная
        let mut vertical_child = TileNode::new(0, 30, 0, 60);   // 30x60 - вертикальная
        
        horizontal_child.is_final = true;
        vertical_child.is_final = true;
        
        parent.child1 = Some(Box::new(horizontal_child));
        parent.child2 = Some(Box::new(vertical_child));
        
        assert_eq!(parent.get_nbr_final_horizontal(), 1);
        assert_eq!(parent.get_nbr_final_vertical(), 1);
    }

    #[test]
    fn test_tile_node_biggest_area() {
        let mut parent = TileNode::new(0, 100, 0, 100);
        let child1 = TileNode::new(0, 30, 0, 40);  // Площадь 1200
        let child2 = TileNode::new(0, 50, 0, 60);  // Площадь 3000
        
        parent.child1 = Some(Box::new(child1));
        parent.child2 = Some(Box::new(child2));
        
        assert_eq!(parent.get_biggest_area(), 3000);
    }

    #[test]
    fn test_tile_node_distinct_tiles() {
        let mut parent = TileNode::new(0, 100, 0, 100);
        let mut child1 = TileNode::new(0, 30, 0, 40);
        let mut child2 = TileNode::new(0, 30, 0, 40); // Такие же размеры
        let mut child3 = TileNode::new(0, 50, 0, 60); // Другие размеры
        
        child1.is_final = true;
        child2.is_final = true;
        child3.is_final = true;
        
        parent.child1 = Some(Box::new(child1));
        parent.child2 = Some(Box::new(child2));
        
        let mut grandparent = TileNode::new(0, 200, 0, 200);
        grandparent.child1 = Some(Box::new(parent));
        grandparent.child2 = Some(Box::new(child3));
        
        let distinct_set = grandparent.get_distinct_tile_set();
        assert!(distinct_set.len() >= 1); // Должно быть как минимум 1 уникальный размер
    }
}

#[cfg(test)]
mod stage2_tests {
    use super::*;

    #[test]
    fn test_horizontal_split_basic() {
        let node = TileNode::new(0, 100, 0, 100);
        let result = CuttingEngine::split_horizontally(&node, 60).unwrap();
        
        // Проверяем верхний узел
        assert_eq!(result.left_node.get_x1(), 0);
        assert_eq!(result.left_node.get_x2(), 100);
        assert_eq!(result.left_node.get_y1(), 0);
        assert_eq!(result.left_node.get_y2(), 60);
        
        // Проверяем нижний узел
        assert_eq!(result.right_node.get_x1(), 0);
        assert_eq!(result.right_node.get_x2(), 100);
        assert_eq!(result.right_node.get_y1(), 60);
        assert_eq!(result.right_node.get_y2(), 100);
        
        // Проверяем разрез
        assert!(result.cut.get_is_horizontal());
    }

    #[test]
    fn test_vertical_split_basic() {
        let node = TileNode::new(0, 100, 0, 100);
        let result = CuttingEngine::split_vertically(&node, 40).unwrap();
        
        // Проверяем левый узел
        assert_eq!(result.left_node.get_x1(), 0);
        assert_eq!(result.left_node.get_x2(), 40);
        assert_eq!(result.left_node.get_y1(), 0);
        assert_eq!(result.left_node.get_y2(), 100);
        
        // Проверяем правый узел
        assert_eq!(result.right_node.get_x1(), 40);
        assert_eq!(result.right_node.get_x2(), 100);
        assert_eq!(result.right_node.get_y1(), 0);
        assert_eq!(result.right_node.get_y2(), 100);
        
        // Проверяем разрез
        assert!(!result.cut.get_is_horizontal());
    }

    #[test]
    fn test_invalid_split_positions() {
        let node = TileNode::new(10, 90, 20, 80);
        
        // Тестируем невалидные позиции для горизонтального разреза
        assert!(matches!(
            CuttingEngine::split_horizontally(&node, 20),
            Err(CuttingError::InvalidCutPosition { .. })
        ));
        assert!(matches!(
            CuttingEngine::split_horizontally(&node, 80),
            Err(CuttingError::InvalidCutPosition { .. })
        ));
        
        // Тестируем невалидные позиции для вертикального разреза
        assert!(matches!(
            CuttingEngine::split_vertically(&node, 10),
            Err(CuttingError::InvalidCutPosition { .. })
        ));
        assert!(matches!(
            CuttingEngine::split_vertically(&node, 90),
            Err(CuttingError::InvalidCutPosition { .. })
        ));
    }

    #[test]
    fn test_place_tile_exact_fit() {
        let mut node = TileNode::new(0, 50, 0, 30);
        let tile = TileDimensions::simple(50, 30);
        
        let result = CuttingEngine::try_place_tile(&mut node, &tile).unwrap();
        assert!(result);
        assert!(node.is_final);
        assert_eq!(node.external_id, tile.id);
        assert!(!node.is_rotated);
    }

    #[test]
    fn test_place_tile_with_rotation() {
        let mut node = TileNode::new(0, 30, 0, 50);
        let tile = TileDimensions::simple(50, 30); // Не помещается без поворота
        
        let result = CuttingEngine::try_place_tile(&mut node, &tile).unwrap();
        assert!(result);
        assert!(node.is_final);
        assert_eq!(node.external_id, tile.id);
        assert!(node.is_rotated);
    }

    #[test]
    fn test_place_tile_does_not_fit() {
        let mut node = TileNode::new(0, 30, 0, 40);
        let tile = TileDimensions::simple(50, 60); // Слишком большая
        
        let result = CuttingEngine::try_place_tile(&mut node, &tile).unwrap();
        assert!(!result);
        assert!(!node.is_final);
    }

    #[test]
    fn test_place_tile_in_occupied_node() {
        let mut node = TileNode::new(0, 50, 0, 30);
        node.is_final = true; // Узел уже занят
        
        let tile = TileDimensions::simple(25, 15);
        let result = CuttingEngine::try_place_tile(&mut node, &tile).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_place_tile_with_cuts() {
        let mut node = TileNode::new(0, 100, 0, 80);
        let tile = TileDimensions::simple(60, 50);
        
        let result = CuttingEngine::try_place_tile(&mut node, &tile).unwrap();
        assert!(result);
        assert!(node.has_children());
        
        // Проверяем, что плитка размещена в одном из дочерних узлов
        let final_nodes = node.get_final_tile_nodes();
        assert_eq!(final_nodes.len(), 1);
        assert_eq!(final_nodes[0].external_id, tile.id);
    }

    #[test]
    fn test_find_best_fit_node() {
        let mut root = TileNode::new(0, 100, 0, 100);
        
        // Создаем дерево с несколькими свободными узлами
        let child1 = TileNode::new(0, 50, 0, 100);   // Площадь 5000
        let child2 = TileNode::new(50, 100, 0, 50);  // Площадь 2500
        let child3 = TileNode::new(50, 100, 50, 100); // Площадь 2500
        
        root.child1 = Some(Box::new(child1));
        root.child2 = Some(Box::new(child2));
        
        if let Some(ref mut child2_ref) = root.child2 {
            child2_ref.child1 = Some(Box::new(child3));
        }
        
        let tile = TileDimensions::simple(40, 30);
        let best_node = CuttingEngine::find_best_fit_node(&root, &tile);
        
        assert!(best_node.is_some());
        // Лучший узел должен иметь наименьшую площадь среди подходящих
        assert_eq!(best_node.unwrap().get_area(), 2500);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_simple_cutting_workflow() {
        // Создаем лист материала 200x150
        let mut sheet = TileNode::new(0, 200, 0, 150);
        
        // Простая плитка для размещения
        let tile = TileDimensions::simple(80, 60);
        
        let result = CuttingEngine::try_place_tile(&mut sheet, &tile).unwrap();
        assert!(result);
        
        // Проверяем финальное состояние листа
        let final_tiles = sheet.get_final_tile_nodes();
        assert_eq!(final_tiles.len(), 1);
        assert_eq!(final_tiles[0].external_id, tile.id);
        
        // Проверяем использование материала
        let used_area = sheet.get_used_area();
        assert!(used_area > 0);
        assert_eq!(used_area, 80 * 60);
    }

    #[test]
    fn test_error_handling() {
        let node = TileNode::new(0, 100, 0, 100);
        
        // Тест ошибки невалидной позиции разреза
        match CuttingEngine::split_horizontally(&node, 0) {
            Err(CuttingError::InvalidCutPosition { position, min, max }) => {
                assert_eq!(position, 0);
                assert_eq!(min, 0);
                assert_eq!(max, 100);
            }
            _ => panic!("Ожидалась ошибка InvalidCutPosition"),
        }
        
        // Тест ошибки невалидной позиции вертикального разреза
        match CuttingEngine::split_vertically(&node, 100) {
            Err(CuttingError::InvalidCutPosition { position, min, max }) => {
                assert_eq!(position, 100);
                assert_eq!(min, 0);
                assert_eq!(max, 100);
            }
            _ => panic!("Ожидалась ошибка InvalidCutPosition"),
        }
    }

    #[test]
    fn test_performance_with_many_tiles() {
        let mut sheet = TileNode::new(0, 1000, 0, 800);
        
        // Создаем несколько маленьких плиток
        let tiles = vec![
            TileDimensions::simple(25, 20),
            TileDimensions::simple(30, 25),
            TileDimensions::simple(35, 30),
        ];
        
        let start_time = std::time::Instant::now();
        let mut placed = 0;
        
        for tile in &tiles {
            if CuttingEngine::try_place_tile(&mut sheet, tile).unwrap_or(false) {
                placed += 1;
            }
        }
        
        let duration = start_time.elapsed();
        
        assert!(placed > 0);
        assert!(duration.as_millis() < 1000, "Алгоритм должен работать быстро");
        
        println!("Размещено {} из {} плиток за {:?}", placed, tiles.len(), duration);
    }
}
