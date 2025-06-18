use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use crate::engine::progress::{ProgressTracker, TaskStatus};
use crate::engine::tasks::RunningTasks;

/// Конфигурация сторожевого таймера
#[derive(Debug, Clone)]
pub struct WatchDogConfig {
    /// Максимальное время выполнения задачи (в секундах)
    pub max_task_duration_secs: u64,
    /// Интервал проверки (в миллисекундах)
    pub check_interval_ms: u64,
    /// Максимальное время без прогресса (в секундах)
    pub max_idle_time_secs: u64,
    /// Включить автоматическое прерывание зависших задач
    pub auto_terminate_stuck_tasks: bool,
    /// Максимальное использование памяти (в байтах)
    pub max_memory_usage_bytes: Option<u64>,
    /// Максимальное количество одновременных задач
    pub max_concurrent_tasks: Option<usize>,
}

impl Default for WatchDogConfig {
    fn default() -> Self {
        Self {
            max_task_duration_secs: 300,      // 5 минут
            check_interval_ms: 5000,          // 5 секунд
            max_idle_time_secs: 60,           // 1 минута без прогресса
            auto_terminate_stuck_tasks: true,
            max_memory_usage_bytes: None,     // Без ограничений
            max_concurrent_tasks: None,       // Без ограничений
        }
    }
}

/// Информация о мониторинге задачи
#[derive(Debug, Clone)]
struct TaskMonitorInfo {
    /// Идентификатор задачи
    task_id: String,
    /// Время начала выполнения
    start_time: Instant,
    /// Последнее время обновления прогресса
    last_progress_update: Instant,
    /// Последний известный процент выполнения
    last_progress_percentage: u8,
    /// Количество предупреждений
    warning_count: u32,
}

/// Тип события сторожевого таймера
#[derive(Debug, Clone)]
pub enum WatchDogEvent {
    /// Задача превысила максимальное время выполнения
    TaskTimeout {
        task_id: String,
        duration: Duration,
        max_duration: Duration,
    },
    /// Задача не показывает прогресс
    TaskStuck {
        task_id: String,
        idle_time: Duration,
        max_idle_time: Duration,
    },
    /// Превышено использование памяти
    MemoryLimitExceeded {
        current_usage: u64,
        max_usage: u64,
    },
    /// Превышено количество одновременных задач
    ConcurrentTaskLimitExceeded {
        current_count: usize,
        max_count: usize,
    },
    /// Задача была принудительно завершена
    TaskTerminated {
        task_id: String,
        reason: String,
    },
    /// Предупреждение о производительности
    PerformanceWarning {
        message: String,
        details: HashMap<String, String>,
    },
}

/// Обработчик событий сторожевого таймера
pub trait WatchDogEventHandler: Send + Sync {
    /// Обрабатывает событие сторожевого таймера
    fn handle_event(&self, event: WatchDogEvent);
}

/// Простой обработчик событий, который логирует в консоль
pub struct ConsoleEventHandler;

impl WatchDogEventHandler for ConsoleEventHandler {
    fn handle_event(&self, event: WatchDogEvent) {
        match event {
            WatchDogEvent::TaskTimeout { task_id, duration, max_duration } => {
                println!("⚠️  TIMEOUT: Задача {} превысила лимит времени: {:?} > {:?}", 
                    task_id, duration, max_duration);
            }
            WatchDogEvent::TaskStuck { task_id, idle_time, max_idle_time } => {
                println!("⚠️  STUCK: Задача {} не показывает прогресс: {:?} > {:?}", 
                    task_id, idle_time, max_idle_time);
            }
            WatchDogEvent::MemoryLimitExceeded { current_usage, max_usage } => {
                println!("⚠️  MEMORY: Превышен лимит памяти: {} > {} байт", 
                    current_usage, max_usage);
            }
            WatchDogEvent::ConcurrentTaskLimitExceeded { current_count, max_count } => {
                println!("⚠️  TASKS: Превышен лимит одновременных задач: {} > {}", 
                    current_count, max_count);
            }
            WatchDogEvent::TaskTerminated { task_id, reason } => {
                println!("🛑 TERMINATED: Задача {} была завершена: {}", task_id, reason);
            }
            WatchDogEvent::PerformanceWarning { message, details } => {
                println!("⚠️  PERFORMANCE: {}", message);
                for (key, value) in details {
                    println!("   {}: {}", key, value);
                }
            }
        }
    }
}

/// Сторожевой таймер для мониторинга выполнения задач
/// 
/// Отслеживает выполняющиеся задачи и автоматически прерывает
/// те, которые превышают установленные лимиты времени или ресурсов.
pub struct WatchDog {
    /// Конфигурация
    config: WatchDogConfig,
    /// Обработчик событий
    event_handler: Arc<dyn WatchDogEventHandler>,
    /// Информация о мониторинге задач
    monitored_tasks: Arc<Mutex<HashMap<String, TaskMonitorInfo>>>,
    /// Флаг активности
    active: Arc<Mutex<bool>>,
    /// Handle потока мониторинга
    monitor_thread: Option<JoinHandle<()>>,
}

impl WatchDog {
    /// Создает новый сторожевой таймер
    /// 
    /// # Аргументы
    /// * `config` - конфигурация сторожевого таймера
    /// * `event_handler` - обработчик событий
    pub fn new(config: WatchDogConfig, event_handler: Arc<dyn WatchDogEventHandler>) -> Self {
        Self {
            config,
            event_handler,
            monitored_tasks: Arc::new(Mutex::new(HashMap::new())),
            active: Arc::new(Mutex::new(false)),
            monitor_thread: None,
        }
    }
    
    /// Создает сторожевой таймер с настройками по умолчанию
    pub fn default() -> Self {
        Self::new(
            WatchDogConfig::default(),
            Arc::new(ConsoleEventHandler)
        )
    }
    
    /// Запускает мониторинг
    /// 
    /// # Аргументы
    /// * `progress_tracker` - трекер прогресса для мониторинга
    /// * `running_tasks` - менеджер выполняющихся задач
    pub fn start(&mut self, 
                 progress_tracker: Arc<ProgressTracker>, 
                 running_tasks: Arc<RunningTasks>) -> Result<(), Box<dyn std::error::Error>> {
        // Устанавливаем флаг активности
        if let Ok(mut active) = self.active.lock() {
            *active = true;
        }
        
        let config = self.config.clone();
        let event_handler = Arc::clone(&self.event_handler);
        let monitored_tasks = Arc::clone(&self.monitored_tasks);
        let active = Arc::clone(&self.active);
        
        // Запускаем поток мониторинга
        let handle = thread::spawn(move || {
            Self::monitor_loop(
                config,
                event_handler,
                monitored_tasks,
                active,
                progress_tracker,
                running_tasks,
            );
        });
        
        self.monitor_thread = Some(handle);
        Ok(())
    }
    
    /// Останавливает мониторинг
    pub fn stop(&mut self) {
        // Устанавливаем флаг неактивности
        if let Ok(mut active) = self.active.lock() {
            *active = false;
        }
        
        // Ждем завершения потока мониторинга
        if let Some(handle) = self.monitor_thread.take() {
            let _ = handle.join();
        }
    }
    
    /// Основной цикл мониторинга
    fn monitor_loop(
        config: WatchDogConfig,
        event_handler: Arc<dyn WatchDogEventHandler>,
        monitored_tasks: Arc<Mutex<HashMap<String, TaskMonitorInfo>>>,
        active: Arc<Mutex<bool>>,
        progress_tracker: Arc<ProgressTracker>,
        running_tasks: Arc<RunningTasks>,
    ) {
        let check_interval = Duration::from_millis(config.check_interval_ms);
        
        while Self::is_active(&active) {
            // Обновляем информацию о мониторинге
            Self::update_monitored_tasks(&monitored_tasks, &progress_tracker);
            
            // Проверяем лимиты времени
            Self::check_time_limits(&config, &event_handler, &monitored_tasks, &progress_tracker);
            
            // Проверяем лимиты ресурсов
            Self::check_resource_limits(&config, &event_handler, &running_tasks);
            
            // Проверяем производительность
            Self::check_performance(&config, &event_handler, &progress_tracker, &running_tasks);
            
            // Пауза перед следующей проверкой
            thread::sleep(check_interval);
        }
    }
    
    /// Проверяет, активен ли сторожевой таймер
    fn is_active(active: &Arc<Mutex<bool>>) -> bool {
        if let Ok(active_flag) = active.lock() {
            *active_flag
        } else {
            false
        }
    }
    
    /// Обновляет информацию о мониторинге задач
    fn update_monitored_tasks(
        monitored_tasks: &Arc<Mutex<HashMap<String, TaskMonitorInfo>>>,
        progress_tracker: &Arc<ProgressTracker>,
    ) {
        let running_tasks = progress_tracker.get_tasks_by_status(TaskStatus::Running);
        
        if let Ok(mut tasks) = monitored_tasks.lock() {
            let now = Instant::now();
            
            // Добавляем новые задачи
            for task_info in &running_tasks {
                if !tasks.contains_key(&task_info.id) {
                    let monitor_info = TaskMonitorInfo {
                        task_id: task_info.id.clone(),
                        start_time: task_info.start_time.unwrap_or(now),
                        last_progress_update: now,
                        last_progress_percentage: task_info.progress_percentage,
                        warning_count: 0,
                    };
                    tasks.insert(task_info.id.clone(), monitor_info);
                } else if let Some(monitor_info) = tasks.get_mut(&task_info.id) {
                    // Обновляем прогресс
                    if task_info.progress_percentage > monitor_info.last_progress_percentage {
                        monitor_info.last_progress_update = now;
                        monitor_info.last_progress_percentage = task_info.progress_percentage;
                    }
                }
            }
            
            // Удаляем завершенные задачи
            let running_task_ids: std::collections::HashSet<_> = 
                running_tasks.iter().map(|t| &t.id).collect();
            tasks.retain(|task_id, _| running_task_ids.contains(task_id));
        }
    }
    
    /// Проверяет лимиты времени выполнения
    fn check_time_limits(
        config: &WatchDogConfig,
        event_handler: &Arc<dyn WatchDogEventHandler>,
        monitored_tasks: &Arc<Mutex<HashMap<String, TaskMonitorInfo>>>,
        progress_tracker: &Arc<ProgressTracker>,
    ) {
        if let Ok(mut tasks) = monitored_tasks.lock() {
            let now = Instant::now();
            let max_duration = Duration::from_secs(config.max_task_duration_secs);
            let max_idle_time = Duration::from_secs(config.max_idle_time_secs);
            
            let mut tasks_to_terminate = Vec::new();
            
            for (task_id, monitor_info) in tasks.iter_mut() {
                let task_duration = now.duration_since(monitor_info.start_time);
                let idle_time = now.duration_since(monitor_info.last_progress_update);
                
                // Проверяем превышение максимального времени выполнения
                if task_duration > max_duration {
                    event_handler.handle_event(WatchDogEvent::TaskTimeout {
                        task_id: task_id.clone(),
                        duration: task_duration,
                        max_duration,
                    });
                    
                    if config.auto_terminate_stuck_tasks {
                        tasks_to_terminate.push((task_id.clone(), "Превышен лимит времени выполнения".to_string()));
                    }
                }
                
                // Проверяем отсутствие прогресса
                if idle_time > max_idle_time {
                    event_handler.handle_event(WatchDogEvent::TaskStuck {
                        task_id: task_id.clone(),
                        idle_time,
                        max_idle_time,
                    });
                    
                    monitor_info.warning_count += 1;
                    
                    if config.auto_terminate_stuck_tasks && monitor_info.warning_count >= 3 {
                        tasks_to_terminate.push((task_id.clone(), "Задача не показывает прогресс".to_string()));
                    }
                }
            }
            
            // Завершаем проблемные задачи
            for (task_id, reason) in tasks_to_terminate {
                progress_tracker.update_task_status(&task_id, TaskStatus::Cancelled);
                event_handler.handle_event(WatchDogEvent::TaskTerminated {
                    task_id: task_id.clone(),
                    reason: reason.clone(),
                });
                tasks.remove(&task_id);
            }
        }
    }
    
    /// Проверяет лимиты ресурсов
    fn check_resource_limits(
        config: &WatchDogConfig,
        event_handler: &Arc<dyn WatchDogEventHandler>,
        running_tasks: &Arc<RunningTasks>,
    ) {
        // Проверяем лимит одновременных задач
        if let Some(max_concurrent) = config.max_concurrent_tasks {
            let current_count = running_tasks.get_active_task_count();
            if current_count > max_concurrent {
                event_handler.handle_event(WatchDogEvent::ConcurrentTaskLimitExceeded {
                    current_count,
                    max_count: max_concurrent,
                });
            }
        }
        
        // Проверяем использование памяти (упрощенная версия)
        if let Some(max_memory) = config.max_memory_usage_bytes {
            // В реальной реализации здесь был бы код для получения текущего использования памяти
            // Для примера используем фиктивное значение
            let current_memory = Self::get_current_memory_usage();
            if current_memory > max_memory {
                event_handler.handle_event(WatchDogEvent::MemoryLimitExceeded {
                    current_usage: current_memory,
                    max_usage: max_memory,
                });
            }
        }
    }
    
    /// Проверяет производительность системы
    fn check_performance(
        _config: &WatchDogConfig,
        event_handler: &Arc<dyn WatchDogEventHandler>,
        progress_tracker: &Arc<ProgressTracker>,
        running_tasks: &Arc<RunningTasks>,
    ) {
        let report = progress_tracker.generate_report();
        let (avg_time, min_time, max_time) = progress_tracker.get_execution_time_statistics();
        
        // Проверяем на аномально долгое выполнение
        if max_time > 0 && avg_time > 0.0 && max_time as f64 > avg_time * 3.0 {
            let mut details = HashMap::new();
            details.insert("average_time_ms".to_string(), avg_time.to_string());
            details.insert("max_time_ms".to_string(), max_time.to_string());
            details.insert("active_tasks".to_string(), running_tasks.get_active_task_count().to_string());
            
            event_handler.handle_event(WatchDogEvent::PerformanceWarning {
                message: "Обнаружена аномально долгая задача".to_string(),
                details,
            });
        }
        
        // Проверяем на низкий процент успешных задач
        let (successful, failed, _cancelled) = running_tasks.get_execution_statistics();
        let total_completed = successful + failed;
        if total_completed > 10 && failed as f64 / total_completed as f64 > 0.3 {
            let mut details = HashMap::new();
            details.insert("successful".to_string(), successful.to_string());
            details.insert("failed".to_string(), failed.to_string());
            details.insert("failure_rate".to_string(), format!("{:.1}%", (failed as f64 / total_completed as f64) * 100.0));
            
            event_handler.handle_event(WatchDogEvent::PerformanceWarning {
                message: "Высокий процент неудачных задач".to_string(),
                details,
            });
        }
    }
    
    /// Получает текущее использование памяти (упрощенная версия)
    fn get_current_memory_usage() -> u64 {
        // В реальной реализации здесь был бы код для получения реального использования памяти
        // Например, через /proc/self/status на Linux или GetProcessMemoryInfo на Windows
        0
    }
    
    /// Проверяет, активен ли сторожевой таймер
    pub fn is_running(&self) -> bool {
        Self::is_active(&self.active)
    }
    
    /// Получает текущую конфигурацию
    pub fn get_config(&self) -> &WatchDogConfig {
        &self.config
    }
    
    /// Обновляет конфигурацию
    pub fn update_config(&mut self, config: WatchDogConfig) {
        self.config = config;
    }
    
    /// Получает статистику мониторинга
    pub fn get_monitoring_stats(&self) -> HashMap<String, String> {
        let mut stats = HashMap::new();
        
        if let Ok(tasks) = self.monitored_tasks.lock() {
            stats.insert("monitored_tasks".to_string(), tasks.len().to_string());
            
            let now = Instant::now();
            let total_warnings: u32 = tasks.values().map(|t| t.warning_count).sum();
            stats.insert("total_warnings".to_string(), total_warnings.to_string());
            
            if let Some(oldest_task) = tasks.values().min_by_key(|t| t.start_time) {
                let oldest_duration = now.duration_since(oldest_task.start_time);
                stats.insert("oldest_task_duration_ms".to_string(), 
                    oldest_duration.as_millis().to_string());
            }
        }
        
        stats.insert("is_active".to_string(), self.is_running().to_string());
        stats.insert("check_interval_ms".to_string(), self.config.check_interval_ms.to_string());
        stats.insert("max_task_duration_secs".to_string(), self.config.max_task_duration_secs.to_string());
        
        stats
    }
}

impl Drop for WatchDog {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;
    
    struct TestEventHandler {
        event_count: Arc<AtomicUsize>,
    }
    
    impl TestEventHandler {
        fn new() -> Self {
            Self {
                event_count: Arc::new(AtomicUsize::new(0)),
            }
        }
        
        fn get_event_count(&self) -> usize {
            self.event_count.load(Ordering::Relaxed)
        }
    }
    
    impl WatchDogEventHandler for TestEventHandler {
        fn handle_event(&self, _event: WatchDogEvent) {
            self.event_count.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    #[test]
    fn test_watchdog_config_default() {
        let config = WatchDogConfig::default();
        assert_eq!(config.max_task_duration_secs, 300);
        assert_eq!(config.check_interval_ms, 5000);
        assert_eq!(config.max_idle_time_secs, 60);
        assert!(config.auto_terminate_stuck_tasks);
    }
    
    #[test]
    fn test_watchdog_creation() {
        let config = WatchDogConfig::default();
        let handler = Arc::new(TestEventHandler::new());
        let watchdog = WatchDog::new(config, handler);
        
        assert!(!watchdog.is_running());
        assert_eq!(watchdog.get_config().max_task_duration_secs, 300);
    }
    
    #[test]
    fn test_console_event_handler() {
        let handler = ConsoleEventHandler;
        
        // Тестируем, что обработчик не паникует при различных событиях
        handler.handle_event(WatchDogEvent::TaskTimeout {
            task_id: "test".to_string(),
            duration: Duration::from_secs(10),
            max_duration: Duration::from_secs(5),
        });
        
        handler.handle_event(WatchDogEvent::PerformanceWarning {
            message: "Test warning".to_string(),
            details: HashMap::new(),
        });
    }
    
    #[test]
    fn test_monitoring_stats() {
        let config = WatchDogConfig::default();
        let handler = Arc::new(TestEventHandler::new());
        let watchdog = WatchDog::new(config, handler);
        
        let stats = watchdog.get_monitoring_stats();
        assert!(stats.contains_key("monitored_tasks"));
        assert!(stats.contains_key("is_active"));
        assert_eq!(stats["is_active"], "false");
    }
}
