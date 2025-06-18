use crate::engine::model::tile::{TileNode, TileDimensions};
use crate::engine::model::cut::Cut;
use crate::engine::cutting::CuttingEngine;
use crate::error::CuttingError;
use serde::{Serialize, Deserialize};

/// Мозаика - представляет один лист материала с размещенными деталями и разрезами
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mosaic {
    /// Корневой узел дерева размещения
    pub root_tile_node: TileNode,
    /// Список всех разрезов на листе
    pub cuts: Vec<Cut>,
    /// Тип материала
    pub material: String,
    /// ID стокового листа
    pub stock_id: i32,
    /// Ориентация листа (направление волокон)
    pub orientation: i32,
    /// Толщина реза (в пикселях)
    pub kerf_width: i32,
}

impl Mosaic {
    /// Создать новую мозаику из стокового листа
    pub fn new(stock_tile: &TileDimensions) -> Self {
        Self {
            root_tile_node: TileNode::from_dimensions(stock_tile),
            cuts: Vec::new(),
            material: stock_tile.material.clone(),
            stock_id: stock_tile.id,
            orientation: stock_tile.orientation,
            kerf_width: 3, // Толщина реза по умолчанию
        }
    }

    /// Создать новую мозаику с указанной толщиной реза
    pub fn new_with_kerf(stock_tile: &TileDimensions, kerf_width: i32) -> Self {
        Self {
            root_tile_node: TileNode::from_dimensions(stock_tile),
            cuts: Vec::new(),
            material: stock_tile.material.clone(),
            stock_id: stock_tile.id,
            orientation: stock_tile.orientation,
            kerf_width,
        }
    }

    /// Получить корневой узел
    pub fn get_root_tile_node(&self) -> &TileNode {
        &self.root_tile_node
    }

    /// Получить корневой узел (мутабельная версия)
    pub fn get_root_tile_node_mut(&mut self) -> &mut TileNode {
        &mut self.root_tile_node
    }

    /// Получить материал
    pub fn get_material(&self) -> &str {
        &self.material
    }

    /// Получить ID стокового листа
    pub fn get_stock_id(&self) -> i32 {
        self.stock_id
    }

    /// Получить список разрезов
    pub fn get_cuts(&self) -> &Vec<Cut> {
        &self.cuts
    }

    /// Получить используемую площадь
    pub fn get_used_area(&mut self) -> i64 {
        self.root_tile_node.get_used_area()
    }

    /// Получить неиспользуемую площадь
    pub fn get_unused_area(&mut self) -> i64 {
        self.root_tile_node.get_unused_area()
    }

    /// Получить общую площадь листа
    pub fn get_total_area(&self) -> i64 {
        self.root_tile_node.get_area()
    }

    /// Получить количество разрезов
    pub fn get_nbr_cuts(&self) -> usize {
        self.cuts.len()
    }

    /// Получить финальные узлы (размещенные детали)
    pub fn get_final_tile_nodes(&self) -> Vec<&TileNode> {
        self.root_tile_node.get_final_tile_nodes()
    }

    /// Получить неиспользуемые узлы
    pub fn get_unused_tile_nodes(&self) -> Vec<&TileNode> {
        self.root_tile_node.get_unused_tiles()
    }

    /// Получить самый большой неиспользуемый узел
    pub fn get_biggest_unused_tile(&self) -> Option<&TileNode> {
        let unused_tiles = self.get_unused_tile_nodes();
        unused_tiles.iter()
            .max_by_key(|tile| tile.get_area())
            .copied()
    }

    /// Получить разность между горизонтальными и вертикальными деталями
    pub fn get_hv_diff(&self) -> i32 {
        let horizontal = self.root_tile_node.get_nbr_final_horizontal();
        let vertical = self.root_tile_node.get_nbr_final_vertical();
        (horizontal - vertical).abs()
    }

    /// Получить расстояние центра масс до начала координат
    pub fn get_center_of_mass_distance_to_origin(&self) -> f64 {
        let final_nodes = self.get_final_tile_nodes();
        if final_nodes.is_empty() {
            return 0.0;
        }

        let mut total_area = 0i64;
        let mut weighted_x = 0f64;
        let mut weighted_y = 0f64;

        for node in final_nodes {
            let area = node.get_area();
            let center_x = (node.get_x1() + node.get_x2()) as f64 / 2.0;
            let center_y = (node.get_y1() + node.get_y2()) as f64 / 2.0;

            total_area += area;
            weighted_x += center_x * area as f64;
            weighted_y += center_y * area as f64;
        }

        if total_area == 0 {
            return 0.0;
        }

        let center_of_mass_x = weighted_x / total_area as f64;
        let center_of_mass_y = weighted_y / total_area as f64;

        (center_of_mass_x * center_of_mass_x + center_of_mass_y * center_of_mass_y).sqrt()
    }

    /// Найти кандидатов для размещения детали
    pub fn find_candidates(&self, tile_dimensions: &TileDimensions) -> Vec<&TileNode> {
        let mut candidates = Vec::new();
        self.find_candidates_recursive(&self.root_tile_node, tile_dimensions, &mut candidates);
        candidates
    }

    /// Рекурсивный поиск кандидатов
    fn find_candidates_recursive<'a>(
        &'a self,
        node: &'a TileNode,
        tile_dimensions: &TileDimensions,
        candidates: &mut Vec<&'a TileNode>,
    ) {
        // Проверяем только свободные узлы
        if node.is_final || node.has_children() {
            if let Some(ref child1) = node.child1 {
                self.find_candidates_recursive(child1, tile_dimensions, candidates);
            }
            if let Some(ref child2) = node.child2 {
                self.find_candidates_recursive(child2, tile_dimensions, candidates);
            }
            return;
        }

        // Проверяем, помещается ли деталь в узел
        let fits_normal = node.get_width() >= tile_dimensions.width 
            && node.get_height() >= tile_dimensions.height;
        let fits_rotated = !tile_dimensions.is_square() 
            && node.get_width() >= tile_dimensions.height 
            && node.get_height() >= tile_dimensions.width;

        if fits_normal || fits_rotated {
            candidates.push(node);
        }
    }

    /// Размещение детали в мозаике
    /// Возвращает список возможных результирующих мозаик
    pub fn add(
        &self,
        tile_dimensions: &TileDimensions,
        consider_grain_direction: bool,
    ) -> Result<Vec<Mosaic>, CuttingError> {
        let candidates = self.find_candidates(tile_dimensions);
        
        if candidates.is_empty() {
            return Ok(Vec::new()); // Деталь не помещается
        }

        let mut result_mosaics = Vec::new();

        for candidate in candidates {
            // Пробуем разместить в исходной ориентации
            if let Ok(mosaic) = self.try_place_in_node(candidate, tile_dimensions, false) {
                result_mosaics.push(mosaic);
            }

            // Пробуем разместить с поворотом (если не квадрат и разрешен поворот)
            if !tile_dimensions.is_square() && !consider_grain_direction {
                if let Ok(mosaic) = self.try_place_in_node(candidate, tile_dimensions, true) {
                    result_mosaics.push(mosaic);
                }
            }
        }

        Ok(result_mosaics)
    }

    /// Попытка размещения детали в конкретном узле
    fn try_place_in_node(
        &self,
        target_node: &TileNode,
        tile_dimensions: &TileDimensions,
        rotated: bool,
    ) -> Result<Mosaic, CuttingError> {
        let mut new_mosaic = self.clone();
        
        // Сохраняем координаты для поиска
        let x1 = target_node.get_x1();
        let x2 = target_node.get_x2();
        let y1 = target_node.get_y1();
        let y2 = target_node.get_y2();
        
        // Размещаем деталь используя координаты
        Self::place_tile_at_coordinates(&mut new_mosaic, x1, x2, y1, y2, tile_dimensions, rotated)?;

        Ok(new_mosaic)
    }

    /// Размещение детали по координатам
    fn place_tile_at_coordinates(
        mosaic: &mut Mosaic,
        x1: i32,
        x2: i32,
        y1: i32,
        y2: i32,
        tile_dimensions: &TileDimensions,
        rotated: bool,
    ) -> Result<(), CuttingError> {
        // Сначала находим узел, затем размещаем деталь
        let node_found = Self::find_node_mut_by_coordinates_static(&mut mosaic.root_tile_node, x1, x2, y1, y2).is_some();
        
        if !node_found {
            return Err(CuttingError::GeneralCuttingError("Target node not found".to_string()));
        }

        // Теперь снова находим узел для размещения
        if let Some(target_node) = Self::find_node_mut_by_coordinates_static(&mut mosaic.root_tile_node, x1, x2, y1, y2) {
            Self::place_tile_in_node_direct(target_node, tile_dimensions, rotated, &mut mosaic.cuts)
        } else {
            Err(CuttingError::GeneralCuttingError("Target node not found".to_string()))
        }
    }

    /// Прямое размещение детали в узле без передачи всей мозаики
    fn place_tile_in_node_direct(
        node: &mut TileNode,
        tile_dimensions: &TileDimensions,
        rotated: bool,
        cuts: &mut Vec<Cut>,
    ) -> Result<(), CuttingError> {
        let (tile_width, tile_height) = if rotated {
            (tile_dimensions.height, tile_dimensions.width)
        } else {
            (tile_dimensions.width, tile_dimensions.height)
        };

        // Проверяем точное совпадение
        if node.get_width() == tile_width && node.get_height() == tile_height {
            node.is_final = true;
            node.external_id = tile_dimensions.id;
            node.is_rotated = rotated;
            return Ok(());
        }

        // Определяем необходимые разрезы
        let width_diff = node.get_width() - tile_width;
        let height_diff = node.get_height() - tile_height;

        if width_diff > 0 && height_diff > 0 {
            // Нужны оба разреза - используем алгоритм splitHV
            Self::split_hv_direct(node, tile_dimensions, tile_width, tile_height, rotated, cuts)
        } else if width_diff > 0 {
            // Только вертикальный разрез
            Self::split_vertical_only_direct(node, tile_dimensions, tile_width, rotated, cuts)
        } else if height_diff > 0 {
            // Только горизонтальный разрез
            Self::split_horizontal_only_direct(node, tile_dimensions, tile_height, rotated, cuts)
        } else {
            Err(CuttingError::GeneralCuttingError("Invalid placement scenario".to_string()))
        }
    }

    /// Алгоритм splitHV - прямая версия
    fn split_hv_direct(
        node: &mut TileNode,
        tile_dimensions: &TileDimensions,
        tile_width: i32,
        tile_height: i32,
        rotated: bool,
        cuts: &mut Vec<Cut>,
    ) -> Result<(), CuttingError> {
        // Сначала вертикальный разрез по ширине детали
        let cut_x = node.get_x1() + tile_width;
        let cut_result = CuttingEngine::split_vertically(node, cut_x)?;
        
        cuts.push(cut_result.cut);
        node.child1 = Some(Box::new(cut_result.left_node));
        node.child2 = Some(Box::new(cut_result.right_node));

        // Теперь горизонтальный разрез в левом дочернем узле
        if let Some(ref mut left_child) = node.child1 {
            let cut_y = left_child.get_y1() + tile_height;
            let cut_result2 = CuttingEngine::split_horizontally(left_child, cut_y)?;
            
            cuts.push(cut_result2.cut);
            left_child.child1 = Some(Box::new(cut_result2.left_node));
            left_child.child2 = Some(Box::new(cut_result2.right_node));

            // Размещаем деталь в левом-верхнем узле
            if let Some(ref mut final_node) = left_child.child1 {
                final_node.is_final = true;
                final_node.external_id = tile_dimensions.id;
                final_node.is_rotated = rotated;
            }
        }

        Ok(())
    }

    /// Только вертикальный разрез - прямая версия
    fn split_vertical_only_direct(
        node: &mut TileNode,
        tile_dimensions: &TileDimensions,
        tile_width: i32,
        rotated: bool,
        cuts: &mut Vec<Cut>,
    ) -> Result<(), CuttingError> {
        let cut_x = node.get_x1() + tile_width;
        let cut_result = CuttingEngine::split_vertically(node, cut_x)?;
        
        cuts.push(cut_result.cut);
        node.child1 = Some(Box::new(cut_result.left_node));
        node.child2 = Some(Box::new(cut_result.right_node));

        // Размещаем деталь в левом дочернем узле
        if let Some(ref mut left_child) = node.child1 {
            left_child.is_final = true;
            left_child.external_id = tile_dimensions.id;
            left_child.is_rotated = rotated;
        }

        Ok(())
    }

    /// Только горизонтальный разрез - прямая версия
    fn split_horizontal_only_direct(
        node: &mut TileNode,
        tile_dimensions: &TileDimensions,
        tile_height: i32,
        rotated: bool,
        cuts: &mut Vec<Cut>,
    ) -> Result<(), CuttingError> {
        let cut_y = node.get_y1() + tile_height;
        let cut_result = CuttingEngine::split_horizontally(node, cut_y)?;
        
        cuts.push(cut_result.cut);
        node.child1 = Some(Box::new(cut_result.left_node));
        node.child2 = Some(Box::new(cut_result.right_node));

        // Размещаем деталь в верхнем дочернем узле
        if let Some(ref mut top_child) = node.child1 {
            top_child.is_final = true;
            top_child.external_id = tile_dimensions.id;
            top_child.is_rotated = rotated;
        }

        Ok(())
    }

    /// Статический поиск узла по координатам
    fn find_node_mut_by_coordinates_static(
        node: &mut TileNode,
        x1: i32,
        x2: i32,
        y1: i32,
        y2: i32,
    ) -> Option<&mut TileNode> {
        if node.get_x1() == x1 && node.get_x2() == x2 && node.get_y1() == y1 && node.get_y2() == y2 {
            return Some(node);
        }

        if let Some(ref mut child1) = node.child1 {
            if let Some(found) = Self::find_node_mut_by_coordinates_static(child1, x1, x2, y1, y2) {
                return Some(found);
            }
        }

        if let Some(ref mut child2) = node.child2 {
            return Self::find_node_mut_by_coordinates_static(child2, x1, x2, y1, y2);
        }

        None
    }



    /// Проверить, есть ли свободные узлы
    pub fn has_unused_nodes(&self) -> bool {
        !self.get_unused_tile_nodes().is_empty()
    }

    /// Получить коэффициент использования материала
    pub fn get_utilization_ratio(&mut self) -> f64 {
        let total_area = self.get_total_area();
        if total_area == 0 {
            return 0.0;
        }
        self.get_used_area() as f64 / total_area as f64
    }

    /// Создать копию мозаики
    pub fn copy(&self) -> Mosaic {
        self.clone()
    }

    /// Получить коэффициент использования площади (неизменяемая версия)
    pub fn get_used_area_ratio(&self) -> f64 {
        let total_area = self.get_total_area();
        if total_area == 0 {
            return 0.0;
        }
        // Используем клон для получения мутабельной версии
        let mut mosaic_clone = self.clone();
        mosaic_clone.get_used_area() as f64 / total_area as f64
    }

    /// Получить финальные тайлы (алиас для get_final_tile_nodes)
    pub fn get_final_tiles(&self) -> Result<Vec<&TileNode>, CuttingError> {
        Ok(self.get_final_tile_nodes())
    }

    /// Получить максимальную глубину дерева
    pub fn get_max_depth(&self) -> i32 {
        self.calculate_max_depth(&self.root_tile_node)
    }

    /// Рекурсивно вычислить максимальную глубину
    fn calculate_max_depth(&self, node: &TileNode) -> i32 {
        if node.child1.is_none() && node.child2.is_none() {
            return 1;
        }

        let mut max_depth = 0;
        if let Some(ref child1) = node.child1 {
            max_depth = max_depth.max(self.calculate_max_depth(child1));
        }
        if let Some(ref child2) = node.child2 {
            max_depth = max_depth.max(self.calculate_max_depth(child2));
        }

        max_depth + 1
    }

    /// Получить набор различных тайлов
    pub fn get_distinct_tile_set(&self) -> Vec<i32> {
        let hashset = self.root_tile_node.get_distinct_tile_set();
        hashset.into_iter().collect()
    }

    /// Получить самую большую площадь среди неиспользуемых узлов
    pub fn get_biggest_area(&self) -> i64 {
        self.get_biggest_unused_tile()
            .map(|tile| tile.get_area())
            .unwrap_or(0)
    }

    /// Получить используемую площадь (неизменяемая версия)
    pub fn get_used_area_immutable(&self) -> i64 {
        let mut mosaic_clone = self.clone();
        mosaic_clone.get_used_area()
    }

    /// Получить неиспользуемую площадь (неизменяемая версия)
    pub fn get_unused_area_immutable(&self) -> i64 {
        let mut mosaic_clone = self.clone();
        mosaic_clone.get_unused_area()
    }
}

impl std::fmt::Display for Mosaic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mosaic[{}x{}, material={}, cuts={}, used_area={}]",
            self.root_tile_node.get_width(),
            self.root_tile_node.get_height(),
            self.material,
            self.cuts.len(),
            // Не можем вызвать get_used_area() здесь, так как нужна мутабельная ссылка
            "N/A"
        )
    }
}
