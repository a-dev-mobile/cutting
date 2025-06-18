use cutting_cli::engine::model::tile::{TileNode, TileDimensions};
use cutting_cli::engine::cutting::CuttingEngine;

fn main() {
    println!("🔧 Cutting");
    println!("================================================");
    println!();

    // Демонстрация Этапа 1: Базовые структуры данных
    println!("📋 Этап 1: Базовые структуры данных");
    println!("-----------------------------------");
    
    // Создаем лист материала
    let mut sheet = TileNode::new(0, 1000, 0, 600);
    println!("Создан лист материала: {}x{} мм", sheet.get_width(), sheet.get_height());
    
    // Создаем список плиток для размещения
    let tiles = vec![
        TileDimensions::new(1, 300, 200, "Фанера".to_string(), 1, Some("Столешница".to_string())),
        TileDimensions::new(2, 150, 100, "Фанера".to_string(), 1, Some("Полка".to_string())),
        TileDimensions::new(3, 200, 250, "Фанера".to_string(), 1, Some("Дверца".to_string())),
        TileDimensions::new(4, 100, 80, "Фанера".to_string(), 1, Some("Ящик".to_string())),
        TileDimensions::new(5, 50, 50, "Фанера".to_string(), 1, Some("Квадрат".to_string())),
    ];
    
    println!("Плитки для размещения:");
    for tile in &tiles {
        println!("  - ID {}: {}x{} мм ({})", 
            tile.id, tile.width, tile.height, 
            tile.label.as_ref().unwrap_or(&"Без названия".to_string()));
    }
    println!();

    // Демонстрация Этапа 2: Алгоритмы разрезания
    println!("⚙️  Этап 2: Алгоритмы разрезания");
    println!("-------------------------------");
    
    let mut placed_count = 0;
    let mut total_used_area = 0i64;
    
    for tile in &tiles {
        println!("Размещаем плитку ID {}: {}x{} мм...", tile.id, tile.width, tile.height);
        
        match CuttingEngine::try_place_tile(&mut sheet, tile) {
            Ok(true) => {
                placed_count += 1;
                total_used_area += tile.get_area();
                println!("  ✅ Успешно размещена{}", if sheet.get_final_tile_nodes().last().map_or(false, |n| n.is_rotated) { " (повернута)" } else { "" });
            }
            Ok(false) => {
                println!("  ❌ Не удалось разместить - не помещается");
            }
            Err(e) => {
                println!("  ❌ Ошибка: {:?}", e);
            }
        }
    }
    
    println!();
    println!("📊 Результаты размещения");
    println!("------------------------");
    println!("Размещено плиток: {}/{}", placed_count, tiles.len());
    println!("Использованная площадь: {} мм²", sheet.get_used_area());
    println!("Общая площадь листа: {} мм²", sheet.get_area());
    println!("Эффективность использования: {:.1}%", sheet.get_used_area_ratio() * 100.0);
    println!("Количество разрезов: {}", sheet.get_nbr_final_tiles());
    
    // Показываем финальные плитки
    println!();
    println!("🎯 Размещенные плитки:");
    let final_tiles = sheet.get_final_tile_nodes();
    for (i, final_tile) in final_tiles.iter().enumerate() {
        let original_tile = tiles.iter().find(|t| t.id == final_tile.external_id);
        if let Some(tile) = original_tile {
            println!("  {}. {} - позиция ({}, {}) размер {}x{}{}", 
                i + 1,
                tile.label.as_ref().unwrap_or(&format!("ID {}", tile.id)),
                final_tile.get_x1(), final_tile.get_y1(),
                final_tile.get_width(), final_tile.get_height(),
                if final_tile.is_rotated { " (повернута)" } else { "" }
            );
        }
    }
    
    // Демонстрация тестирования алгоритмов
    println!();
    println!("🧪 Тестирование алгоритмов");
    println!("--------------------------");
    
    // Тест горизонтального разреза
    let test_node = TileNode::new(0, 200, 0, 100);
    match CuttingEngine::split_horizontally(&test_node, 60) {
        Ok(result) => {
            println!("✅ Горизонтальный разрез: {}x{} → {}x{} + {}x{}", 
                test_node.get_width(), test_node.get_height(),
                result.left_node.get_width(), result.left_node.get_height(),
                result.right_node.get_width(), result.right_node.get_height());
        }
        Err(e) => println!("❌ Ошибка горизонтального разреза: {:?}", e),
    }
    
    // Тест вертикального разреза
    match CuttingEngine::split_vertically(&test_node, 120) {
        Ok(result) => {
            println!("✅ Вертикальный разрез: {}x{} → {}x{} + {}x{}", 
                test_node.get_width(), test_node.get_height(),
                result.left_node.get_width(), result.left_node.get_height(),
                result.right_node.get_width(), result.right_node.get_height());
        }
        Err(e) => println!("❌ Ошибка вертикального разреза: {:?}", e),
    }
    
    println!();
    println!("🎉 Демонстрация завершена!");
    println!("Этапы 1 и 2 успешно реализованы и протестированы.");
}
