use crate::engine::model::tile::TileDimensions;

/// Структура для группированных панелей (как в Java GroupedTileDimensions)
#[derive(Debug, Clone)]
pub struct GroupedTileDimensions {
    pub tile: TileDimensions,
    pub group_id: i32,
}

impl GroupedTileDimensions {
    pub fn new(tile: TileDimensions, group_id: i32) -> Self {
        Self { tile, group_id }
    }

    /// Получить площадь панели
    pub fn get_area(&self) -> i64 {
        self.tile.get_area()
    }

    /// Получить строковое представление размеров
    pub fn dimensions_to_string(&self) -> String {
        format!("{}x{}", self.tile.width, self.tile.height)
    }

    /// Получить строковое представление с группой
    pub fn to_string_with_group(&self) -> String {
        format!("{}x{}_g{}", self.tile.width, self.tile.height, self.group_id)
    }

    /// Получить хеш-код на основе размеров (как в Java dimensionsBasedHashCode)
    pub fn dimensions_based_hash_code(&self) -> i32 {
        self.tile.dimensions_based_hash_code()
    }
}

impl PartialEq for GroupedTileDimensions {
    fn eq(&self, other: &Self) -> bool {
        self.tile.width == other.tile.width 
            && self.tile.height == other.tile.height 
            && self.group_id == other.group_id
    }
}

impl std::fmt::Display for GroupedTileDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{} (group: {})", self.tile.width, self.tile.height, self.group_id)
    }
}

/// Утилиты для работы с группировкой панелей
pub struct GroupingUtils;

impl GroupingUtils {
    /// Проверяет, является ли оптимизация одномерной (как в Java isOneDimensionalOptimization)
    pub fn is_one_dimensional_optimization(
        tiles: &[TileDimensions], 
        stock_tiles: &[TileDimensions]
    ) -> bool {
        if tiles.is_empty() || stock_tiles.is_empty() {
            return false;
        }

        // Собираем все уникальные размеры из первой панели
        let first_tile = &tiles[0];
        let mut common_dimensions = vec![first_tile.width, first_tile.height];

        // Проверяем панели для размещения
        for tile in tiles {
            common_dimensions.retain(|&dim| dim == tile.width || dim == tile.height);
            if common_dimensions.is_empty() {
                return false;
            }
        }

        // Проверяем складские панели
        for stock_tile in stock_tiles {
            common_dimensions.retain(|&dim| dim == stock_tile.width || dim == stock_tile.height);
            if common_dimensions.is_empty() {
                return false;
            }
        }

        !common_dimensions.is_empty()
    }

    /// Создает карту количества панелей по размерам
    pub fn create_panel_count_map(tiles: &[TileDimensions]) -> std::collections::HashMap<String, i32> {
        let mut map = std::collections::HashMap::new();
        
        for tile in tiles {
            let key = format!("{}x{}", tile.width, tile.height);
            *map.entry(key).or_insert(0) += 1;
        }
        
        map
    }

    /// Создает строковое представление набора панелей для логирования
    pub fn create_tiles_summary(
        tiles: &[TileDimensions],
        panel_counts: &std::collections::HashMap<String, i32>
    ) -> String {
        let mut summary = String::new();
        
        for (dimensions, count) in panel_counts {
            if !summary.is_empty() {
                summary.push(' ');
            }
            summary.push_str(&format!("{}*{}", dimensions, count));
        }
        
        summary
    }

    /// Вычисляет оптимальный размер группы на основе общего количества панелей
    pub fn calculate_optimal_group_size(total_panels: usize, panel_count: i32) -> i32 {
        let base_size = std::cmp::max(total_panels / 100, 1) as i32;
        
        // Если одномерная оптимизация, используем группы размером 1
        if Self::should_use_single_groups(total_panels, panel_count) {
            return 1;
        }
        
        base_size
    }

    /// Определяет, следует ли использовать группы размером 1
    fn should_use_single_groups(total_panels: usize, panel_count: i32) -> bool {
        // Эвристика: если панелей мало или они очень разнообразны
        total_panels < 50 || panel_count < 5
    }

    /// Разбивает большие группы на подгруппы (как в Java логике splitPanel)
    pub fn should_split_group(
        current_group_count: i32, 
        total_panel_count: i32, 
        max_group_size: i32
    ) -> bool {
        total_panel_count > max_group_size && 
        current_group_count > total_panel_count / 4
    }

    /// Создает сводку по группам для логирования
    pub fn create_groups_summary(groups: &[GroupedTileDimensions]) -> String {
        let mut group_counts: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        
        for group in groups {
            let key = group.to_string_with_group();
            *group_counts.entry(key).or_insert(0) += 1;
        }
        
        let mut summary = String::new();
        let mut group_number = 1;
        
        for (group_key, count) in group_counts {
            if !summary.is_empty() {
                summary.push(' ');
            }
            summary.push_str(&format!("group[{}:{}*{}]", group_number, group_key, count));
            group_number += 1;
        }
        
        summary
    }

    /// Сортирует группы по площади (убывание) как в Java
    pub fn sort_groups_by_area_desc(groups: &mut Vec<GroupedTileDimensions>) {
        groups.sort_by(|a, b| b.get_area().cmp(&a.get_area()));
    }

    /// Конвертирует группированные панели обратно в список панелей с учетом порядка групп
    pub fn convert_grouped_to_tiles_list(
        group_permutation: &[GroupedTileDimensions],
        original_grouped_tiles: &[GroupedTileDimensions]
    ) -> Vec<TileDimensions> {
        let mut result = Vec::new();
        
        // Создаем индекс для быстрого поиска
        let mut tile_indices: Vec<usize> = Vec::new();
        
        for target_group in group_permutation {
            // Находим все панели с такими же размерами и группой в исходном списке
            for (idx, original_group) in original_grouped_tiles.iter().enumerate() {
                if original_group.tile.width == target_group.tile.width &&
                   original_group.tile.height == target_group.tile.height &&
                   original_group.group_id == target_group.group_id {
                    tile_indices.push(idx);
                }
            }
        }
        
        // Сортируем индексы и извлекаем панели в правильном порядке
        tile_indices.sort();
        for &idx in &tile_indices {
            result.push(original_grouped_tiles[idx].tile.clone());
        }
        
        result
    }

    /// Проверяет целостность группировки
    pub fn validate_grouping(
        original_tiles: &[TileDimensions],
        grouped_tiles: &[GroupedTileDimensions]
    ) -> Result<(), String> {
        // Проверяем количество
        if original_tiles.len() != grouped_tiles.len() {
            return Err(format!(
                "Несоответствие количества: исходных панелей {}, группированных {}",
                original_tiles.len(),
                grouped_tiles.len()
            ));
        }

        // Проверяем, что все исходные панели присутствуют в группах
        let mut original_sorted = original_tiles.to_vec();
        original_sorted.sort_by(|a, b| {
            match a.width.cmp(&b.width) {
                std::cmp::Ordering::Equal => a.height.cmp(&b.height),
                other => other,
            }
        });

        let mut grouped_sorted: Vec<TileDimensions> = grouped_tiles
            .iter()
            .map(|g| g.tile.clone())
            .collect();
        grouped_sorted.sort_by(|a, b| {
            match a.width.cmp(&b.width) {
                std::cmp::Ordering::Equal => a.height.cmp(&b.height),
                other => other,
            }
        });

        for (i, (original, grouped)) in original_sorted.iter().zip(grouped_sorted.iter()).enumerate() {
            if original.width != grouped.width || original.height != grouped.height {
                return Err(format!(
                    "Несоответствие панели {}: исходная {}x{}, группированная {}x{}",
                    i, original.width, original.height, grouped.width, grouped.height
                ));
            }
        }

        Ok(())
    }

    /// Анализирует распределение групп
    pub fn analyze_group_distribution(groups: &[GroupedTileDimensions]) -> GroupDistributionAnalysis {
        let mut group_sizes: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut dimension_groups: std::collections::HashMap<String, Vec<i32>> = std::collections::HashMap::new();
        
        for group in groups {
            // Подсчитываем размеры групп
            *group_sizes.entry(group.group_id).or_insert(0) += 1;
            
            // Группируем по размерам
            let dim_key = format!("{}x{}", group.tile.width, group.tile.height);
            dimension_groups.entry(dim_key).or_insert_with(Vec::new).push(group.group_id);
        }

        let total_groups = group_sizes.len();
        let avg_group_size = if total_groups > 0 {
            groups.len() as f64 / total_groups as f64
        } else {
            0.0
        };

        let max_group_size = group_sizes.values().max().copied().unwrap_or(0);
        let min_group_size = group_sizes.values().min().copied().unwrap_or(0);

        GroupDistributionAnalysis {
            total_groups,
            avg_group_size,
            max_group_size,
            min_group_size,
            unique_dimensions: dimension_groups.len(),
            group_sizes,
            dimension_groups,
        }
    }
}

/// Результат анализа распределения групп
#[derive(Debug)]
pub struct GroupDistributionAnalysis {
    pub total_groups: usize,
    pub avg_group_size: f64,
    pub max_group_size: i32,
    pub min_group_size: i32,
    pub unique_dimensions: usize,
    pub group_sizes: std::collections::HashMap<i32, i32>,
    pub dimension_groups: std::collections::HashMap<String, Vec<i32>>,
}

impl std::fmt::Display for GroupDistributionAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Groups: {} total, avg size: {:.1}, range: {}-{}, unique dimensions: {}",
            self.total_groups,
            self.avg_group_size,
            self.min_group_size,
            self.max_group_size,
            self.unique_dimensions
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grouped_tile_dimensions_creation() {
        let tile = TileDimensions::simple(100, 200);
        let grouped = GroupedTileDimensions::new(tile, 1);
        
        assert_eq!(grouped.group_id, 1);
        assert_eq!(grouped.get_area(), 20000);
        assert_eq!(grouped.dimensions_to_string(), "100x200");
    }

    #[test]
    fn test_one_dimensional_optimization_detection() {
        let tiles = vec![
            TileDimensions::simple(100, 50),
            TileDimensions::simple(100, 75),
            TileDimensions::simple(100, 100),
        ];
        
        let stock_tiles = vec![
            TileDimensions::simple(100, 300),
            TileDimensions::simple(100, 400),
        ];
        
        assert!(GroupingUtils::is_one_dimensional_optimization(&tiles, &stock_tiles));
    }

    #[test]
    fn test_panel_count_map() {
        let tiles = vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ];
        
        let count_map = GroupingUtils::create_panel_count_map(&tiles);
        
        assert_eq!(count_map.get("100x200"), Some(&2));
        assert_eq!(count_map.get("150x300"), Some(&1));
    }

    #[test]
    fn test_group_validation() {
        let original = vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ];
        
        let grouped = vec![
            GroupedTileDimensions::new(TileDimensions::simple(100, 200), 0),
            GroupedTileDimensions::new(TileDimensions::simple(150, 300), 0),
        ];
        
        assert!(GroupingUtils::validate_grouping(&original, &grouped).is_ok());
    }
}