use crate::engine::model::request::CalculationRequest;
use crate::engine::model::response::{TaskStatusResponse, Stats, StatusCode, CalculationSubmissionResult};
use crate::engine::model::solution::Solution;
use crate::engine::logger::CutListLogger;
use crate::engine::tasks::{RunningTasks, Task, TaskPriority};
use crate::engine::watchdog::{WatchDog, WatchDogConfig, ConsoleEventHandler};
use crate::engine::progress::ProgressTracker;
use crate::error::CuttingError;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use chrono::{DateTime, Utc};
use std::sync::atomic::{AtomicU64, Ordering};

/// Константы из Java реализации
pub const MAX_PERMUTATION_ITERATIONS: usize = 1000;
pub const MAX_STOCK_ITERATIONS: usize = 1000;
pub const MAX_ACTIVE_THREADS_PER_TASK: usize = 5;
pub const MAX_PERMUTATIONS_WITH_SOLUTION: usize = 150;
pub const MAX_ALLOWED_DIGITS: usize = 6;
pub const THREAD_QUEUE_SIZE: usize = 1000;


/// Статус задачи для сервиса (отличается от ProgressTaskStatus)
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceTaskStatus {
    Idle,
    Running,
    Completed,
    Stopped,
    Terminated,
    Error,
}

/// Информация о задаче для сервиса
#[derive(Debug, Clone)]
pub struct ServiceTaskInfo {
    pub id: String,
    pub client_id: String,
    pub status: ServiceTaskStatus,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub progress_percentage: u8,
    pub solution: Option<Solution>,
}

impl ServiceTaskInfo {
    pub fn new(id: String, client_id: String) -> Self {
        Self {
            id,
            client_id,
            status: ServiceTaskStatus::Idle,
            start_time: None,
            end_time: None,
            progress_percentage: 0,
            solution: None,
        }
    }
}

/// Трейт для сервиса оптимизатора раскроя
pub trait CutListOptimizerService {
    /// Инициализация сервиса
    fn init(&mut self, threads: usize) -> Result<(), CuttingError>;
    
    /// Синхронная оптимизация
    fn optimize(&mut self, request: CalculationRequest) -> Result<crate::engine::model::response::CalculationResponse, CuttingError>;
    
    /// Отправка задачи на расчет
    fn submit_task(&mut self, request: CalculationRequest) -> Result<CalculationSubmissionResult, CuttingError>;
    
    /// Получение статуса задачи
    fn get_task_status(&self, task_id: &str) -> Result<Option<TaskStatusResponse>, CuttingError>;
    
    /// Остановка задачи
    fn stop_task(&mut self, task_id: &str) -> Result<Option<TaskStatusResponse>, CuttingError>;
    
    /// Принудительное завершение задачи
    fn terminate_task(&mut self, task_id: &str) -> Result<i32, CuttingError>;
    
    /// Получение списка задач клиента
    fn get_tasks(&self, client_id: &str, status: Option<ServiceTaskStatus>) -> Result<Vec<ServiceTaskInfo>, CuttingError>;
    
    /// Получение статистики системы
    fn get_stats(&self) -> Result<Stats, CuttingError>;
    
    /// Установка разрешения множественных задач на клиента
    fn set_allow_multiple_tasks_per_client(&mut self, allow: bool);
    
    /// Установка логгера
    fn set_cut_list_logger(&mut self, logger: Arc<dyn CutListLogger>);
}

/// Реализация сервиса оптимизатора раскроя
pub struct CutListOptimizerServiceImpl {
    /// Логгер
    cut_list_logger: Arc<dyn CutListLogger>,
    /// Менеджер выполняющихся задач
    running_tasks: Arc<RunningTasks>,
    /// Сторожевой таймер
    watch_dog: Option<WatchDog>,
    /// Счетчик идентификаторов задач
    task_id_counter: Arc<AtomicU64>,
    /// Разрешение множественных задач на клиента
    allow_multiple_tasks_per_client: bool,
    /// Количество потоков
    thread_count: usize,
    /// Активные задачи по клиентам
    client_tasks: Arc<Mutex<HashMap<String, Vec<String>>>>,
    /// Информация о задачах
    task_info: Arc<Mutex<HashMap<String, ServiceTaskInfo>>>,
}

impl CutListOptimizerServiceImpl {
    /// Создает новый экземпляр сервиса
    pub fn new(logger: Arc<dyn CutListLogger>) -> Self {
        Self {
            cut_list_logger: logger,
            running_tasks: Arc::new(RunningTasks::new(MAX_ACTIVE_THREADS_PER_TASK)),
            watch_dog: None,
            task_id_counter: Arc::new(AtomicU64::new(0)),
            allow_multiple_tasks_per_client: false,
            thread_count: 1,
            client_tasks: Arc::new(Mutex::new(HashMap::new())),
            task_info: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Генерирует уникальный идентификатор задачи
    fn generate_task_id(&self) -> String {
        let now = Utc::now();
        let date_part = now.format("%Y%m%d%H%M").to_string();
        let counter = self.task_id_counter.fetch_add(1, Ordering::SeqCst);
        format!("{}{}", date_part, counter)
    }

    /// Проверяет валидность панелей
    fn validate_panels(&self, panels: &[crate::engine::model::request::Panel]) -> (usize, StatusCode) {
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
    fn validate_stock_panels(&self, stock_panels: &[crate::engine::model::request::Panel]) -> (usize, StatusCode) {
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

    /// Проверяет, может ли клиент запустить новую задачу
    fn can_client_start_task(&self, client_id: &str, max_tasks: usize) -> bool {
        if let Ok(client_tasks) = self.client_tasks.lock() {
            if let Some(tasks) = client_tasks.get(client_id) {
                return tasks.len() < max_tasks;
            }
        }

        true
    }

    /// Добавляет задачу к клиенту
    fn add_task_to_client(&self, client_id: &str, task_id: &str) {
        if let Ok(mut client_tasks) = self.client_tasks.lock() {
            client_tasks
                .entry(client_id.to_string())
                .or_insert_with(Vec::new)
                .push(task_id.to_string());
        }
    }

    /// Удаляет задачу у клиента
    #[allow(dead_code)]
    fn remove_task_from_client(&self, client_id: &str, task_id: &str) {
        if let Ok(mut client_tasks) = self.client_tasks.lock() {
            if let Some(tasks) = client_tasks.get_mut(client_id) {
                tasks.retain(|id| id != task_id);
                if tasks.is_empty() {
                    client_tasks.remove(client_id);
                }
            }
        }
    }

    /// Вычисляет задачу оптимизации
    #[allow(dead_code)]
    fn compute(&self, request: CalculationRequest, task_id: String) {
        let client_id = request.client_info.id.clone();
        let logger = Arc::clone(&self.cut_list_logger);
        let running_tasks = Arc::clone(&self.running_tasks);
        let _client_tasks = Arc::clone(&self.client_tasks);
        
        // Клонируем переменные для использования в замыкании
        let task_id_for_closure = task_id.clone();
        let logger_for_closure = Arc::clone(&logger);
        
        // Создаем задачу для выполнения
        let task = Task::new(
            task_id.clone(),
            "Оптимизация раскроя".to_string(),
            TaskPriority::Normal,
            move || {
                logger_for_closure.info(&format!("Начало выполнения задачи {}", task_id_for_closure));
                
                // Здесь будет основная логика оптимизации
                // Пока что возвращаем пустой результат
                let solutions = vec![Solution::new()];
                
                logger_for_closure.info(&format!("Задача {} завершена успешно", task_id_for_closure));
                Ok(solutions)
            },
        );

        // Добавляем задачу в менеджер
        if let Err(e) = running_tasks.submit_task(task) {
            logger.error(&format!("Ошибка при добавлении задачи {}: {}", task_id, e));
            self.remove_task_from_client(&client_id, &task_id);
        }
    }
}

impl CutListOptimizerService for CutListOptimizerServiceImpl {
    fn optimize(&mut self, request: CalculationRequest) -> Result<crate::engine::model::response::CalculationResponse, CuttingError> {
        self.cut_list_logger.info("Начало синхронной оптимизации");
        
        // Валидируем панели
        let (_panel_count, panel_status) = self.validate_panels(&request.panels);
        if panel_status != StatusCode::Ok {
            return Err(CuttingError::GeneralCuttingError(
                format!("Ошибка валидации панелей: {}", panel_status.description())
            ));
        }

        // Валидируем складские панели
        let (_stock_count, stock_status) = self.validate_stock_panels(&request.stock_panels);
        if stock_status != StatusCode::Ok {
            return Err(CuttingError::GeneralCuttingError(
                format!("Ошибка валидации складских панелей: {}", stock_status.description())
            ));
        }

        self.cut_list_logger.info(&format!(
            "Валидация прошла успешно: {} панелей, {} складских панелей", 
            _panel_count, _stock_count
        ));

        // Здесь будет основная логика оптимизации
        // Пока что возвращаем базовый ответ
        let mut response = crate::engine::model::response::CalculationResponse::new();
        
        // Обновляем статистику
        response.statistics.update(
            request.panels.len(),
            0, // пока что 0 размещенных панелей
            0.0, // общая площадь
            0.0  // использованная площадь
        );
        
        // Добавляем метаданные
        response.add_metadata("optimization_type".to_string(), "synchronous".to_string());
        response.add_metadata("panel_count".to_string(), _panel_count.to_string());
        response.add_metadata("stock_count".to_string(), _stock_count.to_string());
        
        self.cut_list_logger.info("Синхронная оптимизация завершена");
        Ok(response)
    }

    fn init(&mut self, threads: usize) -> Result<(), CuttingError> {
        self.thread_count = threads;
        
        // Инициализируем менеджер задач
        self.running_tasks = Arc::new(RunningTasks::new(threads));
        
        // Инициализируем сторожевой таймер
        let config = WatchDogConfig::default();
        let event_handler = Arc::new(ConsoleEventHandler);
        let mut watch_dog = WatchDog::new(config, event_handler);
        
        // Запускаем сторожевой таймер
        let progress_tracker = Arc::new(ProgressTracker::new(1000));
        let running_tasks_clone = Arc::clone(&self.running_tasks);
        
        if let Err(e) = watch_dog.start(progress_tracker, running_tasks_clone) {
            return Err(CuttingError::GeneralCuttingError(
                format!("Ошибка запуска сторожевого таймера: {}", e)
            ));
        }
        
        self.watch_dog = Some(watch_dog);
        
        self.cut_list_logger.info(&format!("Сервис инициализирован с {} потоками", threads));
        Ok(())
    }

    fn submit_task(&mut self, request: CalculationRequest) -> Result<CalculationSubmissionResult, CuttingError> {
        let client_id = &request.client_info.id;
        
        // Валидируем конфигурацию
        if !request.configuration.is_valid() {
            return Ok(CalculationSubmissionResult::error(
                StatusCode::InvalidTiles, 
                Some("Неверная конфигурация".to_string())
            ));
        }
        
        // Проверяем производительные пороги
        let performance_thresholds = request.configuration.performance_thresholds
            .as_ref()
            .map(|pt| pt.max_simultaneous_tasks)
            .unwrap_or(2);

        // Проверяем, может ли клиент запустить новую задачу
        if !self.can_client_start_task(client_id, performance_thresholds) {
            self.cut_list_logger.warning(&format!(
                "Отклонение задачи клиента {} из-за превышения лимита одновременных задач",
                client_id
            ));
            return Ok(CalculationSubmissionResult::error(StatusCode::TaskAlreadyRunning, None));
        }

        // Валидируем панели
        let (_panel_count, panel_status) = self.validate_panels(&request.panels);
        if panel_status != StatusCode::Ok {
            return Ok(CalculationSubmissionResult::error(panel_status, None));
        }

        // Валидируем складские панели
        let (_stock_count, stock_status) = self.validate_stock_panels(&request.stock_panels);
        if stock_status != StatusCode::Ok {
            return Ok(CalculationSubmissionResult::error(stock_status, None));
        }

        // Генерируем идентификатор задачи
        let task_id = self.generate_task_id();
        
        // Создаем информацию о задаче
        let mut task_info = ServiceTaskInfo::new(task_id.clone(), client_id.clone());
        task_info.status = ServiceTaskStatus::Running;
        task_info.start_time = Some(Utc::now());
        
        // Сохраняем информацию о задаче
        if let Ok(mut task_info_map) = self.task_info.lock() {
            task_info_map.insert(task_id.clone(), task_info);
        }
        
        // Добавляем задачу к клиенту
        self.add_task_to_client(client_id, &task_id);
        
        // Запускаем вычисление напрямую, используя клонированные данные
        let task_id_clone = task_id.clone();
        let client_id_clone = client_id.clone();
        let logger_clone = Arc::clone(&self.cut_list_logger);
        let running_tasks_clone = Arc::clone(&self.running_tasks);
        let client_tasks_clone = Arc::clone(&self.client_tasks);
        let task_info_clone = Arc::clone(&self.task_info);
        
        thread::spawn(move || {
            // Клонируем переменные для использования в замыкании задачи
            let task_id_for_task = task_id_clone.clone();
            let logger_for_task = Arc::clone(&logger_clone);
            let task_info_for_task = Arc::clone(&task_info_clone);
            
            // Создаем задачу для выполнения
            let task = Task::new(
                task_id_clone.clone(),
                "Оптимизация раскроя".to_string(),
                TaskPriority::Normal,
                move || {
                    logger_for_task.info(&format!("Начало выполнения задачи {}", task_id_for_task));
                    
                    // Обновляем статус задачи
                    if let Ok(mut task_info_map) = task_info_for_task.lock() {
                        if let Some(info) = task_info_map.get_mut(&task_id_for_task) {
                            info.status = ServiceTaskStatus::Running;
                            info.progress_percentage = 50;
                        }
                    }
                    
                    // Имитируем работу
                    thread::sleep(std::time::Duration::from_millis(100));
                    
                    // Здесь будет основная логика оптимизации
                    // Пока что возвращаем пустой результат
                    let solutions = vec![Solution::new()];
                    
                    // Обновляем статус задачи на завершенную
                    if let Ok(mut task_info_map) = task_info_for_task.lock() {
                        if let Some(info) = task_info_map.get_mut(&task_id_for_task) {
                            info.status = ServiceTaskStatus::Completed;
                            info.progress_percentage = 100;
                            info.end_time = Some(Utc::now());
                        }
                    }
                    
                    logger_for_task.info(&format!("Задача {} завершена успешно", task_id_for_task));
                    Ok(solutions)
                },
            );

            // Добавляем задачу в менеджер
            if let Err(e) = running_tasks_clone.submit_task(task) {
                logger_clone.error(&format!("Ошибка при добавлении задачи {}: {}", task_id_clone, e));
                // Удаляем задачу у клиента при ошибке
                if let Ok(mut client_tasks) = client_tasks_clone.lock() {
                    if let Some(tasks) = client_tasks.get_mut(&client_id_clone) {
                        tasks.retain(|id| id != &task_id_clone);
                        if tasks.is_empty() {
                            client_tasks.remove(&client_id_clone);
                        }
                    }
                }
                
                // Обновляем статус задачи на ошибку
                if let Ok(mut task_info_map) = task_info_clone.lock() {
                    if let Some(info) = task_info_map.get_mut(&task_id_clone) {
                        info.status = ServiceTaskStatus::Error;
                        info.end_time = Some(Utc::now());
                    }
                }
            }
        });

        self.cut_list_logger.info(&format!("Задача {} отправлена на выполнение", task_id));
        
        Ok(CalculationSubmissionResult::success(task_id))
    }

    fn get_task_status(&self, _task_id: &str) -> Result<Option<TaskStatusResponse>, CuttingError> {
        // Здесь должна быть логика получения статуса задачи из running_tasks
        // Пока что возвращаем заглушку
        Ok(Some(TaskStatusResponse {
            status: "RUNNING".to_string(),
            init_percentage: 50,
            percentage_done: 25,
            details: Some("Выполняется оптимизация".to_string()),
            solution: None,
            last_updated: Utc::now().timestamp_millis() as u64,
        }))
    }

    fn stop_task(&mut self, task_id: &str) -> Result<Option<TaskStatusResponse>, CuttingError> {
        self.cut_list_logger.info(&format!("Остановка задачи {}", task_id));
        
        // Здесь должна быть логика остановки задачи
        // Пока что возвращаем заглушку
        Ok(Some(TaskStatusResponse {
            status: "STOPPED".to_string(),
            init_percentage: 100,
            percentage_done: 100,
            details: Some("Задача остановлена".to_string()),
            solution: None,
            last_updated: Utc::now().timestamp_millis() as u64,
        }))
    }

    fn terminate_task(&mut self, task_id: &str) -> Result<i32, CuttingError> {
        self.cut_list_logger.info(&format!("Принудительное завершение задачи {}", task_id));
        
        // Здесь должна быть логика принудительного завершения задачи
        // Возвращаем 0 при успехе, -1 при ошибке
        Ok(0)
    }

    fn get_tasks(&self, client_id: &str, status: Option<ServiceTaskStatus>) -> Result<Vec<ServiceTaskInfo>, CuttingError> {
        let mut result = Vec::new();
        
        if let Ok(client_tasks) = self.client_tasks.lock() {
            if let Some(task_ids) = client_tasks.get(client_id) {
                if let Ok(task_info) = self.task_info.lock() {
                    for task_id in task_ids {
                        if let Some(info) = task_info.get(task_id) {
                            // Фильтрация по статусу, если указан
                            if let Some(ref filter_status) = status {
                                if &info.status == filter_status {
                                    result.push(info.clone());
                                }
                            } else {
                                result.push(info.clone());
                            }
                        }
                    }
                }
            }
        }
        
        Ok(result)
    }

    fn get_stats(&self) -> Result<Stats, CuttingError> {
        let (successful, failed, cancelled) = self.running_tasks.get_execution_statistics();
        let active_count = self.running_tasks.get_active_task_count();
        let completed_count = self.running_tasks.get_completed_task_count();
        
        // Подсчитываем задачи по статусам из нашего task_info
        let mut running_tasks = 0u64;
        let mut idle_tasks = 0u64;
        let mut finished_tasks = 0u64;
        let mut stopped_tasks = 0u64;
        let mut terminated_tasks = 0u64;
        let mut error_tasks = 0u64;
        
        if let Ok(task_info) = self.task_info.lock() {
            for info in task_info.values() {
                match info.status {
                    ServiceTaskStatus::Running => running_tasks += 1,
                    ServiceTaskStatus::Idle => idle_tasks += 1,
                    ServiceTaskStatus::Completed => finished_tasks += 1,
                    ServiceTaskStatus::Stopped => stopped_tasks += 1,
                    ServiceTaskStatus::Terminated => terminated_tasks += 1,
                    ServiceTaskStatus::Error => error_tasks += 1,
                }
            }
        }
        
        Ok(Stats {
            nbr_running_tasks: running_tasks + (active_count as u64),
            nbr_idle_tasks: idle_tasks,
            nbr_finished_tasks: finished_tasks + (successful as u64),
            nbr_stopped_tasks: stopped_tasks + (cancelled as u64),
            nbr_terminated_tasks: terminated_tasks,
            nbr_error_tasks: error_tasks + (failed as u64),
            nbr_running_threads: active_count as i32,
            nbr_queued_threads: 0,
            nbr_finished_threads: completed_count as u64,
            task_reports: self.running_tasks.get_completed_reports(),
        })
    }

    fn set_allow_multiple_tasks_per_client(&mut self, allow: bool) {
        self.allow_multiple_tasks_per_client = allow;
        self.cut_list_logger.info(&format!(
            "Множественные задачи на клиента: {}",
            if allow { "разрешены" } else { "запрещены" }
        ));
    }

    fn set_cut_list_logger(&mut self, logger: Arc<dyn CutListLogger>) {
        self.cut_list_logger = logger;
        
        // Обновляем логгер в сторожевом таймере
        if let Some(ref mut _watch_dog) = self.watch_dog {
            // В реальной реализации здесь должен быть метод для обновления логгера
        }
    }
}

/// Синглтон экземпляр сервиса (как в Java реализации)
static INSTANCE: std::sync::OnceLock<std::sync::Mutex<CutListOptimizerServiceImpl>> = std::sync::OnceLock::new();

impl CutListOptimizerServiceImpl {
    /// Получает синглтон экземпляр сервиса
    pub fn get_instance(logger: Arc<dyn CutListLogger>) -> &'static std::sync::Mutex<CutListOptimizerServiceImpl> {
        INSTANCE.get_or_init(|| {
            std::sync::Mutex::new(CutListOptimizerServiceImpl::new(logger))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::logger::CutListLoggerImpl;

    #[test]
    fn test_status_code_values() {
        assert_eq!(StatusCode::Ok.get_value(), 0);
        assert_eq!(StatusCode::InvalidTiles.get_value(), 1);
        assert_eq!(StatusCode::TaskAlreadyRunning.get_value(), 3);
    }

    #[test]
    fn test_task_id_generation() {
        let logger = Arc::new(CutListLoggerImpl::new());
        let service = CutListOptimizerServiceImpl::new(logger);
        
        let id1 = service.generate_task_id();
        let id2 = service.generate_task_id();
        
        assert_ne!(id1, id2);
        assert!(id1.len() >= 12); // Минимум дата + счетчик
    }

    #[test]
    fn test_panel_validation() {
        let logger = Arc::new(CutListLoggerImpl::new());
        let service = CutListOptimizerServiceImpl::new(logger);
        
        let valid_panels = vec![
            crate::engine::model::request::Panel::new(1, "100".to_string(), "200".to_string(), 2, None),
        ];
        
        let (count, status) = service.validate_panels(&valid_panels);
        assert_eq!(count, 2);
        assert_eq!(status, StatusCode::Ok);
        
        let empty_panels = vec![];
        let (count, status) = service.validate_panels(&empty_panels);
        assert_eq!(count, 0);
        assert_eq!(status, StatusCode::InvalidTiles);
    }

    #[test]
    fn test_calculation_submission_result() {
        let success_result = CalculationSubmissionResult::success("task123".to_string());
        assert!(success_result.is_success());
        assert_eq!(success_result.task_id, Some("task123".to_string()));
        
        let error_result = CalculationSubmissionResult::error(
            StatusCode::InvalidTiles,
            Some("Invalid tiles".to_string())
        );
        assert!(!error_result.is_success());
        assert_eq!(error_result.task_id, None);
    }
}
