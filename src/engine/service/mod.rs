use crate::engine::logger::CutListLogger;
use crate::engine::model::request::CalculationRequest;
use crate::engine::model::response::{
    CalculationResponse, CalculationSubmissionResult, Stats, StatusCode, TaskStatusResponse,
};
use crate::engine::model::solution::Solution;
use crate::engine::model::tile::TileDimensions;
use crate::engine::progress::ProgressTracker;
use crate::engine::stock::StockSolution;
use crate::engine::tasks::{RunningTasks, Task, TaskPriority};
use crate::engine::watchdog::{ConsoleEventHandler, WatchDog, WatchDogConfig};
use crate::error::CuttingError;
use crate::{OptimizedPanel, PanelPosition, ResponseStatistics};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

// Подмодули
mod grouping;
mod optimization;
mod permutations;
mod validation;

pub use grouping::*;
pub use optimization::*;
pub use permutations::*;
pub use validation::*;

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
    fn init(&mut self, threads: usize) -> Result<(), CuttingError>;
    fn optimize(
        &mut self,
        request: CalculationRequest,
    ) -> Result<CalculationResponse, CuttingError>;
    fn submit_task(
        &mut self,
        request: CalculationRequest,
    ) -> Result<CalculationSubmissionResult, CuttingError>;
    fn get_task_status(&self, task_id: &str) -> Result<Option<TaskStatusResponse>, CuttingError>;
    fn stop_task(&mut self, task_id: &str) -> Result<Option<TaskStatusResponse>, CuttingError>;
    fn terminate_task(&mut self, task_id: &str) -> Result<i32, CuttingError>;
    fn get_tasks(
        &self,
        client_id: &str,
        status: Option<ServiceTaskStatus>,
    ) -> Result<Vec<ServiceTaskInfo>, CuttingError>;
    fn get_stats(&self) -> Result<Stats, CuttingError>;
    fn set_allow_multiple_tasks_per_client(&mut self, allow: bool);
    fn get_allow_multiple_tasks_per_client(&self) -> bool;
    fn set_cut_list_logger(&mut self, logger: Arc<dyn CutListLogger>);
}

/// Реализация сервиса оптимизатора раскроя
pub struct CutListOptimizerServiceImpl {
    /// Логгер
    pub cut_list_logger: Arc<dyn CutListLogger>,
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
    /// Генератор перестановок
    permutation_generator: PermutationGenerator,
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
            permutation_generator: PermutationGenerator::new(),
        }
    }
    /// Главный метод оптимизации с интегрированным преобразованием результатов
    pub fn optimize(
        &self,
        request: CalculationRequest,
    ) -> Result<CalculationResponse, CuttingError> {
        let start_time = std::time::Instant::now();

        println!("🚀 Запуск полного цикла оптимизации");
        self.cut_list_logger
            .info("Начинаем полный цикл оптимизации раскроя");

        // Этап 1: Валидация запроса
        let validation_summary = validate_calculation_request(&request)
            .map_err(|e| CuttingError::GeneralCuttingError(format!("Ошибка валидации: {}", e)))?;

        println!("✅ Валидация завершена: {}", validation_summary);

        // Этап 2: Выполнение оптимизации с сохранением информации о масштабировании
        let optimization_result =
            self.perform_optimization_with_tracking(&request, &validation_summary)?;

        // Этап 3: Преобразование результатов в итоговый ответ
        let mut response = self.convert_optimization_to_response(
            &optimization_result,
            &request,
            validation_summary.scale_factor,
        )?;

        // Этап 4: Финализация метаданных
        let calculation_time = start_time.elapsed().as_millis() as u64;
        response.statistics.calculation_time_ms = calculation_time;

        response.add_metadata(
            "calculation_time_ms".to_string(),
            calculation_time.to_string(),
        );
        response.add_metadata(
            "scale_factor".to_string(),
            format!("{:.2}", validation_summary.scale_factor),
        );
        response.add_metadata(
            "validation_summary".to_string(),
            validation_summary.to_string(),
        );

        println!(
            "🎉 Оптимизация завершена за {:.2} сек: {}/{} панелей размещено, эффективность {:.1}%",
            calculation_time as f64 / 1000.0,
            response.statistics.placed_panels,
            response.statistics.total_panels,
            response.statistics.efficiency_percentage
        );

        self.cut_list_logger.info(&format!(
            "Полный цикл оптимизации завершен: время {:.2}с, эффективность {:.1}%",
            calculation_time as f64 / 1000.0,
            response.statistics.efficiency_percentage
        ));

        Ok(response)
    }

    /// Выполнение оптимизации с отслеживанием исходной информации
    fn perform_optimization_with_tracking(
        &self,
        request: &CalculationRequest,
        validation_summary: &ValidationSummary,
    ) -> Result<OptimizationResultWithTracking, CuttingError> {
        println!("🔧 perform_optimization_with_tracking: Начинаем оптимизацию с отслеживанием");

        // Создаем расширенную структуру для отслеживания
        let mut tracking_info = OptimizationTrackingInfo::new(validation_summary.scale_factor);

        // Обрабатываем панели для размещения
        let mut tile_dimensions_list = Vec::new();
        for panel in &request.panels {
            if panel.is_valid() {
                if let (Ok(width_f64), Ok(height_f64)) =
                    (panel.width.parse::<f64>(), panel.height.parse::<f64>())
                {
                    let scaled_width = (width_f64 * validation_summary.scale_factor).round() as i32;
                    let scaled_height =
                        (height_f64 * validation_summary.scale_factor).round() as i32;

                    // Сохраняем исходную информацию
                    tracking_info.add_panel_info(
                        panel.id,
                        width_f64,
                        height_f64,
                        scaled_width,
                        scaled_height,
                        panel.count,
                        panel.material.clone(),
                    );

                    for i in 0..panel.count {
                        let tile_dimensions = TileDimensions::new(
                            panel.id,
                            scaled_width,
                            scaled_height,
                            panel.material.clone(),
                            panel.orientation,
                            panel.label.clone(),
                        );
                        tile_dimensions_list.push(tile_dimensions);
                    }
                }
            }
        }

        // Обрабатываем складские панели
        let mut stock_tile_dimensions = Vec::new();
        for stock_panel in &request.stock_panels {
            if stock_panel.is_valid() {
                if let (Ok(width_f64), Ok(height_f64)) = (
                    stock_panel.width.parse::<f64>(),
                    stock_panel.height.parse::<f64>(),
                ) {
                    let scaled_width = (width_f64 * validation_summary.scale_factor).round() as i32;
                    let scaled_height =
                        (height_f64 * validation_summary.scale_factor).round() as i32;

                    // Сохраняем исходную информацию о складских панелях
                    tracking_info.add_stock_info(
                        stock_panel.id,
                        width_f64,
                        height_f64,
                        scaled_width,
                        scaled_height,
                        stock_panel.count,
                        stock_panel.material.clone(),
                    );

                    for _i in 0..stock_panel.count {
                        let tile_dimensions = TileDimensions::new(
                            stock_panel.id,
                            scaled_width,
                            scaled_height,
                            stock_panel.material.clone(),
                            stock_panel.orientation,
                            stock_panel.label.clone(),
                        );
                        stock_tile_dimensions.push(tile_dimensions);
                    }
                }
            }
        }

        if tile_dimensions_list.is_empty() || stock_tile_dimensions.is_empty() {
            return Ok(OptimizationResultWithTracking::empty(tracking_info));
        }

        // Сортируем панели по убыванию площади
        tile_dimensions_list.sort_by(|a, b| b.get_area().cmp(&a.get_area()));

        // Выполняем оптимизацию
        let optimization_result =
            self.compute_optimal_solution_improved(&tile_dimensions_list, &stock_tile_dimensions)?;

        println!(
            "✅ Оптимизация завершена: размещено {} панелей",
            optimization_result.placed_panels_count
        );

        Ok(OptimizationResultWithTracking {
            optimization_result,
            tracking_info,
        })
    }

    /// Преобразование результатов оптимизации в финальный ответ
    fn convert_optimization_to_response(
        &self,
        optimization_with_tracking: &OptimizationResultWithTracking,
        original_request: &CalculationRequest,
        scale_factor: f64,
    ) -> Result<CalculationResponse, CuttingError> {
        println!("🔄 Преобразуем результаты оптимизации в финальный ответ");

        let optimization_result = &optimization_with_tracking.optimization_result;
        let tracking_info = &optimization_with_tracking.tracking_info;

        let mut response = CalculationResponse::new();
        let mut optimized_panels = Vec::new();

        // Обрабатываем решения
        for (solution_idx, solution) in optimization_result.solutions.iter().enumerate() {
            println!(
                "🔄 Обрабатываем решение {}/{}",
                solution_idx + 1,
                optimization_result.solutions.len()
            );

            for mosaic in solution.get_mosaics() {
                let final_nodes = mosaic.get_final_tile_nodes();
                let stock_panel_id = format!("stock_{}", mosaic.get_stock_id());

                for node in final_nodes {
                    if let Some(panel_info) = tracking_info.get_panel_info(node.external_id) {
                        // Восстанавливаем исходные размеры с учетом поворота
                        let (final_width, final_height, is_rotated) =
                            self.restore_original_dimensions(node, panel_info, scale_factor)?;

                        // Создаем TileDimensions с правильными размерами
                        let tile_dimensions = TileDimensions::new_with_rotation(
                            node.external_id,
                            final_width,
                            final_height,
                            panel_info.material.clone(),
                            0,    // orientation
                            None, // label
                            is_rotated,
                        );

                        // Позиция на складской панели
                        let position = PanelPosition::new(
                            (node.get_x1() as f64 / scale_factor).round() as i32,
                            (node.get_y1() as f64 / scale_factor).round() as i32,
                            final_width,
                            final_height,
                            is_rotated,
                        );

                        // Store position values before moving
                        let pos_x = position.x;
                        let pos_y = position.y;

                        let optimized_panel = OptimizedPanel::new(
                            tile_dimensions,
                            position,
                            stock_panel_id.clone(),
                            panel_info.material.clone(),
                        );

                        optimized_panels.push(optimized_panel);

                        println!(
                            "✅ Панель ID {}: {}x{} → позиция ({},{}) повернуто: {}",
                            node.external_id,
                            final_width,
                            final_height,
                            pos_x,
                            pos_y,
                            is_rotated
                        );
                    }
                }
            }
        }

        // Обрабатываем неразмещенные панели
        let mut no_fit_panels = Vec::new();
        for solution in &optimization_result.solutions {
            for no_fit_panel in solution.get_no_fit_panels() {
                if let Some(panel_info) = tracking_info.get_panel_info(no_fit_panel.id) {
                    let restored_tile = TileDimensions::new(
                        no_fit_panel.id,
                        panel_info.original_width as i32,
                        panel_info.original_height as i32,
                        panel_info.material.clone(),
                        0,
                        None,
                    );
                    no_fit_panels.push(restored_tile);
                }
            }
        }

        // Обновляем статистику
        let mut statistics = ResponseStatistics::new();
        statistics.update(
            optimization_result.placed_panels_count + no_fit_panels.len(),
            optimization_result.placed_panels_count,
            optimization_result.total_area / scale_factor,
            optimization_result.used_area / scale_factor,
        );
        statistics.stock_panels_used = optimization_result
            .solutions
            .first()
            .map(|s| s.get_mosaics().len())
            .unwrap_or(0);

        // Собираем ответ
        response.panels = optimized_panels;
        response.no_fit_panels = no_fit_panels;
        response.statistics = statistics;

        // Метаданные
        response.add_metadata(
            "placed_panels".to_string(),
            response.statistics.placed_panels.to_string(),
        );
        response.add_metadata(
            "panel_count".to_string(),
            response.statistics.total_panels.to_string(),
        );
        response.add_metadata(
            "stock_count".to_string(),
            response.statistics.stock_panels_used.to_string(),
        );
        response.add_metadata(
            "cuts_count".to_string(),
            optimization_result.cuts_count.to_string(),
        );
        response.add_metadata("optimization_type".to_string(), "synchronous".to_string());
        response.add_metadata(
            "efficiency".to_string(),
            format!("{:.2}%", response.statistics.efficiency_percentage),
        );

        println!(
            "📊 Финальная статистика: {}/{} панелей, эффективность {:.1}%",
            response.statistics.placed_panels, response.statistics.total_panels, response.statistics.efficiency_percentage
        );

        Ok(response)
    }

    /// Восстанавливает исходные размеры панели с учетом поворота
    fn restore_original_dimensions(
        &self,
        node: &crate::engine::model::tile::TileNode,
        panel_info: &PanelTrackingInfo,
        scale_factor: f64,
    ) -> Result<(i32, i32, bool), CuttingError> {
        let node_width = node.get_width();
        let node_height = node.get_height();

        // Проверяем, соответствуют ли размеры узла исходным размерам панели
        let matches_original =
            (node_width == panel_info.scaled_width && node_height == panel_info.scaled_height);
        let matches_rotated =
            (node_width == panel_info.scaled_height && node_height == panel_info.scaled_width);

        let (final_width, final_height, is_rotated) =
            if matches_original {
                // Панель не повернута
                (
                    panel_info.original_width as i32,
                    panel_info.original_height as i32,
                    false,
                )
            } else if matches_rotated {
                // Панель повернута
                (
                    panel_info.original_height as i32,
                    panel_info.original_width as i32,
                    true,
                )
            } else {
                // Неожиданное соответствие - используем расчетные размеры
                println!(
                "⚠️ Неожиданные размеры для панели ID {}: узел {}x{}, ожидалось {}x{} или {}x{}",
                node.external_id, node_width, node_height,
                panel_info.scaled_width, panel_info.scaled_height,
                panel_info.scaled_height, panel_info.scaled_width
            );

                let calculated_width = (node_width as f64 / scale_factor).round() as i32;
                let calculated_height = (node_height as f64 / scale_factor).round() as i32;
                (calculated_width, calculated_height, node.is_rotated)
            };

        Ok((final_width, final_height, is_rotated))
    }
    /// Генерирует уникальный идентификатор задачи
    pub fn generate_task_id(&self) -> String {
        let now = Utc::now();
        let date_part = now.format("%Y%m%d%H%M").to_string();
        let counter = self.task_id_counter.fetch_add(1, Ordering::SeqCst);
        format!("{}{}", date_part, counter)
    }

    /// Проверяет, может ли клиент запустить новую задачу
    pub fn can_client_start_task(&self, client_id: &str, max_tasks: usize) -> bool {
        if let Ok(client_tasks) = self.client_tasks.lock() {
            if let Some(tasks) = client_tasks.get(client_id) {
                return tasks.len() < max_tasks;
            }
        }
        true
    }

    /// Добавляет задачу к клиенту
    pub fn add_task_to_client(&self, client_id: &str, task_id: &str) {
        if let Ok(mut client_tasks) = self.client_tasks.lock() {
            client_tasks
                .entry(client_id.to_string())
                .or_insert_with(Vec::new)
                .push(task_id.to_string());
        }
    }

    /// Преобразует решения оптимизации в ответ с корректными размерами
    pub fn convert_solutions_to_response(
        &self,
        solutions: &[Solution],
        original_request: &CalculationRequest,
        scale_factor: f64,
        optimization_result: &OptimizationResult,
    ) -> Result<CalculationResponse, CuttingError> {
        let mut response = CalculationResponse::new();
        let mut optimized_panels = Vec::new();

        // Создаем карту исходных панелей для быстрого поиска
        let mut original_panels_map = std::collections::HashMap::new();
        for panel in &original_request.panels {
            original_panels_map.insert(panel.id, panel);
        }

        let mut original_stock_map = std::collections::HashMap::new();
        for stock_panel in &original_request.stock_panels {
            original_stock_map.insert(stock_panel.id, stock_panel);
        }

        // Обрабатываем каждое решение
        for (solution_idx, solution) in solutions.iter().enumerate() {
            println!(
                "🔄 Обрабатываем решение {}/{}",
                solution_idx + 1,
                solutions.len()
            );

            for (mosaic_idx, mosaic) in solution.get_mosaics().iter().enumerate() {
                let final_nodes = mosaic.get_final_tile_nodes();

                // Определяем ID стоковой панели
                let stock_panel_id = format!("stock_{}", mosaic.get_stock_id());

                for node in final_nodes {
                    if let Some(original_panel) = original_panels_map.get(&node.external_id) {
                        // ИСПРАВЛЕНИЕ: Восстанавливаем исходные размеры
                        let original_width_f64 =
                            original_panel.width.parse::<f64>().map_err(|_| {
                                CuttingError::GeneralCuttingError("Invalid panel width".to_string())
                            })?;
                        let original_height_f64 =
                            original_panel.height.parse::<f64>().map_err(|_| {
                                CuttingError::GeneralCuttingError(
                                    "Invalid panel height".to_string(),
                                )
                            })?;

                        // Преобразуем обратно из масштабированных размеров
                        let scaled_back_width = (node.get_width() as f64 / scale_factor).round();
                        let scaled_back_height = (node.get_height() as f64 / scale_factor).round();

                        // Определяем, была ли панель повернута
                        let (final_width, final_height, rotated) = if node.is_rotated {
                            // Панель была повернута - размеры поменялись местами
                            if (scaled_back_width - original_height_f64).abs() < 0.1
                                && (scaled_back_height - original_width_f64).abs() < 0.1
                            {
                                (original_height_f64 as i32, original_width_f64 as i32, true)
                            } else {
                                println!("⚠️ Неожиданные размеры повернутой панели ID {}: ожидалось {}x{}, получено {}x{}", 
                                    node.external_id, original_height_f64, original_width_f64, scaled_back_width, scaled_back_height);
                                (scaled_back_width as i32, scaled_back_height as i32, true)
                            }
                        } else {
                            // Панель не была повернута
                            if (scaled_back_width - original_width_f64).abs() < 0.1
                                && (scaled_back_height - original_height_f64).abs() < 0.1
                            {
                                (original_width_f64 as i32, original_height_f64 as i32, false)
                            } else {
                                println!("⚠️ Неожиданные размеры панели ID {}: ожидалось {}x{}, получено {}x{}", 
                                    node.external_id, original_width_f64, original_height_f64, scaled_back_width, scaled_back_height);
                                (scaled_back_width as i32, scaled_back_height as i32, false)
                            }
                        };

                        // Создаем корректный TileDimensions с исходными размерами
                        let tile_dimensions = TileDimensions::new_with_rotation(
                            node.external_id,
                            final_width,
                            final_height,
                            original_panel.material.clone(),
                            original_panel.orientation,
                            original_panel.label.clone(),
                            rotated,
                        );

                        // Позиция на стоковой панели (тоже нужно масштабировать обратно)
                        let pos_x = (node.get_x1() as f64 / scale_factor).round() as i32;
                        let pos_y = (node.get_y1() as f64 / scale_factor).round() as i32;
                        let position = PanelPosition::new(
                            pos_x,
                            pos_y,
                            final_width,
                            final_height,
                            rotated,
                        );

                        let optimized_panel = OptimizedPanel::new(
                            tile_dimensions,
                            position,
                            stock_panel_id.clone(),
                            original_panel.material.clone(),
                        );

                        optimized_panels.push(optimized_panel);

                        println!(
                            "✅ Добавлена панель ID {}: {}x{} в позиции ({},{}) повернуто: {}",
                            node.external_id,
                            final_width,
                            final_height,
                            pos_x,
                            pos_y,
                            rotated
                        );
                    } else {
                        println!("⚠️ Не найдена исходная панель для ID {}", node.external_id);
                    }
                }
            }
        }

        // Находим панели, которые не поместились
        let mut no_fit_panels = Vec::new();
        for solution in solutions {
            for no_fit_panel in solution.get_no_fit_panels() {
                if let Some(original_panel) = original_panels_map.get(&no_fit_panel.id) {
                    let original_width = original_panel.width.parse::<f64>().unwrap_or(0.0) as i32;
                    let original_height =
                        original_panel.height.parse::<f64>().unwrap_or(0.0) as i32;

                    let restored_tile = TileDimensions::new(
                        no_fit_panel.id,
                        original_width,
                        original_height,
                        original_panel.material.clone(),
                        original_panel.orientation,
                        original_panel.label.clone(),
                    );
                    no_fit_panels.push(restored_tile);
                }
            }
        }

        // Обновляем статистику
        let mut statistics = ResponseStatistics::new();
        statistics.update(
            optimization_result.placed_panels_count + no_fit_panels.len(),
            optimization_result.placed_panels_count,
            optimization_result.total_area / scale_factor,
            optimization_result.used_area / scale_factor,
        );
        statistics.calculation_time_ms = 0; // Будет установлено позже
        statistics.stock_panels_used = solutions
            .first()
            .map(|s| s.get_mosaics().len())
            .unwrap_or(0);

        response.panels = optimized_panels;
        response.no_fit_panels = no_fit_panels;
        response.statistics = statistics;

        // Добавляем метаданные
        response.add_metadata(
            "placed_panels".to_string(),
            optimization_result.placed_panels_count.to_string(),
        );
        response.add_metadata(
            "panel_count".to_string(),
            response.statistics.total_panels.to_string(),
        );
        response.add_metadata(
            "stock_count".to_string(),
            response.statistics.stock_panels_used.to_string(),
        );
        response.add_metadata(
            "cuts_count".to_string(),
            optimization_result.cuts_count.to_string(),
        );
        response.add_metadata("optimization_type".to_string(), "synchronous".to_string());
        response.add_metadata(
            "efficiency".to_string(),
            format!("{:.2}%", response.statistics.efficiency_percentage),
        );

        println!(
            "📊 Итоговая статистика: размещено {}/{} панелей, эффективность {:.2}%",
            response.statistics.placed_panels, response.statistics.total_panels, response.statistics.efficiency_percentage
        );

        Ok(response)
    }
}

impl CutListOptimizerService for CutListOptimizerServiceImpl {
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
            return Err(CuttingError::GeneralCuttingError(format!(
                "Ошибка запуска сторожевого таймера: {}",
                e
            )));
        }

        self.watch_dog = Some(watch_dog);

        self.cut_list_logger
            .info(&format!("Сервис инициализирован с {} потоками", threads));
        Ok(())
    }

    fn optimize(
        &mut self,
        request: CalculationRequest,
    ) -> Result<CalculationResponse, CuttingError> {
        println!("🔧 Начало синхронной оптимизации через CalculationRequest");
        self.cut_list_logger.info("Начало синхронной оптимизации");

        // Валидируем панели
        let (_panel_count, panel_status) = validation::validate_panels(&request.panels);
        println!(
            "📋 Валидация панелей: count={}, status={:?}",
            _panel_count, panel_status
        );
        if panel_status != StatusCode::Ok {
            return Err(CuttingError::GeneralCuttingError(format!(
                "Ошибка валидации панелей: {}",
                panel_status.description()
            )));
        }

        // Валидируем складские панели
        let (_stock_count, stock_status) = validation::validate_stock_panels(&request.stock_panels);
        println!(
            "📦 Валидация складских панелей: count={}, status={:?}",
            _stock_count, stock_status
        );
        if stock_status != StatusCode::Ok {
            return Err(CuttingError::GeneralCuttingError(format!(
                "Ошибка валидации складских панелей: {}",
                stock_status.description()
            )));
        }

        self.cut_list_logger.info(&format!(
            "Валидация прошла успешно: {} панелей, {} складских панелей",
            _panel_count, _stock_count
        ));

        println!("🚀 Запуск perform_optimization...");
        // Выполняем оптимизацию
        let optimization_result = self.perform_optimization(&request)?;
        println!(
            "✅ perform_optimization завершен: размещено {} панелей",
            optimization_result.placed_panels_count
        );

        // Создаем ответ с результатами оптимизации
        let mut response = CalculationResponse::new();

        // Подсчитываем общее количество панелей из запроса (с учетом count)
        let total_panels_count: usize = request
            .panels
            .iter()
            .map(|panel| panel.count as usize)
            .sum();

        // Обновляем статистику с реальными данными
        response.statistics.update(
            total_panels_count,
            optimization_result.placed_panels_count,
            optimization_result.total_area,
            optimization_result.used_area,
        );

        // Конвертируем решения в панели ответа
        if !optimization_result.solutions.is_empty() {
            let best_solution = &optimization_result.solutions[0];

            // Создаем карту для быстрого поиска оригинальных размеров панелей по ID
            let mut original_panels_map = std::collections::HashMap::new();
            for panel in &request.panels {
                original_panels_map.insert(panel.id, panel);
            }

            // Получаем все размещенные панели из мозаик с правильными stock_panel_id
            for mosaic in best_solution.get_mosaics() {
                let stock_panel_id = format!("stock_{}", mosaic.get_stock_id());
                let final_tile_nodes = mosaic.get_final_tile_nodes();

                for tile_node in final_tile_nodes {
                    // Получаем оригинальные размеры панели из исходных данных
                    let original_panel_id = (tile_node.external_id - 1000) / 1000;
                    let (original_width, original_height) =
                        if let Some(original_panel) = original_panels_map.get(&original_panel_id) {
                            let width = original_panel.width.parse::<f64>().unwrap_or(0.0) as i32;
                            let height = original_panel.height.parse::<f64>().unwrap_or(0.0) as i32;
                            (width, height)
                        } else {
                            if tile_node.is_rotated {
                                (tile_node.get_height(), tile_node.get_width())
                            } else {
                                (tile_node.get_width(), tile_node.get_height())
                            }
                        };

                    let (tile_width, tile_height) = if tile_node.is_rotated {
                        (original_height, original_width)
                    } else {
                        (original_width, original_height)
                    };

                    let tile_dimensions = TileDimensions::new(
                        tile_node.external_id,
                        tile_width,
                        tile_height,
                        mosaic.get_material().to_string(),
                        0,
                        None,
                    );

                    let (actual_width, actual_height) = (tile_width, tile_height);

                    let position = crate::engine::model::response::PanelPosition::new(
                        tile_node.get_x1(),
                        tile_node.get_y1(),
                        actual_width,
                        actual_height,
                        tile_node.is_rotated,
                    );

                    let optimized_panel = crate::engine::model::response::OptimizedPanel::new(
                        tile_dimensions,
                        position,
                        stock_panel_id.clone(),
                        mosaic.get_material().to_string(),
                    );

                    response.panels.push(optimized_panel);
                }
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
        response.add_metadata(
            "placed_panels".to_string(),
            optimization_result.placed_panels_count.to_string(),
        );
        response.add_metadata(
            "efficiency".to_string(),
            format!("{:.2}%", optimization_result.efficiency),
        );
        response.add_metadata(
            "cuts_count".to_string(),
            optimization_result.cuts_count.to_string(),
        );

        self.cut_list_logger.info(&format!(
            "Синхронная оптимизация завершена: размещено {}/{} панелей, эффективность {:.2}%",
            optimization_result.placed_panels_count,
            request.panels.len(),
            optimization_result.efficiency
        ));

        Ok(response)
    }

    fn submit_task(
        &mut self,
        request: CalculationRequest,
    ) -> Result<CalculationSubmissionResult, CuttingError> {
        let client_id = &request.client_info.id;

        // Валидируем конфигурацию
        if !request.configuration.is_valid() {
            return Ok(CalculationSubmissionResult::error(
                StatusCode::InvalidTiles,
                Some("Неверная конфигурация".to_string()),
            ));
        }

        // Проверяем производительные пороги
        let performance_thresholds = request
            .configuration
            .performance_thresholds
            .as_ref()
            .map(|pt| pt.max_simultaneous_tasks)
            .unwrap_or(2);

        // Проверяем, может ли клиент запустить новую задачу
        if !self.can_client_start_task(client_id, performance_thresholds) {
            self.cut_list_logger.warning(&format!(
                "Отклонение задачи клиента {} из-за превышения лимита одновременных задач",
                client_id
            ));
            return Ok(CalculationSubmissionResult::error(
                StatusCode::TaskAlreadyRunning,
                None,
            ));
        }

        // Валидируем панели
        let (_panel_count, panel_status) = validation::validate_panels(&request.panels);
        if panel_status != StatusCode::Ok {
            return Ok(CalculationSubmissionResult::error(panel_status, None));
        }

        // Валидируем складские панели
        let (_stock_count, stock_status) = validation::validate_stock_panels(&request.stock_panels);
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
                let temp_service = CutListOptimizerServiceImpl::new(Arc::clone(&logger_clone));
                let optimization_result = temp_service.perform_optimization(&request_clone);

                match optimization_result {
                    Ok(result) => {
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

                        logger_clone.error(&format!(
                            "Ошибка выполнения задачи {}: {}",
                            task_id_clone, e
                        ));
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

        self.cut_list_logger
            .info(&format!("Задача {} отправлена на выполнение", task_id));

        Ok(CalculationSubmissionResult::success(task_id))
    }

    fn get_task_status(&self, task_id: &str) -> Result<Option<TaskStatusResponse>, CuttingError> {
        if let Ok(task_info) = self.task_info.lock() {
            if let Some(info) = task_info.get(task_id) {
                let mut response = TaskStatusResponse::new(format!("{:?}", info.status));
                response.update_progress(info.progress_percentage, info.progress_percentage);
                response.details = Some(format!("Задача {}: {:?}", task_id, info.status));

                if let Some(ref solution) = info.solution {
                    let mut calc_response = CalculationResponse::new();
                    calc_response.statistics.update(
                        solution.get_nbr_final_tiles() as usize,
                        solution.get_nbr_final_tiles() as usize,
                        solution.get_total_area() as f64,
                        solution.get_used_area() as f64,
                    );
                    response.set_solution(calc_response);
                }

                return Ok(Some(response));
            }
        }

        if self.running_tasks.get_active_task_count() > 0 {
            let mut response = TaskStatusResponse::new("RUNNING".to_string());
            response.update_progress(50, 25);
            response.details = Some("Выполняется оптимизация".to_string());
            return Ok(Some(response));
        }

        Ok(None)
    }

    fn stop_task(&mut self, task_id: &str) -> Result<Option<TaskStatusResponse>, CuttingError> {
        self.cut_list_logger
            .info(&format!("Остановка задачи {}", task_id));

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
        self.cut_list_logger
            .info(&format!("Принудительное завершение задачи {}", task_id));

        if let Ok(mut task_info) = self.task_info.lock() {
            if let Some(info) = task_info.get_mut(task_id) {
                info.status = ServiceTaskStatus::Terminated;
                info.end_time = Some(Utc::now());
                return Ok(0);
            }
        }

        Ok(-1)
    }

    fn get_tasks(
        &self,
        client_id: &str,
        status: Option<ServiceTaskStatus>,
    ) -> Result<Vec<ServiceTaskInfo>, CuttingError> {
        let mut result = Vec::new();

        if let Ok(client_tasks) = self.client_tasks.lock() {
            if let Some(task_ids) = client_tasks.get(client_id) {
                if let Ok(task_info) = self.task_info.lock() {
                    for task_id in task_ids {
                        if let Some(info) = task_info.get(task_id) {
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
            if allow {
                "разрешены"
            } else {
                "запрещены"
            }
        ));
    }

    fn get_allow_multiple_tasks_per_client(&self) -> bool {
        self.allow_multiple_tasks_per_client
    }

    fn set_cut_list_logger(&mut self, logger: Arc<dyn CutListLogger>) {
        self.cut_list_logger = logger;
    }
}

/// Синглтон экземпляр сервиса (как в Java реализации)
static INSTANCE: std::sync::OnceLock<std::sync::Mutex<CutListOptimizerServiceImpl>> =
    std::sync::OnceLock::new();

impl CutListOptimizerServiceImpl {
    /// Получает синглтон экземпляр сервиса
    pub fn get_instance(
        logger: Arc<dyn CutListLogger>,
    ) -> &'static std::sync::Mutex<CutListOptimizerServiceImpl> {
        INSTANCE.get_or_init(|| std::sync::Mutex::new(CutListOptimizerServiceImpl::new(logger)))
    }
}
/// Информация о панели для отслеживания
#[derive(Debug, Clone)]
struct PanelTrackingInfo {
    original_width: f64,
    original_height: f64,
    scaled_width: i32,
    scaled_height: i32,
    count: i32,
    material: String,
}
/// Расширенная структура результата оптимизации с отслеживанием
#[derive(Debug, Clone)]
struct OptimizationResultWithTracking {
    optimization_result: OptimizationResult,
    tracking_info: OptimizationTrackingInfo,
}

impl OptimizationResultWithTracking {
    fn empty(tracking_info: OptimizationTrackingInfo) -> Self {
        Self {
            optimization_result: OptimizationResult::new(),
            tracking_info,
        }
    }
}
/// Информация для отслеживания панелей в процессе оптимизации
#[derive(Debug, Clone)]
struct OptimizationTrackingInfo {
    scale_factor: f64,
    panel_info: std::collections::HashMap<i32, PanelTrackingInfo>,
    stock_info: std::collections::HashMap<i32, PanelTrackingInfo>,
}

impl OptimizationTrackingInfo {
    fn new(scale_factor: f64) -> Self {
        Self {
            scale_factor,
            panel_info: std::collections::HashMap::new(),
            stock_info: std::collections::HashMap::new(),
        }
    }

    fn add_panel_info(
        &mut self,
        id: i32,
        original_width: f64,
        original_height: f64,
        scaled_width: i32,
        scaled_height: i32,
        count: i32,
        material: String,
    ) {
        self.panel_info.insert(
            id,
            PanelTrackingInfo {
                original_width,
                original_height,
                scaled_width,
                scaled_height,
                count,
                material,
            },
        );
    }

    fn add_stock_info(
        &mut self,
        id: i32,
        original_width: f64,
        original_height: f64,
        scaled_width: i32,
        scaled_height: i32,
        count: i32,
        material: String,
    ) {
        self.stock_info.insert(
            id,
            PanelTrackingInfo {
                original_width,
                original_height,
                scaled_width,
                scaled_height,
                count,
                material,
            },
        );
    }

    fn get_panel_info(&self, id: i32) -> Option<&PanelTrackingInfo> {
        self.panel_info.get(&id)
    }

    fn get_stock_info(&self, id: i32) -> Option<&PanelTrackingInfo> {
        self.stock_info.get(&id)
    }
}
