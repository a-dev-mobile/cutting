use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::engine::utils::Utils;

/// Статус выполнения задачи
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// Задача в очереди
    Queued,
    /// Задача выполняется
    Running,
    /// Задача завершена успешно
    Completed,
    /// Задача завершена с ошибкой
    Failed,
    /// Задача отменена
    Cancelled,
}

/// Информация о задаче для отслеживания прогресса
#[derive(Debug, Clone)]
pub struct TaskInfo {
    /// Уникальный идентификатор задачи
    pub id: String,
    /// Название задачи
    pub name: String,
    /// Статус выполнения
    pub status: TaskStatus,
    /// Процент выполнения (0-100)
    pub progress_percentage: u8,
    /// Время начала выполнения
    pub start_time: Option<Instant>,
    /// Время завершения
    pub end_time: Option<Instant>,
    /// Дополнительная информация
    pub details: Option<String>,
    /// Приоритет задачи
    pub priority: u8,
}

impl TaskInfo {
    /// Создает новую информацию о задаче
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            status: TaskStatus::Queued,
            progress_percentage: 0,
            start_time: None,
            end_time: None,
            details: None,
            priority: 0,
        }
    }
    
    /// Получает продолжительность выполнения задачи
    pub fn get_duration(&self) -> Option<Duration> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            (Some(start), None) => Some(start.elapsed()),
            _ => None,
        }
    }
    
    /// Получает продолжительность в миллисекундах
    pub fn get_duration_millis(&self) -> u64 {
        self.get_duration()
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }
    
    /// Проверяет, завершена ли задача
    pub fn is_finished(&self) -> bool {
        matches!(self.status, TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled)
    }
    
    /// Проверяет, выполняется ли задача
    pub fn is_running(&self) -> bool {
        self.status == TaskStatus::Running
    }
}

/// Отчет о прогрессе выполнения
#[derive(Debug, Clone)]
pub struct ProgressReport {
    /// Общее количество задач
    pub total_tasks: usize,
    /// Количество задач в очереди
    pub queued_tasks: usize,
    /// Количество выполняющихся задач
    pub running_tasks: usize,
    /// Количество завершенных задач
    pub completed_tasks: usize,
    /// Количество неудачных задач
    pub failed_tasks: usize,
    /// Количество отмененных задач
    pub cancelled_tasks: usize,
    /// Общий процент выполнения
    pub overall_progress: f64,
    /// Время начала отслеживания
    pub start_time: Instant,
    /// Текущее время
    pub current_time: Instant,
    /// Оценочное время завершения
    pub estimated_completion: Option<Instant>,
}

impl ProgressReport {
    /// Получает общее время выполнения
    pub fn get_total_duration(&self) -> Duration {
        self.current_time.duration_since(self.start_time)
    }
    
    /// Получает общее время в миллисекундах
    pub fn get_total_duration_millis(&self) -> u64 {
        self.get_total_duration().as_millis() as u64
    }
    
    /// Получает оценочное время до завершения
    pub fn get_estimated_remaining_time(&self) -> Option<Duration> {
        self.estimated_completion.map(|completion| {
            if completion > self.current_time {
                completion.duration_since(self.current_time)
            } else {
                Duration::from_secs(0)
            }
        })
    }
    
    /// Форматирует отчет в строку
    pub fn format_summary(&self) -> String {
        let total_time = Utils::format_duration(self.get_total_duration_millis());
        let remaining_time = self.get_estimated_remaining_time()
            .map(|d| Utils::format_duration(d.as_millis() as u64))
            .unwrap_or_else(|| "неизвестно".to_string());
        
        format!(
            "Прогресс: {:.1}% ({}/{} задач) | Время: {} | Осталось: {} | Активных: {}",
            self.overall_progress,
            self.completed_tasks,
            self.total_tasks,
            total_time,
            remaining_time,
            self.running_tasks
        )
    }
}

/// Трекер прогресса выполнения задач
/// 
/// Отслеживает состояние множественных задач и предоставляет
/// информацию о прогрессе выполнения.
pub struct ProgressTracker {
    /// Карта задач по их идентификаторам
    tasks: Arc<Mutex<HashMap<String, TaskInfo>>>,
    /// Время начала отслеживания
    start_time: Instant,
    /// Интервал обновления отчетов (в миллисекундах)
    update_interval_ms: u64,
    /// Последнее время обновления
    last_update: Arc<Mutex<Instant>>,
    /// Флаг активности трекера
    active: Arc<Mutex<bool>>,
}

impl ProgressTracker {
    /// Создает новый трекер прогресса
    /// 
    /// # Аргументы
    /// * `update_interval_ms` - интервал обновления в миллисекундах
    pub fn new(update_interval_ms: u64) -> Self {
        let now = Instant::now();
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            start_time: now,
            update_interval_ms,
            last_update: Arc::new(Mutex::new(now)),
            active: Arc::new(Mutex::new(true)),
        }
    }
    
    /// Создает трекер с настройками по умолчанию
    pub fn default() -> Self {
        Self::new(1000) // Обновление каждую секунду
    }
    
    /// Добавляет новую задачу для отслеживания
    /// 
    /// # Аргументы
    /// * `task_info` - информация о задаче
    pub fn add_task(&self, task_info: TaskInfo) {
        if let Ok(mut tasks) = self.tasks.lock() {
            tasks.insert(task_info.id.clone(), task_info);
        }
    }
    
    /// Обновляет статус задачи
    /// 
    /// # Аргументы
    /// * `task_id` - идентификатор задачи
    /// * `status` - новый статус
    pub fn update_task_status(&self, task_id: &str, status: TaskStatus) {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.get_mut(task_id) {
                let old_status = task.status;
                task.status = status;
                
                // Обновляем временные метки
                match status {
                    TaskStatus::Running if old_status == TaskStatus::Queued => {
                        task.start_time = Some(Instant::now());
                    }
                    TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled => {
                        if task.end_time.is_none() {
                            task.end_time = Some(Instant::now());
                        }
                        if status == TaskStatus::Completed {
                            task.progress_percentage = 100;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
    /// Обновляет прогресс задачи
    /// 
    /// # Аргументы
    /// * `task_id` - идентификатор задачи
    /// * `progress` - процент выполнения (0-100)
    pub fn update_task_progress(&self, task_id: &str, progress: u8) {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.get_mut(task_id) {
                task.progress_percentage = progress.min(100);
            }
        }
    }
    
    /// Обновляет детали задачи
    /// 
    /// # Аргументы
    /// * `task_id` - идентификатор задачи
    /// * `details` - дополнительная информация
    pub fn update_task_details(&self, task_id: &str, details: String) {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.get_mut(task_id) {
                task.details = Some(details);
            }
        }
    }
    
    /// Удаляет задачу из отслеживания
    /// 
    /// # Аргументы
    /// * `task_id` - идентификатор задачи
    pub fn remove_task(&self, task_id: &str) {
        if let Ok(mut tasks) = self.tasks.lock() {
            tasks.remove(task_id);
        }
    }
    
    /// Получает информацию о задаче
    /// 
    /// # Аргументы
    /// * `task_id` - идентификатор задачи
    /// 
    /// # Возвращает
    /// Клон информации о задаче или None, если задача не найдена
    pub fn get_task(&self, task_id: &str) -> Option<TaskInfo> {
        if let Ok(tasks) = self.tasks.lock() {
            tasks.get(task_id).cloned()
        } else {
            None
        }
    }
    
    /// Получает список всех задач
    /// 
    /// # Возвращает
    /// Вектор клонов всех задач
    pub fn get_all_tasks(&self) -> Vec<TaskInfo> {
        if let Ok(tasks) = self.tasks.lock() {
            tasks.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    /// Получает задачи с определенным статусом
    /// 
    /// # Аргументы
    /// * `status` - статус для фильтрации
    /// 
    /// # Возвращает
    /// Вектор задач с указанным статусом
    pub fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<TaskInfo> {
        if let Ok(tasks) = self.tasks.lock() {
            tasks.values()
                .filter(|task| task.status == status)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Генерирует отчет о текущем прогрессе
    /// 
    /// # Возвращает
    /// Отчет о прогрессе выполнения
    pub fn generate_report(&self) -> ProgressReport {
        let current_time = Instant::now();
        let tasks = self.get_all_tasks();
        
        let total_tasks = tasks.len();
        let mut queued_tasks = 0;
        let mut running_tasks = 0;
        let mut completed_tasks = 0;
        let mut failed_tasks = 0;
        let mut cancelled_tasks = 0;
        let mut total_progress = 0u64;
        
        for task in &tasks {
            match task.status {
                TaskStatus::Queued => queued_tasks += 1,
                TaskStatus::Running => running_tasks += 1,
                TaskStatus::Completed => completed_tasks += 1,
                TaskStatus::Failed => failed_tasks += 1,
                TaskStatus::Cancelled => cancelled_tasks += 1,
            }
            total_progress += task.progress_percentage as u64;
        }
        
        let overall_progress = if total_tasks > 0 {
            total_progress as f64 / total_tasks as f64
        } else {
            0.0
        };
        
        // Оценка времени завершения на основе текущего прогресса
        let estimated_completion = if overall_progress > 0.0 && overall_progress < 100.0 {
            let elapsed = current_time.duration_since(self.start_time);
            let estimated_total = elapsed.as_secs_f64() * (100.0 / overall_progress);
            Some(self.start_time + Duration::from_secs_f64(estimated_total))
        } else {
            None
        };
        
        ProgressReport {
            total_tasks,
            queued_tasks,
            running_tasks,
            completed_tasks,
            failed_tasks,
            cancelled_tasks,
            overall_progress,
            start_time: self.start_time,
            current_time,
            estimated_completion,
        }
    }
    
    /// Проверяет, нужно ли обновить отчет
    /// 
    /// # Возвращает
    /// true, если прошло достаточно времени для обновления
    pub fn should_update(&self) -> bool {
        if let Ok(last_update) = self.last_update.lock() {
            last_update.elapsed().as_millis() as u64 >= self.update_interval_ms
        } else {
            false
        }
    }
    
    /// Отмечает время последнего обновления
    pub fn mark_updated(&self) {
        if let Ok(mut last_update) = self.last_update.lock() {
            *last_update = Instant::now();
        }
    }
    
    /// Получает количество активных задач
    /// 
    /// # Возвращает
    /// Количество задач в статусе Running
    pub fn get_active_task_count(&self) -> usize {
        self.get_tasks_by_status(TaskStatus::Running).len()
    }
    
    /// Получает количество завершенных задач
    /// 
    /// # Возвращает
    /// Количество задач в статусе Completed
    pub fn get_completed_task_count(&self) -> usize {
        self.get_tasks_by_status(TaskStatus::Completed).len()
    }
    
    /// Получает общее количество задач
    /// 
    /// # Возвращает
    /// Общее количество отслеживаемых задач
    pub fn get_total_task_count(&self) -> usize {
        if let Ok(tasks) = self.tasks.lock() {
            tasks.len()
        } else {
            0
        }
    }
    
    /// Проверяет, все ли задачи завершены
    /// 
    /// # Возвращает
    /// true, если все задачи завершены (успешно или с ошибкой)
    pub fn are_all_tasks_finished(&self) -> bool {
        if let Ok(tasks) = self.tasks.lock() {
            tasks.values().all(|task| task.is_finished())
        } else {
            false
        }
    }
    
    /// Останавливает трекер
    pub fn stop(&self) {
        if let Ok(mut active) = self.active.lock() {
            *active = false;
        }
    }
    
    /// Проверяет, активен ли трекер
    /// 
    /// # Возвращает
    /// true, если трекер активен
    pub fn is_active(&self) -> bool {
        if let Ok(active) = self.active.lock() {
            *active
        } else {
            false
        }
    }
    
    /// Очищает все завершенные задачи
    pub fn clear_finished_tasks(&self) {
        if let Ok(mut tasks) = self.tasks.lock() {
            tasks.retain(|_, task| !task.is_finished());
        }
    }
    
    /// Получает статистику по времени выполнения
    /// 
    /// # Возвращает
    /// Кортеж (среднее, минимум, максимум) времени выполнения в миллисекундах
    pub fn get_execution_time_statistics(&self) -> (f64, u64, u64) {
        let completed_tasks = self.get_tasks_by_status(TaskStatus::Completed);
        
        if completed_tasks.is_empty() {
            return (0.0, 0, 0);
        }
        
        let durations: Vec<u64> = completed_tasks.iter()
            .filter_map(|task| task.get_duration())
            .map(|d| d.as_millis() as u64)
            .collect();
        
        if durations.is_empty() {
            return (0.0, 0, 0);
        }
        
        let sum: u64 = durations.iter().sum();
        let average = sum as f64 / durations.len() as f64;
        let min = *durations.iter().min().unwrap_or(&0);
        let max = *durations.iter().max().unwrap_or(&0);
        
        (average, min, max)
    }
}

impl Default for ProgressTracker {
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
    fn test_task_info_creation() {
        let task = TaskInfo::new("task1".to_string(), "Test Task".to_string());
        assert_eq!(task.id, "task1");
        assert_eq!(task.name, "Test Task");
        assert_eq!(task.status, TaskStatus::Queued);
        assert_eq!(task.progress_percentage, 0);
    }
    
    #[test]
    fn test_progress_tracker_basic_operations() {
        let tracker = ProgressTracker::new(100);
        
        let task = TaskInfo::new("task1".to_string(), "Test Task".to_string());
        tracker.add_task(task);
        
        assert_eq!(tracker.get_total_task_count(), 1);
        assert_eq!(tracker.get_active_task_count(), 0);
        
        tracker.update_task_status("task1", TaskStatus::Running);
        assert_eq!(tracker.get_active_task_count(), 1);
        
        tracker.update_task_progress("task1", 50);
        let task_info = tracker.get_task("task1").unwrap();
        assert_eq!(task_info.progress_percentage, 50);
        
        tracker.update_task_status("task1", TaskStatus::Completed);
        assert_eq!(tracker.get_completed_task_count(), 1);
        assert_eq!(tracker.get_active_task_count(), 0);
    }
    
    #[test]
    fn test_progress_report_generation() {
        let tracker = ProgressTracker::new(100);
        
        // Добавляем несколько задач
        for i in 0..5 {
            let task = TaskInfo::new(format!("task{}", i), format!("Task {}", i));
            tracker.add_task(task);
        }
        
        // Обновляем статусы
        tracker.update_task_status("task0", TaskStatus::Completed);
        tracker.update_task_status("task1", TaskStatus::Running);
        tracker.update_task_progress("task1", 75);
        
        let report = tracker.generate_report();
        assert_eq!(report.total_tasks, 5);
        assert_eq!(report.completed_tasks, 1);
        assert_eq!(report.running_tasks, 1);
        assert_eq!(report.queued_tasks, 3);
        
        // Проверяем общий прогресс: (100 + 75 + 0 + 0 + 0) / 5 = 35%
        assert!((report.overall_progress - 35.0).abs() < 0.1);
    }
    
    #[test]
    fn test_task_duration_calculation() {
        let mut task = TaskInfo::new("task1".to_string(), "Test Task".to_string());
        
        // Задача еще не начата
        assert!(task.get_duration().is_none());
        
        // Запускаем задачу
        task.start_time = Some(Instant::now());
        task.status = TaskStatus::Running;
        
        // Небольшая задержка
        thread::sleep(Duration::from_millis(10));
        
        // Завершаем задачу
        task.end_time = Some(Instant::now());
        task.status = TaskStatus::Completed;
        
        let duration = task.get_duration().unwrap();
        assert!(duration.as_millis() >= 10);
    }
}
