use crate::types::{next_id, DEFAULT_MATERIAL};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Базовый прямоугольник с координатами
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Tile {
    /// Создать новую плитку из размеров
    pub fn from_dimensions(dimensions: &TileDimensions) -> Self {
        Self {
            x1: 0,
            x2: dimensions.width,
            y1: 0,
            y2: dimensions.height,
        }
    }

    /// Создать новую плитку с координатами
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        Self { x1, x2, y1, y2 }
    }

    /// Получить ширину
    pub fn get_width(&self) -> i32 {
        self.x2 - self.x1
    }

    /// Получить высоту
    pub fn get_height(&self) -> i32 {
        self.y2 - self.y1
    }

    /// Получить площадь
    pub fn get_area(&self) -> i64 {
        (self.get_width() as i64) * (self.get_height() as i64)
    }

    /// Получить максимальную сторону
    pub fn get_max_side(&self) -> i32 {
        self.get_width().max(self.get_height())
    }

    /// Проверить, является ли горизонтальной (ширина > высоты)
    pub fn is_horizontal(&self) -> bool {
        self.get_width() > self.get_height()
    }

    /// Проверить, является ли вертикальной (высота > ширины)
    pub fn is_vertical(&self) -> bool {
        self.get_height() > self.get_width()
    }
}

/// Размеры панели с материалом
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TileDimensions {
    pub id: i32,
    pub width: i32,
    pub height: i32,
    pub material: String,
    pub orientation: i32,
    pub label: Option<String>,
    pub is_rotated: bool,
}

impl TileDimensions {
    /// Создать новые размеры с полными параметрами
    pub fn new(
        id: i32,
        width: i32,
        height: i32,
        material: String,
        orientation: i32,
        label: Option<String>,
    ) -> Self {
        Self {
            id,
            width,
            height,
            material,
            orientation,
            label,
            is_rotated: false,
        }
    }
    /// Проверяет, помещается ли другая плитка в эту
    pub fn fits(&self, other: &TileDimensions) -> bool {
        (self.width >= other.width && self.height >= other.height)
            || (!other.is_square() && self.width >= other.height && self.height >= other.width)
    }

    /// Поворачивает плитку на 90 градусов
    pub fn rotate90(&self) -> Self {
        let mut rotated = self.clone();
        rotated.width = self.height;
        rotated.height = self.width;
        rotated.is_rotated = !self.is_rotated;
        rotated.orientation = if self.orientation == 0 { 1 } else { 0 };
        rotated
    }
    /// Создать новые размеры с полными параметрами включая поворот
    pub fn new_with_rotation(
        id: i32,
        width: i32,
        height: i32,
        material: String,
        orientation: i32,
        label: Option<String>,
        is_rotated: bool,
    ) -> Self {
        Self {
            id,
            width,
            height,
            material,
            orientation,
            label,
            is_rotated,
        }
    }

    /// Создать простые размеры только с шириной и высотой
    pub fn simple(width: i32, height: i32) -> Self {
        Self {
            id: -1,
            width,
            height,
            material: DEFAULT_MATERIAL.to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        }
    }

    /// Получить максимальное измерение
    pub fn get_max_dimension(&self) -> i32 {
        self.width.max(self.height)
    }

    /// Получить площадь
    pub fn get_area(&self) -> i64 {
        (self.width as i64) * (self.height as i64)
    }

    /// Проверить, является ли квадратом
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Проверить, является ли горизонтальной
    pub fn is_horizontal(&self) -> bool {
        self.width > self.height
    }

    /// Проверить, имеет ли те же размеры (с учетом поворота)
    pub fn has_same_dimensions(&self, other: &TileDimensions) -> bool {
        (self.width == other.width && self.height == other.height)
            || (self.width == other.height && self.height == other.width)
    }

    /// Строковое представление размеров
    pub fn dimensions_to_string(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }

    /// Хеш-код на основе размеров
    pub fn dimensions_based_hash_code(&self) -> i32 {
        self.width * 31 + self.height
    }
}

impl std::fmt::Display for TileDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id={}[{}x{}]", self.id, self.width, self.height)
    }
}

impl std::hash::Hash for TileDimensions {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

impl Eq for TileDimensions {}

/// Узел бинарного дерева для представления прямоугольной области
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileNode {
    pub id: i32,
    pub tile: Tile,
    pub child1: Option<Box<TileNode>>,
    pub child2: Option<Box<TileNode>>,
    pub is_final: bool,
    pub is_rotated: bool,
    pub external_id: i32,
    pub is_area_totally_used: bool,
    pub totally_used_area: i64,
}

impl TileNode {
    /// Создать новый узел с координатами
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        Self {
            id: next_id(),
            tile: Tile::new(x1, x2, y1, y2),
            child1: None,
            child2: None,
            is_final: false,
            is_rotated: false,
            external_id: -1,
            is_area_totally_used: false,
            totally_used_area: 0,
        }
    }

    /// Создать новый узел из размеров
    pub fn from_dimensions(dimensions: &TileDimensions) -> Self {
        Self {
            id: next_id(),
            tile: Tile::from_dimensions(dimensions),
            child1: None,
            child2: None,
            is_final: false,
            is_rotated: false,
            external_id: -1,
            is_area_totally_used: false,
            totally_used_area: 0,
        }
    }

    /// Проверить, есть ли дочерние узлы
    pub fn has_children(&self) -> bool {
        self.child1.is_some() || self.child2.is_some()
    }

    /// Найти узел в дереве
    pub fn find_tile(&self, target: &TileNode) -> Option<&TileNode> {
        if self == target {
            return Some(self);
        }

        if let Some(ref child1) = self.child1 {
            if let Some(found) = child1.find_tile(target) {
                return Some(found);
            }
        }

        if let Some(ref child2) = self.child2 {
            return child2.find_tile(target);
        }

        None
    }

    /// Найти узел в дереве (мутабельная версия)
    pub fn find_tile_mut(&mut self, target: &TileNode) -> Option<&mut TileNode> {
        if self == target {
            return Some(self);
        }

        if let Some(ref mut child1) = self.child1 {
            if let Some(found) = child1.find_tile_mut(target) {
                return Some(found);
            }
        }

        if let Some(ref mut child2) = self.child2 {
            return child2.find_tile_mut(target);
        }

        None
    }

    /// Заменить узел в дереве
    pub fn replace_tile(&mut self, new_node: TileNode, target: &TileNode) -> Option<&mut TileNode> {
        if let Some(ref mut child1) = self.child1 {
            if child1.as_ref() == target {
                *child1 = Box::new(new_node);
                return Some(child1.as_mut());
            }
            if let Some(found) = child1.replace_tile(new_node.clone(), target) {
                return Some(found);
            }
        }

        if let Some(ref mut child2) = self.child2 {
            if child2.as_ref() == target {
                *child2 = Box::new(new_node);
                return Some(child2.as_mut());
            }
            return child2.replace_tile(new_node, target);
        }

        None
    }

    /// Получить используемую площадь
    pub fn get_used_area(&mut self) -> i64 {
        if self.is_area_totally_used {
            return self.totally_used_area;
        }

        if self.is_final {
            return self.get_area();
        }

        let mut used_area = 0i64;

        if let Some(ref mut child1) = self.child1 {
            used_area += child1.get_used_area();
        }

        if let Some(ref mut child2) = self.child2 {
            used_area += child2.get_used_area();
        }

        if used_area == self.get_area() {
            self.is_area_totally_used = true;
            self.totally_used_area = self.get_area();
        }

        used_area
    }

    /// Получить неиспользуемые узлы
    pub fn get_unused_tiles(&self) -> Vec<&TileNode> {
        let mut unused = Vec::new();
        self.collect_unused_tiles(&mut unused);
        unused
    }

    /// Собрать неиспользуемые узлы рекурсивно
    fn collect_unused_tiles<'a>(&'a self, unused: &mut Vec<&'a TileNode>) {
        if !self.is_final && self.child1.is_none() && self.child2.is_none() {
            unused.push(self);
        }

        if let Some(ref child1) = self.child1 {
            child1.collect_unused_tiles(unused);
        }

        if let Some(ref child2) = self.child2 {
            child2.collect_unused_tiles(unused);
        }
    }

    /// Получить финальные узлы
    pub fn get_final_tile_nodes(&self) -> Vec<&TileNode> {
        let mut final_nodes = Vec::new();
        self.collect_final_tile_nodes(&mut final_nodes);
        final_nodes
    }

    /// Собрать финальные узлы рекурсивно
    fn collect_final_tile_nodes<'a>(&'a self, final_nodes: &mut Vec<&'a TileNode>) {
        if self.is_final {
            final_nodes.push(self);
        }

        if let Some(ref child1) = self.child1 {
            child1.collect_final_tile_nodes(final_nodes);
        }

        if let Some(ref child2) = self.child2 {
            child2.collect_final_tile_nodes(final_nodes);
        }
    }

    /// Получить неиспользуемую площадь
    pub fn get_unused_area(&mut self) -> i64 {
        self.get_area() - self.get_used_area()
    }

    /// Получить коэффициент использования площади
    pub fn get_used_area_ratio(&mut self) -> f32 {
        let total_area = self.get_area();
        if total_area == 0 {
            0.0
        } else {
            self.get_used_area() as f32 / total_area as f32
        }
    }

    /// Проверить, есть ли финальные узлы
    pub fn has_final(&self) -> bool {
        if self.is_final {
            return true;
        }

        if let Some(ref child1) = self.child1 {
            if child1.has_final() {
                return true;
            }
        }

        if let Some(ref child2) = self.child2 {
            if child2.has_final() {
                return true;
            }
        }

        false
    }

    /// Получить количество неиспользуемых узлов
    pub fn get_nbr_unused_tiles(&self) -> i32 {
        let mut count = 0;

        if !self.is_final && self.child1.is_none() && self.child2.is_none() {
            count += 1;
        }

        if let Some(ref child1) = self.child1 {
            count += child1.get_nbr_unused_tiles();
        }

        if let Some(ref child2) = self.child2 {
            count += child2.get_nbr_unused_tiles();
        }

        count
    }

    /// Получить количество финальных узлов
    pub fn get_nbr_final_tiles(&self) -> i32 {
        let mut count = if self.is_final { 1 } else { 0 };

        if let Some(ref child1) = self.child1 {
            count += child1.get_nbr_final_tiles();
        }

        if let Some(ref child2) = self.child2 {
            count += child2.get_nbr_final_tiles();
        }

        count
    }

    /// Получить самую большую свободную площадь
    pub fn get_biggest_area(&self) -> i64 {
        let mut area = if self.child1.is_none() && self.child2.is_none() && !self.is_final {
            self.get_area()
        } else {
            0
        };

        if let Some(ref child1) = self.child1 {
            area = area.max(child1.get_biggest_area());
        }

        if let Some(ref child2) = self.child2 {
            area = area.max(child2.get_biggest_area());
        }

        area
    }

    /// Получить количество финальных горизонтальных узлов
    pub fn get_nbr_final_horizontal(&self) -> i32 {
        let mut count = if self.is_final && self.is_horizontal() {
            1
        } else {
            0
        };

        if let Some(ref child1) = self.child1 {
            count += child1.get_nbr_final_horizontal();
        }

        if let Some(ref child2) = self.child2 {
            count += child2.get_nbr_final_horizontal();
        }

        count
    }

    /// Получить количество финальных вертикальных узлов
    pub fn get_nbr_final_vertical(&self) -> i32 {
        let mut count = if self.is_final && self.is_vertical() {
            1
        } else {
            0
        };

        if let Some(ref child1) = self.child1 {
            count += child1.get_nbr_final_vertical();
        }

        if let Some(ref child2) = self.child2 {
            count += child2.get_nbr_final_vertical();
        }

        count
    }

    /// Получить набор различных размеров плиток
    pub fn get_distinct_tile_set(&self) -> HashSet<i32> {
        let mut set = HashSet::new();
        self.collect_distinct_tiles(&mut set);
        set
    }

    /// Собрать различные размеры плиток
    fn collect_distinct_tiles(&self, set: &mut HashSet<i32>) {
        if self.is_final {
            let width = self.get_width();
            let height = self.get_height();
            let sum = width + height;
            let hash = ((sum * (sum + 1)) / 2) + height;
            set.insert(hash);
        } else {
            if let Some(ref child1) = self.child1 {
                child1.collect_distinct_tiles(set);
            }

            if let Some(ref child2) = self.child2 {
                child2.collect_distinct_tiles(set);
            }
        }
    }

    /// Преобразовать в TileDimensions
    pub fn to_tile_dimensions(&self) -> TileDimensions {
        TileDimensions::simple(self.get_width(), self.get_height())
    }

    /// Создать строковый идентификатор
    pub fn to_string_identifier(&self) -> String {
        let mut result = String::new();
        self.append_to_string_identifier(&mut result);
        result
    }

    /// Добавить к строковому идентификатору
    fn append_to_string_identifier(&self, sb: &mut String) {
        sb.push_str(&self.tile.x1.to_string());
        sb.push_str(&self.tile.y1.to_string());
        sb.push_str(&self.tile.x2.to_string());
        sb.push_str(&self.tile.y2.to_string());
        sb.push_str(&self.is_final.to_string());

        if let Some(ref child1) = self.child1 {
            child1.append_to_string_identifier(sb);
        }

        if let Some(ref child2) = self.child2 {
            child2.append_to_string_identifier(sb);
        }
    }

    // Делегированные методы от Tile
    pub fn get_x1(&self) -> i32 {
        self.tile.x1
    }

    pub fn get_x2(&self) -> i32 {
        self.tile.x2
    }

    pub fn get_y1(&self) -> i32 {
        self.tile.y1
    }

    pub fn get_y2(&self) -> i32 {
        self.tile.y2
    }

    pub fn get_width(&self) -> i32 {
        self.tile.get_width()
    }

    pub fn get_height(&self) -> i32 {
        self.tile.get_height()
    }

    pub fn get_area(&self) -> i64 {
        self.tile.get_area()
    }

    pub fn get_max_side(&self) -> i32 {
        self.tile.get_max_side()
    }

    pub fn is_horizontal(&self) -> bool {
        self.tile.is_horizontal()
    }

    pub fn is_vertical(&self) -> bool {
        self.tile.is_vertical()
    }
}

impl PartialEq for TileNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.tile == other.tile
            && self.is_final == other.is_final
            && self.child1 == other.child1
            && self.child2 == other.child2
    }
}

impl std::fmt::Display for TileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.append_to_string(""))
    }
}

impl TileNode {
    /// Добавить к строковому представлению с отступом
    pub fn append_to_string(&self, indent: &str) -> String {
        let mut result = format!(
            "\n{}({}, {})({}, {})",
            indent,
            self.get_x1(),
            self.get_y1(),
            self.get_x2(),
            self.get_y2()
        );

        if self.is_final {
            result.push('*');
        }

        if let Some(ref child1) = self.child1 {
            let new_indent = format!("{}    ", indent);
            result.push_str(&child1.append_to_string(&new_indent));
        }

        if let Some(ref child2) = self.child2 {
            let new_indent = format!("{}    ", indent);
            result.push_str(&child2.append_to_string(&new_indent));
        }

        result
    }
}
