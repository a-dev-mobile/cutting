use std::fmt;

/// Основной тип ошибок для CutList Optimizer
#[derive(Debug, Clone)]
pub enum CutListError {
    /// Ошибка валидации входных данных
    ValidationError(String),
    /// Ошибка вычислений
    ComputationError(String),
    /// Ошибка ввода/вывода
    IoError(String),
    /// Общая ошибка
    GeneralError(String),
}

impl fmt::Display for CutListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CutListError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            CutListError::ComputationError(msg) => write!(f, "Computation error: {}", msg),
            CutListError::IoError(msg) => write!(f, "IO error: {}", msg),
            CutListError::GeneralError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for CutListError {}

/// Результат операций CutList Optimizer
pub type CutListResult<T> = Result<T, CutListError>;

/// Ошибки алгоритмов разрезания
#[derive(Debug, Clone)]
pub enum CuttingError {
    /// Невалидная позиция разреза
    InvalidCutPosition {
        position: i32,
        min: i32,
        max: i32,
    },
    /// Плитка не помещается
    TileDoesNotFit {
        tile_width: i32,
        tile_height: i32,
        node_width: i32,
        node_height: i32,
    },
    /// Узел уже занят
    NodeAlreadyOccupied,
    /// Ошибка работы с потоками
    ThreadError(String),
    /// Ошибка генерации перестановок
    PermutationError(String),
    /// Общая ошибка разрезания
    GeneralCuttingError(String),
    /// Ошибка ограничения ресурсов
    ResourceLimit(String),
    /// Ошибка валидации входных данных
    InvalidInput(String),
    /// Операция была отменена
    OperationCancelled(String),
    /// Превышено время ожидания
    Timeout(String),
    
    // Ошибки сервиса
    /// Неверные плитки
    InvalidTiles(String),
    /// Неверные стоковые плитки
    InvalidStockTiles(String),
    /// Слишком много панелей
    TooManyPanels(usize),
    /// Слишком много стоковых панелей
    TooManyStockPanels(usize),
    /// Слишком много цифр в размерах
    TooManyDigits(usize),
    /// Задача уже выполняется
    TaskAlreadyRunning(String),
    /// Задача не найдена
    TaskNotFound(String),
    /// Неверное состояние задачи
    InvalidTaskState(String),
    /// Сервис не инициализирован
    ServiceNotInitialized,
    /// Сервис уже инициализирован
    ServiceAlreadyInitialized,
    /// Ошибка отправки задачи
    TaskSubmissionFailed(String),
    /// Ошибка оптимизации
    OptimizationFailed(String),
    /// Задача остановлена
    TaskStopped(String),
}

impl fmt::Display for CuttingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CuttingError::InvalidCutPosition { position, min, max } => {
                write!(f, "Invalid cut position {} (must be between {} and {})", position, min, max)
            }
            CuttingError::TileDoesNotFit { tile_width, tile_height, node_width, node_height } => {
                write!(f, "Tile {}x{} does not fit in node {}x{}", tile_width, tile_height, node_width, node_height)
            }
            CuttingError::NodeAlreadyOccupied => {
                write!(f, "Node is already occupied")
            }
            CuttingError::ThreadError(msg) => {
                write!(f, "Thread error: {}", msg)
            }
            CuttingError::PermutationError(msg) => {
                write!(f, "Permutation error: {}", msg)
            }
            CuttingError::GeneralCuttingError(msg) => {
                write!(f, "Cutting error: {}", msg)
            }
            CuttingError::ResourceLimit(msg) => {
                write!(f, "Resource limit error: {}", msg)
            }
            CuttingError::InvalidInput(msg) => {
                write!(f, "Invalid input error: {}", msg)
            }
            CuttingError::OperationCancelled(msg) => {
                write!(f, "Operation cancelled: {}", msg)
            }
            CuttingError::Timeout(msg) => {
                write!(f, "Timeout error: {}", msg)
            }
            CuttingError::InvalidTiles(msg) => {
                write!(f, "Invalid tiles: {}", msg)
            }
            CuttingError::InvalidStockTiles(msg) => {
                write!(f, "Invalid stock tiles: {}", msg)
            }
            CuttingError::TooManyPanels(count) => {
                write!(f, "Too many panels: {}", count)
            }
            CuttingError::TooManyStockPanels(count) => {
                write!(f, "Too many stock panels: {}", count)
            }
            CuttingError::TooManyDigits(count) => {
                write!(f, "Too many digits: {}", count)
            }
            CuttingError::TaskAlreadyRunning(client_id) => {
                write!(f, "Task already running for client: {}", client_id)
            }
            CuttingError::TaskNotFound(task_id) => {
                write!(f, "Task not found: {}", task_id)
            }
            CuttingError::InvalidTaskState(msg) => {
                write!(f, "Invalid task state: {}", msg)
            }
            CuttingError::ServiceNotInitialized => {
                write!(f, "Service not initialized")
            }
            CuttingError::ServiceAlreadyInitialized => {
                write!(f, "Service already initialized")
            }
            CuttingError::TaskSubmissionFailed(msg) => {
                write!(f, "Task submission failed: {}", msg)
            }
            CuttingError::OptimizationFailed(msg) => {
                write!(f, "Optimization failed: {}", msg)
            }
            CuttingError::TaskStopped(task_id) => {
                write!(f, "Task stopped: {}", task_id)
            }
        }
    }
}

impl std::error::Error for CuttingError {}

/// Результат операций разрезания
pub type CuttingResult<T> = Result<T, CuttingError>;
