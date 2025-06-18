use std::collections::HashMap;
use crate::engine::model::{solution::Solution, tile::TileDimensions};
use crate::error::CuttingError;

/// Утилиты для работы с алгоритмом оптимизации раскроя
pub struct Utils;

impl Utils {
    /// Вычисляет общую площадь списка деталей
    /// 
    /// # Аргументы
    /// * `tiles` - список деталей для вычисления площади
    /// 
    /// # Возвращает
    /// Общую площадь всех деталей в квадратных единицах
    pub fn calculate_total_area(tiles: &[TileDimensions]) -> u64 {
        tiles.iter()
            .map(|tile| tile.width as u64 * tile.height as u64)
            .sum()
    }
    
    /// Вычисляет общую площадь деталей с учетом количества
    /// 
    /// # Аргументы
    /// * `tiles_with_count` - карта деталей с их количеством
    /// 
    /// # Возвращает
    /// Общую площадь всех деталей с учетом количества
    pub fn calculate_total_area_with_count(tiles_with_count: &HashMap<TileDimensions, u32>) -> u64 {
        tiles_with_count.iter()
            .map(|(tile, count)| {
                let area = tile.width as u64 * tile.height as u64;
                area * (*count as u64)
            })
            .sum()
    }
    
    /// Группирует одинаковые детали и подсчитывает их количество
    /// 
    /// # Аргументы
    /// * `tiles` - список деталей для группировки
    /// 
    /// # Возвращает
    /// Карту уникальных деталей с их количеством
    pub fn group_tiles_by_dimensions(tiles: &[TileDimensions]) -> HashMap<TileDimensions, u32> {
        let mut grouped = HashMap::new();
        
        for tile in tiles {
            *grouped.entry(tile.clone()).or_insert(0) += 1;
        }
        
        grouped
    }
    
    /// Разворачивает сгруппированные детали обратно в список
    /// 
    /// # Аргументы
    /// * `grouped_tiles` - карта деталей с количеством
    /// 
    /// # Возвращает
    /// Список всех деталей
    pub fn expand_grouped_tiles(grouped_tiles: &HashMap<TileDimensions, u32>) -> Vec<TileDimensions> {
        let mut tiles = Vec::new();
        
        for (tile, count) in grouped_tiles {
            for _ in 0..*count {
                tiles.push(tile.clone());
            }
        }
        
        tiles
    }
    
    /// Сортирует детали по убыванию площади
    /// 
    /// # Аргументы
    /// * `tiles` - список деталей для сортировки
    /// 
    /// # Возвращает
    /// Отсортированный список деталей
    pub fn sort_tiles_by_area_desc(mut tiles: Vec<TileDimensions>) -> Vec<TileDimensions> {
        tiles.sort_by(|a, b| {
            let area_a = a.width as u64 * a.height as u64;
            let area_b = b.width as u64 * b.height as u64;
            area_b.cmp(&area_a) // По убыванию
        });
        tiles
    }
    
    /// Сортирует детали по убыванию максимального размера
    /// 
    /// # Аргументы
    /// * `tiles` - список деталей для сортировки
    /// 
    /// # Возвращает
    /// Отсортированный список деталей
    pub fn sort_tiles_by_max_dimension_desc(mut tiles: Vec<TileDimensions>) -> Vec<TileDimensions> {
        tiles.sort_by(|a, b| {
            let max_a = a.width.max(a.height);
            let max_b = b.width.max(b.height);
            max_b.cmp(&max_a) // По убыванию
        });
        tiles
    }
    
    /// Проверяет, может ли деталь поместиться в заданные размеры
    /// 
    /// # Аргументы
    /// * `tile` - деталь для проверки
    /// * `available_width` - доступная ширина
    /// * `available_height` - доступная высота
    /// * `allow_rotation` - разрешить поворот детали
    /// 
    /// # Возвращает
    /// true, если деталь помещается
    pub fn can_tile_fit(
        tile: &TileDimensions,
        available_width: u32,
        available_height: u32,
        allow_rotation: bool,
    ) -> bool {
        // Проверяем в исходной ориентации
        if tile.width <= available_width as i32 && tile.height <= available_height as i32 {
            return true;
        }
        
        // Проверяем с поворотом на 90 градусов
        if allow_rotation && !tile.is_square() {
            if tile.height <= available_width as i32 && tile.width <= available_height as i32 {
                return true;
            }
        }
        
        false
    }
    
    /// Вычисляет коэффициент использования материала
    /// 
    /// # Аргументы
    /// * `used_area` - использованная площадь
    /// * `total_area` - общая площадь материала
    /// 
    /// # Возвращает
    /// Коэффициент использования от 0.0 до 1.0
    pub fn calculate_material_utilization(used_area: u64, total_area: u64) -> f64 {
        if total_area == 0 {
            return 0.0;
        }
        used_area as f64 / total_area as f64
    }
    
    /// Вычисляет процент отходов
    /// 
    /// # Аргументы
    /// * `used_area` - использованная площадь
    /// * `total_area` - общая площадь материала
    /// 
    /// # Возвращает
    /// Процент отходов от 0.0 до 100.0
    pub fn calculate_waste_percentage(used_area: u64, total_area: u64) -> f64 {
        if total_area == 0 {
            return 0.0;
        }
        let waste_area = total_area.saturating_sub(used_area);
        (waste_area as f64 / total_area as f64) * 100.0
    }
    
    /// Находит наилучшее решение из списка по заданному критерию
    /// 
    /// # Аргументы
    /// * `solutions` - список решений для анализа
    /// * `criterion` - функция для оценки решения (меньше = лучше)
    /// 
    /// # Возвращает
    /// Индекс наилучшего решения или None, если список пуст
    pub fn find_best_solution<F>(solutions: &[Solution], criterion: F) -> Option<usize>
    where
        F: Fn(&Solution) -> f64,
    {
        if solutions.is_empty() {
            return None;
        }
        
        let mut best_index = 0;
        let mut best_score = criterion(&solutions[0]);
        
        for (index, solution) in solutions.iter().enumerate().skip(1) {
            let score = criterion(solution);
            if score < best_score {
                best_score = score;
                best_index = index;
            }
        }
        
        Some(best_index)
    }
    
    /// Генерирует уникальный идентификатор для конфигурации деталей
    /// 
    /// # Аргументы
    /// * `tiles` - список деталей
    /// 
    /// # Возвращает
    /// Строковый идентификатор конфигурации
    pub fn generate_tiles_configuration_id(tiles: &[TileDimensions]) -> String {
        let grouped = Self::group_tiles_by_dimensions(tiles);
        let mut items: Vec<_> = grouped.iter().collect();
        items.sort_by_key(|(tile, _)| (tile.width, tile.height));
        
        items.iter()
            .map(|(tile, count)| format!("{}x{}:{}", tile.width, tile.height, count))
            .collect::<Vec<_>>()
            .join(";")
    }
    
    /// Проверяет валидность размеров детали
    /// 
    /// # Аргументы
    /// * `tile` - деталь для проверки
    /// * `min_dimension` - минимальный допустимый размер
    /// * `max_dimension` - максимальный допустимый размер
    /// 
    /// # Возвращает
    /// Result с ошибкой, если размеры невалидны
    pub fn validate_tile_dimensions(
        tile: &TileDimensions,
        min_dimension: u32,
        max_dimension: u32,
    ) -> Result<(), CuttingError> {
        if tile.width == 0 || tile.height == 0 {
            return Err(CuttingError::InvalidInput(
                "Размеры детали не могут быть нулевыми".to_string()
            ));
        }
        
        if tile.width < min_dimension as i32 || tile.height < min_dimension as i32 {
            return Err(CuttingError::InvalidInput(
                format!("Размеры детали меньше минимального: {}", min_dimension)
            ));
        }
        
        if tile.width > max_dimension as i32 || tile.height > max_dimension as i32 {
            return Err(CuttingError::InvalidInput(
                format!("Размеры детали больше максимального: {}", max_dimension)
            ));
        }
        
        Ok(())
    }
    
    /// Вычисляет статистику по списку решений
    /// 
    /// # Аргументы
    /// * `solutions` - список решений для анализа
    /// 
    /// # Возвращает
    /// Кортеж (среднее, минимум, максимум) для площади отходов
    pub fn calculate_waste_statistics(solutions: &[Solution]) -> (f64, f64, f64) {
        if solutions.is_empty() {
            return (0.0, 0.0, 0.0);
        }
        
        let waste_areas: Vec<f64> = solutions.iter()
            .map(|solution| solution.get_unused_area() as f64)
            .collect();
        
        let sum: f64 = waste_areas.iter().sum();
        let average = sum / waste_areas.len() as f64;
        let min = waste_areas.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = waste_areas.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        (average, min, max)
    }
    
    /// Форматирует размер в человекочитаемый вид
    /// 
    /// # Аргументы
    /// * `size_bytes` - размер в байтах
    /// 
    /// # Возвращает
    /// Отформатированную строку с размером
    pub fn format_size(size_bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = size_bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", size_bytes, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }
    
    /// Форматирует время в человекочитаемый вид
    /// 
    /// # Аргументы
    /// * `millis` - время в миллисекундах
    /// 
    /// # Возвращает
    /// Отформатированную строку с временем
    pub fn format_duration(millis: u64) -> String {
        let seconds = millis / 1000;
        let minutes = seconds / 60;
        let hours = minutes / 60;
        
        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes % 60, seconds % 60)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds % 60)
        } else if seconds > 0 {
            format!("{}.{}s", seconds, (millis % 1000) / 100)
        } else {
            format!("{}ms", millis)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_total_area() {
        let tiles = vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
            TileDimensions::simple(50, 100),
        ];
        
        let total_area = Utils::calculate_total_area(&tiles);
        assert_eq!(total_area, 20000 + 45000 + 5000);
    }
    
    #[test]
    fn test_group_tiles_by_dimensions() {
        let tiles = vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ];
        
        let grouped = Utils::group_tiles_by_dimensions(&tiles);
        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped[&TileDimensions::simple(100, 200)], 2);
        assert_eq!(grouped[&TileDimensions::simple(150, 300)], 1);
    }
    
    #[test]
    fn test_can_tile_fit() {
        let tile = TileDimensions::simple(100, 200);
        
        // Помещается в исходной ориентации
        assert!(Utils::can_tile_fit(&tile, 150, 250, false));
        
        // Не помещается в исходной ориентации
        assert!(!Utils::can_tile_fit(&tile, 80, 250, false));
        
        // Помещается с поворотом
        assert!(Utils::can_tile_fit(&tile, 250, 150, true));
    }
    
    #[test]
    fn test_calculate_material_utilization() {
        let utilization = Utils::calculate_material_utilization(800, 1000);
        assert_eq!(utilization, 0.8);
        
        let utilization_zero = Utils::calculate_material_utilization(0, 0);
        assert_eq!(utilization_zero, 0.0);
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(Utils::format_duration(500), "500ms");
        assert_eq!(Utils::format_duration(1500), "1.5s");
        assert_eq!(Utils::format_duration(65000), "1m 5s");
        assert_eq!(Utils::format_duration(3665000), "1h 1m 5s");
    }
}
