use crate::engine::model::{CalculationRequest, CalculationResponse, TaskStatusResponse, Stats};
use crate::engine::logger::CutListLogger;
use crate::engine::cutting::CuttingEngine;
use crate::engine::model::solution::Solution;
use crate::engine::model::response::OptimizedPanel;
use crate::engine::model::tile::{TileNode, TileDimensions};
use crate::error::CuttingError;
use std::sync::Arc;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Трейт для оптимизатора раскроя
pub trait CutListOptimizer {
    fn new(logger: Arc<dyn CutListLogger>) -> Self;
    fn init_with_config(&mut self, threads: usize) -> Result<(), CuttingError>;
    fn optimize(&mut self, request: CalculationRequest) -> Result<CalculationResponse, CuttingError>;
}

/// Реализация оптимизатора раскроя
pub struct CutListOptimizerImpl {
    logger: Arc<dyn CutListLogger>,
    threads: usize,
}

impl CutListOptimizer for CutListOptimizerImpl {
    fn new(logger: Arc<dyn CutListLogger>) -> Self {
        Self {
            logger,
            threads: 1,
        }
    }

    fn init_with_config(&mut self, threads: usize) -> Result<(), CuttingError> {
        self.threads = threads;
        self.logger.info(&format!("Инициализация оптимизатора с {} потоками", threads));
        Ok(())
    }

    fn optimize(&mut self, request: CalculationRequest) -> Result<CalculationResponse, CuttingError> {
        self.logger.info("Начало оптимизации раскроя");
        
        // Простая реализация оптимизации
        let mut solution = Solution::new();
        let mut optimized_panels = Vec::new();
        let mut no_fit_panels = Vec::new();
        let mut no_material_panels = Vec::new();

        // Создаем стоковые панели для размещения
        for stock_panel in &request.stock_panels {
            let width = stock_panel.width.parse::<i32>()
                .map_err(|_| CuttingError::GeneralCuttingError("Invalid width format".to_string()))?;
            let height = stock_panel.height.parse::<i32>()
                .map_err(|_| CuttingError::GeneralCuttingError("Invalid height format".to_string()))?;
            
            let mut root_node = TileNode::new(0, width, 0, height);
            
            // Пытаемся разместить панели на этом стоке
            for panel in &request.panels {
                let panel_width = panel.width.parse::<i32>()
                    .map_err(|_| CuttingError::GeneralCuttingError("Invalid panel width format".to_string()))?;
                let panel_height = panel.height.parse::<i32>()
                    .map_err(|_| CuttingError::GeneralCuttingError("Invalid panel height format".to_string()))?;
                
                let tile_dimensions = TileDimensions::new(
                    panel.id,
                    panel_width,
                    panel_height,
                    panel.material.clone(),
                    0,
                    None,
                );

                match CuttingEngine::try_place_tile(&mut root_node, &tile_dimensions) {
                    Ok(true) => {
                        optimized_panels.push(OptimizedPanel::new(
                            tile_dimensions.clone(),
                            crate::engine::model::response::PanelPosition::new(0, 0, panel_width, panel_height, false),
                            stock_panel.id.to_string(),
                            panel.material.clone(),
                        ));
                    }
                    Ok(false) => {
                        no_fit_panels.push(tile_dimensions);
                    }
                    Err(_) => {
                        no_material_panels.push(tile_dimensions);
                    }
                }
            }
        }

        // Вычисляем статистику
        let total_panels = request.panels.len();
        let placed_panels = optimized_panels.len();
        let mut statistics = crate::engine::model::response::ResponseStatistics::new();
        statistics.update(total_panels, placed_panels, 0.0, 0.0);

        let response = CalculationResponse {
            panels: optimized_panels,
            no_fit_panels,
            no_material_panels,
            statistics,
            metadata: std::collections::HashMap::new(),
        };

        self.logger.info("Оптимизация завершена");
        Ok(response)
    }
}

/// Статус задачи
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Stopped,
    Terminated,
    Error,
}

/// Информация о задаче
#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub id: String,
    pub status: TaskStatus,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

/// Результат отправки задачи
#[derive(Debug, Clone)]
pub struct CalculationSubmissionResult {
    pub task_id: Option<String>,
    pub error_message: Option<String>,
    pub success: bool,
}

impl CalculationSubmissionResult {
    pub fn success(task_id: String) -> Self {
        Self {
            task_id: Some(task_id),
            error_message: None,
            success: true,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            task_id: None,
            error_message: Some(message),
            success: false,
        }
    }

    pub fn is_success(&self) -> bool {
        self.success
    }
}

/// Трейт для сервиса оптимизатора
pub trait CutListOptimizerService {
    fn new(logger: Arc<dyn CutListLogger>) -> Self;
    fn init(&mut self, threads: usize) -> Result<(), CuttingError>;
    fn submit_task(&mut self, request: CalculationRequest) -> Result<CalculationSubmissionResult, CuttingError>;
    fn get_task_status(&self, task_id: &str) -> Result<TaskStatusResponse, CuttingError>;
    fn stop_task(&mut self, task_id: &str) -> Result<(), CuttingError>;
    fn terminate_task(&mut self, task_id: &str) -> Result<(), CuttingError>;
    fn get_tasks(&self, client_id: &str, status: Option<TaskStatus>) -> Result<Vec<TaskInfo>, CuttingError>;
    fn get_stats(&self) -> Result<Stats, CuttingError>;
}

/// Реализация сервиса оптимизатора
pub struct CutListOptimizerServiceImpl {
    logger: Arc<dyn CutListLogger>,
    tasks: Arc<Mutex<HashMap<String, TaskInfo>>>,
    threads: usize,
}

impl CutListOptimizerService for CutListOptimizerServiceImpl {
    fn new(logger: Arc<dyn CutListLogger>) -> Self {
        Self {
            logger,
            tasks: Arc::new(Mutex::new(HashMap::new())),
            threads: 1,
        }
    }

    fn init(&mut self, threads: usize) -> Result<(), CuttingError> {
        self.threads = threads;
        self.logger.info(&format!("Инициализация сервиса с {} потоками", threads));
        Ok(())
    }

    fn submit_task(&mut self, _request: CalculationRequest) -> Result<CalculationSubmissionResult, CuttingError> {
        let task_id = Uuid::new_v4().to_string();
        
        let task_info = TaskInfo {
            id: task_id.clone(),
            status: TaskStatus::Pending,
            start_time: Some(Utc::now()),
            end_time: None,
        };

        if let Ok(mut tasks) = self.tasks.lock() {
            tasks.insert(task_id.clone(), task_info);
        }

        self.logger.info(&format!("Задача {} отправлена", task_id));
        Ok(CalculationSubmissionResult::success(task_id))
    }

    fn get_task_status(&self, task_id: &str) -> Result<TaskStatusResponse, CuttingError> {
        if let Ok(tasks) = self.tasks.lock() {
            if let Some(task) = tasks.get(task_id) {
                return Ok(TaskStatusResponse {
                    status: format!("{:?}", task.status),
                    init_percentage: 100,
                    percentage_done: 100,
                    details: None,
                    solution: None,
                    last_updated: chrono::Utc::now().timestamp_millis() as u64,
                });
            }
        }
        
        Err(CuttingError::GeneralCuttingError(format!("Задача {} не найдена", task_id)))
    }

    fn stop_task(&mut self, task_id: &str) -> Result<(), CuttingError> {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.get_mut(task_id) {
                task.status = TaskStatus::Stopped;
                task.end_time = Some(Utc::now());
                self.logger.info(&format!("Задача {} остановлена", task_id));
                return Ok(());
            }
        }
        
        Err(CuttingError::GeneralCuttingError(format!("Задача {} не найдена", task_id)))
    }

    fn terminate_task(&mut self, task_id: &str) -> Result<(), CuttingError> {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.get_mut(task_id) {
                task.status = TaskStatus::Terminated;
                task.end_time = Some(Utc::now());
                self.logger.info(&format!("Задача {} завершена принудительно", task_id));
                return Ok(());
            }
        }
        
        Err(CuttingError::GeneralCuttingError(format!("Задача {} не найдена", task_id)))
    }

    fn get_tasks(&self, _client_id: &str, _status: Option<TaskStatus>) -> Result<Vec<TaskInfo>, CuttingError> {
        if let Ok(tasks) = self.tasks.lock() {
            let task_list: Vec<TaskInfo> = tasks.values().cloned().collect();
            Ok(task_list)
        } else {
            Ok(Vec::new())
        }
    }

    fn get_stats(&self) -> Result<Stats, CuttingError> {
        let stats = Stats {
            nbr_running_tasks: 0,
            nbr_idle_tasks: 0,
            nbr_finished_tasks: 0,
            nbr_stopped_tasks: 0,
            nbr_terminated_tasks: 0,
            nbr_error_tasks: 0,
            nbr_running_threads: 0,
            nbr_queued_threads: 0,
            nbr_finished_threads: 0,
            task_reports: Vec::new(),
        };
        
        Ok(stats)
    }
}
