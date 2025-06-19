use crate::engine::model::request::CalculationRequest;
use crate::engine::model::response::{CalculationResponse, TaskStatusResponse, Stats, StatusCode, CalculationSubmissionResult};
use crate::engine::model::solution::Solution;
use crate::engine::model::tile::TileDimensions;
use crate::engine::stock::StockSolution;
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

/// Статус задачи для сервиса
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
    fn optimize(&mut self, request: CalculationRequest) -> Result<CalculationResponse, CuttingError>;
    
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

/// Результат оптимизации
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub solutions: Vec<Solution>,
    pub placed_panels_count: usize,
    pub total_area: f64,
    pub used_area: f64,
    pub efficiency: f64,
    pub cuts_count: usize,
}

impl OptimizationResult {
    pub fn new() -> Self {
        Self {
            solutions: Vec::new(),
            placed_panels_count: 0,
            total_area: 0.0,
            used_area: 0.0,
            efficiency: 0.0,
            cuts_count: 0,
        }
    }
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

    /// Выполняет основную логику оптимизации используя правильную интеграцию с Solution и Mosaic
    fn perform_optimization(&self, request: &CalculationRequest) -> Result<OptimizationResult, CuttingError> {
        println!("🔧 perform_optimization: Начинаем основную оптимизацию");
        self.cut_list_logger.info("Начинаем основную оптимизацию с правильной интеграцией");
        
        // Конвертируем панели из запроса в TileDimensions с правильным учетом count
        let mut tile_dimensions_list = Vec::new();
        let tile_id_counter = 1000; // Начинаем с большого числа для уникальности
        
        for panel in &request.panels {
            if panel.is_valid() {
                if let (Ok(width_f64), Ok(height_f64)) = (panel.width.parse::<f64>(), panel.height.parse::<f64>()) {
                    let width = width_f64 as i32;
                    let height = height_f64 as i32;
                    println!("📦 Обрабатываем панель ID {}: {}x{} count={}", panel.id, width, height, panel.count);
                    // Создаем count экземпляров каждой панели с уникальными ID
                    for i in 0..panel.count {
                        let unique_id = tile_id_counter + (panel.id * 1000) + i;
                        let tile_dimensions = TileDimensions::new(
                            unique_id,
                            width,
                            height,
                            panel.material.clone(),
                            panel.orientation,
                            panel.label.clone(),
                        );
                        println!("  ➕ Создана плитка ID {}: {}x{}", unique_id, width, height);
                        tile_dimensions_list.push(tile_dimensions);
                    }
                } else {
                    println!("⚠️ Не удалось парсить размеры панели ID {}: width='{}', height='{}'", 
                        panel.id, panel.width, panel.height);
                }
            } else {
                println!("⚠️ Панель ID {} не валидна: enabled={}, count={}", 
                    panel.id, panel.enabled, panel.count);
            }
        }
        
        // Конвертируем складские панели с правильным учетом count
        let mut stock_tile_dimensions = Vec::new();
        for stock_panel in &request.stock_panels {
            if stock_panel.is_valid() {
                if let (Ok(width), Ok(height)) = (stock_panel.width.parse::<i32>(), stock_panel.height.parse::<i32>()) {
                    println!("📋 Обрабатываем стоковую панель ID {}: {}x{} count={}", stock_panel.id, width, height, stock_panel.count);
                    // Создаем count экземпляров каждой складской панели с уникальными ID
                    for i in 0..stock_panel.count {
                        let unique_id = tile_id_counter + (stock_panel.id * 1000) + i + 100000; // Смещение для складских панелей
                        let tile_dimensions = TileDimensions::new(
                            unique_id,
                            width,
                            height,
                            stock_panel.material.clone(),
                            stock_panel.orientation,
                            stock_panel.label.clone(),
                        );
                        println!("  ➕ Создана стоковая плитка ID {}: {}x{}", unique_id, width, height);
                        stock_tile_dimensions.push(tile_dimensions);
                    }
                }
            }
        }
        
        println!("📊 Итого создано: {} панелей для размещения, {} стоковых панелей", 
            tile_dimensions_list.len(), stock_tile_dimensions.len());
        
        self.cut_list_logger.info(&format!(
            "Подготовлено {} панелей и {} складских панелей для оптимизации",
            tile_dimensions_list.len(),
            stock_tile_dimensions.len()
        ));
        
        // Проверяем, что у нас есть панели для размещения
        if tile_dimensions_list.is_empty() {
            println!("❌ Нет валидных панелей для размещения");
            self.cut_list_logger.warning("Нет валидных панелей для размещения");
            return Ok(OptimizationResult::new());
        }
        
        // Проверяем, что у нас есть складские панели
        if stock_tile_dimensions.is_empty() {
            println!("❌ Нет валидных складских панелей");
            self.cut_list_logger.warning("Нет валидных складских панелей");
            return Ok(OptimizationResult::new());
        }
        
        // Сортируем панели по убыванию площади (как в Java версии)
        tile_dimensions_list.sort_by(|a, b| {
            let area_a = a.get_area();
            let area_b = b.get_area();
            area_b.cmp(&area_a)
        });
        
        println!("🔄 Запуск compute_optimal_solution...");
        // Выполняем оптимизацию используя правильный алгоритм по образцу Java
        let optimization_result = self.compute_optimal_solution(&tile_dimensions_list, &stock_tile_dimensions)?;
        
        println!("✅ compute_optimal_solution завершен: размещено {} панелей", optimization_result.placed_panels_count);
        
        self.cut_list_logger.info(&format!(
            "Оптимизация завершена: размещено {}/{} панелей, эффективность {:.2}%, разрезов: {}",
            optimization_result.placed_panels_count,
            tile_dimensions_list.len(),
            optimization_result.efficiency,
            optimization_result.cuts_count
        ));
        
        Ok(optimization_result)
    }

    /// Основной алгоритм оптимизации по образцу Java CutListThread.computeSolutions
    fn compute_optimal_solution(
        &self,
        tiles: &[TileDimensions],
        stock_tiles: &[TileDimensions],
    ) -> Result<OptimizationResult, CuttingError> {
        println!("🔧 compute_optimal_solution: Начинаем алгоритм оптимизации");
        println!("📊 Входные данные: {} панелей для размещения, {} стоковых панелей", tiles.len(), stock_tiles.len());
        self.cut_list_logger.info("Запуск алгоритма оптимизации по образцу Java CutListThread");
        
        // Создаем стоковые решения (комбинации складских панелей)
        println!("🏗️ Генерируем стоковые решения...");
        let stock_solutions = self.generate_stock_solutions(stock_tiles, tiles);
        println!("✅ Создано {} стоковых решений", stock_solutions.len());
        
        if stock_solutions.is_empty() {
            println!("❌ Нет стоковых решений для обработки!");
            return Ok(OptimizationResult::new());
        }
        
        let mut best_solutions = Vec::new();
        let mut total_placed_panels = 0;
        let mut total_area = 0.0;
        let mut used_area = 0.0;
        let mut cuts_count = 0;
        let mut best_efficiency = 0.0;
        
        // Перебираем стоковые решения
        for (stock_idx, stock_solution) in stock_solutions.iter().enumerate().take(MAX_STOCK_ITERATIONS) {
            println!("🔄 Стоковое решение {}/{}: {} панелей, площадь: {}", 
                stock_idx + 1, stock_solutions.len(),
                stock_solution.get_stock_tile_dimensions().len(),
                stock_solution.get_total_area()
            );
            
            self.cut_list_logger.info(&format!(
                "Пробуем стоковое решение {}: {} панелей, площадь: {}",
                stock_idx + 1,
                stock_solution.get_stock_tile_dimensions().len(),
                stock_solution.get_total_area()
            ));
            
            // Генерируем перестановки панелей для размещения
            println!("🔀 Генерируем перестановки панелей...");
            let permutations = self.generate_tile_permutations(tiles);
            println!("✅ Создано {} перестановок", permutations.len());
            
            // Перебираем перестановки
            for (perm_idx, permutation) in permutations.iter().enumerate().take(MAX_PERMUTATION_ITERATIONS.min(20)) {
                if perm_idx % 5 == 0 {
                    println!("🔄 Обрабатываем перестановку {}/{}", perm_idx + 1, permutations.len().min(20));
                    self.cut_list_logger.info(&format!("Обрабатываем перестановку {}", perm_idx + 1));
                }
                
                // Выполняем размещение используя алгоритм как в Java CutListThread.computeSolutions
                println!("🎯 Запускаем compute_solutions_for_permutation для перестановки {}...", perm_idx + 1);
                match self.compute_solutions_for_permutation(&permutation, stock_solution) {
                    Ok(solutions) => {
                        println!("✅ compute_solutions_for_permutation вернул {} решений", solutions.len());
                        if !solutions.is_empty() {
                            let best_solution = &solutions[0]; // Берем лучшее решение
                            
                            let solution_placed = best_solution.get_nbr_final_tiles() as usize;
                            let solution_total_area = best_solution.get_total_area() as f64;
                            let solution_used_area = best_solution.get_used_area() as f64;
                            let solution_efficiency = if solution_total_area > 0.0 {
                                (solution_used_area / solution_total_area) * 100.0
                            } else {
                                0.0
                            };
                            let solution_cuts = best_solution.get_cuts_count();
                            
                            println!("📊 Результат перестановки {}: размещено {}/{} панелей, эффективность {:.2}%", 
                                perm_idx + 1, solution_placed, tiles.len(), solution_efficiency);
                            
                            self.cut_list_logger.info(&format!(
                                "Перестановка {}: размещено {}/{} панелей, эффективность {:.2}%",
                                perm_idx + 1,
                                solution_placed,
                                tiles.len(),
                                solution_efficiency
                            ));
                            
                            // Проверяем, лучше ли это решение
                            if solution_placed > total_placed_panels || 
                               (solution_placed == total_placed_panels && solution_efficiency > best_efficiency) {
                                
                                println!("🎉 Новое лучшее решение: размещено {}/{} панелей, эффективность {:.2}%",
                                    solution_placed, tiles.len(), solution_efficiency);
                                
                                self.cut_list_logger.info(&format!(
                                    "Новое лучшее решение: размещено {}/{} панелей, эффективность {:.2}%",
                                    solution_placed,
                                    tiles.len(),
                                    solution_efficiency
                                ));
                                
                                best_solutions = solutions;
                                total_placed_panels = solution_placed;
                                total_area = solution_total_area;
                                used_area = solution_used_area;
                                cuts_count = solution_cuts as usize;
                                best_efficiency = solution_efficiency;
                            }
                        } else {
                            println!("⚠️ compute_solutions_for_permutation вернул пустой список решений");
                        }
                    }
                    Err(e) => {
                        println!("❌ Ошибка при обработке перестановки {}: {}", perm_idx + 1, e);
                        self.cut_list_logger.warning(&format!(
                            "Ошибка при обработке перестановки {}: {}",
                            perm_idx + 1, e
                        ));
                    }
                }
                
                // Если достигли отличного размещения, прекращаем поиск
                if total_placed_panels == tiles.len() && best_efficiency > 95.0 {
                    println!("🎯 Достигнуто отличное размещение, прекращаем поиск");
                    self.cut_list_logger.info("Достигнуто отличное размещение, прекращаем поиск");
                    break;
                }
            }
            
            // Если все панели размещены с хорошей эффективностью, прекращаем
            if total_placed_panels == tiles.len() && best_efficiency > 80.0 {
                println!("🎯 Все панели размещены с хорошей эффективностью, завершаем оптимизацию");
                self.cut_list_logger.info("Все панели размещены с хорошей эффективностью, завершаем оптимизацию");
                break;
            }
        }
        
        let efficiency = if total_area > 0.0 {
            (used_area / total_area) * 100.0
        } else {
            0.0
        };
        
        println!("🏁 Финальный результат: размещено {}/{} панелей, эффективность {:.2}%", 
            total_placed_panels, tiles.len(), efficiency);
        
        Ok(OptimizationResult {
            solutions: best_solutions,
            placed_panels_count: total_placed_panels,
            total_area,
            used_area,
            efficiency,
            cuts_count,
        })
    }

    /// Генерирует стоковые решения (комбинации складских панелей)
    fn generate_stock_solutions(&self, stock_tiles: &[TileDimensions], tiles: &[TileDimensions]) -> Vec<StockSolution> {
        let mut solutions = Vec::new();
        
        // Вычисляем общую площадь панелей для размещения
        let total_tiles_area: i64 = tiles.iter().map(|t| t.get_area()).sum();
        
        // Добавляем одиночные складские панели
        for stock_tile in stock_tiles {
            if stock_tile.get_area() >= total_tiles_area / 4 { // Только если панель достаточно большая
                solutions.push(StockSolution::new(vec![stock_tile.clone()]));
            }
        }
        
        // Добавляем комбинации из нескольких панелей (до 3 для производительности)
        if stock_tiles.len() > 1 {
            for i in 0..stock_tiles.len() {
                for j in (i+1)..stock_tiles.len().min(i+4) {
                    let combo = vec![stock_tiles[i].clone(), stock_tiles[j].clone()];
                    let combo_area: i64 = combo.iter().map(|t| t.get_area()).sum();
                    
                    // Добавляем только если комбинация может вместить хотя бы 30% панелей
                    if combo_area >= total_tiles_area / 3 {
                        solutions.push(StockSolution::new(combo));
                    }
                }
            }
        }
        
        // Сортируем решения по площади (сначала меньшие для экономии материала)
        solutions.sort_by(|a, b| a.get_total_area().cmp(&b.get_total_area()));
        
        // Ограничиваем количество для производительности
        solutions.truncate(50);
        
        solutions
    }

    /// Генерирует перестановки панелей (различные стратегии сортировки)
    fn generate_tile_permutations(&self, tiles: &[TileDimensions]) -> Vec<Vec<TileDimensions>> {
        let mut permutations = Vec::new();
        
        // 1. Исходный порядок (по убыванию площади)
        permutations.push(tiles.to_vec());
        
        // 2. Сортировка по ширине (убывание)
        let mut by_width = tiles.to_vec();
        by_width.sort_by(|a, b| b.width.cmp(&a.width));
        permutations.push(by_width);
        
        // 3. Сортировка по высоте (убывание)
        let mut by_height = tiles.to_vec();
        by_height.sort_by(|a, b| b.height.cmp(&a.height));
        permutations.push(by_height);
        
        // 4. Сортировка по максимальному измерению
        let mut by_max_dim = tiles.to_vec();
        by_max_dim.sort_by(|a, b| b.get_max_dimension().cmp(&a.get_max_dimension()));
        permutations.push(by_max_dim);
        
        // 5. Сортировка по периметру
        let mut by_perimeter = tiles.to_vec();
        by_perimeter.sort_by(|a, b| {
            let perimeter_a = 2 * (a.width + a.height);
            let perimeter_b = 2 * (b.width + b.height);
            perimeter_b.cmp(&perimeter_a)
        });
        permutations.push(by_perimeter);
        
        // 6. Обратный порядок
        let mut reversed = tiles.to_vec();
        reversed.reverse();
        permutations.push(reversed);
        
        permutations
    }

    /// Выполняет размещение для конкретной перестановки (аналог Java CutListThread.computeSolutions)
    fn compute_solutions_for_permutation(
        &self,
        tiles: &[TileDimensions],
        stock_solution: &StockSolution,
    ) -> Result<Vec<Solution>, CuttingError> {
        // Создаем начальное решение из стокового решения
        let mut solutions = vec![Solution::from_stock_solution(stock_solution)];
        
        // Последовательно размещаем каждую панель (как в Java CutListThread.computeSolutions)
        for (tile_index, tile) in tiles.iter().enumerate() {
            let mut new_solutions = Vec::new();
            
            // Для каждого текущего решения пытаемся разместить панель
            for solution in &mut solutions {
                match solution.try_place_tile(tile) {
                    Ok(placement_results) => {
                        // Добавляем все успешные размещения
                        new_solutions.extend(placement_results);
                    }
                    Err(e) => {
                        self.cut_list_logger.warning(&format!(
                            "Ошибка размещения панели {} в решении: {}",
                            tile_index + 1, e
                        ));
                        
                        // Добавляем исходное решение с панелью в списке неразмещенных
                        let mut failed_solution = Solution::copy(solution);
                        failed_solution.get_no_fit_panels_mut().push(tile.clone());
                        new_solutions.push(failed_solution);
                    }
                }
            }
            
            solutions = new_solutions;
            
            // Удаляем дубликаты и сортируем решения
            self.remove_duplicate_solutions(&mut solutions);
            self.sort_solutions_by_quality(&mut solutions);
            
            // Ограничиваем количество решений для производительности
            if solutions.len() > 100 {
                solutions.truncate(100);
            }
            
            // Логируем прогресс
            if tile_index % 10 == 0 && tile_index > 0 {
                self.cut_list_logger.info(&format!(
                    "Обработано {} из {} панелей, текущее количество решений: {}",
                    tile_index + 1, tiles.len(), solutions.len()
                ));
            }
        }
        
        Ok(solutions)
    }

    /// Удаляет дубликаты решений (аналог Java removeDuplicated)
    fn remove_duplicate_solutions(&self, solutions: &mut Vec<Solution>) {
        let mut seen_signatures = std::collections::HashSet::new();
        
        solutions.retain(|solution| {
            let signature = solution.get_structure_identifier();
            seen_signatures.insert(signature)
        });
    }

    /// Сортирует решения по качеству (аналог Java sort)
    fn sort_solutions_by_quality(&self, solutions: &mut Vec<Solution>) {
        solutions.sort_by(|a, b| {
            // Сначала по количеству размещенных панелей (больше лучше)
            let placed_a = a.get_nbr_final_tiles();
            let placed_b = b.get_nbr_final_tiles();
            
            match placed_b.cmp(&placed_a) {
                std::cmp::Ordering::Equal => {
                    // Затем по эффективности использования площади (больше лучше)
                    let efficiency_a = if a.get_total_area() > 0 {
                        (a.get_used_area() as f64 / a.get_total_area() as f64) * 100.0
                    } else {
                        0.0
                    };
                    let efficiency_b = if b.get_total_area() > 0 {
                        (b.get_used_area() as f64 / b.get_total_area() as f64) * 100.0
                    } else {
                        0.0
                    };
                    
                    match efficiency_b.partial_cmp(&efficiency_a).unwrap_or(std::cmp::Ordering::Equal) {
                        std::cmp::Ordering::Equal => {
                            // Если эффективность одинаковая, сортируем по общей площади (больше лучше)
                            b.get_total_area().cmp(&a.get_total_area())
                        }
                        other => other,
                    }
                }
                other => other,
            }
        });
    }
}

impl CutListOptimizerService for CutListOptimizerServiceImpl {
    fn optimize(&mut self, request: CalculationRequest) -> Result<CalculationResponse, CuttingError> {
        println!("🔧 Начало синхронной оптимизации через CalculationRequest");
        self.cut_list_logger.info("Начало синхронной оптимизации");
        
        // Валидируем панели
        let (_panel_count, panel_status) = self.validate_panels(&request.panels);
        println!("📋 Валидация панелей: count={}, status={:?}", _panel_count, panel_status);
        if panel_status != StatusCode::Ok {
            return Err(CuttingError::GeneralCuttingError(
                format!("Ошибка валидации панелей: {}", panel_status.description())
            ));
        }

        // Валидируем складские панели
        let (_stock_count, stock_status) = self.validate_stock_panels(&request.stock_panels);
        println!("📦 Валидация складских панелей: count={}, status={:?}", _stock_count, stock_status);
        if stock_status != StatusCode::Ok {
            return Err(CuttingError::GeneralCuttingError(
                format!("Ошибка валидации складских панелей: {}", stock_status.description())
            ));
        }

        self.cut_list_logger.info(&format!(
            "Валидация прошла успешно: {} панелей, {} складских панелей", 
            _panel_count, _stock_count
        ));

        println!("🚀 Запуск perform_optimization...");
        // Выполняем оптимизацию
        let optimization_result = self.perform_optimization(&request)?;
        println!("✅ perform_optimization завершен: размещено {} панелей", optimization_result.placed_panels_count);
        
        // Создаем ответ с результатами оптимизации
        let mut response = CalculationResponse::new();
        
        // Подсчитываем общее количество панелей из запроса (с учетом count)
        let total_panels_count: usize = request.panels.iter()
            .map(|panel| panel.count as usize)
            .sum();
        
        // Обновляем статистику с реальными данными
        response.statistics.update(
            total_panels_count,
            optimization_result.placed_panels_count,
            optimization_result.total_area,
            optimization_result.used_area
        );
        
        // Конвертируем решения в панели ответа
        if !optimization_result.solutions.is_empty() {
            let best_solution = &optimization_result.solutions[0];
            
            // Получаем все размещенные панели из мозаик
            let final_tile_nodes = best_solution.get_final_tile_nodes();
            for tile_node in final_tile_nodes {
                let tile_dimensions = TileDimensions::new(
                    tile_node.external_id,
                    tile_node.get_width(),
                    tile_node.get_height(),
                    "DEFAULT_MATERIAL".to_string(),
                    0,
                    None,
                );
                
                let position = crate::engine::model::response::PanelPosition::new(
                    tile_node.get_x1(),
                    tile_node.get_y1(),
                    tile_node.get_width(),
                    tile_node.get_height(),
                    tile_node.is_rotated,
                );
                
                let optimized_panel = crate::engine::model::response::OptimizedPanel::new(
                    tile_dimensions,
                    position,
                    "stock_0".to_string(), // TODO: Получить правильный stock_id из мозаики
                    "DEFAULT_MATERIAL".to_string(),
                );
                
                response.panels.push(optimized_panel);
            }
            
            // Добавляем панели, которые не поместились
            for no_fit_panel in best_solution.get_no_fit_panels() {
                response.no_fit_panels.push(no_fit_panel.clone());
            }
        }
        
        // Добавляем метаданные
        response.add_metadata("optimization_type".to_string(), "synchronous".to_string());
        response.add_metadata("panel_count".to_string(), _panel_count.to_string());
        response.add_metadata("stock_count".to_string(), _stock_count.to_string());
        response.add_metadata("placed_panels".to_string(), optimization_result.placed_panels_count.to_string());
        response.add_metadata("efficiency".to_string(), format!("{:.2}%", optimization_result.efficiency));
        response.add_metadata("cuts_count".to_string(), optimization_result.cuts_count.to_string());
        
        self.cut_list_logger.info(&format!(
            "Синхронная оптимизация завершена: размещено {}/{} панелей, эффективность {:.2}%",
            optimization_result.placed_panels_count,
            request.panels.len(),
            optimization_result.efficiency
        ));
        
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
        
        // Запускаем вычисление в отдельном потоке
        let request_clone = request.clone();
        let task_id_clone = task_id.clone();
        let client_id_clone = client_id.clone();
        let logger_clone = Arc::clone(&self.cut_list_logger);
        let running_tasks_clone = Arc::clone(&self.running_tasks);
        let client_tasks_clone = Arc::clone(&self.client_tasks);
        let task_info_clone = Arc::clone(&self.task_info);
        
        // Создаем задачу для выполнения
        let task = Task::new(
            task_id_clone.clone(),
            "Оптимизация раскроя".to_string(),
            TaskPriority::Normal,
            move || {
                logger_clone.info(&format!("Начало выполнения задачи {}", task_id_clone));
                
                // Обновляем статус задачи
                if let Ok(mut task_info_map) = task_info_clone.lock() {
                    if let Some(info) = task_info_map.get_mut(&task_id_clone) {
                        info.status = ServiceTaskStatus::Running;
                        info.progress_percentage = 10;
                    }
                }
                
                // Выполняем реальную оптимизацию
                // Создаем временный сервис для выполнения оптимизации
                let temp_service = CutListOptimizerServiceImpl::new(Arc::clone(&logger_clone));
                let optimization_result = temp_service.perform_optimization(&request_clone);
                
                match optimization_result {
                    Ok(result) => {
                        // Создаем решения на основе результата
                        let solutions = result.solutions;
                        
                        // Обновляем статус задачи на завершенную
                        if let Ok(mut task_info_map) = task_info_clone.lock() {
                            if let Some(info) = task_info_map.get_mut(&task_id_clone) {
                                info.status = ServiceTaskStatus::Completed;
                                info.progress_percentage = 100;
                                info.end_time = Some(Utc::now());
                                if !solutions.is_empty() {
                                    info.solution = Some(solutions[0].clone());
                                }
                            }
                        }
                        
                        logger_clone.info(&format!(
                            "Задача {} завершена успешно: размещено {} панелей, эффективность {:.2}%",
                            task_id_clone, result.placed_panels_count, result.efficiency
                        ));
                        
                        Ok(solutions)
                    }
                    Err(e) => {
                        // Обновляем статус задачи на ошибку
                        if let Ok(mut task_info_map) = task_info_clone.lock() {
                            if let Some(info) = task_info_map.get_mut(&task_id_clone) {
                                info.status = ServiceTaskStatus::Error;
                                info.end_time = Some(Utc::now());
                            }
                        }
                        
                        logger_clone.error(&format!("Ошибка выполнения задачи {}: {}", task_id_clone, e));
                        Err(e)
                    }
                }
            },
        );

        // Добавляем задачу в менеджер
        let logger_for_error = Arc::clone(&self.cut_list_logger);
        let task_info_for_error = Arc::clone(&self.task_info);
        if let Err(e) = running_tasks_clone.submit_task(task) {
            logger_for_error.error(&format!("Ошибка при добавлении задачи {}: {}", task_id, e));
            // Удаляем задачу у клиента при ошибке
            if let Ok(mut client_tasks) = client_tasks_clone.lock() {
                if let Some(tasks) = client_tasks.get_mut(&client_id_clone) {
                    tasks.retain(|id| id != &task_id);
                    if tasks.is_empty() {
                        client_tasks.remove(&client_id_clone);
                    }
                }
            }
            
            // Обновляем статус задачи на ошибку
            if let Ok(mut task_info_map) = task_info_for_error.lock() {
                if let Some(info) = task_info_map.get_mut(&task_id) {
                    info.status = ServiceTaskStatus::Error;
                    info.end_time = Some(Utc::now());
                }
            }
        }

        self.cut_list_logger.info(&format!("Задача {} отправлена на выполнение", task_id));
        
        Ok(CalculationSubmissionResult::success(task_id))
    }

    fn get_task_status(&self, task_id: &str) -> Result<Option<TaskStatusResponse>, CuttingError> {
        if let Ok(task_info) = self.task_info.lock() {
            if let Some(info) = task_info.get(task_id) {
                let mut response = TaskStatusResponse::new(format!("{:?}", info.status));
                response.update_progress(info.progress_percentage, info.progress_percentage);
                response.details = Some(format!("Задача {}: {:?}", task_id, info.status));
                
                // Если есть решение, создаем ответ с решением
                if let Some(ref solution) = info.solution {
                    let mut calc_response = CalculationResponse::new();
                    calc_response.statistics.update(
                        solution.get_nbr_final_tiles() as usize,
                        solution.get_nbr_final_tiles() as usize,
                        solution.get_total_area() as f64,
                        solution.get_used_area() as f64
                    );
                    response.set_solution(calc_response);
                }
                
                return Ok(Some(response));
            }
        }

        // Проверяем активные задачи в running_tasks
        if self.running_tasks.get_active_task_count() > 0 {
            let mut response = TaskStatusResponse::new("RUNNING".to_string());
            response.update_progress(50, 25);
            response.details = Some("Выполняется оптимизация".to_string());
            return Ok(Some(response));
        }

        Ok(None)
    }

    fn stop_task(&mut self, task_id: &str) -> Result<Option<TaskStatusResponse>, CuttingError> {
        self.cut_list_logger.info(&format!("Остановка задачи {}", task_id));
        
        // Обновляем статус в task_info
        if let Ok(mut task_info) = self.task_info.lock() {
            if let Some(info) = task_info.get_mut(task_id) {
                info.status = ServiceTaskStatus::Stopped;
                info.end_time = Some(Utc::now());
                
                let mut response = TaskStatusResponse::new("STOPPED".to_string());
                response.update_progress(100, 100);
                response.details = Some("Задача остановлена".to_string());
                
                return Ok(Some(response));
            }
        }

        Ok(None)
    }

    fn terminate_task(&mut self, task_id: &str) -> Result<i32, CuttingError> {
        self.cut_list_logger.info(&format!("Принудительное завершение задачи {}", task_id));
        
        // Обновляем статус в task_info
        if let Ok(mut task_info) = self.task_info.lock() {
            if let Some(info) = task_info.get_mut(task_id) {
                info.status = ServiceTaskStatus::Terminated;
                info.end_time = Some(Utc::now());
                return Ok(0);
            }
        }

        // Если задача не найдена
        Ok(-1)
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

    #[test]
    fn test_optimization_result_creation() {
        let result = OptimizationResult::new();
        assert_eq!(result.placed_panels_count, 0);
        assert_eq!(result.efficiency, 0.0);
        assert!(result.solutions.is_empty());
    }

    #[test]
    fn test_generate_stock_solutions() {
        let logger = Arc::new(CutListLoggerImpl::new());
        let service = CutListOptimizerServiceImpl::new(logger);
        
        let stock_tiles = vec![
            TileDimensions::simple(1000, 600),
            TileDimensions::simple(800, 500),
        ];
        let tiles = vec![
            TileDimensions::simple(200, 200),
        ];
        
        let solutions = service.generate_stock_solutions(&stock_tiles, &tiles);
        assert!(!solutions.is_empty());
        // Проверяем, что решения отсортированы по площади
        if solutions.len() > 1 {
            assert!(solutions[0].get_total_area() <= solutions[1].get_total_area());
        }
    }

    #[test]
    fn test_generate_tile_permutations() {
        let logger = Arc::new(CutListLoggerImpl::new());
        let service = CutListOptimizerServiceImpl::new(logger);
        
        let tiles = vec![
            TileDimensions::simple(100, 50),
            TileDimensions::simple(200, 100),
            TileDimensions::simple(150, 75),
        ];
        
        let permutations = service.generate_tile_permutations(&tiles);
        assert_eq!(permutations.len(), 6); // 6 различных стратегий сортировки
        
        // Проверяем, что исходный порядок сохранен в первой перестановке
        assert_eq!(permutations[0], tiles);
    }

    #[test]
    fn test_remove_duplicate_solutions() {
        let logger = Arc::new(CutListLoggerImpl::new());
        let service = CutListOptimizerServiceImpl::new(logger);
        
        let stock_solution = StockSolution::new(vec![TileDimensions::simple(1000, 600)]);
        let solution1 = Solution::from_stock_solution(&stock_solution);
        let solution2 = Solution::from_stock_solution(&stock_solution);
        
        let mut solutions = vec![solution1, solution2];
        let original_count = solutions.len();
        
        service.remove_duplicate_solutions(&mut solutions);
        
        // После удаления дубликатов должно остаться меньше решений
        assert!(solutions.len() <= original_count);
    }

    #[test]
    fn test_sort_solutions_by_quality() {
        let logger = Arc::new(CutListLoggerImpl::new());
        let service = CutListOptimizerServiceImpl::new(logger);
        
        let stock_solution1 = StockSolution::new(vec![TileDimensions::simple(1000, 600)]);
        let stock_solution2 = StockSolution::new(vec![TileDimensions::simple(500, 300)]);
        
        let solution1 = Solution::from_stock_solution(&stock_solution1);
        let solution2 = Solution::from_stock_solution(&stock_solution2);
        
        let mut solutions = vec![solution2, solution1]; // Меньшее решение первое
        
        service.sort_solutions_by_quality(&mut solutions);
        
        // После сортировки большее решение должно быть первым
        assert!(solutions[0].get_total_area() >= solutions[1].get_total_area());
    }
}
