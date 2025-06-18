use crate::engine::model::TileDimensions;
use std::collections::HashMap;

/// Решение со складскими панелями - комбинация панелей для раскроя
#[derive(Debug, Clone)]
pub struct StockSolution {
    stock_tile_dimensions: Vec<TileDimensions>,
}

impl StockSolution {
    /// Создать новое решение из списка панелей
    pub fn new(stock_tile_dimensions: Vec<TileDimensions>) -> Self {
        Self {
            stock_tile_dimensions,
        }
    }

    /// Создать новое решение из массива панелей
    pub fn from_array(tiles: &[TileDimensions]) -> Self {
        Self {
            stock_tile_dimensions: tiles.to_vec(),
        }
    }

    /// Копирующий конструктор
    pub fn copy(other: &StockSolution) -> Self {
        Self {
            stock_tile_dimensions: other.stock_tile_dimensions.clone(),
        }
    }

    /// Добавить стоковую панель
    pub fn add_stock_tile(&mut self, tile_dimensions: TileDimensions) {
        self.stock_tile_dimensions.push(tile_dimensions);
    }

    /// Получить список стоковых панелей
    pub fn get_stock_tile_dimensions(&self) -> &Vec<TileDimensions> {
        &self.stock_tile_dimensions
    }

    /// Установить список стоковых панелей
    pub fn set_stock_tile_dimensions(&mut self, stock_tile_dimensions: Vec<TileDimensions>) {
        self.stock_tile_dimensions = stock_tile_dimensions;
    }

    /// Сортировать панели по возрастанию площади
    pub fn sort_panels_asc(&mut self) {
        self.stock_tile_dimensions.sort_by(|a, b| a.get_area().cmp(&b.get_area()));
    }

    /// Сортировать панели по убыванию площади
    pub fn sort_panels_desc(&mut self) {
        self.stock_tile_dimensions.sort_by(|a, b| b.get_area().cmp(&a.get_area()));
    }

    /// Проверить, имеют ли все панели одинаковый размер
    pub fn has_unique_panel_size(&self) -> bool {
        if self.stock_tile_dimensions.is_empty() {
            return true;
        }

        let first = &self.stock_tile_dimensions[0];
        self.stock_tile_dimensions.iter().all(|tile| tile.has_same_dimensions(first))
    }

    /// Получить общую площадь всех панелей
    pub fn get_total_area(&self) -> i64 {
        self.stock_tile_dimensions.iter().map(|tile| tile.get_area()).sum()
    }

    /// Строковое представление
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for tile in &self.stock_tile_dimensions {
            result.push_str(&format!("[{}x{}]", tile.width, tile.height));
        }
        result
    }

    /// Строковое представление с группировкой одинаковых размеров
    pub fn to_string_grouped(&self) -> String {
        let mut groups: HashMap<String, i32> = HashMap::new();
        
        for tile in &self.stock_tile_dimensions {
            let key = format!("{}x{}", tile.width, tile.height);
            *groups.entry(key).or_insert(0) += 1;
        }

        let mut result = String::new();
        for (size, count) in groups {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&format!("{}*{}", size, count));
        }

        result
    }
}

impl PartialEq for StockSolution {
    fn eq(&self, other: &Self) -> bool {
        if self.stock_tile_dimensions.len() != other.stock_tile_dimensions.len() {
            return false;
        }

        // Создаем копии для сортировки
        let mut self_tiles = self.stock_tile_dimensions.clone();
        let mut other_tiles = other.stock_tile_dimensions.clone();
        
        // Сортируем по размерам для корректного сравнения
        self_tiles.sort_by(|a, b| {
            let a_key = (a.width, a.height);
            let b_key = (b.width, b.height);
            a_key.cmp(&b_key)
        });
        
        other_tiles.sort_by(|a, b| {
            let a_key = (a.width, a.height);
            let b_key = (b.width, b.height);
            a_key.cmp(&b_key)
        });

        // Сравниваем каждую панель
        for (self_tile, other_tile) in self_tiles.iter().zip(other_tiles.iter()) {
            if !self_tile.has_same_dimensions(other_tile) {
                return false;
            }
        }

        true
    }
}

impl std::hash::Hash for StockSolution {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Создаем отсортированную копию для консистентного хеширования
        let mut sorted_tiles = self.stock_tile_dimensions.clone();
        sorted_tiles.sort_by(|a, b| {
            let a_key = (a.width, a.height);
            let b_key = (b.width, b.height);
            a_key.cmp(&b_key)
        });
        
        for tile in sorted_tiles {
            tile.width.hash(state);
            tile.height.hash(state);
        }
    }
}

impl std::fmt::Display for StockSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_solution_creation() {
        let tiles = vec![
            TileDimensions::simple(1000, 600),
            TileDimensions::simple(800, 500),
        ];
        
        let stock_solution = StockSolution::new(tiles.clone());
        assert_eq!(stock_solution.get_stock_tile_dimensions().len(), 2);
        assert_eq!(stock_solution.get_total_area(), 1000 * 600 + 800 * 500);
    }

    #[test]
    fn test_stock_solution_sorting() {
        let mut stock_solution = StockSolution::new(vec![
            TileDimensions::simple(1000, 600), // 600000
            TileDimensions::simple(800, 500),  // 400000
            TileDimensions::simple(1200, 800), // 960000
        ]);

        stock_solution.sort_panels_asc();
        let areas: Vec<i64> = stock_solution.get_stock_tile_dimensions()
            .iter()
            .map(|t| t.get_area())
            .collect();
        assert_eq!(areas, vec![400000, 600000, 960000]);

        stock_solution.sort_panels_desc();
        let areas: Vec<i64> = stock_solution.get_stock_tile_dimensions()
            .iter()
            .map(|t| t.get_area())
            .collect();
        assert_eq!(areas, vec![960000, 600000, 400000]);
    }

    #[test]
    fn test_has_unique_panel_size() {
        let unique_solution = StockSolution::new(vec![
            TileDimensions::simple(1000, 600),
            TileDimensions::simple(1000, 600),
        ]);
        assert!(unique_solution.has_unique_panel_size());

        let mixed_solution = StockSolution::new(vec![
            TileDimensions::simple(1000, 600),
            TileDimensions::simple(800, 500),
        ]);
        assert!(!mixed_solution.has_unique_panel_size());
    }

    #[test]
    fn test_to_string_grouped() {
        let stock_solution = StockSolution::new(vec![
            TileDimensions::simple(1000, 600),
            TileDimensions::simple(1000, 600),
            TileDimensions::simple(800, 500),
        ]);

        let grouped = stock_solution.to_string_grouped();
        assert!(grouped.contains("1000x600*2"));
        assert!(grouped.contains("800x500*1"));
    }

    #[test]
    fn test_equality() {
        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(1000, 600),
            TileDimensions::simple(800, 500),
        ]);

        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(800, 500),
            TileDimensions::simple(1000, 600),
        ]);

        assert_eq!(solution1, solution2); // Порядок не важен
    }
}
