use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};
use std::collections::VecDeque;
use crate::engine::progress::{ProgressTracker, TaskInfo, TaskStatus};
use crate::engine::tasks::{Task, TaskPriority, RunningTasks};
use crate::engine::model::solution::Solution;
use crate::error::CuttingError;

/// Управляет созданием и выполнением потоков для обработки перестановок
/// 
/// Этот класс контролирует количество одновременно работающих потоков
/// и обеспечивает эффективное использование ресурсов системы.
pub struct PermutationThreadSpawner {
    /// Максимальное количество одновременно работающих потоков
    max_alive_spawner_threads: usize,
    /// Интервал между проверками состояния потоков (в миллисекундах)
    interval_between_max_alive_check: u64,
    /// Список всех созданных потоков
    threads: Arc<Mutex<Vec<JoinHandle<()>>>>,
    /// Трекер прогресса выполнения
    progress_tracker: Arc<ProgressTracker>,
    /// Менеджер задач
    running_tasks: Arc<RunningTasks>,
    /// Флаг остановки
    shutdown: Arc<Mutex<bool>>,
    /// Счетчик созданных потоков
    thread_counter: Arc<Mutex<usize>>,
}

impl PermutationThreadSpawner {
    /// Создает новый spawner с заданными параметрами
    /// 
    /// # Аргументы
    /// * `max_alive_spawner_threads` - максимальное количество одновременных потоков
    /// * `interval_between_max_alive_check` - интервал проверки в миллисекундах
    pub fn new(max_alive_spawner_threads: usize, interval_between_max_alive_check: u64) -> Self {
        Self {
            max_alive_spawner_threads,
            interval_between_max_alive_check,
            threads: Arc::new(Mutex::new(Vec::new())),
            progress_tracker: Arc::new(ProgressTracker::new(interval_between_max_alive_check)),
            running_tasks: Arc::new(RunningTasks::new(max_alive_spawner_threads)),
            shutdown: Arc::new(Mutex::new(false)),
            thread_counter: Arc::new(Mutex::new(0)),
        }
    }
    
    /// Создает spawner с настройками по умолчанию
    pub fn default() -> Self {
        Self::new(5, 1000) // 5 потоков, проверка каждую секунду
    }
    
    /// Запускает новый поток для выполнения задачи
    /// 
    /// # Аргументы
    /// * `task_name` - название задачи
    /// * `task_fn` - функция для выполнения в потоке
    /// 
    /// # Возвращает
    /// Result с успехом или ошибкой
    pub fn spawn<F>(&self, task_name: String, task_fn: F) -> Result<(), CuttingError>
    where
        F: FnOnce() -> Result<Vec<Solution>, CuttingError> + Send + 'static,
    {
        // Ждем, пока не освободится место для нового потока
        self.wait_for_available_slot()?;
        
        // Генерируем уникальный ID для задачи
        let task_id = self.generate_task_id();
        
        // Создаем задачу
        let task = Task::new(task_id.clone(), task_name, TaskPriority::Normal, task_fn);
        
        // Запускаем задачу через менеджер
        self.running_tasks.submit_task(task)?;
        
        // Создаем поток для мониторинга
        let threads = Arc::clone(&self.threads);
        let progress_tracker = Arc::clone(&self.progress_tracker);
        let running_tasks = Arc::clone(&self.running_tasks);
        let shutdown = Arc::clone(&self.shutdown);
        
        let handle = thread::spawn(move || {
            // Мониторим выполнение задачи
            loop {
                // Проверяем флаг остановки
                if let Ok(shutdown_flag) = shutdown.lock() {
                    if *shutdown_flag {
                        break;
                    }
                }
                
                // Обновляем состояние завершенных потоков
                running_tasks.update_completed_threads();
                
                // Небольшая пауза
                thread::sleep(Duration::from_millis(100));
                
                // Проверяем, завершилась ли наша задача
                let progress_report = progress_tracker.generate_report();
                if progress_report.completed_tasks > 0 {
                    break;
                }
            }
        });
        
        // Добавляем handle в список потоков
        if let Ok(mut threads_list) = self.threads.lock() {
            threads_list.push(handle);
        }
        
        Ok(())
    }
    
    /// Ждет освобождения слота для нового потока
    fn wait_for_available_slot(&self) -> Result<(), CuttingError> {
        let start_time = Instant::now();
        let timeout = Duration::from_secs(30); // Таймаут 30 секунд
        
        loop {
            // Проверяем флаг остановки
            if let Ok(shutdown) = self.shutdown.lock() {
                if *shutdown {
                    return Err(CuttingError::OperationCancelled(
                        "Spawner был остановлен".to_string()
                    ));
                }
            }
            
            // Очищаем завершенные потоки
            self.cleanup_finished_threads();
            
            // Проверяем, есть ли свободные слоты
            let active_count = self.get_nbr_unfinished_threads();
            if active_count < self.max_alive_spawner_threads {
                return Ok(());
            }
            
            // Проверяем таймаут
            if start_time.elapsed() > timeout {
                return Err(CuttingError::Timeout(
                    "Таймаут ожидания свободного слота для потока".to_string()
                ));
            }
            
            // Ждем перед следующей проверкой
            thread::sleep(Duration::from_millis(self.interval_between_max_alive_check));
        }
    }
    
    /// Очищает завершенные потоки из списка
    fn cleanup_finished_threads(&self) {
        if let Ok(mut threads) = self.threads.lock() {
            threads.retain(|handle| !handle.is_finished());
        }
        
        // Также обновляем состояние в running_tasks
        self.running_tasks.update_completed_threads();
    }
    
    /// Генерирует уникальный ID для задачи
    fn generate_task_id(&self) -> String {
        if let Ok(mut counter) = self.thread_counter.lock() {
            *counter += 1;
            format!("permutation_task_{}", *counter)
        } else {
            format!("permutation_task_{}", Instant::now().elapsed().as_millis())
        }
    }
    
    /// Получает количество незавершенных потоков
    pub fn get_nbr_unfinished_threads(&self) -> usize {
        self.running_tasks.get_active_task_count()
    }
    
    /// Получает общее количество созданных потоков
    pub fn get_nbr_total_threads(&self) -> usize {
        self.running_tasks.get_total_task_count()
    }
    
    /// Получает количество завершенных потоков
    pub fn get_nbr_finished_threads(&self) -> usize {
        self.running_tasks.get_completed_task_count()
    }
    
    /// Ожидает завершения всех активных потоков
    /// 
    /// # Аргументы
    /// * `timeout` - максимальное время ожидания
    /// 
    /// # Возвращает
    /// true, если все потоки завершились в указанное время
    pub fn wait_for_all_threads(&self, timeout: Option<Duration>) -> bool {
        self.running_tasks.wait_for_completion(timeout)
    }
    
    /// Останавливает spawner и отменяет все активные потоки
    pub fn shutdown(&self) {
        // Устанавливаем флаг остановки
        if let Ok(mut shutdown) = self.shutdown.lock() {
            *shutdown = true;
        }
        
        // Отменяем все активные задачи
        self.running_tasks.cancel_all_tasks();
    }
    
    /// Получает отчет о прогрессе выполнения
    pub fn get_progress_report(&self) -> crate::engine::progress::ProgressReport {
        self.progress_tracker.generate_report()
    }
    
    /// Получает статистику выполнения
    /// 
    /// # Возвращает
    /// Кортеж (успешных, неудачных, отмененных) задач
    pub fn get_execution_statistics(&self) -> (usize, usize, usize) {
        self.running_tasks.get_execution_statistics()
    }
    
    /// Форматирует сводку состояния в строку
    pub fn format_status_summary(&self) -> String {
        let progress_report = self.get_progress_report();
        let (successful, failed, cancelled) = self.get_execution_statistics();
        
        format!(
            "Spawner: {} активных, {} завершено, {} всего | Успешно: {}, Ошибок: {}, Отменено: {}",
            progress_report.running_tasks,
            progress_report.completed_tasks,
            self.get_nbr_total_threads(),
            successful,
            failed,
            cancelled
        )
    }
    
    /// Проверяет, работает ли spawner
    pub fn is_running(&self) -> bool {
        if let Ok(shutdown) = self.shutdown.lock() {
            !*shutdown
        } else {
            false
        }
    }
    
    /// Получает максимальное количество одновременных потоков
    pub fn get_max_alive_threads(&self) -> usize {
        self.max_alive_spawner_threads
    }
    
    /// Устанавливает новое максимальное количество одновременных потоков
    /// 
    /// # Аргументы
    /// * `max_threads` - новое максимальное количество потоков
    pub fn set_max_alive_threads(&mut self, max_threads: usize) {
        self.max_alive_spawner_threads = max_threads;
    }
    
    /// Получает интервал проверки состояния потоков
    pub fn get_check_interval(&self) -> u64 {
        self.interval_between_max_alive_check
    }
    
    /// Устанавливает новый интервал проверки состояния потоков
    /// 
    /// # Аргументы
    /// * `interval_ms` - новый интервал в миллисекундах
    pub fn set_check_interval(&mut self, interval_ms: u64) {
        self.interval_between_max_alive_check = interval_ms;
    }
}

impl Default for PermutationThreadSpawner {
    fn default() -> Self {
        Self::default()
    }
}

/// Пакетный обработчик перестановок
/// 
/// Обрабатывает множество перестановок пакетами для оптимизации производительности
pub struct PermutationBatchProcessor {
    /// Spawner для управления потоками
    spawner: PermutationThreadSpawner,
    /// Размер пакета для обработки
    batch_size: usize,
    /// Очередь перестановок для обработки
    permutation_queue: Arc<Mutex<VecDeque<Vec<String>>>>,
}

impl PermutationBatchProcessor {
    /// Создает новый пакетный обработчик
    /// 
    /// # Аргументы
    /// * `max_threads` - максимальное количество потоков
    /// * `batch_size` - размер пакета для обработки
    pub fn new(max_threads: usize, batch_size: usize) -> Self {
        Self {
            spawner: PermutationThreadSpawner::new(max_threads, 1000),
            batch_size,
            permutation_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    /// Добавляет перестановки в очередь для обработки
    /// 
    /// # Аргументы
    /// * `permutations` - список перестановок для добавления
    pub fn add_permutations(&self, permutations: Vec<Vec<String>>) {
        if let Ok(mut queue) = self.permutation_queue.lock() {
            for perm in permutations {
                queue.push_back(perm);
            }
        }
    }
    
    /// Обрабатывает все перестановки в очереди
    /// 
    /// # Аргументы
    /// * `process_fn` - функция для обработки пакета перестановок
    /// 
    /// # Возвращает
    /// Result с количеством обработанных пакетов
    pub fn process_all<F>(&self, process_fn: F) -> Result<usize, CuttingError>
    where
        F: Fn(Vec<Vec<String>>) -> Result<Vec<Solution>, CuttingError> + Send + Sync + 'static,
    {
        let process_fn = Arc::new(process_fn);
        let mut batch_count = 0;
        
        loop {
            // Извлекаем пакет из очереди
            let batch = {
                if let Ok(mut queue) = self.permutation_queue.lock() {
                    let mut batch = Vec::new();
                    for _ in 0..self.batch_size {
                        if let Some(perm) = queue.pop_front() {
                            batch.push(perm);
                        } else {
                            break;
                        }
                    }
                    batch
                } else {
                    break;
                }
            };
            
            if batch.is_empty() {
                break;
            }
            
            // Запускаем обработку пакета
            let process_fn_clone = Arc::clone(&process_fn);
            let task_name = format!("Batch_{}", batch_count + 1);
            
            self.spawner.spawn(task_name, move || {
                process_fn_clone(batch)
            })?;
            
            batch_count += 1;
        }
        
        Ok(batch_count)
    }
    
    /// Ожидает завершения всех пакетов
    /// 
    /// # Аргументы
    /// * `timeout` - максимальное время ожидания
    /// 
    /// # Возвращает
    /// true, если все пакеты обработаны
    pub fn wait_for_completion(&self, timeout: Option<Duration>) -> bool {
        self.spawner.wait_for_all_threads(timeout)
    }
    
    /// Получает количество оставшихся перестановок в очереди
    pub fn get_queue_size(&self) -> usize {
        if let Ok(queue) = self.permutation_queue.lock() {
            queue.len()
        } else {
            0
        }
    }
    
    /// Очищает очередь перестановок
    pub fn clear_queue(&self) {
        if let Ok(mut queue) = self.permutation_queue.lock() {
            queue.clear();
        }
    }
    
    /// Получает статистику обработки
    pub fn get_processing_statistics(&self) -> (usize, usize, usize) {
        self.spawner.get_execution_statistics()
    }
    
    /// Останавливает обработку
    pub fn shutdown(&self) {
        self.spawner.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_spawner_creation() {
        let spawner = PermutationThreadSpawner::new(3, 500);
        assert_eq!(spawner.get_max_alive_threads(), 3);
        assert_eq!(spawner.get_check_interval(), 500);
        assert!(spawner.is_running());
    }
    
    #[test]
    fn test_spawner_default() {
        let spawner = PermutationThreadSpawner::default();
        assert_eq!(spawner.get_max_alive_threads(), 5);
        assert_eq!(spawner.get_check_interval(), 1000);
    }
    
    #[test]
    fn test_spawn_simple_task() {
        let spawner = PermutationThreadSpawner::new(2, 100);
        
        let result = spawner.spawn("test_task".to_string(), || {
            thread::sleep(Duration::from_millis(50));
            Ok(vec![])
        });
        
        assert!(result.is_ok());
        
        // Ждем завершения
        assert!(spawner.wait_for_all_threads(Some(Duration::from_secs(1))));
        
        let (successful, failed, cancelled) = spawner.get_execution_statistics();
        assert_eq!(successful, 1);
        assert_eq!(failed, 0);
        assert_eq!(cancelled, 0);
    }
    
    #[test]
    fn test_spawn_multiple_tasks() {
        let spawner = PermutationThreadSpawner::new(2, 100);
        
        // Запускаем 3 задачи (больше чем лимит потоков)
        for i in 0..3 {
            let task_name = format!("task_{}", i);
            let result = spawner.spawn(task_name, move || {
                thread::sleep(Duration::from_millis(100));
                Ok(vec![])
            });
            assert!(result.is_ok());
        }
        
        // Ждем завершения всех задач
        assert!(spawner.wait_for_all_threads(Some(Duration::from_secs(2))));
        
        assert_eq!(spawner.get_nbr_total_threads(), 3);
        let (successful, failed, cancelled) = spawner.get_execution_statistics();
        assert_eq!(successful, 3);
    }
    
    #[test]
    fn test_batch_processor() {
        let processor = PermutationBatchProcessor::new(2, 2);
        
        // Добавляем перестановки
        let permutations = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["B".to_string(), "A".to_string()],
            vec!["A".to_string(), "C".to_string()],
        ];
        processor.add_permutations(permutations);
        
        assert_eq!(processor.get_queue_size(), 3);
        
        // Обрабатываем
        let result = processor.process_all(|batch| {
            thread::sleep(Duration::from_millis(50));
            Ok(vec![]) // Возвращаем пустой результат для теста
        });
        
        assert!(result.is_ok());
        assert!(processor.wait_for_completion(Some(Duration::from_secs(1))));
        
        let (successful, failed, cancelled) = processor.get_processing_statistics();
        assert!(successful > 0);
    }
    
    #[test]
    fn test_spawner_shutdown() {
        let spawner = PermutationThreadSpawner::new(2, 100);
        
        // Запускаем долгую задачу
        let result = spawner.spawn("long_task".to_string(), || {
            thread::sleep(Duration::from_secs(10)); // Долгая задача
            Ok(vec![])
        });
        assert!(result.is_ok());
        
        // Останавливаем spawner
        spawner.shutdown();
        assert!(!spawner.is_running());
        
        // Попытка запустить новую задачу должна завершиться ошибкой
        let result = spawner.spawn("new_task".to_string(), || Ok(vec![]));
        assert!(result.is_err());
    }
}
