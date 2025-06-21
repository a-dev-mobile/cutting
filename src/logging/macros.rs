//! Удобные макросы для логирования

// Реэкспорт базовых макросов tracing
pub use tracing::{debug, error, info, trace, warn};

/// Макрос для логирования начала операции
#[macro_export]
macro_rules! log_operation_start {
    ($operation:expr) => {
        tracing::info!("🚀 Начинаем: {}", $operation);
    };
    ($operation:expr, $($arg:tt)*) => {
        tracing::info!("🚀 Начинаем: {}", format!($operation, $($arg)*));
    };
}

/// Макрос для логирования успешного завершения операции
#[macro_export]
macro_rules! log_operation_success {
    ($operation:expr) => {
        tracing::info!("✅ Завершено: {}", $operation);
    };
    ($operation:expr, $($arg:tt)*) => {
        tracing::info!("✅ Завершено: {}", format!($operation, $($arg)*));
    };
}

/// Макрос для логирования ошибки операции
#[macro_export]
macro_rules! log_operation_error {
    ($operation:expr, $error:expr) => {
        tracing::error!("❌ Ошибка в {}: {}", $operation, $error);
    };
    ($operation:expr, $error:expr, $($arg:tt)*) => {
        tracing::error!("❌ Ошибка в {}: {}", format!($operation, $($arg)*), $error);
    };
}

/// Макрос для логирования прогресса
#[macro_export]
macro_rules! log_progress {
    ($message:expr) => {
        tracing::info!("⏳ {}", $message);
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::info!("⏳ {}", format!($message, $($arg)*));
    };
}

/// Макрос для логирования результатов
#[macro_export]
macro_rules! log_result {
    ($message:expr) => {
        tracing::info!("📊 {}", $message);
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::info!("📊 {}", format!($message, $($arg)*));
    };
}
