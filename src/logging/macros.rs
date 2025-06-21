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

/// Макрос для логирования критических ошибок
#[macro_export]
macro_rules! log_fatal {
    ($message:expr) => {
        tracing::error!("💀 FATAL: {}", $message);
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::error!("💀 FATAL: {}", format!($message, $($arg)*));
    };
}

/// Макрос для логирования информации
#[macro_export]
macro_rules! log_info {
    ($message:expr) => {
        tracing::info!("ℹ️ {}", $message);
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::info!("ℹ️ {}", format!($message, $($arg)*));
    };
}

/// Макрос для логирования ошибок
#[macro_export]
macro_rules! log_error {
    ($message:expr) => {
        tracing::error!("❌ {}", $message);
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::error!("❌ {}", format!($message, $($arg)*));
    };
}

/// Макрос для логирования предупреждений
#[macro_export]
macro_rules! log_warn {
    ($message:expr) => {
        tracing::warn!("⚠️ {}", $message);
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::warn!("⚠️ {}", format!($message, $($arg)*));
    };
}

/// Макрос для логирования отладочной информации
#[macro_export]
macro_rules! log_debug {
    ($message:expr) => {
        tracing::debug!("🐛 {}", $message);
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::debug!("🐛 {}", format!($message, $($arg)*));
    };
}
