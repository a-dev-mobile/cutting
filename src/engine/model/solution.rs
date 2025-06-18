use crate::engine::model::{Mosaic, TileDimensions, TileNode, Tile};
use crate::engine::stock::StockSolution;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Атомарный счетчик для уникальных ID решений
static ID_COUNTER: AtomicI32 = AtomicI32::new(0);

/// Решение раскроя - коллекция мозаик с размещенными деталями
#[derive(Debug, Clone)]
pub struct Solution {
    /// Уникальный ID решения
    id: i32,
    /// Список мозаик (листов материала) с размещенными деталями
    mosaics: Vec<Mosaic>,
    /// Детали, которые не удалось разместить
    no_fit_panels: Vec<TileDimensions>,
    /// Неиспользованные стоковые панели
    unused_stock_panels: VecDeque<TileDimensions>,
    /// Время создания решения
    timestamp: u64,
    /// ID группы потоков, создавшей решение
    creator_thread_group: Option<String>,
    /// Дополнительная информация
    aux_info: Option<String>,
}

impl Solution {
    /// Создать пустое решение
    pub fn new() -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            mosaics: Vec::new(),
            no_fit_panels: Vec::new(),
            unused_stock_panels: VecDeque::new(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            creator_thread_group: None,
            aux_info: None,
        }
    }

    /// Создать решение из одной панели
    pub fn from_tile_dimensions(tile_dimensions: TileDimensions) -> Self {
        let mut solution = Self::new();
        solution.add_mosaic(Mosaic::new(&tile_dimensions));
        solution
    }

    /// Создать решение из стокового решения
    pub fn from_stock_solution(stock_solution: &StockSolution) -> Self {
        let mut solution = Self::new();
        
        // Добавляем все стоковые панели в очередь неиспользованных
        for tile in stock_solution.get_stock_tile_dimensions() {
            solution.unused_stock_panels.push_back(tile.clone());
        }
        
        // Берем первую панель и создаем из неё мозаику
        if let Some(first_tile) = solution.unused_stock_panels.pop_front() {
            solution.add_mosaic(Mosaic::new(&first_tile));
        }
        
        solution
    }

    /// Копирующий конструктор
    pub fn copy(other: &Solution) -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            mosaics: other.mosaics.iter().map(|m| m.clone()).collect(),
            no_fit_panels: other.no_fit_panels.clone(),
            unused_stock_panels: other.unused_stock_panels.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            creator_thread_group: other.creator_thread_group.clone(),
            aux_info: other.aux_info.clone(),
        }
    }

    /// Создать решение исключающее указанную мозаику
    pub fn copy_excluding_mosaic(other: &Solution, mosaic_to_exclude: &Mosaic) -> Self {
        let mut solution = Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            mosaics: Vec::new(),
            no_fit_panels: other.no_fit_panels.clone(),
            unused_stock_panels: other.unused_stock_panels.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            creator_thread_group: other.creator_thread_group.clone(),
            aux_info: other.aux_info.clone(),
        };

        // Копируем все мозаики кроме исключаемой
        for mosaic in &other.mosaics {
            if mosaic != mosaic_to_exclude {
                solution.mosaics.push(mosaic.clone());
            }
        }

        solution.sort_mosaics();
        solution
    }

    /// Получить ID решения
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Получить время создания
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Получить/установить группу потоков создателя
    pub fn get_creator_thread_group(&self) -> Option<&String> {
        self.creator_thread_group.as_ref()
    }

    pub fn set_creator_thread_group(&mut self, group: String) {
        self.creator_thread_group = Some(group);
    }

    /// Получить/установить дополнительную информацию
    pub fn get_aux_info(&self) -> Option<&String> {
        self.aux_info.as_ref()
    }

    pub fn set_aux_info(&mut self, info: String) {
        self.aux_info = Some(info);
    }

    /// Сортировать мозаики по неиспользуемой площади (по возрастанию)
    fn sort_mosaics(&mut self) {
        self.mosaics.sort_by(|a, b| {
            // Используем немутабельные методы для сравнения
            let a_area = a.get_unused_area_immutable();
            let b_area = b.get_unused_area_immutable();
            a_area.cmp(&b_area)
        });
    }

    /// Добавить мозаику
    pub fn add_mosaic(&mut self, mosaic: Mosaic) {
        self.mosaics.push(mosaic);
        self.sort_mosaics();
    }

    /// Добавить несколько мозаик
    pub fn add_all_mosaics(&mut self, mosaics: Vec<Mosaic>) {
        self.mosaics.extend(mosaics);
        self.sort_mosaics();
    }

    /// Получить список мозаик
    pub fn get_mosaics(&self) -> &Vec<Mosaic> {
        &self.mosaics
    }

    /// Получить мутабельный список мозаик
    pub fn get_mosaics_mut(&mut self) -> &mut Vec<Mosaic> {
        &mut self.mosaics
    }

    /// Удалить мозаику
    pub fn remove_mosaic(&mut self, mosaic: &Mosaic) {
        self.mosaics.retain(|m| m != mosaic);
    }

    /// Получить неиспользованные стоковые панели
    pub fn get_unused_stock_panels(&self) -> &VecDeque<TileDimensions> {
        &self.unused_stock_panels
    }

    /// Получить мутабельные неиспользованные стоковые панели
    pub fn get_unused_stock_panels_mut(&mut self) -> &mut VecDeque<TileDimensions> {
        &mut self.unused_stock_panels
    }

    /// Получить детали, которые не поместились
    pub fn get_no_fit_panels(&self) -> &Vec<TileDimensions> {
        &self.no_fit_panels
    }

    /// Получить мутабельные детали, которые не поместились
    pub fn get_no_fit_panels_mut(&mut self) -> &mut Vec<TileDimensions> {
        &mut self.no_fit_panels
    }

    /// Установить детали, которые не поместились
    pub fn set_no_fit_panels(&mut self, panels: Vec<TileDimensions>) {
        self.no_fit_panels = panels;
    }

    /// Добавить детали, которые не поместились
    pub fn add_all_no_fit_panels(&mut self, panels: Vec<TileDimensions>) {
        self.no_fit_panels.extend(panels);
    }

    /// Получить коэффициент использования площади
    pub fn get_used_area_ratio(&self) -> f32 {
        if self.mosaics.is_empty() {
            return 0.0;
        }

        let total_ratio: f32 = self.mosaics.iter()
            .map(|m| m.get_used_area_ratio() as f32)
            .sum();
        
        total_ratio / self.mosaics.len() as f32
    }

    /// Проверить, есть ли неиспользуемая базовая плитка
    pub fn has_unused_base_tile(&self) -> bool {
        self.mosaics.first()
            .map(|m| !m.get_root_tile_node().has_final())
            .unwrap_or(false)
    }

    /// Получить количество неиспользуемых плиток
    pub fn get_nbr_unused_tiles(&self) -> i32 {
        self.mosaics.iter()
            .map(|m| m.get_root_tile_node().get_nbr_unused_tiles())
            .sum()
    }

    /// Получить строковое представление базовых размеров
    pub fn get_bases_as_string(&self) -> String {
        let mut result = String::new();
        for mosaic in &self.mosaics {
            let root = mosaic.get_root_tile_node();
            result.push_str(&format!("[{}x{}]", root.get_width(), root.get_height()));
        }
        result
    }

    /// Получить количество горизонтальных плиток
    pub fn get_nbr_horizontal(&self) -> i32 {
        self.mosaics.iter()
            .map(|m| m.get_root_tile_node().get_nbr_final_horizontal())
            .sum()
    }

    /// Получить все финальные узлы плиток
    pub fn get_final_tile_nodes(&self) -> Vec<TileNode> {
        let mut result = Vec::new();
        for mosaic in &self.mosaics {
            let nodes = mosaic.get_root_tile_node().get_final_tile_nodes();
            for node in nodes {
                result.push(node.clone());
            }
        }
        result
    }

    /// Получить все финальные плитки
    pub fn get_final_tiles(&self) -> Vec<Tile> {
        let mut result = Vec::new();
        for mosaic in &self.mosaics {
            let final_nodes = mosaic.get_root_tile_node().get_final_tile_nodes();
            for node in final_nodes {
                result.push(Tile::new(node.get_x1(), node.get_x2(), node.get_y1(), node.get_y2()));
            }
        }
        result
    }

    /// Получить количество финальных плиток
    pub fn get_nbr_final_tiles(&self) -> i32 {
        self.mosaics.iter()
            .map(|m| m.get_root_tile_node().get_nbr_final_tiles())
            .sum()
    }

    /// Получить разность горизонтальных и вертикальных
    pub fn get_hv_diff(&self) -> f32 {
        if self.mosaics.is_empty() {
            return 0.0;
        }

        let total_diff: f32 = self.mosaics.iter()
            .map(|m| m.get_hv_diff() as f32)
            .sum();
        
        total_diff / self.mosaics.len() as f32
    }

    /// Получить общую площадь
    pub fn get_total_area(&self) -> i64 {
        self.mosaics.iter()
            .map(|m| m.get_root_tile_node().get_area())
            .sum()
    }

    /// Получить используемую площадь
    pub fn get_used_area(&self) -> i64 {
        self.mosaics.iter()
            .map(|m| m.get_used_area_immutable())
            .sum()
    }

    /// Получить неиспользуемую площадь
    pub fn get_unused_area(&self) -> i64 {
        self.mosaics.iter()
            .map(|m| m.get_unused_area_immutable())
            .sum()
    }

    /// Получить максимальную глубину
    pub fn get_max_depth(&self) -> i32 {
        self.mosaics.iter()
            .map(|m| m.get_max_depth())
            .fold(0, |acc, depth| acc.max(depth))
    }

    /// Получить количество разрезов
    pub fn get_nbr_cuts(&self) -> i32 {
        self.mosaics.iter()
            .map(|m| m.get_nbr_cuts() as i32)
            .sum()
    }

    /// Получить количество разрезов (алиас для совместимости)
    pub fn get_cuts_count(&self) -> i32 {
        self.get_nbr_cuts()
    }

    /// Получить общее количество разрезов (алиас для совместимости)
    pub fn get_total_cuts_count(&self) -> i32 {
        self.get_nbr_cuts()
    }

    /// Получить потраченную площадь (алиас для get_unused_area)
    pub fn get_wasted_area(&self) -> i64 {
        self.get_unused_area()
    }

    /// Получить общую потраченную площадь (алиас для get_unused_area)
    pub fn get_total_wasted_area(&self) -> i64 {
        self.get_unused_area()
    }

    /// Получить размер различного набора плиток
    pub fn get_distinct_tile_set(&self) -> i32 {
        self.mosaics.iter()
            .map(|m| m.get_distinct_tile_set().len())
            .fold(0, |acc, size| acc.max(size)) as i32
    }

    /// Получить количество мозаик
    pub fn get_nbr_mosaics(&self) -> i32 {
        self.mosaics.len() as i32
    }

    /// Получить размеры стоковых плиток
    pub fn get_stock_tiles_dimensions(&self) -> Vec<TileDimensions> {
        self.mosaics.iter()
            .map(|m| m.get_root_tile_node().to_tile_dimensions())
            .collect()
    }

    /// Получить наибольшую неиспользуемую площадь панели
    pub fn get_most_unused_panel_area(&self) -> i64 {
        self.mosaics.iter()
            .map(|m| {
                let mut mosaic = m.clone();
                mosaic.get_unused_area()
            })
            .max()
            .unwrap_or(0)
    }

    /// Получить расстояние центра масс до начала координат
    pub fn get_center_of_mass_distance_to_origin(&self) -> f32 {
        if self.mosaics.is_empty() {
            return 0.0;
        }

        let total_distance: f32 = self.mosaics.iter()
            .map(|m| m.get_center_of_mass_distance_to_origin() as f32)
            .sum();
        
        total_distance / self.get_nbr_mosaics() as f32
    }

    /// Получить наибольшую площадь
    pub fn get_biggest_area(&self) -> i64 {
        self.mosaics.iter()
            .map(|m| m.get_biggest_area())
            .max()
            .unwrap_or(0)
    }

    /// Получить материал (из первой мозаики)
    pub fn get_material(&self) -> Option<String> {
        self.mosaics.first()
            .map(|m| m.get_material().to_string())
    }

    /// Получить структурный идентификатор решения для удаления дубликатов
    pub fn get_structure_identifier(&self) -> String {
        let mut identifiers = Vec::new();
        
        for mosaic in &self.mosaics {
            identifiers.push(mosaic.get_root_tile_node().to_string_identifier());
        }
        
        identifiers.sort();
        identifiers.join("|")
    }

    /// Получить наибольшую неиспользуемую площадь плитки
    pub fn get_biggest_unused_tile_area(&self) -> i64 {
        self.get_biggest_area()
    }

    /// Попытаться разместить деталь в решении
    pub fn try_place_tile(&mut self, tile_to_place: &TileDimensions) -> Result<Vec<Solution>, crate::error::CuttingError> {
        let mut new_solutions = Vec::new();

        // Пытаемся разместить в существующих мозаиках
        for (_i, mosaic) in self.mosaics.iter().enumerate() {
            match mosaic.add(tile_to_place, false) {
                Ok(result_mosaics) => {
                    for result_mosaic in result_mosaics {
                        // Создаем новое решение исключающее текущую мозаику
                        let mut new_solution = Solution::copy_excluding_mosaic(self, mosaic);
                        new_solution.add_mosaic(result_mosaic);
                        new_solutions.push(new_solution);
                    }
                }
                Err(_) => {
                    // Не удалось разместить в этой мозаике, пробуем следующую
                    continue;
                }
            }
        }

        // Если не удалось разместить ни в одной мозаике, пытаемся создать новую
        if new_solutions.is_empty() {
            if let Some(unused_tile) = self.unused_stock_panels.front() {
                let new_mosaic = Mosaic::new(&unused_tile);
                match new_mosaic.add(tile_to_place, false) {
                    Ok(result_mosaics) => {
                        for result_mosaic in result_mosaics {
                            let mut new_solution = Solution::copy(self);
                            new_solution.unused_stock_panels.pop_front(); // Убираем использованную панель
                            new_solution.add_mosaic(result_mosaic);
                            new_solutions.push(new_solution);
                        }
                    }
                    Err(_) => {
                        // Деталь не помещается даже в новую панель
                        let mut new_solution = Solution::copy(self);
                        new_solution.no_fit_panels.push(tile_to_place.clone());
                        new_solutions.push(new_solution);
                    }
                }
            } else {
                // Нет доступных стоковых панелей
                let mut new_solution = Solution::copy(self);
                new_solution.no_fit_panels.push(tile_to_place.clone());
                new_solutions.push(new_solution);
            }
        }

        Ok(new_solutions)
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Solution[id={}, mosaics={}, used_area={}, unused_area={}]", 
               self.id, 
               self.mosaics.len(),
               self.get_used_area(),
               self.get_unused_area())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_creation_from_stock() {
        let stock_solution = StockSolution::new(vec![
            TileDimensions::simple(1000, 600),
        ]);
        
        let solution = Solution::from_stock_solution(&stock_solution);
        
        assert_eq!(solution.get_mosaics().len(), 1);
        assert_eq!(solution.get_unused_stock_panels().len(), 0);
        assert_eq!(solution.get_no_fit_panels().len(), 0);
        assert_eq!(solution.get_total_area(), 600000);
        assert_eq!(solution.get_used_area(), 0);
        assert_eq!(solution.get_unused_area(), 600000);
    }

    #[test]
    fn test_solution_place_tile() {
        let stock_solution = StockSolution::new(vec![
            TileDimensions::simple(1000, 600),
        ]);
        
        let mut solution = Solution::from_stock_solution(&stock_solution);
        let tile_to_place = TileDimensions::simple(400, 300);
        
        let new_solutions = solution.try_place_tile(&tile_to_place).unwrap();
        
        assert!(!new_solutions.is_empty());
        let first_solution = &new_solutions[0];
        assert_eq!(first_solution.get_nbr_final_tiles(), 1);
        assert_eq!(first_solution.get_used_area(), 120000); // 400 * 300
        assert!(first_solution.get_no_fit_panels().is_empty());
    }

    #[test]
    fn test_solution_copy() {
        let stock_solution = StockSolution::new(vec![
            TileDimensions::simple(1000, 600),
        ]);
        
        let original = Solution::from_stock_solution(&stock_solution);
        let copy = Solution::copy(&original);
        
        assert_ne!(original.get_id(), copy.get_id());
        assert_eq!(original.get_mosaics().len(), copy.get_mosaics().len());
        assert_eq!(original.get_total_area(), copy.get_total_area());
    }

    #[test]
    fn test_solution_metrics() {
        let stock_solution = StockSolution::new(vec![
            TileDimensions::simple(1000, 600),
            TileDimensions::simple(800, 500),
        ]);
        
        let solution = Solution::from_stock_solution(&stock_solution);
        
        assert_eq!(solution.get_nbr_mosaics(), 1);
        assert_eq!(solution.get_total_area(), 600000);
        assert_eq!(solution.get_unused_stock_panels().len(), 1);
        assert_eq!(solution.get_material(), Some("DEFAULT_MATERIAL".to_string()));
    }

    #[test]
    fn test_solution_no_fit_panels() {
        let stock_solution = StockSolution::new(vec![
            TileDimensions::simple(100, 100), // Маленькая панель
        ]);
        
        let mut solution = Solution::from_stock_solution(&stock_solution);
        let large_tile = TileDimensions::simple(200, 200); // Большая деталь
        
        let new_solutions = solution.try_place_tile(&large_tile).unwrap();
        
        assert!(!new_solutions.is_empty());
        let first_solution = &new_solutions[0];
        assert_eq!(first_solution.get_no_fit_panels().len(), 1);
        assert_eq!(first_solution.get_nbr_final_tiles(), 0);
    }
}
