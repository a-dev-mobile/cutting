use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::engine::model::{Solution, TileDimensions};
use crate::engine::model::request::CalculationRequest;
use crate::TaskReport;

/// Ответ на запрос расчета оптимизации раскроя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResponse {
    /// Список оптимизированных панелей
    pub panels: Vec<OptimizedPanel>,
    /// Панели, которые не поместились
    pub no_fit_panels: Vec<TileDimensions>,
    /// Панели без подходящего материала
    pub no_material_panels: Vec<TileDimensions>,
    /// Общая статистика
    pub statistics: ResponseStatistics,
    /// Метаданные ответа
    pub metadata: HashMap<String, String>,
}

impl CalculationResponse {
    /// Создает новый ответ
    pub fn new() -> Self {
        Self {
            panels: Vec::new(),
            no_fit_panels: Vec::new(),
            no_material_panels: Vec::new(),
            statistics: ResponseStatistics::new(),
            metadata: HashMap::new(),
        }
    }

    /// Проверяет, есть ли панели в ответе
    pub fn has_panels(&self) -> bool {
        !self.panels.is_empty()
    }

    /// Проверяет, все ли панели поместились
    pub fn all_panels_fit(&self) -> bool {
        self.no_fit_panels.is_empty() && self.no_material_panels.is_empty()
    }

    /// Возвращает общее количество панелей
    pub fn total_panels_count(&self) -> usize {
        self.panels.len() + self.no_fit_panels.len() + self.no_material_panels.len()
    }

    /// Добавляет метаданные
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

impl Default for CalculationResponse {
    fn default() -> Self {
        Self::new()
    }
}

/// Оптимизированная панель в ответе
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedPanel {
    /// Исходная информация о панели
    pub tile_dimensions: TileDimensions,
    /// Позиция на складской панели
    pub position: PanelPosition,
    /// Идентификатор складской панели
    pub stock_panel_id: String,
    /// Материал
    pub material: String,
    /// Дополнительная информация
    pub metadata: HashMap<String, String>,
}

impl OptimizedPanel {
    /// Создает новую оптимизированную панель
    pub fn new(
        tile_dimensions: TileDimensions,
        position: PanelPosition,
        stock_panel_id: String,
        material: String,
    ) -> Self {
        Self {
            tile_dimensions,
            position,
            stock_panel_id,
            material,
            metadata: HashMap::new(),
        }
    }
}

/// Позиция панели на складской панели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelPosition {
    /// Координата X (левый край)
    pub x: i32,
    /// Координата Y (верхний край)
    pub y: i32,
    /// Ширина панели
    pub width: i32,
    /// Высота панели
    pub height: i32,
    /// Повернута ли панель
    pub rotated: bool,
}

impl PanelPosition {
    /// Создает новую позицию
    pub fn new(x: i32, y: i32, width: i32, height: i32, rotated: bool) -> Self {
        Self {
            x,
            y,
            width,
            height,
            rotated,
        }
    }

    /// Возвращает правую координату
    pub fn right(&self) -> i32 {
        self.x + self.width
    }

    /// Возвращает нижнюю координату
    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }

    /// Возвращает площадь
    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    /// Проверяет пересечение с другой позицией
    pub fn intersects(&self, other: &PanelPosition) -> bool {
        !(self.right() <= other.x || 
          other.right() <= self.x || 
          self.bottom() <= other.y || 
          other.bottom() <= self.y)
    }
}

/// Статистика ответа
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStatistics {
    /// Общее количество панелей
    pub total_panels: usize,
    /// Количество размещенных панелей
    pub placed_panels: usize,
    /// Количество неразмещенных панелей
    pub unplaced_panels: usize,
    /// Общая площадь панелей
    pub total_area: f64,
    /// Использованная площадь
    pub used_area: f64,
    /// Потраченная площадь
    pub wasted_area: f64,
    /// Эффективность использования (%)
    pub efficiency_percentage: f64,
    /// Время расчета (миллисекунды)
    pub calculation_time_ms: u64,
    /// Количество использованных складских панелей
    pub stock_panels_used: usize,
}

impl ResponseStatistics {
    /// Создает новую статистику
    pub fn new() -> Self {
        Self {
            total_panels: 0,
            placed_panels: 0,
            unplaced_panels: 0,
            total_area: 0.0,
            used_area: 0.0,
            wasted_area: 0.0,
            efficiency_percentage: 0.0,
            calculation_time_ms: 0,
            stock_panels_used: 0,
        }
    }

    /// Обновляет статистику на основе данных
    pub fn update(&mut self, 
                  total_panels: usize, 
                  placed_panels: usize, 
                  total_area: f64, 
                  used_area: f64) {
        self.total_panels = total_panels;
        self.placed_panels = placed_panels;
        self.unplaced_panels = total_panels - placed_panels;
        self.total_area = total_area;
        self.used_area = used_area;
        self.wasted_area = total_area - used_area;
        self.efficiency_percentage = if total_area > 0.0 {
            (used_area / total_area) * 100.0
        } else {
            0.0
        };
    }
}

impl Default for ResponseStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// Результат отправки задачи на расчет
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationSubmissionResult {
    /// Код статуса
    pub status_code: String,
    /// Идентификатор задачи (если успешно)
    pub task_id: Option<String>,
    /// Сообщение об ошибке (если неуспешно)
    pub error_message: Option<String>,
    /// Время отправки
    pub submission_time: u64,
}

impl CalculationSubmissionResult {
    /// Создает успешный результат
    pub fn success(task_id: String) -> Self {
        Self {
            status_code: StatusCode::Ok.to_string(),
            task_id: Some(task_id),
            error_message: None,
            submission_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Создает результат с ошибкой
    pub fn error(status_code: StatusCode, error_message: Option<String>) -> Self {
        Self {
            status_code: status_code.to_string(),
            task_id: None,
            error_message,
            submission_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Проверяет, успешен ли результат
    pub fn is_success(&self) -> bool {
        self.status_code == StatusCode::Ok.to_string()
    }
}

/// Ответ на запрос статуса задачи
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatusResponse {
    /// Статус задачи
    pub status: String,
    /// Процент инициализации
    pub init_percentage: u8,
    /// Процент выполнения
    pub percentage_done: u8,
    /// Решение (если доступно)
    pub solution: Option<CalculationResponse>,
    /// Время последнего обновления
    pub last_updated: u64,
    /// Дополнительная информация
    pub details: Option<String>,
}

impl TaskStatusResponse {
    /// Создает новый ответ статуса
    pub fn new(status: String) -> Self {
        Self {
            status,
            init_percentage: 0,
            percentage_done: 0,
            solution: None,
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            details: None,
        }
    }

    /// Обновляет прогресс
    pub fn update_progress(&mut self, init_percentage: u8, percentage_done: u8) {
        self.init_percentage = init_percentage;
        self.percentage_done = percentage_done;
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
    }

    /// Устанавливает решение
    pub fn set_solution(&mut self, solution: CalculationResponse) {
        self.solution = Some(solution);
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
    }
}

/// Коды статуса операций
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    /// Успешно
    Ok = 0,
    /// Неверные панели
    InvalidTiles = 1,
    /// Неверные складские панели
    InvalidStockTiles = 2,
    /// Задача уже выполняется
    TaskAlreadyRunning = 3,
    /// Сервер недоступен
    ServerUnavailable = 4,
    /// Слишком много панелей
    TooManyPanels = 5,
    /// Слишком много складских панелей
    TooManyStockPanels = 6,
}

impl StatusCode {
    /// Возвращает числовое значение
    pub fn value(&self) -> i32 {
        *self as i32
    }

    /// Возвращает числовое значение (альтернативное имя)
    pub fn get_value(&self) -> i32 {
        self.value()
    }

    /// Возвращает строковое значение
    pub fn to_string(&self) -> String {
        self.value().to_string()
    }

    /// Возвращает строковое значение (альтернативное имя)
    pub fn get_string_value(&self) -> String {
        self.to_string()
    }

    /// Создает из числового значения
    pub fn from_value(value: i32) -> Option<Self> {
        match value {
            0 => Some(StatusCode::Ok),
            1 => Some(StatusCode::InvalidTiles),
            2 => Some(StatusCode::InvalidStockTiles),
            3 => Some(StatusCode::TaskAlreadyRunning),
            4 => Some(StatusCode::ServerUnavailable),
            5 => Some(StatusCode::TooManyPanels),
            6 => Some(StatusCode::TooManyStockPanels),
            _ => None,
        }
    }

    /// Возвращает описание статуса
    pub fn description(&self) -> &'static str {
        match self {
            StatusCode::Ok => "Успешно",
            StatusCode::InvalidTiles => "Неверные панели",
            StatusCode::InvalidStockTiles => "Неверные складские панели",
            StatusCode::TaskAlreadyRunning => "Задача уже выполняется",
            StatusCode::ServerUnavailable => "Сервер недоступен",
            StatusCode::TooManyPanels => "Слишком много панелей",
            StatusCode::TooManyStockPanels => "Слишком много складских панелей",
        }
    }
}

/// Статистика системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    /// Количество задач в ожидании
    pub nbr_idle_tasks: u64,
    /// Количество выполняющихся задач
    pub nbr_running_tasks: u64,
    /// Количество завершенных задач
    pub nbr_finished_tasks: u64,
    /// Количество остановленных задач
    pub nbr_stopped_tasks: u64,
    /// Количество принудительно завершенных задач
    pub nbr_terminated_tasks: u64,
    /// Количество задач с ошибками
    pub nbr_error_tasks: u64,
    /// Количество выполняющихся потоков
    pub nbr_running_threads: i32,
    /// Количество потоков в очереди
    pub nbr_queued_threads: i32,
    /// Количество завершенных потоков
    pub nbr_finished_threads: u64,
    /// Отчеты о задачах
    pub task_reports: Vec<TaskReport>,
}

impl Stats {
    /// Создает новую статистику
    pub fn new() -> Self {
        Self {
            nbr_idle_tasks: 0,
            nbr_running_tasks: 0,
            nbr_finished_tasks: 0,
            nbr_stopped_tasks: 0,
            nbr_terminated_tasks: 0,
            nbr_error_tasks: 0,
            nbr_running_threads: 0,
            nbr_queued_threads: 0,
            nbr_finished_threads: 0,
            task_reports: Vec::new(),
        }
    }

    /// Возвращает общее количество задач
    pub fn total_tasks(&self) -> u64 {
        self.nbr_idle_tasks + 
        self.nbr_running_tasks + 
        self.nbr_finished_tasks + 
        self.nbr_stopped_tasks + 
        self.nbr_terminated_tasks + 
        self.nbr_error_tasks
    }

    /// Возвращает общее количество потоков
    pub fn total_threads(&self) -> i64 {
        self.nbr_running_threads as i64 + 
        self.nbr_queued_threads as i64 + 
        self.nbr_finished_threads as i64
    }

    /// Форматирует статистику в строку
    pub fn format_summary(&self) -> String {
        format!(
            "Задачи: {} ожидают, {} выполняются, {} завершены | Потоки: {} выполняются, {} в очереди",
            self.nbr_idle_tasks,
            self.nbr_running_tasks,
            self.nbr_finished_tasks,
            self.nbr_running_threads,
            self.nbr_queued_threads
        )
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

/// Строитель ответа на расчет
pub struct CalculationResponseBuilder {
    response: CalculationResponse,
}

impl CalculationResponseBuilder {
    /// Создает новый строитель
    pub fn new() -> Self {
        Self {
            response: CalculationResponse::new(),
        }
    }

    /// Устанавливает панели
    pub fn with_panels(mut self, panels: Vec<OptimizedPanel>) -> Self {
        self.response.panels = panels;
        self
    }

    /// Устанавливает неразмещенные панели
    pub fn with_no_fit_panels(mut self, no_fit_panels: Vec<TileDimensions>) -> Self {
        self.response.no_fit_panels = no_fit_panels;
        self
    }

    /// Устанавливает панели без материала
    pub fn with_no_material_panels(mut self, no_material_panels: Vec<TileDimensions>) -> Self {
        self.response.no_material_panels = no_material_panels;
        self
    }

    /// Устанавливает статистику
    pub fn with_statistics(mut self, statistics: ResponseStatistics) -> Self {
        self.response.statistics = statistics;
        self
    }

    /// Добавляет метаданные
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.response.metadata.insert(key, value);
        self
    }

    /// Строит ответ на основе задачи и решений
    pub fn from_task_and_solutions(
        _task: &crate::engine::tasks::Task,
        _request: &CalculationRequest,
        _solutions: &HashMap<String, Vec<Solution>>,
        no_material_panels: &[TileDimensions],
    ) -> Self {
        let mut builder = Self::new();
        
        // Добавляем панели без материала
        builder.response.no_material_panels = no_material_panels.to_vec();
        
        // TODO: Здесь должна быть логика преобразования решений в оптимизированные панели
        // Это будет реализовано в следующих этапах
        
        builder
    }

    /// Завершает построение
    pub fn build(self) -> CalculationResponse {
        self.response
    }
}

impl Default for CalculationResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_calculation_response_creation() {
        let response = CalculationResponse::new();
        assert!(!response.has_panels());
        assert!(response.all_panels_fit());
        assert_eq!(response.total_panels_count(), 0);
    }

    #[test]
    fn test_panel_position() {
        let pos1 = PanelPosition::new(0, 0, 100, 200, false);
        let pos2 = PanelPosition::new(50, 50, 100, 200, false);
        let pos3 = PanelPosition::new(200, 200, 100, 200, false);

        assert_eq!(pos1.right(), 100);
        assert_eq!(pos1.bottom(), 200);
        assert_eq!(pos1.area(), 20000);

        assert!(pos1.intersects(&pos2));
        assert!(!pos1.intersects(&pos3));
    }

    #[test]
    fn test_submission_result() {
        let success = CalculationSubmissionResult::success("task123".to_string());
        assert!(success.is_success());
        assert_eq!(success.task_id, Some("task123".to_string()));

        let error = CalculationSubmissionResult::error(
            StatusCode::InvalidTiles, 
            Some("Invalid input".to_string())
        );
        assert!(!error.is_success());
        assert_eq!(error.error_message, Some("Invalid input".to_string()));
    }

    #[test]
    fn test_status_code() {
        assert_eq!(StatusCode::Ok.value(), 0);
        assert_eq!(StatusCode::InvalidTiles.value(), 1);
        assert_eq!(StatusCode::Ok.to_string(), "0");
        
        assert_eq!(StatusCode::from_value(0), Some(StatusCode::Ok));
        assert_eq!(StatusCode::from_value(999), None);
        
        assert_eq!(StatusCode::Ok.description(), "Успешно");
    }

    #[test]
    fn test_response_statistics() {
        let mut stats = ResponseStatistics::new();
        stats.update(100, 80, 1000.0, 800.0);
        
        assert_eq!(stats.total_panels, 100);
        assert_eq!(stats.placed_panels, 80);
        assert_eq!(stats.unplaced_panels, 20);
        assert_eq!(stats.efficiency_percentage, 80.0);
        assert_eq!(stats.wasted_area, 200.0);
    }

    #[test]
    fn test_stats_summary() {
        let mut stats = Stats::new();
        stats.nbr_running_tasks = 5;
        stats.nbr_finished_tasks = 10;
        stats.nbr_running_threads = 3;
        
        assert_eq!(stats.total_tasks(), 15);
        assert!(stats.format_summary().contains("5 выполняются"));
    }

    #[test]
    fn test_task_status_response() {
        let mut response = TaskStatusResponse::new("RUNNING".to_string());
        response.update_progress(50, 75);
        
        assert_eq!(response.init_percentage, 50);
        assert_eq!(response.percentage_done, 75);
        assert!(response.last_updated > 0);
    }
}
