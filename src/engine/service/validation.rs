use crate::engine::model::response::StatusCode;

/// Проверяет валидность панелей
pub fn validate_panels(panels: &[crate::engine::model::request::Panel]) -> (usize, StatusCode) {
    let mut count = 0;
    for panel in panels {
        if panel.is_valid() {
            count += panel.count as usize;
        }
    }

    if count == 0 {
        return (0, StatusCode::InvalidTiles);
    }

    if count > 5000 {
        return (count, StatusCode::TooManyPanels);
    }

    (count, StatusCode::Ok)
}

/// Проверяет валидность складских панелей
pub fn validate_stock_panels(stock_panels: &[crate::engine::model::request::Panel]) -> (usize, StatusCode) {
    let mut count = 0;
    for panel in stock_panels {
        if panel.is_valid() {
            count += panel.count as usize;
        }
    }

    if count == 0 {
        return (0, StatusCode::InvalidStockTiles);
    }

    if count > 5000 {
        return (count, StatusCode::TooManyStockPanels);
    }

    (count, StatusCode::Ok)
}

/// Проверяет валидность размеров панели
pub fn validate_panel_dimensions(width: &str, height: &str) -> Result<(i32, i32), String> {
    let width_f64 = width.parse::<f64>()
        .map_err(|_| format!("Неверный формат ширины: '{}'", width))?;
    
    let height_f64 = height.parse::<f64>()
        .map_err(|_| format!("Неверный формат высоты: '{}'", height))?;

    if width_f64 <= 0.0 || height_f64 <= 0.0 {
        return Err("Размеры должны быть положительными".to_string());
    }

    if width_f64 > 100000.0 || height_f64 > 100000.0 {
        return Err("Размеры слишком большие".to_string());
    }

    Ok((width_f64 as i32, height_f64 as i32))
}

/// Проверяет количество цифр в числе (как в Java)
pub fn get_number_of_decimal_places(value: &str) -> usize {
    if let Some(dot_pos) = value.find('.') {
        value.len() - dot_pos - 1
    } else {
        0
    }
}

/// Проверяет количество цифр в целой части
pub fn get_number_of_integer_places(value: &str) -> usize {
    if let Some(dot_pos) = value.find('.') {
        dot_pos
    } else {
        value.len()
    }
}

/// Проверяет общее количество значащих цифр
pub fn validate_precision(
    panels: &[crate::engine::model::request::Panel],
    stock_panels: &[crate::engine::model::request::Panel],
    cut_thickness: &str,
    min_trim_dimension: &str,
    max_allowed_digits: usize
) -> Result<f64, String> {
    let mut max_decimal_places = 0;
    let mut max_integer_places = 0;

    // Проверяем панели
    for panel in panels {
        if panel.is_valid() {
            max_decimal_places = max_decimal_places
                .max(get_number_of_decimal_places(&panel.width))
                .max(get_number_of_decimal_places(&panel.height));
            
            max_integer_places = max_integer_places
                .max(get_number_of_integer_places(&panel.width))
                .max(get_number_of_integer_places(&panel.height));
        }
    }

    // Проверяем складские панели
    for panel in stock_panels {
        if panel.is_valid() {
            max_decimal_places = max_decimal_places
                .max(get_number_of_decimal_places(&panel.width))
                .max(get_number_of_decimal_places(&panel.height));
            
            max_integer_places = max_integer_places
                .max(get_number_of_integer_places(&panel.width))
                .max(get_number_of_integer_places(&panel.height));
        }
    }

    // Проверяем параметры конфигурации
    max_decimal_places = max_decimal_places
        .max(get_number_of_decimal_places(cut_thickness))
        .max(get_number_of_decimal_places(min_trim_dimension));
    
    max_integer_places = max_integer_places
        .max(get_number_of_integer_places(cut_thickness))
        .max(get_number_of_integer_places(min_trim_dimension));

    // Проверяем общее количество цифр
    if max_decimal_places + max_integer_places > max_allowed_digits {
        // Ограничиваем количество десятичных знаков
        max_decimal_places = max_allowed_digits.saturating_sub(max_integer_places);
        
        println!("⚠️ Превышено максимальное количество цифр: десятичных[{}] целых[{}] максимум[{}]", 
            max_decimal_places, max_integer_places, max_allowed_digits);
    }

    // ИСПРАВЛЕНИЕ: Возвращаем правильный коэффициент масштабирования
    // В Java это было Math.pow(10.0d, maxDecimalPlaces)
    // где maxDecimalPlaces - это количество десятичных знаков после обработки
    let scale_factor = 10_f64.powi(max_decimal_places as i32);
    
    Ok(scale_factor)
}

/// Проверяет валидность конфигурации
pub fn validate_configuration(config: &crate::engine::model::request::Configuration) -> Result<(), String> {
    // Проверяем толщину реза
    if let Err(e) = config.cut_thickness.parse::<f64>() {
        return Err(format!("Неверная толщина реза: {}", e));
    }

    // Проверяем минимальный размер обрезки
    if let Err(e) = config.min_trim_dimension.parse::<f64>() {
        return Err(format!("Неверный минимальный размер обрезки: {}", e));
    }

    // Проверяем коэффициент оптимизации
    if config.optimization_factor < 0.0 || config.optimization_factor > 10.0 {
        return Err("Коэффициент оптимизации должен быть от 0.0 до 10.0".to_string());
    }

    // Проверяем предпочтение ориентации разрезов
    if config.cut_orientation_preference < 0 || config.cut_orientation_preference > 2 {
        return Err("Предпочтение ориентации разрезов должно быть 0, 1 или 2".to_string());
    }

    // Проверяем пороги производительности
    if let Some(ref thresholds) = config.performance_thresholds {
        if thresholds.max_simultaneous_tasks == 0 {
            return Err("Максимальное количество одновременных задач должно быть больше 0".to_string());
        }
        
        if thresholds.max_simultaneous_threads == 0 {
            return Err("Максимальное количество одновременных потоков должно быть больше 0".to_string());
        }
        
        if thresholds.thread_check_interval == 0 {
            return Err("Интервал проверки потоков должен быть больше 0".to_string());
        }
    }

    Ok(())
}

/// Проверяет валидность информации о клиенте
pub fn validate_client_info(client_info: &crate::engine::model::request::ClientInfo) -> Result<(), String> {
    if client_info.id.is_empty() {
        return Err("ID клиента не может быть пустым".to_string());
    }

    if client_info.id.len() > 100 {
        return Err("ID клиента слишком длинный (максимум 100 символов)".to_string());
    }

    // Проверяем, что ID содержит только допустимые символы
    if !client_info.id.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err("ID клиента может содержать только буквы, цифры, _ и -".to_string());
    }

    Ok(())
}

/// Комплексная проверка запроса на расчет
pub fn validate_calculation_request(request: &crate::engine::model::request::CalculationRequest) -> Result<ValidationSummary, String> {
    let mut summary = ValidationSummary::new();

    // Проверяем информацию о клиенте
    validate_client_info(&request.client_info)?;
    summary.client_validated = true;

    // Проверяем конфигурацию
    validate_configuration(&request.configuration)?;
    summary.configuration_validated = true;

    // Проверяем панели
    let (panel_count, panel_status) = validate_panels(&request.panels);
    if panel_status != StatusCode::Ok {
        return Err(format!("Ошибка валидации панелей: {}", panel_status.description()));
    }
    summary.panel_count = panel_count;
    summary.panels_validated = true;

    // Проверяем складские панели
    let (stock_count, stock_status) = validate_stock_panels(&request.stock_panels);
    if stock_status != StatusCode::Ok {
        return Err(format!("Ошибка валидации складских панелей: {}", stock_status.description()));
    }
    summary.stock_panel_count = stock_count;
    summary.stock_panels_validated = true;

    // Проверяем точность
    let scale_factor = validate_precision(
        &request.panels,
        &request.stock_panels,
        &request.configuration.cut_thickness,
        &request.configuration.min_trim_dimension,
        6 // MAX_ALLOWED_DIGITS
    )?;
    summary.scale_factor = scale_factor;
    summary.precision_validated = true;

    // Проверяем размеры панелей
    let mut invalid_panels = Vec::new();
    for (i, panel) in request.panels.iter().enumerate() {
        if panel.is_valid() {
            if let Err(e) = validate_panel_dimensions(&panel.width, &panel.height) {
                invalid_panels.push(format!("Панель {}: {}", i + 1, e));
            }
        }
    }

    for (i, panel) in request.stock_panels.iter().enumerate() {
        if panel.is_valid() {
            if let Err(e) = validate_panel_dimensions(&panel.width, &panel.height) {
                invalid_panels.push(format!("Складская панель {}: {}", i + 1, e));
            }
        }
    }

    if !invalid_panels.is_empty() {
        return Err(format!("Неверные размеры панелей: {}", invalid_panels.join("; ")));
    }
    summary.dimensions_validated = true;

    Ok(summary)
}

/// Сводка результатов валидации
#[derive(Debug, Clone)]
pub struct ValidationSummary {
    pub client_validated: bool,
    pub configuration_validated: bool,
    pub panels_validated: bool,
    pub stock_panels_validated: bool,
    pub precision_validated: bool,
    pub dimensions_validated: bool,
    pub panel_count: usize,
    pub stock_panel_count: usize,
    pub scale_factor: f64,
}

impl ValidationSummary {
    pub fn new() -> Self {
        Self {
            client_validated: false,
            configuration_validated: false,
            panels_validated: false,
            stock_panels_validated: false,
            precision_validated: false,
            dimensions_validated: false,
            panel_count: 0,
            stock_panel_count: 0,
            scale_factor: 1.0,
        }
    }

    pub fn is_fully_validated(&self) -> bool {
        self.client_validated &&
        self.configuration_validated &&
        self.panels_validated &&
        self.stock_panels_validated &&
        self.precision_validated &&
        self.dimensions_validated
    }

    pub fn get_total_panels(&self) -> usize {
        self.panel_count + self.stock_panel_count
    }
}

impl std::fmt::Display for ValidationSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Validation: {} panels, {} stock panels, scale factor: {:.0}, fully validated: {}",
            self.panel_count,
            self.stock_panel_count,
            self.scale_factor,
            self.is_fully_validated()
        )
    }
}

/// Утилиты для валидации материалов
pub struct MaterialValidation;

impl MaterialValidation {
    /// Проверяет совместимость материалов между панелями и складскими панелями
    pub fn validate_material_compatibility(
        panels: &[crate::engine::model::request::Panel],
        stock_panels: &[crate::engine::model::request::Panel]
    ) -> Result<MaterialCompatibilitySummary, String> {
        let mut panel_materials = std::collections::HashSet::new();
        let mut stock_materials = std::collections::HashSet::new();

        // Собираем материалы из панелей
        for panel in panels {
            if panel.is_valid() {
                panel_materials.insert(panel.material.clone());
            }
        }

        // Собираем материалы из складских панелей
        for stock_panel in stock_panels {
            if stock_panel.is_valid() {
                stock_materials.insert(stock_panel.material.clone());
            }
        }

        // Находим пересечения и различия
        let compatible_materials: std::collections::HashSet<_> = panel_materials
            .intersection(&stock_materials)
            .cloned()
            .collect();

        let panels_without_stock: std::collections::HashSet<_> = panel_materials
            .difference(&stock_materials)
            .cloned()
            .collect();

        let stock_without_panels: std::collections::HashSet<_> = stock_materials
            .difference(&panel_materials)
            .cloned()
            .collect();

        let summary = MaterialCompatibilitySummary {
            panel_materials,
            stock_materials,
            compatible_materials,
            panels_without_stock,
            stock_without_panels,
        };

        // Предупреждаем о несовместимых материалах
        if !summary.panels_without_stock.is_empty() {
            println!("⚠️ Предупреждение: панели с материалами {:?} не имеют соответствующих складских панелей", 
                summary.panels_without_stock);
        }

        if summary.compatible_materials.is_empty() {
            return Err("Нет совместимых материалов между панелями и складскими панелями".to_string());
        }

        Ok(summary)
    }
}

/// Сводка совместимости материалов
#[derive(Debug, Clone)]
pub struct MaterialCompatibilitySummary {
    pub panel_materials: std::collections::HashSet<String>,
    pub stock_materials: std::collections::HashSet<String>,
    pub compatible_materials: std::collections::HashSet<String>,
    pub panels_without_stock: std::collections::HashSet<String>,
    pub stock_without_panels: std::collections::HashSet<String>,
}

impl MaterialCompatibilitySummary {
    pub fn get_compatibility_ratio(&self) -> f64 {
        if self.panel_materials.is_empty() {
            return 0.0;
        }
        self.compatible_materials.len() as f64 / self.panel_materials.len() as f64
    }

    pub fn has_full_compatibility(&self) -> bool {
        self.panels_without_stock.is_empty()
    }
}

impl std::fmt::Display for MaterialCompatibilitySummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Materials: {} panel types, {} stock types, {} compatible ({:.1}% coverage)",
            self.panel_materials.len(),
            self.stock_materials.len(),
            self.compatible_materials.len(),
            self.get_compatibility_ratio() * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::model::request::Panel;

    #[test]
    fn test_validate_panel_dimensions() {
        assert!(validate_panel_dimensions("100", "200").is_ok());
        assert!(validate_panel_dimensions("100.5", "200.7").is_ok());
        assert!(validate_panel_dimensions("0", "200").is_err());
        assert!(validate_panel_dimensions("abc", "200").is_err());
    }

    #[test]
    fn test_decimal_places() {
        assert_eq!(get_number_of_decimal_places("123.45"), 2);
        assert_eq!(get_number_of_decimal_places("123"), 0);
        assert_eq!(get_number_of_decimal_places("0.1"), 1);
    }

    #[test]
    fn test_integer_places() {
        assert_eq!(get_number_of_integer_places("123.45"), 3);
        assert_eq!(get_number_of_integer_places("123"), 3);
        assert_eq!(get_number_of_integer_places("0.1"), 1);
    }

    #[test]
    fn test_validate_panels() {
        let panels = vec![
            Panel::new(1, "100".to_string(), "200".to_string(), 2, None),
            Panel::new(2, "150".to_string(), "300".to_string(), 1, None),
        ];
        
        let (count, status) = validate_panels(&panels);
        assert_eq!(count, 3); // 2 + 1
        assert_eq!(status, StatusCode::Ok);
    }
}
