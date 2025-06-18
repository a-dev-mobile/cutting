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
    /// Общая ошибка разрезания
    GeneralCuttingError(String),
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
            CuttingError::GeneralCuttingError(msg) => {
                write!(f, "Cutting error: {}", msg)
            }
        }
    }
}

impl std::error::Error for CuttingError {}

/// Результат операций разрезания
pub type CuttingResult<T> = Result<T, CuttingError>;
