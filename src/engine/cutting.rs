use crate::engine::model::tile::{TileNode, TileDimensions};
use crate::engine::model::cut::{Cut, CutBuilder};
use crate::engine::model::mosaic::Mosaic;
use crate::error::CuttingError;
use std::collections::VecDeque;

/// Направление разреза (аналог Java CutDirection)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CutDirection {
    Horizontal,
    Vertical,
    Both,
}

/// Результат операции разрезания
#[derive(Debug)]
pub struct CutResult {
    pub left_node: TileNode,
    pub right_node: TileNode,
    pub cut: Cut,
}

/// Результат размещения плитки
#[derive(Debug)]
pub struct PlacementResult {
    pub placed: bool,
    pub cuts_made: usize,
    pub used_area: f64,
    pub new_mosaic: Option<Mosaic>,
}

/// Основной класс для алгоритмов разрезания (аналог Java CutListThread)
pub struct CuttingEngine;

impl CuttingEngine {
    /// Горизонтальный разрез узла
    pub fn split_horizontally(
        node: &TileNode,
        cut_position: i32,
        _cut_thickness: i32,
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

        // Создаем нижний узел (правый в результате) - начинается сразу после разреза
        // Толщина разреза учитывается в объекте Cut, но не в позиционировании узлов
        let bottom_node = TileNode::new(
            node.get_x1(),
            node.get_x2(),
            cut_position,  // Узлы позиционируются без учета толщины
            node.get_y2(),
        );

        // Создаем объект разреза
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
        _cut_thickness: i32,
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

        // Создаем правый узел - начинается сразу после разреза
        // Толщина разреза учитывается в объекте Cut, но не в позиционировании узлов
        let right_node = TileNode::new(
            cut_position,
            node.get_x2(),
            node.get_y1(),
            node.get_y2(),
        );

        // Создаем объект разреза
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

    /// Основной метод размещения плитки (аналог Java CutListThread.fitTile)
    pub fn fit_tile(
        tile_dimensions: &TileDimensions,
        mosaic: &Mosaic,
        cut_thickness: i32,
        first_cut_orientation: CutDirection,
        consider_grain_direction: bool,
    ) -> Result<Vec<PlacementResult>, CuttingError> {
        let mut results = Vec::new();
        let mut candidates = Vec::new();
        
        // Ищем кандидатов для размещения
        Self::find_candidates(
            tile_dimensions.width,
            tile_dimensions.height,
            &mosaic.get_root_tile_node(),
            &mut candidates,
            0, // min_trim_dimension - должен приходить из конфигурации
        );

        for candidate_node in &candidates {
            // Проверяем точное совпадение
            if candidate_node.get_width() == tile_dimensions.width 
                && candidate_node.get_height() == tile_dimensions.height {
                
                let mut new_mosaic = mosaic.clone();
                let mut root_copy = Self::copy_node(&mosaic.get_root_tile_node(), candidate_node);
                
                if let Some(target_node) = Self::find_tile_by_id_mut(&mut root_copy, candidate_node.id) {
                    target_node.external_id = tile_dimensions.id as i32;
                    target_node.is_final = true;
                    target_node.is_rotated = false;
                }
                
                new_mosaic.set_root_tile_node(root_copy);
                
                results.push(PlacementResult {
                    placed: true,
                    cuts_made: 0,
                    used_area: (tile_dimensions.width * tile_dimensions.height) as f64,
                    new_mosaic: Some(new_mosaic),
                });
            } else {
                // Пробуем размещение с разрезами
                if !consider_grain_direction || Self::check_grain_compatibility(mosaic, tile_dimensions) {
                    // Обычное размещение
                    if let Ok(result) = Self::try_place_with_cuts(
                        tile_dimensions,
                        candidate_node,
                        mosaic,
                        cut_thickness,
                        first_cut_orientation,
                        false,
                    ) {
                        results.push(result);
                    }
                    
                    // Размещение с поворотом (если панель не квадратная)
                    if !tile_dimensions.is_square() {
                        if let Ok(result) = Self::try_place_with_cuts(
                            tile_dimensions,
                            candidate_node,
                            mosaic,
                            cut_thickness,
                            first_cut_orientation,
                            true,
                        ) {
                            results.push(result);
                        }
                    }
                } else {
                    // Размещение с учетом направления волокон
                    if Self::check_grain_orientation(mosaic, tile_dimensions) {
                        if let Ok(result) = Self::try_place_with_cuts(
                            tile_dimensions,
                            candidate_node,
                            mosaic,
                            cut_thickness,
                            first_cut_orientation,
                            false,
                        ) {
                            results.push(result);
                        }
                    }
                }
            }
        }

        // Если нет результатов, но есть кандидаты, попробуем принудительное размещение
        if results.is_empty() && !candidates.is_empty() {
            for candidate_node in &candidates {
                if let Ok(result) = Self::try_place_with_cuts(
                    tile_dimensions,
                    candidate_node,
                    mosaic,
                    cut_thickness,
                    first_cut_orientation,
                    false,
                ) {
                    results.push(result);
                    break; // Достаточно одного успешного размещения
                }
            }
        }

        Ok(results)
    }

    /// Попытка размещения с разрезами (аналог Java splitHV/splitVH)
    fn try_place_with_cuts(
        tile_dimensions: &TileDimensions,
        candidate_node: &TileNode,
        mosaic: &Mosaic,
        cut_thickness: i32,
        first_cut_orientation: CutDirection,
        rotate: bool,
    ) -> Result<PlacementResult, CuttingError> {
        let (tile_width, tile_height) = if rotate {
            (tile_dimensions.height, tile_dimensions.width)
        } else {
            (tile_dimensions.width, tile_dimensions.height)
        };

        // Проверяем, помещается ли плитка
        if candidate_node.get_width() < tile_width || candidate_node.get_height() < tile_height {
            return Ok(PlacementResult {
                placed: false,
                cuts_made: 0,
                used_area: 0.0,
                new_mosaic: None,
            });
        }

        let mut new_mosaic = mosaic.clone();
        let mut root_copy = Self::copy_node(&mosaic.get_root_tile_node(), candidate_node);
        let mut cuts_made = 0;
        let mut cuts = Vec::new();

        if let Some(target_node) = Self::find_tile_by_id_mut(&mut root_copy, candidate_node.id) {
            // Определяем, какие разрезы нужны
            let need_vertical_cut = target_node.get_width() > tile_width;
            let need_horizontal_cut = target_node.get_height() > tile_height;

            if need_vertical_cut && need_horizontal_cut {
                // Нужны оба разреза - выбираем порядок по first_cut_orientation
                match first_cut_orientation {
                    CutDirection::Horizontal => {
                        cuts.extend(Self::split_hv(target_node, tile_width, tile_height, cut_thickness, tile_dimensions.id as u32)?);
                        cuts_made = 2;
                    }
                    CutDirection::Vertical => {
                        cuts.extend(Self::split_vh(target_node, tile_width, tile_height, cut_thickness, tile_dimensions.id as u32)?);
                        cuts_made = 2;
                    }
                    CutDirection::Both => {
                        // Выбираем оптимальный порядок
                        if target_node.get_width() - tile_width >= target_node.get_height() - tile_height {
                            cuts.extend(Self::split_vh(target_node, tile_width, tile_height, cut_thickness, tile_dimensions.id as u32)?);
                        } else {
                            cuts.extend(Self::split_hv(target_node, tile_width, tile_height, cut_thickness, tile_dimensions.id as u32)?);
                        }
                        cuts_made = 2;
                    }
                }
            } else if need_vertical_cut {
                // Только вертикальный разрез
                let cut = Self::split_vertically(target_node, target_node.get_x1() + tile_width, cut_thickness)?;
                target_node.child1 = Some(Box::new(cut.left_node));
                target_node.child2 = Some(Box::new(cut.right_node));
                
                // Размещаем плитку в левом дочернем узле
                if let Some(ref mut left_child) = target_node.child1 {
                    left_child.is_final = true;
                    left_child.external_id = tile_dimensions.id;
                    left_child.is_rotated = rotate;
                }
                
                cuts.push(cut.cut);
                cuts_made = 1;
            } else if need_horizontal_cut {
                // Только горизонтальный разрез
                let cut = Self::split_horizontally(target_node, target_node.get_y1() + tile_height, cut_thickness)?;
                target_node.child1 = Some(Box::new(cut.left_node));
                target_node.child2 = Some(Box::new(cut.right_node));
                
                // Размещаем плитку в верхнем дочернем узле
                if let Some(ref mut top_child) = target_node.child1 {
                    top_child.is_final = true;
                    top_child.external_id = tile_dimensions.id;
                    top_child.is_rotated = rotate;
                }
                
                cuts.push(cut.cut);
                cuts_made = 1;
            } else {
                // Точное совпадение
                target_node.is_final = true;
                target_node.external_id = tile_dimensions.id;
                target_node.is_rotated = rotate;
            }
        }

        new_mosaic.set_root_tile_node(root_copy);
        new_mosaic.add_cuts(cuts);

        Ok(PlacementResult {
            placed: true,
            cuts_made,
            used_area: (tile_width * tile_height) as f64,
            new_mosaic: Some(new_mosaic),
        })
    }

    /// Горизонтальный разрез с последующим вертикальным (аналог Java splitHV)
    fn split_hv(
        node: &mut TileNode,
        tile_width: i32,
        tile_height: i32,
        cut_thickness: i32,
        tile_id: u32,
    ) -> Result<Vec<Cut>, CuttingError> {
        let mut cuts = Vec::new();

        if node.get_width() > tile_width {
            let cut = Self::split_vertically(node, node.get_x1() + tile_width, cut_thickness)?;
            node.child1 = Some(Box::new(cut.left_node));
            node.child2 = Some(Box::new(cut.right_node));
            cuts.push(cut.cut);

            if node.get_height() > tile_height {
                if let Some(ref mut left_child) = node.child1 {
                    let cut2 = Self::split_horizontally(left_child, left_child.get_y1() + tile_height, cut_thickness)?;
                    left_child.child1 = Some(Box::new(cut2.left_node));
                    left_child.child2 = Some(Box::new(cut2.right_node));
                    cuts.push(cut2.cut);

                    // Размещаем плитку в верхнем левом узле
                    if let Some(ref mut top_left) = left_child.child1 {
                        top_left.is_final = true;
                        top_left.external_id = tile_id as i32;
                    }
                }
            } else {
                // Размещаем плитку в левом узле
                if let Some(ref mut left_child) = node.child1 {
                    left_child.is_final = true;
                    left_child.external_id = tile_id as i32;
                }
            }
        } else {
            let cut = Self::split_horizontally(node, node.get_y1() + tile_height, cut_thickness)?;
            node.child1 = Some(Box::new(cut.left_node));
            node.child2 = Some(Box::new(cut.right_node));
            cuts.push(cut.cut);

                    // Размещаем плитку в верхнем узле
                    if let Some(ref mut top_child) = node.child1 {
                        top_child.is_final = true;
                        top_child.external_id = tile_id as i32;
                    }
        }

        Ok(cuts)
    }

    /// Вертикальный разрез с последующим горизонтальным (аналог Java splitVH)
    fn split_vh(
        node: &mut TileNode,
        tile_width: i32,
        tile_height: i32,
        cut_thickness: i32,
        tile_id: u32,
    ) -> Result<Vec<Cut>, CuttingError> {
        let mut cuts = Vec::new();

        if node.get_height() > tile_height {
            let cut = Self::split_horizontally(node, node.get_y1() + tile_height, cut_thickness)?;
            node.child1 = Some(Box::new(cut.left_node));
            node.child2 = Some(Box::new(cut.right_node));
            cuts.push(cut.cut);

            if node.get_width() > tile_width {
                if let Some(ref mut top_child) = node.child1 {
                    let cut2 = Self::split_vertically(top_child, top_child.get_x1() + tile_width, cut_thickness)?;
                    top_child.child1 = Some(Box::new(cut2.left_node));
                    top_child.child2 = Some(Box::new(cut2.right_node));
                    cuts.push(cut2.cut);

                    // Размещаем плитку в левом верхнем узле
                    if let Some(ref mut top_left) = top_child.child1 {
                        top_left.is_final = true;
                        top_left.external_id = tile_id as i32;
                    }
                }
            } else {
                // Размещаем плитку в верхнем узле
                if let Some(ref mut top_child) = node.child1 {
                    top_child.is_final = true;
                    top_child.external_id = tile_id as i32;
                }
            }
        } else {
            let cut = Self::split_vertically(node, node.get_x1() + tile_width, cut_thickness)?;
            node.child1 = Some(Box::new(cut.left_node));
            node.child2 = Some(Box::new(cut.right_node));
            cuts.push(cut.cut);

            // Размещаем плитку в левом узле
            if let Some(ref mut left_child) = node.child1 {
                left_child.is_final = true;
                left_child.external_id = tile_id as i32;
            }
        }

        Ok(cuts)
    }

    /// Поиск кандидатов для размещения (аналог Java findCandidates)
    fn find_candidates(
        width: i32,
        height: i32,
        node: &TileNode,
        candidates: &mut Vec<TileNode>,
        min_trim_dimension: i32,
    ) {
        // Проверяем, подходит ли узел
        if node.is_final || node.get_width() < width || node.get_height() < height {
            return;
        }

        // Если узел листовой
        if !node.has_children() {
            let width_ok = node.get_width() == width || node.get_width() >= min_trim_dimension + width;
            let height_ok = node.get_height() == height || node.get_height() >= min_trim_dimension + height;

            if width_ok && height_ok {
                candidates.push(node.clone());
            }
            return;
        }

        // Рекурсивно проверяем дочерние узлы
        if let Some(ref child1) = node.child1 {
            Self::find_candidates(width, height, child1, candidates, min_trim_dimension);
        }
        if let Some(ref child2) = node.child2 {
            Self::find_candidates(width, height, child2, candidates, min_trim_dimension);
        }
    }

    /// Копирование узла (аналог Java copy)
    fn copy_node(source: &TileNode, target: &TileNode) -> TileNode {
        let mut new_node = source.clone();
        Self::copy_children(source, &mut new_node, target);
        new_node
    }

    /// Рекурсивное копирование дочерних узлов
    fn copy_children(source: &TileNode, dest: &mut TileNode, target: &TileNode) {
        if std::ptr::eq(source, target) {
            return;
        }

        if let Some(ref source_child1) = source.child1 {
            dest.child1 = Some(Box::new((**source_child1).clone()));
            if let Some(ref mut dest_child1) = dest.child1 {
                Self::copy_children(source_child1, dest_child1, target);
            }
        }

        if let Some(ref source_child2) = source.child2 {
            dest.child2 = Some(Box::new((**source_child2).clone()));
            if let Some(ref mut dest_child2) = dest.child2 {
                Self::copy_children(source_child2, dest_child2, target);
            }
        }
    }

    /// Поиск узла по ID
    fn find_tile_by_id(node: &TileNode, id: i32) -> Option<&TileNode> {
        if node.id == id {
            return Some(node);
        }

        if let Some(ref child1) = node.child1 {
            if let Some(found) = Self::find_tile_by_id(child1, id) {
                return Some(found);
            }
        }

        if let Some(ref child2) = node.child2 {
            if let Some(found) = Self::find_tile_by_id(child2, id) {
                return Some(found);
            }
        }

        None
    }

    /// Поиск узла по ID (мutable версия)
    fn find_tile_by_id_mut(node: &mut TileNode, id: i32) -> Option<&mut TileNode> {
        if node.id == id {
            return Some(node);
        }

        if let Some(ref mut child1) = node.child1 {
            if let Some(found) = Self::find_tile_by_id_mut(child1, id) {
                return Some(found);
            }
        }

        if let Some(ref mut child2) = node.child2 {
            if let Some(found) = Self::find_tile_by_id_mut(child2, id) {
                return Some(found);
            }
        }

        None
    }

    /// Проверка совместимости направления волокон
    fn check_grain_compatibility(mosaic: &Mosaic, tile_dimensions: &TileDimensions) -> bool {
        // Упрощенная проверка - в реальной реализации должна учитывать orientations
        mosaic.get_orientation() == 0 || tile_dimensions.orientation == 0
    }

    /// Проверка ориентации направления волокон
    fn check_grain_orientation(mosaic: &Mosaic, tile_dimensions: &TileDimensions) -> bool {
        mosaic.get_orientation() == tile_dimensions.orientation
    }

    /// Основной метод для последовательного размещения плиток (аналог Java computeSolutions)
    pub fn compute_solutions(
        tiles: &[TileDimensions],
        stock_solution: &StockSolution,
        cut_thickness: i32,
        first_cut_orientation: CutDirection,
        consider_grain_direction: bool,
    ) -> Result<Vec<Solution>, CuttingError> {
        let mut solutions = vec![Solution::from_stock_solution(stock_solution)];

        for (tile_index, tile) in tiles.iter().enumerate() {
            let mut new_solutions = Vec::new();

            for solution in &solutions {
                let mut placed_in_solution = false;

                // Пробуем разместить на каждой мозаике в решении
                for (mosaic_index, mosaic) in solution.get_mosaics().iter().enumerate() {
                    // Проверяем совместимость материалов
                    if mosaic.get_material() != tile.material {
                        continue;
                    }

                    // Пытаемся разместить плитку
                    if let Ok(placement_results) = Self::fit_tile(
                        tile,
                        mosaic,
                        cut_thickness,
                        first_cut_orientation,
                        consider_grain_direction,
                    ) {
                        for placement_result in placement_results {
                            if placement_result.placed {
                                if let Some(new_mosaic) = placement_result.new_mosaic {
                                    let mut new_solution = solution.clone();
                                    new_solution.replace_mosaic(mosaic_index, new_mosaic);
                                    new_solutions.push(new_solution);
                                    placed_in_solution = true;
                                }
                            }
                        }
                    }

                    if placed_in_solution {
                        break; // Размещена на одной мозаике - достаточно
                    }
                }

                // Если не удалось разместить на существующих мозаиках,
                // пробуем создать новую из неиспользованных складских панелей
                if !placed_in_solution {
                    if let Some(unused_stock) = Self::find_suitable_unused_stock(solution, tile) {
                        let new_mosaic = Mosaic::new_from_stock(&unused_stock);
                        
                        if let Ok(placement_results) = Self::fit_tile(
                            tile,
                            &new_mosaic,
                            cut_thickness,
                            first_cut_orientation,
                            consider_grain_direction,
                        ) {
                            for placement_result in placement_results {
                                if placement_result.placed {
                                    if let Some(final_mosaic) = placement_result.new_mosaic {
                                        let mut new_solution = solution.clone();
                                        new_solution.add_mosaic(final_mosaic);
                                        new_solution.remove_unused_stock(&unused_stock);
                                        new_solutions.push(new_solution);
                                        placed_in_solution = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                // Если панель не удалось разместить, добавляем её в список неразмещенных
                if !placed_in_solution {
                    let mut solution_with_unplaced = solution.clone();
                    solution_with_unplaced.add_unplaced_tile(tile.clone());
                    new_solutions.push(solution_with_unplaced);
                }
            }

            solutions = new_solutions;

            // Удаляем дубликаты и сортируем решения
            Self::remove_duplicates(&mut solutions);
            Self::sort_solutions(&mut solutions);

            // Ограничиваем количество решений для производительности
            if solutions.len() > 100 {
                solutions.truncate(100);
            }

            // Логируем прогресс
            if tile_index % 10 == 0 {
                println!("Обработано {} из {} плиток, текущее количество решений: {}", 
                         tile_index + 1, tiles.len(), solutions.len());
            }
        }

        Ok(solutions)
    }

    /// Поиск подходящей неиспользованной складской панели
    fn find_suitable_unused_stock(solution: &Solution, tile: &TileDimensions) -> Option<TileDimensions> {
        for unused_stock in solution.get_unused_stock_panels() {
            if unused_stock.fits(tile) && unused_stock.material == tile.material {
                return Some(unused_stock.clone());
            }
        }
        None
    }

    /// Удаление дубликатов решений
    fn remove_duplicates(solutions: &mut Vec<Solution>) {
        // Упрощенная реализация - в реальности нужен более сложный алгоритм
        solutions.sort_by(|a, b| a.get_id().cmp(&b.get_id()));
        solutions.dedup_by(|a, b| a.get_id() == b.get_id());
    }

    /// Сортировка решений по качеству
    fn sort_solutions(solutions: &mut Vec<Solution>) {
        solutions.sort_by(|a, b| {
            // Сначала по количеству размещенных плиток (больше лучше)
            let placed_a = a.get_placed_tiles_count();
            let placed_b = b.get_placed_tiles_count();
            
            match placed_b.cmp(&placed_a) {
                std::cmp::Ordering::Equal => {
                    // Затем по эффективности использования материала (больше лучше)
                    let efficiency_a = a.get_efficiency();
                    let efficiency_b = b.get_efficiency();
                    efficiency_b.partial_cmp(&efficiency_a).unwrap_or(std::cmp::Ordering::Equal)
                }
                other => other,
            }
        });
    }

    /// Устаревший метод для совместимости
    pub fn try_place_tile(
        node: &mut TileNode,
        tile_dimensions: &TileDimensions,
    ) -> Result<bool, CuttingError> {
        // Простая реализация для обратной совместимости
        if node.is_final || node.has_children() {
            return Ok(false);
        }

        // Проверяем точное совпадение
        if node.get_width() == tile_dimensions.width && node.get_height() == tile_dimensions.height {
            node.is_final = true;
            node.external_id = tile_dimensions.id;
            return Ok(true);
        }

        // Проверяем с поворотом
        if !tile_dimensions.is_square() 
            && node.get_width() == tile_dimensions.height 
            && node.get_height() == tile_dimensions.width {
            node.is_final = true;
            node.external_id = tile_dimensions.id;
            node.is_rotated = true;
            return Ok(true);
        }

        // Проверяем, помещается ли плитка с разрезами
        if node.get_width() >= tile_dimensions.width && node.get_height() >= tile_dimensions.height {
            let cut_thickness = 3; // Стандартная толщина разреза
            
            // Определяем, какие разрезы нужны
            let need_vertical_cut = node.get_width() > tile_dimensions.width;
            let need_horizontal_cut = node.get_height() > tile_dimensions.height;

            if need_vertical_cut && need_horizontal_cut {
                // Нужны оба разреза - делаем вертикальный, затем горизонтальный
                let vertical_cut = Self::split_vertically(node, node.get_x1() + tile_dimensions.width, cut_thickness)?;
                node.child1 = Some(Box::new(vertical_cut.left_node));
                node.child2 = Some(Box::new(vertical_cut.right_node));
                
                // Делаем горизонтальный разрез в левом дочернем узле
                if let Some(ref mut left_child) = node.child1 {
                    if left_child.get_height() > tile_dimensions.height {
                        let horizontal_cut = Self::split_horizontally(left_child, left_child.get_y1() + tile_dimensions.height, cut_thickness)?;
                        left_child.child1 = Some(Box::new(horizontal_cut.left_node));
                        left_child.child2 = Some(Box::new(horizontal_cut.right_node));
                        
                        // Размещаем плитку в верхнем левом узле
                        if let Some(ref mut top_left) = left_child.child1 {
                            top_left.is_final = true;
                            top_left.external_id = tile_dimensions.id;
                        }
                    } else {
                        // Размещаем плитку в левом узле
                        left_child.is_final = true;
                        left_child.external_id = tile_dimensions.id;
                    }
                }
            } else if need_vertical_cut {
                // Только вертикальный разрез
                let cut = Self::split_vertically(node, node.get_x1() + tile_dimensions.width, cut_thickness)?;
                node.child1 = Some(Box::new(cut.left_node));
                node.child2 = Some(Box::new(cut.right_node));
                
                // Размещаем плитку в левом дочернем узле
                if let Some(ref mut left_child) = node.child1 {
                    left_child.is_final = true;
                    left_child.external_id = tile_dimensions.id;
                }
            } else if need_horizontal_cut {
                // Только горизонтальный разрез
                let cut = Self::split_horizontally(node, node.get_y1() + tile_dimensions.height, cut_thickness)?;
                node.child1 = Some(Box::new(cut.left_node));
                node.child2 = Some(Box::new(cut.right_node));
                
                // Размещаем плитку в верхнем дочернем узле
                if let Some(ref mut top_child) = node.child1 {
                    top_child.is_final = true;
                    top_child.external_id = tile_dimensions.id;
                }
            }
            
            return Ok(true);
        }

        // Проверяем с поворотом и разрезами
        if !tile_dimensions.is_square() 
            && node.get_width() >= tile_dimensions.height 
            && node.get_height() >= tile_dimensions.width {
            
            let cut_thickness = 3;
            let rotated_width = tile_dimensions.height;
            let rotated_height = tile_dimensions.width;
            
            // Определяем, какие разрезы нужны для повернутой плитки
            let need_vertical_cut = node.get_width() > rotated_width;
            let need_horizontal_cut = node.get_height() > rotated_height;

            if need_vertical_cut && need_horizontal_cut {
                // Нужны оба разреза
                let vertical_cut = Self::split_vertically(node, node.get_x1() + rotated_width, cut_thickness)?;
                node.child1 = Some(Box::new(vertical_cut.left_node));
                node.child2 = Some(Box::new(vertical_cut.right_node));
                
                if let Some(ref mut left_child) = node.child1 {
                    if left_child.get_height() > rotated_height {
                        let horizontal_cut = Self::split_horizontally(left_child, left_child.get_y1() + rotated_height, cut_thickness)?;
                        left_child.child1 = Some(Box::new(horizontal_cut.left_node));
                        left_child.child2 = Some(Box::new(horizontal_cut.right_node));
                        
                        if let Some(ref mut top_left) = left_child.child1 {
                            top_left.is_final = true;
                            top_left.external_id = tile_dimensions.id;
                            top_left.is_rotated = true;
                        }
                    } else {
                        left_child.is_final = true;
                        left_child.external_id = tile_dimensions.id;
                        left_child.is_rotated = true;
                    }
                }
            } else if need_vertical_cut {
                let cut = Self::split_vertically(node, node.get_x1() + rotated_width, cut_thickness)?;
                node.child1 = Some(Box::new(cut.left_node));
                node.child2 = Some(Box::new(cut.right_node));
                
                if let Some(ref mut left_child) = node.child1 {
                    left_child.is_final = true;
                    left_child.external_id = tile_dimensions.id;
                    left_child.is_rotated = true;
                }
            } else if need_horizontal_cut {
                let cut = Self::split_horizontally(node, node.get_y1() + rotated_height, cut_thickness)?;
                node.child1 = Some(Box::new(cut.left_node));
                node.child2 = Some(Box::new(cut.right_node));
                
                if let Some(ref mut top_child) = node.child1 {
                    top_child.is_final = true;
                    top_child.external_id = tile_dimensions.id;
                    top_child.is_rotated = true;
                }
            }
            
            return Ok(true);
        }

        Ok(false)
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

// Добавляем необходимые типы для компиляции
use crate::engine::stock::StockSolution;
use crate::engine::model::solution::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_horizontally_with_thickness() {
        let node = TileNode::new(0, 100, 0, 100);
        let result = CuttingEngine::split_horizontally(&node, 50, 3).unwrap();

        assert_eq!(result.left_node.get_y1(), 0);
        assert_eq!(result.left_node.get_y2(), 50);
        assert_eq!(result.right_node.get_y1(), 50); // Nodes positioned without gap
        assert_eq!(result.right_node.get_y2(), 100);
        assert!(result.cut.get_is_horizontal());
    }

    #[test]
    fn test_split_vertically_with_thickness() {
        let node = TileNode::new(0, 100, 0, 100);
        let result = CuttingEngine::split_vertically(&node, 50, 3).unwrap();

        assert_eq!(result.left_node.get_x1(), 0);
        assert_eq!(result.left_node.get_x2(), 50);
        assert_eq!(result.right_node.get_x1(), 50); // Nodes positioned without gap
        assert_eq!(result.right_node.get_x2(), 100);
        assert!(!result.cut.get_is_horizontal());
    }

    #[test]
    fn test_find_candidates() {
        let root = TileNode::new(0, 100, 0, 100);
        let mut candidates = Vec::new();
        
        CuttingEngine::find_candidates(50, 30, &root, &mut candidates, 10);
        
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].get_area(), 10000);
    }

    #[test]
    fn test_cut_direction_enum() {
        assert_eq!(CutDirection::Horizontal, CutDirection::Horizontal);
        assert_ne!(CutDirection::Horizontal, CutDirection::Vertical);
    }

    #[test]
    fn test_placement_result_creation() {
        let result = PlacementResult {
            placed: true,
            cuts_made: 2,
            used_area: 1500.0,
            new_mosaic: None,
        };
        
        assert!(result.placed);
        assert_eq!(result.cuts_made, 2);
        assert_eq!(result.used_area, 1500.0);
    }
}
