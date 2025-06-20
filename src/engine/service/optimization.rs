use super::grouping::GroupedTileDimensions;
use super::{CutListOptimizerServiceImpl, MAX_PERMUTATION_ITERATIONS, MAX_STOCK_ITERATIONS};
use crate::engine::model::mosaic::Mosaic;
use crate::engine::model::request::CalculationRequest;
use crate::engine::model::solution::Solution;
use crate::engine::model::tile::TileDimensions;
use crate::engine::stock::StockSolution;
use crate::error::CuttingError;
use std::collections::HashMap;

/// Результат оптимизации
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub solutions: Vec<Solution>,
    pub placed_panels_count: usize,
    pub total_area: f64,
    pub used_area: f64,
    pub efficiency: f64,
    pub cuts_count: usize,
}

impl OptimizationResult {
    pub fn new() -> Self {
        Self {
            solutions: Vec::new(),
            placed_panels_count: 0,
            total_area: 0.0,
            used_area: 0.0,
            efficiency: 0.0,
            cuts_count: 0,
        }
    }
}

impl CutListOptimizerServiceImpl {
    /// Выполняет основную логику оптимизации
    pub fn perform_optimization(
        &self,
        request: &CalculationRequest,
    ) -> Result<OptimizationResult, CuttingError> {
        println!("🔧 perform_optimization: Начинаем основную оптимизацию");
        self.cut_list_logger
            .info("Начинаем основную оптимизацию с правильной интеграцией");

        // Конвертируем панели из запроса в TileDimensions с правильным учетом count
        let mut tile_dimensions_list = Vec::new();
        let tile_id_counter = 1000;

        for panel in &request.panels {
            if panel.is_valid() {
                if let (Ok(width_f64), Ok(height_f64)) =
                    (panel.width.parse::<f64>(), panel.height.parse::<f64>())
                {
                    let width = width_f64 as i32;
                    let height = height_f64 as i32;
                    println!(
                        "📦 Обрабатываем панель ID {}: {}x{} count={}",
                        panel.id, width, height, panel.count
                    );

                    for i in 0..panel.count {
                        let unique_id = tile_id_counter + (panel.id * 1000) + i;
                        let tile_dimensions = TileDimensions::new(
                            unique_id,
                            width,
                            height,
                            panel.material.clone(),
                            panel.orientation,
                            panel.label.clone(),
                        );
                        println!("  ➕ Создана плитка ID {}: {}x{}", unique_id, width, height);
                        tile_dimensions_list.push(tile_dimensions);
                    }
                } else {
                    println!(
                        "⚠️ Не удалось парсить размеры панели ID {}: width='{}', height='{}'",
                        panel.id, panel.width, panel.height
                    );
                }
            }
        }

        // Конвертируем складские панели с правильным учетом count
        let mut stock_tile_dimensions = Vec::new();
        for stock_panel in &request.stock_panels {
            if stock_panel.is_valid() {
                if let (Ok(width_f64), Ok(height_f64)) = (
                    stock_panel.width.parse::<f64>(),
                    stock_panel.height.parse::<f64>(),
                ) {
                    let width = width_f64 as i32;
                    let height = height_f64 as i32;
                    println!(
                        "📋 Обрабатываем стоковую панель ID {}: {}x{} count={}",
                        stock_panel.id, width, height, stock_panel.count
                    );

                    for i in 0..stock_panel.count {
                        let unique_id = tile_id_counter + (stock_panel.id * 1000) + i + 100000;
                        let tile_dimensions = TileDimensions::new(
                            unique_id,
                            width,
                            height,
                            stock_panel.material.clone(),
                            stock_panel.orientation,
                            stock_panel.label.clone(),
                        );
                        println!(
                            "  ➕ Создана стоковая плитка ID {}: {}x{}",
                            unique_id, width, height
                        );
                        stock_tile_dimensions.push(tile_dimensions);
                    }
                }
            }
        }

        println!(
            "📊 Итого создано: {} панелей для размещения, {} стоковых панелей",
            tile_dimensions_list.len(),
            stock_tile_dimensions.len()
        );

        if tile_dimensions_list.is_empty() {
            println!("❌ Нет валидных панелей для размещения");
            return Ok(OptimizationResult::new());
        }

        if stock_tile_dimensions.is_empty() {
            println!("❌ Нет валидных складских панелей");
            return Ok(OptimizationResult::new());
        }

        // Сортируем панели по убыванию площади
        tile_dimensions_list.sort_by(|a, b| {
            let area_a = a.get_area();
            let area_b = b.get_area();
            area_b.cmp(&area_a)
        });

        println!("🔄 Запуск compute_optimal_solution_improved...");
        let optimization_result =
            self.compute_optimal_solution_improved(&tile_dimensions_list, &stock_tile_dimensions)?;

        println!(
            "✅ compute_optimal_solution_improved завершен: размещено {} панелей",
            optimization_result.placed_panels_count
        );

        self.cut_list_logger.info(&format!(
            "Оптимизация завершена: размещено {}/{} панелей, эффективность {:.2}%, разрезов: {}",
            optimization_result.placed_panels_count,
            tile_dimensions_list.len(),
            optimization_result.efficiency,
            optimization_result.cuts_count
        ));

        Ok(optimization_result)
    }

    /// Улучшенный алгоритм оптимизации (точная копия Java логики)
    pub fn compute_optimal_solution_improved(
        &self,
        tiles: &[TileDimensions],
        stock_tiles: &[TileDimensions],
    ) -> Result<OptimizationResult, CuttingError> {
        println!("🔧 Запуск улучшенного алгоритма оптимизации");
        
        // Проверяем входные данные
        if tiles.is_empty() {
            println!("❌ Нет панелей для размещения");
            return Ok(OptimizationResult::new());
        }
        
        if stock_tiles.is_empty() {
            println!("❌ Нет складских панелей");
            return Ok(OptimizationResult::new());
        }
        
        // Группируем панели как в Java
        let grouped_tiles = self.generate_groups(tiles);
        let distinct_groups = self.get_distinct_grouped_tile_dimensions(&grouped_tiles);
        
        println!("📊 Создано {} групп из {} панелей", distinct_groups.len(), tiles.len());
        
        // Генерируем перестановки групп
        let group_keys: Vec<_> = distinct_groups.keys().cloned().collect();
        
        // ИСПРАВЛЕНИЕ: Проверяем пустой список групп
        if group_keys.is_empty() {
            println!("⚠️ Нет групп для обработки");
            return Ok(OptimizationResult::new());
        }
        
        let permutations = if group_keys.len() <= 7 {
            self.permutation_generator.generate_all_permutations_groups(&group_keys)
        } else {
            let mut limited_keys = group_keys[..7].to_vec();
            let remaining_keys = group_keys[7..].to_vec();
            
            let base_permutations = self.permutation_generator.generate_all_permutations_groups(&limited_keys);
            base_permutations.into_iter().map(|mut perm| {
                perm.extend(remaining_keys.clone());
                perm
            }).collect()
        };
        
        println!("🔀 Создано {} перестановок групп", permutations.len());
        
        // ИСПРАВЛЕНИЕ: Проверяем пустой список перестановок
        if permutations.is_empty() {
            println!("⚠️ Нет перестановок для обработки");
            return Ok(OptimizationResult::new());
        }
        
        // Сохраняем количество перестановок перед перемещением
        let original_permutations_count = permutations.len();
        
        // Конвертируем перестановки групп обратно в перестановки панелей
        let tile_permutations: Vec<Vec<TileDimensions>> = permutations.into_iter()
            .map(|group_perm| self.groups_to_tiles(&group_perm, &grouped_tiles, &distinct_groups))
            .collect();
        
        // Удаляем дубликаты перестановок
        let unique_permutations = self.remove_duplicate_permutations(tile_permutations);
        println!("✅ Осталось {} уникальных перестановок из {} исходных", 
            unique_permutations.len(), original_permutations_count);
        
        // ИСПРАВЛЕНИЕ: Проверяем пустой список уникальных перестановок
        if unique_permutations.is_empty() {
            println!("⚠️ Нет уникальных перестановок для обработки");
            return Ok(OptimizationResult::new());
        }
        
        // Генерируем стоковые решения
        let stock_solutions = self.generate_stock_solutions_improved(stock_tiles, tiles);
        
        // ИСПРАВЛЕНИЕ: Проверяем пустой список стоковых решений
        if stock_solutions.is_empty() {
            println!("⚠️ Нет стоковых решений для обработки");
            return Ok(OptimizationResult::new());
        }
        
        let mut best_solutions = Vec::new();
        let mut best_placed_count = 0;
        let mut best_efficiency = 0.0;
        
        // Основной цикл оптимизации (как в Java)
        for (stock_idx, stock_solution) in stock_solutions.iter().enumerate().take(MAX_STOCK_ITERATIONS) {
            println!("📋 Стоковое решение {}/{}", stock_idx + 1, stock_solutions.len());
            
            for (perm_idx, permutation) in unique_permutations.iter().enumerate().take(MAX_PERMUTATION_ITERATIONS) {
                if perm_idx % 10 == 0 {
                    println!(
                        "🔄 Перестановка {}/{}",
                        perm_idx + 1,
                        unique_permutations.len().min(MAX_PERMUTATION_ITERATIONS)
                    );
                }
                
                match self.compute_solutions_for_permutation_improved(permutation, stock_solution) {
                    Ok(solutions) => {
                        if let Some(best_solution) = solutions.first() {
                            let placed_count = best_solution.get_nbr_final_tiles() as usize;
                            let efficiency = best_solution.get_efficiency();
                            
                            if placed_count > best_placed_count || 
                               (placed_count == best_placed_count && efficiency > best_efficiency) {
                                println!("🎉 Новое лучшее решение: {}/{} панелей, {:.2}% эффективность", 
                                    placed_count, tiles.len(), efficiency);
                                
                                best_solutions = solutions;
                                best_placed_count = placed_count;
                                best_efficiency = efficiency;
                            }
                        }
                    }
                    Err(e) => {
                        self.cut_list_logger.warning(&format!("Ошибка обработки перестановки: {}", e));
                    }
                }

                // Ранний выход при отличном результате
                if best_placed_count == tiles.len() && best_efficiency > 95.0 {
                    println!("🎯 Достигнут отличный результат, завершаем оптимизацию");
                    break;
                }
            }

            if best_placed_count == tiles.len() && best_efficiency > 85.0 {
                break;
            }
        }

        Ok(OptimizationResult {
            solutions: best_solutions.clone(),
            placed_panels_count: best_placed_count,
            total_area: best_solutions.first().map(|s| s.get_total_area() as f64).unwrap_or(0.0),
            used_area: best_solutions.first().map(|s| s.get_used_area() as f64).unwrap_or(0.0),
            efficiency: best_efficiency,
            cuts_count: best_solutions.first().map(|s| s.get_cuts_count() as usize).unwrap_or(0),
        })
    }

    /// Улучшенное размещение для перестановки (точная копия Java CutListThread.computeSolutions)
    pub fn compute_solutions_for_permutation_improved(
        &self,
        tiles: &[TileDimensions],
        stock_solution: &StockSolution,
    ) -> Result<Vec<Solution>, CuttingError> {
        let mut solutions = vec![Solution::from_stock_solution(stock_solution)];

        // Последовательно размещаем каждую панель (как в Java)
        for (tile_index, tile) in tiles.iter().enumerate() {
            let mut new_solutions = Vec::new();

            for solution in &solutions {
                let mut placed_in_existing = false;

                // Пытаемся разместить в существующих мозаиках
                for mosaic in solution.get_mosaics() {
                    match mosaic.add(tile, false) {
                        Ok(result_mosaics) => {
                            for result_mosaic in result_mosaics {
                                let mut new_solution =
                                    Solution::copy_excluding_mosaic(solution, mosaic);
                                new_solution.add_mosaic(result_mosaic);
                                new_solutions.push(new_solution);
                                placed_in_existing = true;
                            }
                        }
                        Err(_) => continue,
                    }
                }

                // Если не поместилось в существующие мозаики, пробуем новую стоковую панель
                if !placed_in_existing {
                    if let Some(unused_stock) = solution.get_unused_stock_panels().front() {
                        let new_mosaic = Mosaic::new(unused_stock);
                        match new_mosaic.add(tile, false) {
                            Ok(result_mosaics) => {
                                for result_mosaic in result_mosaics {
                                    let mut new_solution = Solution::copy(solution);
                                    new_solution.get_unused_stock_panels_mut().pop_front();
                                    new_solution.add_mosaic(result_mosaic);
                                    new_solutions.push(new_solution);
                                    placed_in_existing = true;
                                }
                            }
                            Err(_) => {
                                let mut failed_solution = Solution::copy(solution);
                                failed_solution.get_no_fit_panels_mut().push(tile.clone());
                                new_solutions.push(failed_solution);
                            }
                        }
                    } else {
                        let mut failed_solution = Solution::copy(solution);
                        failed_solution.get_no_fit_panels_mut().push(tile.clone());
                        new_solutions.push(failed_solution);
                    }
                }
            }

            solutions = new_solutions;

            // Удаляем дубликаты и сортируем (как в Java)
            self.remove_duplicate_solutions(&mut solutions);
            self.sort_solutions_by_quality(&mut solutions);

            let accuracy_factor = 100;
            if solutions.len() > accuracy_factor {
                solutions.truncate(accuracy_factor);
            }

            if tile_index % 10 == 0 && tile_index > 0 {
                println!(
                    "  📈 Обработано {}/{} панелей, решений: {}",
                    tile_index + 1,
                    tiles.len(),
                    solutions.len()
                );
            }
        }

        Ok(solutions)
    }

    /// Группировка идентичных панелей (как в Java generateGroups)
    pub fn generate_groups(&self, tiles: &[TileDimensions]) -> Vec<GroupedTileDimensions> {
        if tiles.is_empty() {
            return Vec::new();
        }
        
        let mut panel_counts = HashMap::new();
        for tile in tiles {
            let key = format!("{}x{}", tile.width, tile.height);
            *panel_counts.entry(key).or_insert(0) += 1;
        }

        let mut grouped_tiles = Vec::new();
        let mut group_counter = HashMap::new();

        for tile in tiles {
            let key = format!("{}x{}", tile.width, tile.height);
            let total_count = panel_counts[&key];
            let current_count = group_counter.entry(key.clone()).or_insert(0);
            
            // ИСПРАВЛЕНИЕ: Предотвращаем деление на ноль
            let max_group_size = if total_count > 100 {
                std::cmp::max(total_count / 100, 1)
            } else {
                total_count // Для малых количеств используем все в одной группе
            };
            
            let group_id = if total_count > max_group_size && *current_count > 0 {
                // ИСПРАВЛЕНИЕ: Предотвращаем деление на ноль
                let quarter_size = std::cmp::max(total_count / 4, 1);
                if *current_count > quarter_size {
                    *current_count = 0;
                    (*current_count / quarter_size) as i32
                } else {
                    0
                }
            } else {
                0
            };

            *current_count += 1;
            grouped_tiles.push(GroupedTileDimensions::new(tile.clone(), group_id));
        }

        grouped_tiles
    }

    /// Удаляет дублирующиеся перестановки (как в Java removeDuplicatedPermutations)
    pub fn remove_duplicate_permutations(
        &self,
        permutations: Vec<Vec<TileDimensions>>,
    ) -> Vec<Vec<TileDimensions>> {
        let mut seen_hashes = std::collections::HashSet::new();
        let mut unique_permutations = Vec::new();
        let original_count = permutations.len();

        for permutation in permutations {
            let mut hash = 0i32;
            for tile in &permutation {
                hash = hash
                    .wrapping_mul(31)
                    .wrapping_add(tile.dimensions_based_hash_code());
            }

            if seen_hashes.insert(hash) {
                unique_permutations.push(permutation);
            }
        }

        println!(
            "🔄 Удалено {} дублирующихся перестановок",
            original_count - unique_permutations.len()
        );

        unique_permutations
    }

    /// Удаляет дубликаты решений
    pub fn remove_duplicate_solutions(&self, solutions: &mut Vec<Solution>) {
        let mut seen_signatures = std::collections::HashSet::new();

        solutions.retain(|solution| {
            let signature = solution.get_structure_identifier();
            seen_signatures.insert(signature)
        });
    }

    /// Сортирует решения по качеству
    pub fn sort_solutions_by_quality(&self, solutions: &mut Vec<Solution>) {
        solutions.sort_by(|a, b| {
            let placed_a = a.get_nbr_final_tiles();
            let placed_b = b.get_nbr_final_tiles();

            match placed_b.cmp(&placed_a) {
                std::cmp::Ordering::Equal => {
                    let efficiency_a = if a.get_total_area() > 0 {
                        (a.get_used_area() as f64 / a.get_total_area() as f64) * 100.0
                    } else {
                        0.0
                    };
                    let efficiency_b = if b.get_total_area() > 0 {
                        (b.get_used_area() as f64 / b.get_total_area() as f64) * 100.0
                    } else {
                        0.0
                    };

                    match efficiency_b
                        .partial_cmp(&efficiency_a)
                        .unwrap_or(std::cmp::Ordering::Equal)
                    {
                        std::cmp::Ordering::Equal => b.get_total_area().cmp(&a.get_total_area()),
                        other => other,
                    }
                }
                other => other,
            }
        });
    }

    /// Улучшенная генерация стоковых решений
    pub fn generate_stock_solutions_improved(
        &self,
        stock_tiles: &[TileDimensions],
        tiles: &[TileDimensions],
    ) -> Vec<StockSolution> {
        let mut solutions = Vec::new();

        let total_tiles_area: i64 = tiles.iter().map(|t| t.get_area()).sum();

        let mut sorted_stock = stock_tiles.to_vec();
        sorted_stock.sort_by(|a, b| a.get_area().cmp(&b.get_area()));

        // Одиночные панели
        for stock_tile in &sorted_stock {
            solutions.push(StockSolution::new(vec![stock_tile.clone()]));
        }

        // Комбинации из 2-3 панелей
        for i in 0..sorted_stock.len() {
            for j in (i + 1)..sorted_stock.len().min(i + 10) {
                let combo_area = sorted_stock[i].get_area() + sorted_stock[j].get_area();

                if combo_area >= total_tiles_area / 2 {
                    solutions.push(StockSolution::new(vec![
                        sorted_stock[i].clone(),
                        sorted_stock[j].clone(),
                    ]));
                }

                if j + 1 < sorted_stock.len() {
                    let triple_area = combo_area + sorted_stock[j + 1].get_area();
                    if triple_area >= total_tiles_area * 3 / 4 {
                        solutions.push(StockSolution::new(vec![
                            sorted_stock[i].clone(),
                            sorted_stock[j].clone(),
                            sorted_stock[j + 1].clone(),
                        ]));
                    }
                }
            }
        }

        solutions.sort_by(|a, b| a.get_total_area().cmp(&b.get_total_area()));
        solutions.truncate(100);

        solutions
    }

    /// Помощающие методы для работы с группами
    pub fn get_distinct_grouped_tile_dimensions(
        &self,
        grouped_tiles: &[GroupedTileDimensions],
    ) -> HashMap<String, (TileDimensions, i32)> {
        let mut distinct = HashMap::new();
        for grouped_tile in grouped_tiles {
            let key = format!("{}x{}_g{}", grouped_tile.tile.width, grouped_tile.tile.height, grouped_tile.group_id);
            let current_count = distinct.entry(key.clone()).or_insert((grouped_tile.tile.clone(), 0)).1;
            distinct.insert(key, (grouped_tile.tile.clone(), current_count + 1));
        }
        distinct
    }

    pub fn groups_to_tiles(
        &self,
        group_permutation: &[String],
        grouped_tiles: &[GroupedTileDimensions],
        distinct_groups: &HashMap<String, (TileDimensions, i32)>,
    ) -> Vec<TileDimensions> {
        let mut result = Vec::new();

        for group_key in group_permutation {
            if let Some((tile_template, count)) = distinct_groups.get(group_key) {
                for _ in 0..*count {
                    result.push(tile_template.clone());
                }
            }
        }

        result
    }
}
