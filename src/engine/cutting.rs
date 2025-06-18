use crate::engine::model::tile::{TileNode, TileDimensions};
use crate::engine::model::cut::{Cut, CutBuilder};
use crate::error::CuttingError;

/// Результат операции разрезания
#[derive(Debug)]
pub struct CutResult {
    pub left_node: TileNode,
    pub right_node: TileNode,
    pub cut: Cut,
}

/// Основной класс для алгоритмов разрезания
pub struct CuttingEngine;

impl CuttingEngine {
    /// Горизонтальный разрез узла
    pub fn split_horizontally(
        node: &TileNode,
        cut_position: i32,
    ) -> Result<CutResult, CuttingError> {
        // Проверяем валидность разреза
        if cut_position <= node.get_y1() || cut_position >= node.get_y2() {
            return Err(CuttingError::InvalidCutPosition {
                position: cut_position,
                min: node.get_y1(),
                max: node.get_y2(),
            });
        }

        // Создаем верхний узел (левый в результате)
        let top_node = TileNode::new(
            node.get_x1(),
            node.get_x2(),
            node.get_y1(),
            cut_position,
        );

        // Создаем нижний узел (правый в результате)
        let bottom_node = TileNode::new(
            node.get_x1(),
            node.get_x2(),
            cut_position,
            node.get_y2(),
        );

        // Создаем объект разреза используя CutBuilder
        let cut = CutBuilder::new()
            .set_x1(node.get_x1())
            .set_y1(cut_position)
            .set_x2(node.get_x2())
            .set_y2(cut_position)
            .set_original_width(node.get_width())
            .set_original_height(node.get_height())
            .set_horizontal(true)
            .set_cut_coords(cut_position)
            .set_original_tile_id(node.id)
            .set_child1_tile_id(top_node.id)
            .set_child2_tile_id(bottom_node.id)
            .build();

        Ok(CutResult {
            left_node: top_node,
            right_node: bottom_node,
            cut,
        })
    }

    /// Вертикальный разрез узла
    pub fn split_vertically(
        node: &TileNode,
        cut_position: i32,
    ) -> Result<CutResult, CuttingError> {
        // Проверяем валидность разреза
        if cut_position <= node.get_x1() || cut_position >= node.get_x2() {
            return Err(CuttingError::InvalidCutPosition {
                position: cut_position,
                min: node.get_x1(),
                max: node.get_x2(),
            });
        }

        // Создаем левый узел
        let left_node = TileNode::new(
            node.get_x1(),
            cut_position,
            node.get_y1(),
            node.get_y2(),
        );

        // Создаем правый узел
        let right_node = TileNode::new(
            cut_position,
            node.get_x2(),
            node.get_y1(),
            node.get_y2(),
        );

        // Создаем объект разреза используя CutBuilder
        let cut = CutBuilder::new()
            .set_x1(cut_position)
            .set_y1(node.get_y1())
            .set_x2(cut_position)
            .set_y2(node.get_y2())
            .set_original_width(node.get_width())
            .set_original_height(node.get_height())
            .set_horizontal(false)
            .set_cut_coords(cut_position)
            .set_original_tile_id(node.id)
            .set_child1_tile_id(left_node.id)
            .set_child2_tile_id(right_node.id)
            .build();

        Ok(CutResult {
            left_node,
            right_node,
            cut,
        })
    }

    /// Попытка разместить плитку в узле
    /// Возвращает новый узел с размещенной плиткой или None если не помещается
    pub fn try_place_tile(
        node: &mut TileNode,
        tile_dimensions: &TileDimensions,
    ) -> Result<bool, CuttingError> {
        // Проверяем, что узел свободен
        if node.is_final || node.has_children() {
            return Ok(false);
        }

        // Проверяем, помещается ли плитка точно
        if node.get_width() == tile_dimensions.width && node.get_height() == tile_dimensions.height {
            node.is_final = true;
            node.external_id = tile_dimensions.id;
            return Ok(true);
        }

        // Проверяем, помещается ли плитка с поворотом (если не квадрат)
        if !tile_dimensions.is_square() 
            && node.get_width() == tile_dimensions.height 
            && node.get_height() == tile_dimensions.width {
            node.is_final = true;
            node.external_id = tile_dimensions.id;
            node.is_rotated = true;
            return Ok(true);
        }

        // Проверяем, помещается ли плитка с возможностью разреза
        if node.get_width() >= tile_dimensions.width && node.get_height() >= tile_dimensions.height {
            return Self::place_tile_with_cuts(node, tile_dimensions, false);
        }

        // Проверяем с поворотом
        if !tile_dimensions.is_square() 
            && node.get_width() >= tile_dimensions.height 
            && node.get_height() >= tile_dimensions.width {
            return Self::place_tile_with_cuts(node, tile_dimensions, true);
        }

        Ok(false)
    }

    /// Размещение плитки с разрезами
    fn place_tile_with_cuts(
        node: &mut TileNode,
        tile_dimensions: &TileDimensions,
        rotated: bool,
    ) -> Result<bool, CuttingError> {
        let (tile_width, tile_height) = if rotated {
            (tile_dimensions.height, tile_dimensions.width)
        } else {
            (tile_dimensions.width, tile_dimensions.height)
        };

        // Определяем, какой разрез делать первым
        let width_diff = node.get_width() - tile_width;
        let height_diff = node.get_height() - tile_height;

        if width_diff > 0 && height_diff > 0 {
            // Нужны оба разреза, выбираем оптимальный порядок
            if width_diff >= height_diff {
                // Сначала вертикальный разрез
                Self::split_and_place_vertical_first(node, tile_dimensions, tile_width, tile_height, rotated)
            } else {
                // Сначала горизонтальный разрез
                Self::split_and_place_horizontal_first(node, tile_dimensions, tile_width, tile_height, rotated)
            }
        } else if width_diff > 0 {
            // Только вертикальный разрез
            Self::split_and_place_vertical_only(node, tile_dimensions, tile_width, rotated)
        } else if height_diff > 0 {
            // Только горизонтальный разрез
            Self::split_and_place_horizontal_only(node, tile_dimensions, tile_height, rotated)
        } else {
            // Точное совпадение
            node.is_final = true;
            node.external_id = tile_dimensions.id;
            node.is_rotated = rotated;
            Ok(true)
        }
    }

    /// Вертикальный разрез с последующим размещением
    fn split_and_place_vertical_first(
        node: &mut TileNode,
        tile_dimensions: &TileDimensions,
        tile_width: i32,
        _tile_height: i32,
        _rotated: bool,
    ) -> Result<bool, CuttingError> {
        let cut_position = node.get_x1() + tile_width;
        let cut_result = Self::split_vertically(node, cut_position)?;
        
        node.child1 = Some(Box::new(cut_result.left_node));
        node.child2 = Some(Box::new(cut_result.right_node));

        // Размещаем плитку в левом дочернем узле
        if let Some(ref mut left_child) = node.child1 {
            Self::try_place_tile(left_child, tile_dimensions)?;
        }

        Ok(true)
    }

    /// Горизонтальный разрез с последующим размещением
    fn split_and_place_horizontal_first(
        node: &mut TileNode,
        tile_dimensions: &TileDimensions,
        _tile_width: i32,
        tile_height: i32,
        _rotated: bool,
    ) -> Result<bool, CuttingError> {
        let cut_position = node.get_y1() + tile_height;
        let cut_result = Self::split_horizontally(node, cut_position)?;
        
        node.child1 = Some(Box::new(cut_result.left_node));
        node.child2 = Some(Box::new(cut_result.right_node));

        // Размещаем плитку в верхнем дочернем узле
        if let Some(ref mut top_child) = node.child1 {
            Self::try_place_tile(top_child, tile_dimensions)?;
        }

        Ok(true)
    }

    /// Только вертикальный разрез
    fn split_and_place_vertical_only(
        node: &mut TileNode,
        tile_dimensions: &TileDimensions,
        tile_width: i32,
        rotated: bool,
    ) -> Result<bool, CuttingError> {
        let cut_position = node.get_x1() + tile_width;
        let cut_result = Self::split_vertically(node, cut_position)?;
        
        node.child1 = Some(Box::new(cut_result.left_node));
        node.child2 = Some(Box::new(cut_result.right_node));

        // Размещаем плитку в левом дочернем узле
        if let Some(ref mut left_child) = node.child1 {
            left_child.is_final = true;
            left_child.external_id = tile_dimensions.id;
            left_child.is_rotated = rotated;
        }

        Ok(true)
    }

    /// Только горизонтальный разрез
    fn split_and_place_horizontal_only(
        node: &mut TileNode,
        tile_dimensions: &TileDimensions,
        tile_height: i32,
        rotated: bool,
    ) -> Result<bool, CuttingError> {
        let cut_position = node.get_y1() + tile_height;
        let cut_result = Self::split_horizontally(node, cut_position)?;
        
        node.child1 = Some(Box::new(cut_result.left_node));
        node.child2 = Some(Box::new(cut_result.right_node));

        // Размещаем плитку в верхнем дочернем узле
        if let Some(ref mut top_child) = node.child1 {
            top_child.is_final = true;
            top_child.external_id = tile_dimensions.id;
            top_child.is_rotated = rotated;
        }

        Ok(true)
    }

    /// Найти лучший узел для размещения плитки
    pub fn find_best_fit_node<'a>(
        root: &'a TileNode,
        tile_dimensions: &TileDimensions,
    ) -> Option<&'a TileNode> {
        let mut best_node: Option<&TileNode> = None;
        let mut best_area = i64::MAX;

        Self::find_best_fit_recursive(root, tile_dimensions, &mut best_node, &mut best_area);
        best_node
    }

    /// Рекурсивный поиск лучшего узла
    fn find_best_fit_recursive<'a>(
        node: &'a TileNode,
        tile_dimensions: &TileDimensions,
        best_node: &mut Option<&'a TileNode>,
        best_area: &mut i64,
    ) {
        // Проверяем только свободные узлы
        if node.is_final || node.has_children() {
            if let Some(ref child1) = node.child1 {
                Self::find_best_fit_recursive(child1, tile_dimensions, best_node, best_area);
            }
            if let Some(ref child2) = node.child2 {
                Self::find_best_fit_recursive(child2, tile_dimensions, best_node, best_area);
            }
            return;
        }

        // Проверяем, помещается ли плитка
        let fits_normal = node.get_width() >= tile_dimensions.width 
            && node.get_height() >= tile_dimensions.height;
        let fits_rotated = !tile_dimensions.is_square() 
            && node.get_width() >= tile_dimensions.height 
            && node.get_height() >= tile_dimensions.width;

        if fits_normal || fits_rotated {
            let node_area = node.get_area();
            if node_area < *best_area {
                *best_area = node_area;
                *best_node = Some(node);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_horizontally() {
        let node = TileNode::new(0, 100, 0, 100);
        let result = CuttingEngine::split_horizontally(&node, 50).unwrap();

        assert_eq!(result.left_node.get_y1(), 0);
        assert_eq!(result.left_node.get_y2(), 50);
        assert_eq!(result.right_node.get_y1(), 50);
        assert_eq!(result.right_node.get_y2(), 100);
        assert!(result.cut.get_is_horizontal());
    }

    #[test]
    fn test_split_vertically() {
        let node = TileNode::new(0, 100, 0, 100);
        let result = CuttingEngine::split_vertically(&node, 50).unwrap();

        assert_eq!(result.left_node.get_x1(), 0);
        assert_eq!(result.left_node.get_x2(), 50);
        assert_eq!(result.right_node.get_x1(), 50);
        assert_eq!(result.right_node.get_x2(), 100);
        assert!(!result.cut.get_is_horizontal());
    }

    #[test]
    fn test_invalid_cut_position() {
        let node = TileNode::new(0, 100, 0, 100);
        
        // Тест невалидной позиции для горизонтального разреза
        let result = CuttingEngine::split_horizontally(&node, 0);
        assert!(result.is_err());
        
        let result = CuttingEngine::split_horizontally(&node, 100);
        assert!(result.is_err());

        // Тест невалидной позиции для вертикального разреза
        let result = CuttingEngine::split_vertically(&node, 0);
        assert!(result.is_err());
        
        let result = CuttingEngine::split_vertically(&node, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_place_tile_exact_fit() {
        let mut node = TileNode::new(0, 50, 0, 30);
        let tile = TileDimensions::simple(50, 30);
        
        let result = CuttingEngine::try_place_tile(&mut node, &tile).unwrap();
        assert!(result);
        assert!(node.is_final);
        assert_eq!(node.external_id, tile.id);
    }

    #[test]
    fn test_place_tile_with_rotation() {
        let mut node = TileNode::new(0, 30, 0, 50);
        let tile = TileDimensions::simple(50, 30);
        
        let result = CuttingEngine::try_place_tile(&mut node, &tile).unwrap();
        assert!(result);
        assert!(node.is_final);
        assert!(node.is_rotated);
    }

    #[test]
    fn test_find_best_fit_node() {
        let root = TileNode::new(0, 100, 0, 100);
        let tile = TileDimensions::simple(50, 30);
        
        let best_node = CuttingEngine::find_best_fit_node(&root, &tile);
        assert!(best_node.is_some());
        assert_eq!(best_node.unwrap().get_area(), 10000);
    }
}
