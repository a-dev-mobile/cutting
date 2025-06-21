//! Простой модуль логирования для CLI приложения
//!
//! Предоставляет удобные макросы и функции для логирования с поддержкой
//! различных уровней и настройкой через переменные окружения.

pub mod enums;
pub mod init;
pub mod macros;
pub mod structs;

// Реэкспорт основных типов и функций
pub use enums::LogLevel;
pub use init::{init_cli, init_default, init_logging};
pub use macros::{debug, error, info, trace, warn};
pub use structs::LogConfig;

// Реэкспорт макросов
pub use crate::{
    log_operation_error, log_operation_start, log_operation_success, log_progress, log_result,
};
