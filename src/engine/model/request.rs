use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::types::DEFAULT_MATERIAL;

/// Запрос на расчет оптимизации раскроя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationRequest {
    /// Информация о клиенте
    pub client_info: ClientInfo,
    /// Конфигурация расчета
    pub configuration: Configuration,
    /// Список панелей для раскроя
    pub panels: Vec<Panel>,
    /// Список складских панелей
    pub stock_panels: Vec<Panel>,
}

impl CalculationRequest {
    /// Создает новый запрос на расчет
    pub fn new(
        client_info: ClientInfo,
        configuration: Configuration,
        panels: Vec<Panel>,
        stock_panels: Vec<Panel>,
    ) -> Self {
        Self {
            client_info,
            configuration,
            panels,
            stock_panels,
        }
    }

    /// Возвращает строковое представление панелей
    pub fn tiles_to_string(&self) -> String {
        let mut result = String::new();
        for panel in &self.panels {
            if panel.count > 0 {
                result.push_str(&format!(" {}", panel));
            }
        }
        result
    }

    /// Возвращает строковое представление складских панелей
    pub fn base_tiles_to_string(&self) -> String {
        let mut result = String::new();
        for panel in &self.stock_panels {
            if panel.count > 0 {
                result.push_str(&format!(" {}", panel));
            }
        }
        result
    }

    /// Подсчитывает общее количество валидных панелей
    pub fn count_valid_panels(&self) -> usize {
        self.panels.iter()
            .filter(|panel| panel.is_valid())
            .map(|panel| panel.count as usize)
            .sum()
    }

    /// Подсчитывает общее количество валидных складских панелей
    pub fn count_valid_stock_panels(&self) -> usize {
        self.stock_panels.iter()
            .filter(|panel| panel.is_valid())
            .map(|panel| panel.count as usize)
            .sum()
    }
}

/// Панель для раскроя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    /// Уникальный идентификатор
    pub id: i32,
    /// Ширина (строковое представление для поддержки дробных чисел)
    pub width: String,
    /// Высота (строковое представление для поддержки дробных чисел)
    pub height: String,
    /// Количество панелей
    pub count: i32,
    /// Материал панели
    pub material: String,
    /// Включена ли панель в расчет
    pub enabled: bool,
    /// Ориентация панели (0 - любая, 1 - только горизонтальная, 2 - только вертикальная)
    pub orientation: i32,
    /// Метка панели
    pub label: Option<String>,
    /// Информация о кромке
    pub edge: Option<Edge>,
}

impl Panel {
    /// Создает новую панель
    pub fn new(
        id: i32,
        width: String,
        height: String,
        count: i32,
        material: Option<String>,
    ) -> Self {
        Self {
            id,
            width,
            height,
            count,
            material: material.unwrap_or_else(|| DEFAULT_MATERIAL.to_string()),
            enabled: true,
            orientation: 0,
            label: None,
            edge: None,
        }
    }

    /// Проверяет валидность панели
    pub fn is_valid(&self) -> bool {
        if !self.enabled || self.count <= 0 {
            return false;
        }

        // Проверяем, что ширина и высота являются положительными числами
        match (self.width.parse::<f64>(), self.height.parse::<f64>()) {
            (Ok(w), Ok(h)) => w > 0.0 && h > 0.0,
            _ => false,
        }
    }

    /// Возвращает ширину как число
    pub fn get_width_f64(&self) -> Result<f64, std::num::ParseFloatError> {
        self.width.parse()
    }

    /// Возвращает высоту как число
    pub fn get_height_f64(&self) -> Result<f64, std::num::ParseFloatError> {
        self.height.parse()
    }

    /// Возвращает площадь панели
    pub fn get_area(&self) -> Result<f64, std::num::ParseFloatError> {
        Ok(self.get_width_f64()? * self.get_height_f64()?)
    }
}

impl std::fmt::Display for Panel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}x{}]*{}{}",
            self.width,
            self.height,
            self.count,
            if self.enabled { "" } else { "-disabled" }
        )
    }
}

/// Информация о кромке панели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// Верхняя кромка
    pub top: Option<String>,
    /// Левая кромка
    pub left: Option<String>,
    /// Нижняя кромка
    pub bottom: Option<String>,
    /// Правая кромка
    pub right: Option<String>,
}

impl Edge {
    /// Создает новую кромку
    pub fn new() -> Self {
        Self {
            top: None,
            left: None,
            bottom: None,
            right: None,
        }
    }

    /// Создает кромку со всеми сторонами
    pub fn all_sides(edge_type: String) -> Self {
        Self {
            top: Some(edge_type.clone()),
            left: Some(edge_type.clone()),
            bottom: Some(edge_type.clone()),
            right: Some(edge_type),
        }
    }

    /// Проверяет, есть ли кромка с какой-либо стороны
    pub fn has_any_edge(&self) -> bool {
        self.top.is_some() || self.left.is_some() || self.bottom.is_some() || self.right.is_some()
    }
}

impl Default for Edge {
    fn default() -> Self {
        Self::new()
    }
}

/// Информация о клиенте
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// Уникальный идентификатор клиента
    pub id: String,
    /// Имя клиента
    pub name: Option<String>,
    /// Версия клиентского приложения
    pub version: Option<String>,
    /// Платформа клиента
    pub platform: Option<String>,
    /// Дополнительные метаданные
    pub metadata: HashMap<String, String>,
}

impl ClientInfo {
    /// Создает новую информацию о клиенте
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: None,
            version: None,
            platform: None,
            metadata: HashMap::new(),
        }
    }

    /// Создает информацию о клиенте с дополнительными данными
    pub fn with_details(
        id: String,
        name: Option<String>,
        version: Option<String>,
        platform: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            version,
            platform,
            metadata: HashMap::new(),
        }
    }

    /// Добавляет метаданные
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Получает метаданные по ключу
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Конфигурация расчета
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    /// Толщина пропила
    pub cut_thickness: String,
    /// Минимальный размер обрезки
    pub min_trim_dimension: String,
    /// Фактор оптимизации (0.0 - 1.0)
    pub optimization_factor: f64,
    /// Использовать только одну складскую единицу
    pub use_single_stock_unit: bool,
    /// Предпочтение ориентации разрезов (0 - любая, 1 - горизонтальные, 2 - вертикальные)
    pub cut_orientation_preference: i32,
    /// Пороги производительности
    pub performance_thresholds: Option<PerformanceThresholds>,
}

impl Configuration {
    /// Создает конфигурацию по умолчанию
    pub fn default() -> Self {
        Self {
            cut_thickness: "3.0".to_string(),
            min_trim_dimension: "10.0".to_string(),
            optimization_factor: 1.0,
            use_single_stock_unit: false,
            cut_orientation_preference: 0,
            performance_thresholds: Some(PerformanceThresholds::default()),
        }
    }

    /// Возвращает толщину пропила как число
    pub fn get_cut_thickness_f64(&self) -> Result<f64, std::num::ParseFloatError> {
        self.cut_thickness.parse()
    }

    /// Возвращает минимальный размер обрезки как число
    pub fn get_min_trim_dimension_f64(&self) -> Result<f64, std::num::ParseFloatError> {
        self.min_trim_dimension.parse()
    }

    /// Проверяет валидность конфигурации
    pub fn is_valid(&self) -> bool {
        self.get_cut_thickness_f64().is_ok() &&
        self.get_min_trim_dimension_f64().is_ok() &&
        self.optimization_factor >= 0.0 &&
        self.optimization_factor <= 1.0 &&
        self.cut_orientation_preference >= 0 &&
        self.cut_orientation_preference <= 2
    }
}

/// Пороги производительности
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Максимальное количество одновременных потоков
    pub max_simultaneous_threads: usize,
    /// Интервал проверки потоков (в миллисекундах)
    pub thread_check_interval: u64,
    /// Максимальное количество одновременных задач на клиента
    pub max_simultaneous_tasks: usize,
}

impl PerformanceThresholds {
    /// Создает пороги производительности по умолчанию
    pub fn default() -> Self {
        Self {
            max_simultaneous_threads: 5,
            thread_check_interval: 1000,
            max_simultaneous_tasks: 2,
        }
    }

    /// Создает пороги производительности с заданными параметрами
    pub fn new(
        max_simultaneous_threads: usize,
        thread_check_interval: u64,
        max_simultaneous_tasks: usize,
    ) -> Self {
        Self {
            max_simultaneous_threads,
            thread_check_interval,
            max_simultaneous_tasks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_validation() {
        let valid_panel = Panel::new(1, "100.5".to_string(), "200.0".to_string(), 5, None);
        assert!(valid_panel.is_valid());

        let invalid_panel = Panel {
            enabled: false,
            ..valid_panel.clone()
        };
        assert!(!invalid_panel.is_valid());

        let zero_count_panel = Panel {
            count: 0,
            ..valid_panel.clone()
        };
        assert!(!zero_count_panel.is_valid());

        let invalid_width_panel = Panel {
            width: "invalid".to_string(),
            ..valid_panel.clone()
        };
        assert!(!invalid_width_panel.is_valid());
    }

    #[test]
    fn test_panel_area_calculation() {
        let panel = Panel::new(1, "10.0".to_string(), "20.0".to_string(), 1, None);
        assert_eq!(panel.get_area().unwrap(), 200.0);
    }

    #[test]
    fn test_calculation_request_counting() {
        let panels = vec![
            Panel::new(1, "100".to_string(), "200".to_string(), 3, None),
            Panel::new(2, "150".to_string(), "250".to_string(), 2, None),
        ];
        
        let stock_panels = vec![
            Panel::new(3, "1000".to_string(), "600".to_string(), 1, None),
        ];

        let request = CalculationRequest::new(
            ClientInfo::new("test_client".to_string()),
            Configuration::default(),
            panels,
            stock_panels,
        );

        assert_eq!(request.count_valid_panels(), 5);
        assert_eq!(request.count_valid_stock_panels(), 1);
    }

    #[test]
    fn test_configuration_validation() {
        let valid_config = Configuration::default();
        assert!(valid_config.is_valid());

        let invalid_config = Configuration {
            optimization_factor: 1.5, // Больше 1.0
            ..valid_config
        };
        assert!(!invalid_config.is_valid());
    }

    #[test]
    fn test_edge_functionality() {
        let edge = Edge::all_sides("PVC".to_string());
        assert!(edge.has_any_edge());

        let empty_edge = Edge::new();
        assert!(!empty_edge.has_any_edge());
    }

    #[test]
    fn test_client_info_metadata() {
        let mut client_info = ClientInfo::new("test_client".to_string());
        client_info.add_metadata("app_version".to_string(), "1.0.0".to_string());
        
        assert_eq!(client_info.get_metadata("app_version"), Some(&"1.0.0".to_string()));
        assert_eq!(client_info.get_metadata("nonexistent"), None);
    }
}
