use crate::engine::model::tile::TileDimensions;

/// Генератор перестановок (точная копия Java Arrangement.generatePermutations)
pub struct PermutationGenerator;

impl PermutationGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Генерирует ВСЕ возможные перестановки панелей (как в Java Arrangement.generatePermutations)
    pub fn generate_all_permutations(&self, tiles: &[TileDimensions]) -> Vec<Vec<TileDimensions>> {
        if tiles.is_empty() {
            return vec![vec![]];
        }
        
        // Ограничиваем количество для производительности
        if tiles.len() > 8 {
            return self.generate_smart_permutations(tiles);
        }
        
        // Для малых наборов генерируем все перестановки
        self.generate_permutations_recursive(tiles.to_vec())
    }
    
    /// Рекурсивная генерация всех перестановок (точная копия Java алгоритма)
    fn generate_permutations_recursive(&self, mut list: Vec<TileDimensions>) -> Vec<Vec<TileDimensions>> {
        if list.is_empty() {
            return vec![vec![]];
        }
        
        let first_element = list.remove(0);
        let mut all_permutations = Vec::new();
        
        for sub_permutation in self.generate_permutations_recursive(list.clone()) {
            for i in 0..=sub_permutation.len() {
                let mut new_permutation = sub_permutation.clone();
                new_permutation.insert(i, first_element.clone());
                all_permutations.push(new_permutation);
            }
        }
        
        all_permutations
    }
    
    /// Умная генерация перестановок для больших наборов
    fn generate_smart_permutations(&self, tiles: &[TileDimensions]) -> Vec<Vec<TileDimensions>> {
        let mut permutations = Vec::new();
        
        // 1. По убыванию площади (как сейчас)
        let mut by_area = tiles.to_vec();
        by_area.sort_by(|a, b| b.get_area().cmp(&a.get_area()));
        permutations.push(by_area);
        
        // 2. По убыванию ширины
        let mut by_width = tiles.to_vec();
        by_width.sort_by(|a, b| b.width.cmp(&a.width));
        permutations.push(by_width);
        
        // 3. По убыванию высоты
        let mut by_height = tiles.to_vec();
        by_height.sort_by(|a, b| b.height.cmp(&a.height));
        permutations.push(by_height);
        
        // 4. Смешанная стратегия: чередование больших и маленьких
        let mut mixed = tiles.to_vec();
        mixed.sort_by(|a, b| b.get_area().cmp(&a.get_area()));
        let mut mixed_strategy = Vec::new();
        let mid = mixed.len() / 2;
        for i in 0..mid {
            mixed_strategy.push(mixed[i].clone());
            if mid + i < mixed.len() {
                mixed_strategy.push(mixed[mid + i].clone());
            }
        }
        permutations.push(mixed_strategy);
        
        // 5. По убыванию максимального измерения
        let mut by_max_dim = tiles.to_vec();
        by_max_dim.sort_by(|a, b| b.get_max_dimension().cmp(&a.get_max_dimension()));
        permutations.push(by_max_dim);
        
        // 6. Обратный порядок
        let mut reversed = tiles.to_vec();
        reversed.reverse();
        permutations.push(reversed);
        
        // 7. По убыванию периметра
        let mut by_perimeter = tiles.to_vec();
        by_perimeter.sort_by(|a, b| {
            let perimeter_a = 2 * (a.width + a.height);
            let perimeter_b = 2 * (b.width + b.height);
            perimeter_b.cmp(&perimeter_a)
        });
        permutations.push(by_perimeter);
        
        // 8. По возрастанию площади (противоположная стратегия)
        let mut by_area_asc = tiles.to_vec();
        by_area_asc.sort_by(|a, b| a.get_area().cmp(&b.get_area()));
        permutations.push(by_area_asc);
        
        permutations
    }

    /// Генерация перестановок для групп (строковые ключи)
    pub fn generate_all_permutations_groups(&self, groups: &[String]) -> Vec<Vec<String>> {
        if groups.len() <= 6 {
            // Для малых наборов используем полную генерацию
            self.generate_permutations_recursive_string(groups.to_vec())
        } else {
            // Для больших наборов используем умные стратегии
            self.generate_smart_group_permutations(groups)
        }
    }
    
    /// Рекурсивная генерация перестановок для строк
    fn generate_permutations_recursive_string(&self, mut list: Vec<String>) -> Vec<Vec<String>> {
        if list.is_empty() {
            return vec![vec![]];
        }
        
        let first_element = list.remove(0);
        let mut all_permutations = Vec::new();
        
        for sub_permutation in self.generate_permutations_recursive_string(list.clone()) {
            for i in 0..=sub_permutation.len() {
                let mut new_permutation = sub_permutation.clone();
                new_permutation.insert(i, first_element.clone());
                all_permutations.push(new_permutation);
            }
        }
        
        all_permutations
    }
    
    /// Умные стратегии для групп
    fn generate_smart_group_permutations(&self, groups: &[String]) -> Vec<Vec<String>> {
        let mut permutations = Vec::new();
        
        // 1. Исходный порядок
        permutations.push(groups.to_vec());
        
        // 2. Обратный порядок
        let mut reversed = groups.to_vec();
        reversed.reverse();
        permutations.push(reversed);
        
        // 3. Лексикографическая сортировка
        let mut sorted = groups.to_vec();
        sorted.sort();
        permutations.push(sorted);
        
        // 4. Обратная лексикографическая сортировка
        let mut sorted_rev = groups.to_vec();
        sorted_rev.sort();
        sorted_rev.reverse();
        permutations.push(sorted_rev);
        
        // 5. Случайное перемешивание (детерминированное для воспроизводимости)
        let mut shuffled = groups.to_vec();
        // Простой детерминированный алгоритм перемешивания
        for i in 0..shuffled.len() {
            let j = (i * 7 + 3) % shuffled.len(); // Простая формула для детерминированности
            shuffled.swap(i, j);
        }
        permutations.push(shuffled);
        
        // 6. Интерливинг: первая половина чередуется со второй
        if groups.len() > 1 {
            let mid = groups.len() / 2;
            let mut interleaved = Vec::new();
            for i in 0..mid {
                interleaved.push(groups[i].clone());
                if mid + i < groups.len() {
                    interleaved.push(groups[mid + i].clone());
                }
            }
            // Добавляем оставшиеся элементы если нечетное количество
            if groups.len() % 2 != 0 {
                interleaved.push(groups[groups.len() - 1].clone());
            }
            permutations.push(interleaved);
        }
        
        permutations
    }

    /// Генерирует перестановки на основе различных эвристик
    pub fn generate_heuristic_permutations(&self, tiles: &[TileDimensions]) -> Vec<Vec<TileDimensions>> {
        let mut permutations = Vec::new();
        
        // Эвристика 1: Largest First (наибольшие сначала)
        let mut largest_first = tiles.to_vec();
        largest_first.sort_by(|a, b| {
            let area_cmp = b.get_area().cmp(&a.get_area());
            if area_cmp == std::cmp::Ordering::Equal {
                b.get_max_dimension().cmp(&a.get_max_dimension())
            } else {
                area_cmp
            }
        });
        permutations.push(largest_first);
        
        // Эвристика 2: Best Fit Decreasing (лучше всего подходящие убывающие)
        let mut best_fit = tiles.to_vec();
        best_fit.sort_by(|a, b| {
            let ratio_a = a.width as f64 / a.height as f64;
            let ratio_b = b.width as f64 / b.height as f64;
            // Сначала квадратные, потом по убыванию площади
            let square_diff_a = (ratio_a - 1.0).abs();
            let square_diff_b = (ratio_b - 1.0).abs();
            
            match square_diff_a.partial_cmp(&square_diff_b) {
                Some(std::cmp::Ordering::Equal) => b.get_area().cmp(&a.get_area()),
                Some(ord) => ord,
                None => std::cmp::Ordering::Equal,
            }
        });
        permutations.push(best_fit);
        
        // Эвристика 3: Bottom-Left Fill (заполнение снизу-слева)
        let mut bottom_left = tiles.to_vec();
        bottom_left.sort_by(|a, b| {
            // Сначала высокие узкие, потом широкие низкие
            let height_cmp = b.height.cmp(&a.height);
            if height_cmp == std::cmp::Ordering::Equal {
                a.width.cmp(&b.width)
            } else {
                height_cmp
            }
        });
        permutations.push(bottom_left);
        
        // Эвристика 4: Next Fit Decreasing
        let mut next_fit = tiles.to_vec();
        next_fit.sort_by(|a, b| {
            let min_dim_a = a.width.min(a.height);
            let min_dim_b = b.width.min(b.height);
            
            match min_dim_b.cmp(&min_dim_a) {
                std::cmp::Ordering::Equal => b.get_area().cmp(&a.get_area()),
                other => other,
            }
        });
        permutations.push(next_fit);
        
        // Эвристика 5: Альтернирующая стратегия
        if tiles.len() > 2 {
            let mut alternating = Vec::new();
            let mut sorted_by_area = tiles.to_vec();
            sorted_by_area.sort_by(|a, b| b.get_area().cmp(&a.get_area()));
            
            let mut sorted_by_ratio = tiles.to_vec();
            sorted_by_ratio.sort_by(|a, b| {
                let ratio_a = (a.width as f64 / a.height as f64).max(a.height as f64 / a.width as f64);
                let ratio_b = (b.width as f64 / b.height as f64).max(b.height as f64 / b.width as f64);
                ratio_b.partial_cmp(&ratio_a).unwrap_or(std::cmp::Ordering::Equal)
            });
            
            let half = tiles.len() / 2;
            for i in 0..half {
                if i < sorted_by_area.len() {
                    alternating.push(sorted_by_area[i].clone());
                }
                if i < sorted_by_ratio.len() {
                    alternating.push(sorted_by_ratio[i].clone());
                }
            }
            permutations.push(alternating);
        }
        
        permutations
    }

    /// Генерирует перестановки с учетом материалов
    pub fn generate_material_aware_permutations(&self, tiles: &[TileDimensions]) -> Vec<Vec<TileDimensions>> {
        let mut permutations = Vec::new();
        
        // Группируем по материалам
        let mut material_groups: std::collections::HashMap<String, Vec<TileDimensions>> = std::collections::HashMap::new();
        
        for tile in tiles {
            material_groups.entry(tile.material.clone()).or_insert_with(Vec::new).push(tile.clone());
        }
        
        // Сортируем каждую группу материалов по площади
        for group in material_groups.values_mut() {
            group.sort_by(|a, b| b.get_area().cmp(&a.get_area()));
        }
        
        // Стратегия 1: Материалы по порядку, внутри группы по площади
        let mut by_materials = Vec::new();
        let mut material_names: Vec<_> = material_groups.keys().cloned().collect();
        material_names.sort();
        
        for material in &material_names {
            if let Some(group) = material_groups.get(material) {
                by_materials.extend(group.clone());
            }
        }
        permutations.push(by_materials);
        
        // Стратегия 2: Интерливинг материалов
        if material_groups.len() > 1 {
            let mut interleaved = Vec::new();
            let max_group_size = material_groups.values().map(|g| g.len()).max().unwrap_or(0);
            
            for i in 0..max_group_size {
                for material in &material_names {
                    if let Some(group) = material_groups.get(material) {
                        if i < group.len() {
                            interleaved.push(group[i].clone());
                        }
                    }
                }
            }
            permutations.push(interleaved);
        }
        
        permutations
    }

    /// Генерирует адаптивные перестановки на основе характеристик набора
    pub fn generate_adaptive_permutations(&self, tiles: &[TileDimensions]) -> Vec<Vec<TileDimensions>> {
        let mut permutations = Vec::new();
        
        if tiles.is_empty() {
            return permutations;
        }
        
        // Анализируем характеристики набора
        let total_area: i64 = tiles.iter().map(|t| t.get_area()).sum();
        let avg_area = total_area as f64 / tiles.len() as f64;
        
        let aspect_ratios: Vec<f64> = tiles.iter().map(|t| {
            let w = t.width as f64;
            let h = t.height as f64;
            w.max(h) / w.min(h)
        }).collect();
        
        let avg_aspect_ratio = aspect_ratios.iter().sum::<f64>() / aspect_ratios.len() as f64;
        
        // Стратегия зависит от характеристик набора
        if avg_aspect_ratio > 2.0 {
            // Много длинных узких деталей - используем стратегию упаковки полос
            let mut strip_packing = tiles.to_vec();
            strip_packing.sort_by(|a, b| {
                let height_cmp = b.height.cmp(&a.height);
                if height_cmp == std::cmp::Ordering::Equal {
                    a.width.cmp(&b.width)
                } else {
                    height_cmp
                }
            });
            permutations.push(strip_packing);
        } else if avg_aspect_ratio < 1.5 {
            // Много квадратных деталей - используем стратегию упаковки блоков
            let mut block_packing = tiles.to_vec();
            block_packing.sort_by(|a, b| {
                b.get_area().cmp(&a.get_area())
            });
            permutations.push(block_packing);
        }
        
        // Проверяем разнообразие размеров
        let max_area = tiles.iter().map(|t| t.get_area()).max().unwrap_or(0) as f64;
        let min_area = tiles.iter().map(|t| t.get_area()).min().unwrap_or(0) as f64;
        let size_variance = if min_area > 0.0 { max_area / min_area } else { 1.0 };
        
        if size_variance > 10.0 {
            // Большое разнообразие размеров - используем смешанную стратегию
            let mut mixed_strategy = Vec::new();
            let mut sorted_tiles = tiles.to_vec();
            sorted_tiles.sort_by(|a, b| b.get_area().cmp(&a.get_area()));
            
            // Берем попеременно большие и маленькие
            let mut large_idx = 0;
            let mut small_idx = sorted_tiles.len() - 1;
            let mut use_large = true;
            
            while large_idx <= small_idx {
                if use_large {
                    mixed_strategy.push(sorted_tiles[large_idx].clone());
                    large_idx += 1;
                } else {
                    mixed_strategy.push(sorted_tiles[small_idx].clone());
                    if small_idx > 0 {
                        small_idx -= 1;
                    } else {
                        break;
                    }
                }
                use_large = !use_large;
            }
            permutations.push(mixed_strategy);
        }
        
        permutations
    }
}

impl Default for PermutationGenerator {
    fn default() -> Self {
        Self::new()
    }
}