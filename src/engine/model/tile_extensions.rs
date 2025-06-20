use crate::engine::model::tile::TileDimensions;

impl TileDimensions {


    /// Вычисляет хеш-код на основе размеров (как в Java dimensionsBasedHashCode)
    pub fn dimensions_based_hash_code(&self) -> i32 {
        let mut hash = 1i32;
        hash = hash.wrapping_mul(31).wrapping_add(self.width);
        hash = hash.wrapping_mul(31).wrapping_add(self.height);
        hash
    }

    /// Получает максимальное измерение
    pub fn get_max_dimension(&self) -> i32 {
        self.width.max(self.height)
    }

    /// Получает минимальное измерение
    pub fn get_min_dimension(&self) -> i32 {
        self.width.min(self.height)
    }

    /// Проверяет, является ли панель квадратной
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Вычисляет площадь панели
    pub fn get_area(&self) -> i64 {
        (self.width as i64) * (self.height as i64)
    }

    /// Создает повернутую на 90 градусов копию панели
    pub fn rotate90(&self) -> Self {
        Self {
            id: self.id,
            width: self.height,
            height: self.width,
            material: self.material.clone(),
            orientation: if self.is_rotated { 0 } else { 1 },
            label: self.label.clone(),
            is_rotated: !self.is_rotated,
        }
    }

    /// Проверяет, помещается ли указанная панель в эту
    pub fn fits(&self, other: &TileDimensions) -> bool {
        // Проверяем в исходной ориентации
        let fits_normal = self.width >= other.width && self.height >= other.height;
        
        // Проверяем с поворотом (если панель не квадратная)
        let fits_rotated = !other.is_square() && 
                          self.width >= other.height && 
                          self.height >= other.width;
        
        fits_normal || fits_rotated
    }

    /// Вычисляет коэффициент соотношения сторон
    pub fn get_aspect_ratio(&self) -> f64 {
        if self.height == 0 {
            return f64::INFINITY;
        }
        self.width as f64 / self.height as f64
    }

    /// Вычисляет периметр панели
    pub fn get_perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }


    /// Полное строковое представление с материалом и ориентацией
    pub fn full_description(&self) -> String {
        format!("{}x{} [{}] orient:{} rotated:{}", 
            self.width, self.height, self.material, self.orientation, self.is_rotated)
    }

    /// Проверяет валидность размеров панели
    pub fn is_valid_dimensions(&self) -> bool {
        self.width > 0 && self.height > 0 && 
        self.width <= 100000 && self.height <= 100000
    }

    /// Создает копию панели с новыми размерами
    pub fn with_dimensions(&self, width: i32, height: i32) -> Self {
        Self {
            id: self.id,
            width,
            height,
            material: self.material.clone(),
            orientation: self.orientation,
            label: self.label.clone(),
            is_rotated: self.is_rotated,
        }
    }

    /// Создает копию панели с новым материалом
    pub fn with_material(&self, material: String) -> Self {
        Self {
            id: self.id,
            width: self.width,
            height: self.height,
            material,
            orientation: self.orientation,
            label: self.label.clone(),
            is_rotated: self.is_rotated,
        }
    }

    /// Создает копию панели с новой ориентацией
    pub fn with_orientation(&self, orientation: i32) -> Self {
        Self {
            id: self.id,
            width: self.width,
            height: self.height,
            material: self.material.clone(),
            orientation,
            label: self.label.clone(),
            is_rotated: self.is_rotated,
        }
    }

    /// Проверяет совместимость материалов
    pub fn is_material_compatible(&self, other: &TileDimensions) -> bool {
        self.material == other.material
    }

    /// Вычисляет "вес" панели для сортировки (площадь + коэффициент формы)
    pub fn get_sorting_weight(&self) -> f64 {
        let area = self.get_area() as f64;
        let aspect_ratio = self.get_aspect_ratio();
        let form_factor = if aspect_ratio > 1.0 { aspect_ratio } else { 1.0 / aspect_ratio };
        
        // Больший вес у панелей с большей площадью и более квадратной формой
        area / form_factor
    }

    /// Проверяет, является ли панель "длинной и узкой"
    pub fn is_elongated(&self, threshold: f64) -> bool {
        let aspect_ratio = self.get_aspect_ratio();
        aspect_ratio > threshold || aspect_ratio < (1.0 / threshold)
    }

    /// Вычисляет "сложность" размещения панели
    pub fn get_placement_complexity(&self) -> f64 {
        let aspect_ratio = self.get_aspect_ratio();
        let area = self.get_area() as f64;
        let max_dim = self.get_max_dimension() as f64;
        
        // Большие панели с экстремальными пропорциями сложнее размещать
        let size_factor = (max_dim / 1000.0).min(10.0);
        let ratio_factor = if aspect_ratio > 1.0 { aspect_ratio } else { 1.0 / aspect_ratio };
        
        size_factor * ratio_factor.ln()
    }

    /// Создает уникальную сигнатуру панели для группировки
    pub fn get_grouping_signature(&self) -> String {
        format!("{}x{}_{}", 
            self.width.min(self.height), 
            self.width.max(self.height), 
            self.material)
    }

    /// Создает сигнатуру только по размерам (игнорируя материал)
    pub fn get_size_signature(&self) -> String {
        format!("{}x{}", 
            self.width.min(self.height), 
            self.width.max(self.height))
    }

    /// Проверяет, можно ли повернуть панель (учитывая ограничения материала)
    pub fn can_rotate(&self) -> bool {
        // В большинстве случаев поворот разрешен, если панель не квадратная
        // В будущем можно добавить проверку направления волокон материала
        !self.is_square()
    }

    /// Получает оптимальную ориентацию панели для размещения
    pub fn get_optimal_orientation(&self, container_width: i32, container_height: i32) -> Self {
        if self.is_square() {
            return self.clone();
        }

        // Пробуем разместить в исходной ориентации
        let fits_normal = self.width <= container_width && self.height <= container_height;
        
        // Пробуем разместить с поворотом
        let fits_rotated = self.height <= container_width && self.width <= container_height;
        
        match (fits_normal, fits_rotated) {
            (true, false) => self.clone(),
            (false, true) => self.rotate90(),
            (true, true) => {
                // Выбираем ориентацию с лучшим использованием пространства
                let waste_normal = (container_width * container_height) - (self.width * self.height);
                let waste_rotated = (container_width * container_height) - (self.height * self.width);
                
                if waste_normal <= waste_rotated {
                    self.clone()
                } else {
                    self.rotate90()
                }
            }
            (false, false) => self.clone(), // Не помещается в любом случае
        }
    }

    /// Вычисляет "приоритет" панели для размещения
    pub fn get_placement_priority(&self) -> i32 {
        // Приоритет основан на площади и сложности формы
        let area_priority = self.get_area() / 10000; // Нормализуем площадь
        let aspect_ratio = self.get_aspect_ratio();
        let complexity_penalty = if aspect_ratio > 3.0 || aspect_ratio < 0.33 {
            -1000 // Штраф за сложную форму
        } else {
            0
        };
        
        area_priority as i32 + complexity_penalty
    }
}

/// Утилиты для работы с наборами панелей
pub struct TileUtils;

impl TileUtils {
    /// Группирует панели по размерам
    pub fn group_by_size(tiles: &[TileDimensions]) -> std::collections::HashMap<String, Vec<TileDimensions>> {
        let mut groups = std::collections::HashMap::new();
        
        for tile in tiles {
            let key = tile.get_size_signature();
            groups.entry(key).or_insert_with(Vec::new).push(tile.clone());
        }
        
        groups
    }

    /// Группирует панели по материалам
    pub fn group_by_material(tiles: &[TileDimensions]) -> std::collections::HashMap<String, Vec<TileDimensions>> {
        let mut groups = std::collections::HashMap::new();
        
        for tile in tiles {
            groups.entry(tile.material.clone()).or_insert_with(Vec::new).push(tile.clone());
        }
        
        groups
    }

    /// Находит наиболее подходящую панель для размещения
    pub fn find_best_fit(
        target: &TileDimensions, 
        candidates: &[TileDimensions]
    ) -> Option<(usize, TileDimensions)> {
        let mut best_fit = None;
        let mut best_waste = i64::MAX;
        
        for (index, candidate) in candidates.iter().enumerate() {
            if candidate.fits(target) {
                let waste = candidate.get_area() - target.get_area();
                if waste < best_waste {
                    best_waste = waste;
                    best_fit = Some((index, candidate.clone()));
                }
            }
        }
        
        best_fit
    }

    /// Вычисляет общую статистику набора панелей
    pub fn calculate_statistics(tiles: &[TileDimensions]) -> TileStatistics {
        if tiles.is_empty() {
            return TileStatistics::default();
        }

        let total_area: i64 = tiles.iter().map(|t| t.get_area()).sum();
        let avg_area = total_area as f64 / tiles.len() as f64;
        
        let max_area = tiles.iter().map(|t| t.get_area()).max().unwrap_or(0);
        let min_area = tiles.iter().map(|t| t.get_area()).min().unwrap_or(0);
        
        let max_width = tiles.iter().map(|t| t.width).max().unwrap_or(0);
        let max_height = tiles.iter().map(|t| t.height).max().unwrap_or(0);
        let min_width = tiles.iter().map(|t| t.width).min().unwrap_or(0);
        let min_height = tiles.iter().map(|t| t.height).min().unwrap_or(0);
        
        let aspect_ratios: Vec<f64> = tiles.iter().map(|t| t.get_aspect_ratio()).collect();
        let avg_aspect_ratio = aspect_ratios.iter().sum::<f64>() / aspect_ratios.len() as f64;
        
        let unique_sizes = TileUtils::group_by_size(tiles).len();
        let unique_materials = TileUtils::group_by_material(tiles).len();
        
        let square_count = tiles.iter().filter(|t| t.is_square()).count();
        let elongated_count = tiles.iter().filter(|t| t.is_elongated(2.0)).count();
        
        TileStatistics {
            total_count: tiles.len(),
            total_area,
            avg_area,
            max_area,
            min_area,
            max_width,
            max_height,
            min_width,
            min_height,
            avg_aspect_ratio,
            unique_sizes,
            unique_materials,
            square_count,
            elongated_count,
        }
    }

    /// Сортирует панели по оптимальному порядку размещения
    pub fn sort_for_optimal_placement(tiles: &mut [TileDimensions]) {
        tiles.sort_by(|a, b| {
            // Сначала по приоритету размещения (убывание)
            match b.get_placement_priority().cmp(&a.get_placement_priority()) {
                std::cmp::Ordering::Equal => {
                    // Затем по площади (убывание)
                    match b.get_area().cmp(&a.get_area()) {
                        std::cmp::Ordering::Equal => {
                            // Затем по сложности (возрастание - проще сначала)
                            a.get_placement_complexity().partial_cmp(&b.get_placement_complexity())
                                .unwrap_or(std::cmp::Ordering::Equal)
                        }
                        other => other,
                    }
                }
                other => other,
            }
        });
    }

    /// Проверяет совместимость набора панелей с контейнерами
    pub fn check_compatibility(
        tiles: &[TileDimensions], 
        containers: &[TileDimensions]
    ) -> CompatibilityReport {
        let mut compatible_tiles = 0;
        let mut material_mismatches = 0;
        let mut size_mismatches = 0;
        let mut no_container_found = 0;

        for tile in tiles {
            let mut found_compatible = false;
            let mut found_size_fit = false;
            let mut found_material_match = false;

            for container in containers {
                if container.is_material_compatible(tile) {
                    found_material_match = true;
                    if container.fits(tile) {
                        found_size_fit = true;
                        found_compatible = true;
                        break;
                    }
                }
            }

            if found_compatible {
                compatible_tiles += 1;
            } else if !found_material_match {
                material_mismatches += 1;
            } else if !found_size_fit {
                size_mismatches += 1;
            } else {
                no_container_found += 1;
            }
        }

        CompatibilityReport {
            total_tiles: tiles.len(),
            compatible_tiles,
            material_mismatches,
            size_mismatches,
            no_container_found,
        }
    }

    /// Создает оптимизированные группы панелей для обработки
    pub fn create_processing_groups(
        tiles: &[TileDimensions], 
        max_group_size: usize
    ) -> Vec<Vec<TileDimensions>> {
        let mut groups = Vec::new();
        let size_groups = TileUtils::group_by_size(tiles);

        for (_, size_group) in size_groups {
            let material_groups = TileUtils::group_by_material(&size_group);
            
            for (_, material_group) in material_groups {
                // Разбиваем большие группы на подгруппы
                for chunk in material_group.chunks(max_group_size) {
                    groups.push(chunk.to_vec());
                }
            }
        }

        // Сортируем группы по общей площади (убывание)
        groups.sort_by(|a, b| {
            let area_a: i64 = a.iter().map(|t| t.get_area()).sum();
            let area_b: i64 = b.iter().map(|t| t.get_area()).sum();
            area_b.cmp(&area_a)
        });

        groups
    }

    /// Валидирует набор панелей на корректность
    pub fn validate_tile_set(tiles: &[TileDimensions]) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for (index, tile) in tiles.iter().enumerate() {
            // Проверяем размеры
            if !tile.is_valid_dimensions() {
                errors.push(format!("Панель {}: неверные размеры {}x{}", 
                    index, tile.width, tile.height));
            }

            // Проверяем материал
            if tile.material.is_empty() {
                warnings.push(format!("Панель {}: пустой материал", index));
            }

            // Проверяем экстремальные пропорции
            if tile.is_elongated(10.0) {
                warnings.push(format!("Панель {}: экстремальные пропорции ({}:1)", 
                    index, tile.get_aspect_ratio().max(1.0 / tile.get_aspect_ratio())));
            }
        }

        // Проверяем дубликаты
        let mut seen_signatures = std::collections::HashSet::new();
        for (index, tile) in tiles.iter().enumerate() {
            let signature = format!("{}_{}", tile.get_size_signature(), tile.material);
            if !seen_signatures.insert(signature.clone()) {
                warnings.push(format!("Панель {}: дубликат {}", index, signature));
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
}

/// Статистика набора панелей
#[derive(Debug, Clone)]
pub struct TileStatistics {
    pub total_count: usize,
    pub total_area: i64,
    pub avg_area: f64,
    pub max_area: i64,
    pub min_area: i64,
    pub max_width: i32,
    pub max_height: i32,
    pub min_width: i32,
    pub min_height: i32,
    pub avg_aspect_ratio: f64,
    pub unique_sizes: usize,
    pub unique_materials: usize,
    pub square_count: usize,
    pub elongated_count: usize,
}

impl Default for TileStatistics {
    fn default() -> Self {
        Self {
            total_count: 0,
            total_area: 0,
            avg_area: 0.0,
            max_area: 0,
            min_area: 0,
            max_width: 0,
            max_height: 0,
            min_width: 0,
            min_height: 0,
            avg_aspect_ratio: 0.0,
            unique_sizes: 0,
            unique_materials: 0,
            square_count: 0,
            elongated_count: 0,
        }
    }
}

impl std::fmt::Display for TileStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Панели: {} шт, общая площадь: {}, средняя: {:.0}, размеры: {}x{} - {}x{}, уникальных размеров: {}, материалов: {}, квадратных: {}, вытянутых: {}",
            self.total_count,
            self.total_area,
            self.avg_area,
            self.min_width, self.min_height,
            self.max_width, self.max_height,
            self.unique_sizes,
            self.unique_materials,
            self.square_count,
            self.elongated_count
        )
    }
}

/// Отчет о совместимости панелей с контейнерами
#[derive(Debug, Clone)]
pub struct CompatibilityReport {
    pub total_tiles: usize,
    pub compatible_tiles: usize,
    pub material_mismatches: usize,
    pub size_mismatches: usize,
    pub no_container_found: usize,
}

impl CompatibilityReport {
    pub fn get_compatibility_percentage(&self) -> f64 {
        if self.total_tiles == 0 {
            return 0.0;
        }
        (self.compatible_tiles as f64 / self.total_tiles as f64) * 100.0
    }
}

impl std::fmt::Display for CompatibilityReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Совместимость: {}/{} панелей ({:.1}%), несовпадений материалов: {}, размеров: {}, без контейнера: {}",
            self.compatible_tiles,
            self.total_tiles,
            self.get_compatibility_percentage(),
            self.material_mismatches,
            self.size_mismatches,
            self.no_container_found
        )
    }
}

/// Результат валидации набора панелей
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl std::fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_valid {
            write!(f, "Валидация пройдена")
        } else {
            write!(f, "Валидация не пройдена: {} ошибок", self.errors.len())
        }?;
        
        if !self.warnings.is_empty() {
            write!(f, ", {} предупреждений", self.warnings.len())?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_dimensions_basic() {
        let tile = TileDimensions::simple(100, 200);
        
        assert_eq!(tile.get_area(), 20000);
        assert_eq!(tile.get_max_dimension(), 200);
        assert_eq!(tile.get_min_dimension(), 100);
        assert!(!tile.is_square());
        assert_eq!(tile.get_aspect_ratio(), 0.5);
    }

    #[test]
    fn test_tile_rotation() {
        let tile = TileDimensions::simple(100, 200);
        let rotated = tile.rotate90();
        
        assert_eq!(rotated.width, 200);
        assert_eq!(rotated.height, 100);
        assert_eq!(rotated.is_rotated, true);
        assert_eq!(tile.get_area(), rotated.get_area());
    }

    #[test]
    fn test_tile_fits() {
        let container = TileDimensions::simple(300, 400);
        let tile1 = TileDimensions::simple(200, 300); // Помещается
        let tile2 = TileDimensions::simple(350, 200); // Помещается с поворотом
        let tile3 = TileDimensions::simple(400, 500); // Не помещается
        
        assert!(container.fits(&tile1));
        assert!(container.fits(&tile2));
        assert!(!container.fits(&tile3));
    }

    #[test]
    fn test_tile_statistics() {
        let tiles = vec![
            TileDimensions::simple(100, 100), // Квадратная
            TileDimensions::simple(200, 100), // Обычная
            TileDimensions::simple(500, 50),  // Вытянутая
        ];
        
        let stats = TileUtils::calculate_statistics(&tiles);
        
        assert_eq!(stats.total_count, 3);
        assert_eq!(stats.square_count, 1);
        assert_eq!(stats.elongated_count, 1);
        assert_eq!(stats.unique_sizes, 3);
    }

    #[test]
    fn test_grouping_by_size() {
        let tiles = vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(200, 100), // Тот же размер
            TileDimensions::simple(150, 300),
        ];
        
        let groups = TileUtils::group_by_size(&tiles);
        
        assert_eq!(groups.len(), 2); // 100x200 и 150x300
        assert!(groups.contains_key("100x200"));
        assert!(groups.contains_key("150x300"));
        assert_eq!(groups["100x200"].len(), 2);
    }

    #[test]
    fn test_placement_priority() {
        let small_tile = TileDimensions::simple(50, 50);
        let large_tile = TileDimensions::simple(200, 200);
        let elongated_tile = TileDimensions::simple(500, 25);
        
        assert!(large_tile.get_placement_priority() > small_tile.get_placement_priority());
        assert!(large_tile.get_placement_priority() > elongated_tile.get_placement_priority());
    }

    #[test]
    fn test_validation() {
        let tiles = vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(0, 100),    // Неверные размеры
            TileDimensions::simple(100, 200),  // Дубликат
        ];
        
        let result = TileUtils::validate_tile_set(&tiles);
        
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_optimal_orientation() {
        let tile = TileDimensions::simple(100, 200);
        let container_wide = (300, 150);
        let container_tall = (150, 300);
        
        let oriented_wide = tile.get_optimal_orientation(container_wide.0, container_wide.1);
        let oriented_tall = tile.get_optimal_orientation(container_tall.0, container_tall.1);
        
        // В широком контейнере панель должна лечь горизонтально
        assert_eq!(oriented_wide.width, 200);
        assert_eq!(oriented_wide.height, 100);
        
        // В высоком контейнере панель должна остаться вертикальной
        assert_eq!(oriented_tall.width, 100);
        assert_eq!(oriented_tall.height, 200);
    }
}
