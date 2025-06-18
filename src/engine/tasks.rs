use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::engine::progress::{ProgressTracker, TaskInfo, TaskStatus};
use crate::engine::model::solution::Solution;
use crate::error::CuttingError;

/// Приоритет выполнения задачи
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    /// Низкий приоритет
    Low = 0,
    /// Обычный приоритет
    Normal = 1,
    /// Высокий приоритет
    High = 2,
    /// Критический приоритет
    Critical = 3,
}

impl From<u8> for TaskPriority {
    fn from(value: u8) -> Self {
        match value {
            0 => TaskPriority::Low,
            1 => TaskPriority::Normal,
            2 => TaskPriority::High,
            _ => TaskPriority::Critical,
        }
    }
}

impl From<TaskPriority> for u8 {
    fn from(priority: TaskPriority) -> Self {
        priority as u8
    }
}

/// Результат выполнения задачи
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskResult {
    /// Задача выполнена успешно с результатом
    Success(Vec<Solution>),
    /// Задача завершена с ошибкой
    Error(String),
    /// Задача была отменена
    Cancelled,
}

/// Отчет о выполнении задачи
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskReport {
    /// Идентификатор задачи
    pub task_id: String,
    /// Название задачи
    pub task_name: String,
    /// Статус выполнения
    pub status: TaskStatus,
    /// Результат выполнения
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TaskResult>,
    /// Время начала выполнения
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    /// Время завершения
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    /// Продолжительность выполнения в миллисекундах
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    /// Процент выполнения
    pub progress_percentage: u8,
    /// Дополнительная информация
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// Приоритет задачи
    pub priority: TaskPriority,
    /// Количество найденных решений
    pub solutions_count: usize,
    /// Использованная память (в байтах)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_used: Option<u64>,
    /// Ошибки, возникшие во время выполнения
    pub errors: Vec<String>,
}

impl TaskReport {
    /// Создает новый отчет о задаче
    pub fn new(task_id: String, task_name: String) -> Self {
        Self {
            task_id,
            task_name,
            status: TaskStatus::Queued,
            result: None,
            start_time: None,
            end_time: None,
            duration_ms: None,
            progress_percentage: 0,
            details: None,
            priority: TaskPriority::Normal,
            solutions_count: 0,
            memory_used: None,
            errors: Vec::new(),
        }
    }
    
    /// Обновляет отчет из TaskInfo
    pub fn update_from_task_info(&mut self, task_info: &TaskInfo) {
        self.status = task_info.status;
        self.start_time = task_info.start_time.map(|_| Utc::now());
        self.end_time = task_info.end_time.map(|_| Utc::now());
        self.duration_ms = task_info.get_duration().map(|d| d.as_millis() as u64);
        self.progress_percentage = task_info.progress_percentage;
        self.details = task_info.details.clone();
        self.priority = TaskPriority::from(task_info.priority);
    }
    
    /// Проверяет, завершена ли задача
    pub fn is_finished(&self) -> bool {
        matches!(self.status, TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled)
    }
    
    /// Проверяет, была ли задача успешной
    pub fn is_successful(&self) -> bool {
        self.status == TaskStatus::Completed && 
        matches!(self.result, Some(TaskResult::Success(_)))
    }
    
    /// Получает продолжительность выполнения в миллисекундах
    pub fn get_duration_millis(&self) -> u64 {
        self.duration_ms.unwrap_or(0)
    }
    
    /// Добавляет ошибку в отчет
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
    
    /// Форматирует отчет в строку
    pub fn format_summary(&self) -> String {
        let status_str = match self.status {
            TaskStatus::Queued => "В очереди",
            TaskStatus::Running => "Выполняется",
            TaskStatus::Completed => "Завершена",
            TaskStatus::Failed => "Ошибка",
            TaskStatus::Cancelled => "Отменена",
        };
        
        let duration_str = if let Some(duration_ms) = self.duration_ms {
            format!(" ({})", crate::engine::utils::Utils::format_duration(duration_ms))
        } else {
            String::new()
        };
        
        format!(
            "{}: {} - {}% {}{}",
            self.task_id,
            status_str,
            self.progress_percentage,
            self.task_name,
            duration_str
        )
    }
}

/// Задача для выполнения
pub struct Task {
    /// Уникальный идентификатор
    pub id: String,
    /// Название задачи
    pub name: String,
    /// Приоритет выполнения
    pub priority: TaskPriority,
    /// Функция для выполнения
    pub execution_fn: Box<dyn FnOnce() -> Result<Vec<Solution>, CuttingError> + Send>,
    /// Отчет о выполнении
    pub report: Arc<Mutex<TaskReport>>,
}

impl Task {
    /// Создает новую задачу
    pub fn new<F>(id: String, name: String, priority: TaskPriority, execution_fn: F) -> Self
    where
        F: FnOnce() -> Result<Vec<Solution>, CuttingError> + Send + 'static,
    {
        let report = Arc::new(Mutex::new(TaskReport::new(id.clone(), name.clone())));
        
        Self {
            id,
            name,
            priority,
            execution_fn: Box::new(execution_fn),
            report,
        }
    }
    
    /// Выполняет задачу
    pub fn execute(self) -> TaskReport {
        let start_time = Instant::now();
        
        // Обновляем статус на "выполняется"
        if let Ok(mut report) = self.report.lock() {
            report.status = TaskStatus::Running;
            report.start_time = Some(Utc::now());
        }
        
        // Выполняем задачу
        let result = (self.execution_fn)();
        let end_time = Instant::now();
        
        // Обновляем отчет с результатами
        if let Ok(mut report) = self.report.lock() {
            report.end_time = Some(Utc::now());
            report.duration_ms = Some(end_time.duration_since(start_time).as_millis() as u64);
            
            match result {
                Ok(solutions) => {
                    report.status = TaskStatus::Completed;
                    report.progress_percentage = 100;
                    report.solutions_count = solutions.len();
                    report.result = Some(TaskResult::Success(solutions));
                }
                Err(error) => {
                    report.status = TaskStatus::Failed;
                    report.add_error(error.to_string());
                    report.result = Some(TaskResult::Error(error.to_string()));
                }
            }
            
            report.clone()
        } else {
            // Fallback если не удалось заблокировать мьютекс
            let mut fallback_report = TaskReport::new(self.id, self.name);
            fallback_report.status = TaskStatus::Failed;
            fallback_report.add_error("Не удалось обновить отчет".to_string());
            fallback_report
        }
    }
    
    /// Получает клон текущего отчета
    pub fn get_report(&self) -> Option<TaskReport> {
        self.report.lock().ok().map(|report| report.clone())
    }
}

/// Менеджер выполняющихся задач
/// 
/// Управляет очередью задач, их выполнением в отдельных потоках
/// и отслеживанием состояния.
pub struct RunningTasks {
    /// Трекер прогресса
    progress_tracker: Arc<ProgressTracker>,
    /// Активные потоки выполнения
    active_threads: Arc<Mutex<HashMap<String, JoinHandle<TaskReport>>>>,
    /// Завершенные отчеты
    completed_reports: Arc<Mutex<Vec<TaskReport>>>,
    /// Максимальное количество одновременно выполняющихся задач
    max_concurrent_tasks: usize,
    /// Флаг остановки
    shutdown: Arc<Mutex<bool>>,
}

impl RunningTasks {
    /// Создает новый менеджер задач
    /// 
    /// # Аргументы
    /// * `max_concurrent_tasks` - максимальное количество одновременных задач
    pub fn new(max_concurrent_tasks: usize) -> Self {
        Self {
            progress_tracker: Arc::new(ProgressTracker::new(1000)),
            active_threads: Arc::new(Mutex::new(HashMap::new())),
            completed_reports: Arc::new(Mutex::new(Vec::new())),
            max_concurrent_tasks,
            shutdown: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Создает менеджер с настройками по умолчанию
    pub fn default() -> Self {
        Self::new(4) // По умолчанию 4 одновременные задачи
    }
    
    /// Добавляет задачу для выполнения
    /// 
    /// # Аргументы
    /// * `task` - задача для выполнения
    /// 
    /// # Возвращает
    /// Result с успехом или ошибкой, если не удалось добавить задачу
    pub fn submit_task(&self, task: Task) -> Result<(), CuttingError> {
        // Проверяем, не превышен ли лимит одновременных задач
        if self.get_active_task_count() >= self.max_concurrent_tasks {
            return Err(CuttingError::ResourceLimit(
                format!("Превышен лимит одновременных задач: {}", self.max_concurrent_tasks)
            ));
        }
        
        // Добавляем задачу в трекер прогресса
        let task_info = TaskInfo::new(task.id.clone(), task.name.clone());
        self.progress_tracker.add_task(task_info);
        
        // Запускаем задачу в отдельном потоке
        let task_id = task.id.clone();
        let task_id_for_thread = task_id.clone();
        let progress_tracker = Arc::clone(&self.progress_tracker);
        let completed_reports = Arc::clone(&self.completed_reports);
        
        let handle = thread::spawn(move || {
            // Обновляем статус на "выполняется"
            progress_tracker.update_task_status(&task_id_for_thread, TaskStatus::Running);
            
            // Выполняем задачу
            let report = task.execute();
            
            // Обновляем трекер прогресса
            progress_tracker.update_task_status(&task_id_for_thread, report.status);
            progress_tracker.update_task_progress(&task_id_for_thread, report.progress_percentage);
            
            // Сохраняем отчет
            if let Ok(mut reports) = completed_reports.lock() {
                reports.push(report.clone());
            }
            
            report
        });
        
        // Сохраняем handle потока
        if let Ok(mut threads) = self.active_threads.lock() {
            threads.insert(task_id, handle);
        }
        
        Ok(())
    }
    
    /// Получает количество активных задач
    pub fn get_active_task_count(&self) -> usize {
        if let Ok(threads) = self.active_threads.lock() {
            threads.len()
        } else {
            0
        }
    }
    
    /// Получает количество завершенных задач
    pub fn get_completed_task_count(&self) -> usize {
        if let Ok(reports) = self.completed_reports.lock() {
            reports.len()
        } else {
            0
        }
    }
    
    /// Получает общее количество задач
    pub fn get_total_task_count(&self) -> usize {
        self.progress_tracker.get_total_task_count()
    }
    
    /// Проверяет завершенные потоки и обновляет состояние
    pub fn update_completed_threads(&self) {
        if let Ok(mut threads) = self.active_threads.lock() {
            let mut completed_ids = Vec::new();
            
            // Проверяем какие потоки завершились
            for (task_id, handle) in threads.iter() {
                if handle.is_finished() {
                    completed_ids.push(task_id.clone());
                }
            }
            
            // Удаляем завершенные потоки
            for task_id in completed_ids {
                threads.remove(&task_id);
            }
        }
    }
    
    /// Ожидает завершения всех активных задач
    /// 
    /// # Аргументы
    /// * `timeout` - максимальное время ожидания
    /// 
    /// # Возвращает
    /// true, если все задачи завершились в указанное время
    pub fn wait_for_completion(&self, timeout: Option<Duration>) -> bool {
        let start_time = Instant::now();
        
        loop {
            self.update_completed_threads();
            
            if self.get_active_task_count() == 0 {
                return true;
            }
            
            if let Some(timeout) = timeout {
                if start_time.elapsed() >= timeout {
                    return false;
                }
            }
            
            // Проверяем флаг остановки
            if let Ok(shutdown) = self.shutdown.lock() {
                if *shutdown {
                    return false;
                }
            }
            
            // Небольшая пауза перед следующей проверкой
            thread::sleep(Duration::from_millis(100));
        }
    }
    
    /// Отменяет все активные задачи
    pub fn cancel_all_tasks(&self) {
        // Устанавливаем флаг остановки
        if let Ok(mut shutdown) = self.shutdown.lock() {
            *shutdown = true;
        }
        
        // Обновляем статус всех активных задач
        let active_tasks = self.progress_tracker.get_tasks_by_status(TaskStatus::Running);
        for task in active_tasks {
            self.progress_tracker.update_task_status(&task.id, TaskStatus::Cancelled);
        }
    }
    
    /// Получает отчет о прогрессе
    pub fn get_progress_report(&self) -> crate::engine::progress::ProgressReport {
        self.progress_tracker.generate_report()
    }
    
    /// Получает все завершенные отчеты
    pub fn get_completed_reports(&self) -> Vec<TaskReport> {
        if let Ok(reports) = self.completed_reports.lock() {
            reports.clone()
        } else {
            Vec::new()
        }
    }
    
    /// Получает отчеты с определенным статусом
    pub fn get_reports_by_status(&self, status: TaskStatus) -> Vec<TaskReport> {
        if let Ok(reports) = self.completed_reports.lock() {
            reports.iter()
                .filter(|report| report.status == status)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Очищает завершенные отчеты
    pub fn clear_completed_reports(&self) {
        if let Ok(mut reports) = self.completed_reports.lock() {
            reports.clear();
        }
        self.progress_tracker.clear_finished_tasks();
    }
    
    /// Получает статистику выполнения
    /// 
    /// # Возвращает
    /// Кортеж (успешных, неудачных, отмененных) задач
    pub fn get_execution_statistics(&self) -> (usize, usize, usize) {
        if let Ok(reports) = self.completed_reports.lock() {
            let successful = reports.iter().filter(|r| r.is_successful()).count();
            let failed = reports.iter().filter(|r| r.status == TaskStatus::Failed).count();
            let cancelled = reports.iter().filter(|r| r.status == TaskStatus::Cancelled).count();
            (successful, failed, cancelled)
        } else {
            (0, 0, 0)
        }
    }
    
    /// Получает среднее время выполнения успешных задач
    pub fn get_average_execution_time(&self) -> Duration {
        if let Ok(reports) = self.completed_reports.lock() {
            let successful_durations: Vec<u64> = reports.iter()
                .filter(|r| r.is_successful())
                .filter_map(|r| r.duration_ms)
                .collect();
            
            if successful_durations.is_empty() {
                return Duration::from_secs(0);
            }
            
            let total_millis: u64 = successful_durations.iter().sum();
            
            Duration::from_millis(total_millis / successful_durations.len() as u64)
        } else {
            Duration::from_secs(0)
        }
    }
    
    /// Форматирует сводку состояния в строку
    pub fn format_status_summary(&self) -> String {
        let progress_report = self.get_progress_report();
        let (successful, failed, cancelled) = self.get_execution_statistics();
        let avg_time = self.get_average_execution_time();
        
        format!(
            "Задачи: {} активных, {} завершено | Успешно: {}, Ошибок: {}, Отменено: {} | Среднее время: {}",
            progress_report.running_tasks,
            progress_report.completed_tasks,
            successful,
            failed,
            cancelled,
            crate::engine::utils::Utils::format_duration(avg_time.as_millis() as u64)
        )
    }
}

impl Default for RunningTasks {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_task_creation_and_execution() {
        let task = Task::new(
            "test_task".to_string(),
            "Test Task".to_string(),
            TaskPriority::Normal,
            || Ok(vec![])
        );
        
        assert_eq!(task.id, "test_task");
        assert_eq!(task.name, "Test Task");
        assert_eq!(task.priority, TaskPriority::Normal);
        
        let report = task.execute();
        assert_eq!(report.task_id, "test_task");
        assert_eq!(report.status, TaskStatus::Completed);
        assert!(report.is_successful());
    }
    
    #[test]
    fn test_task_with_error() {
        let task = Task::new(
            "error_task".to_string(),
            "Error Task".to_string(),
            TaskPriority::High,
            || Err(CuttingError::InvalidInput("Test error".to_string()))
        );
        
        let report = task.execute();
        assert_eq!(report.status, TaskStatus::Failed);
        assert!(!report.is_successful());
        assert!(!report.errors.is_empty());
    }
    
    #[test]
    fn test_running_tasks_basic_operations() {
        let running_tasks = RunningTasks::new(2);
        
        assert_eq!(running_tasks.get_active_task_count(), 0);
        assert_eq!(running_tasks.get_completed_task_count(), 0);
        
        // Добавляем задачу
        let task = Task::new(
            "task1".to_string(),
            "Task 1".to_string(),
            TaskPriority::Normal,
            || {
                thread::sleep(Duration::from_millis(50));
                Ok(vec![])
            }
        );
        
        running_tasks.submit_task(task).unwrap();
        
        // Ждем завершения
        assert!(running_tasks.wait_for_completion(Some(Duration::from_secs(1))));
        
        assert_eq!(running_tasks.get_completed_task_count(), 1);
        
        let (successful, failed, cancelled) = running_tasks.get_execution_statistics();
        assert_eq!(successful, 1);
        assert_eq!(failed, 0);
        assert_eq!(cancelled, 0);
    }
    
    #[test]
    fn test_task_priority_conversion() {
        assert_eq!(TaskPriority::from(0), TaskPriority::Low);
        assert_eq!(TaskPriority::from(1), TaskPriority::Normal);
        assert_eq!(TaskPriority::from(2), TaskPriority::High);
        assert_eq!(TaskPriority::from(3), TaskPriority::Critical);
        assert_eq!(TaskPriority::from(99), TaskPriority::Critical);
        
        assert_eq!(u8::from(TaskPriority::Low), 0);
        assert_eq!(u8::from(TaskPriority::Normal), 1);
        assert_eq!(u8::from(TaskPriority::High), 2);
        assert_eq!(u8::from(TaskPriority::Critical), 3);
    }
    
    #[test]
    fn test_task_report_formatting() {
        let mut report = TaskReport::new("task1".to_string(), "Test Task".to_string());
        report.status = TaskStatus::Completed;
        report.progress_percentage = 100;
        
        let summary = report.format_summary();
        assert!(summary.contains("task1"));
        assert!(summary.contains("Завершена"));
        assert!(summary.contains("100%"));
    }
}
