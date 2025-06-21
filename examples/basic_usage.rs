//! Пример базового использования библиотеки cutlist-optimizer-cli

use cutlist_optimizer_cli::prelude::*;

fn main() -> Result<()> {
    println!("Пример использования библиотеки CutList Optimizer");
    
    // Создание конфигурации
    let config = Configuration::default();
    println!("Конфигурация создана с толщиной реза: {} мм", config.cut_thickness);
    
    // Создание размеров плитки (id, width, height)
    let tile = TileDimensions::new(1, 100, 200);
    println!("Размеры плитки: {} (площадь: {} мм²)", tile.dimensions_string(), tile.area());
    
    // Работа с ориентацией
    let orientation = Orientation::Horizontal;
    println!("Ориентация: {:?}", orientation);
    
    // Проверка возможности поворота
    if tile.can_rotate() {
        println!("Плитка может быть повернута");
    }
    
    // Создание контейнера и проверка помещения
    let container = TileDimensions::new(2, 300, 400);
    if tile.fits(&container) {
        println!("Плитка помещается в контейнер {}", container.dimensions_string());
    }
    
    Ok(())
}
