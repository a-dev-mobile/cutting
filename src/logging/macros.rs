//! Удобные макросы для логирования

// Реэкспорт базовых макросов tracing
pub use tracing::{debug, error, info, trace, warn};

/// Macro for logging operation start
#[macro_export]
macro_rules! log_operation_start {
    ($operation:expr) => {
        tracing::info!("🚀 Starting: {}", $operation);
    };
    ($operation:expr, $($arg:tt)*) => {
        tracing::info!("🚀 Starting: {}", format!($operation, $($arg)*));
    };
}

/// Macro for logging successful operation completion
#[macro_export]
macro_rules! log_operation_success {
    ($operation:expr) => {
        tracing::info!("✅ Completed: {}", $operation);
    };
    ($operation:expr, $($arg:tt)*) => {
        tracing::info!("✅ Completed: {}", format!($operation, $($arg)*));
    };
}

/// Macro for logging operation errors
#[macro_export]
macro_rules! log_operation_error {
    ($operation:expr, $error:expr) => {
        tracing::error!("❌ Error in {}: {}", $operation, $error);
    };
    ($operation:expr, $error:expr, $($arg:tt)*) => {
        tracing::error!("❌ Error in {}: {}", format!($operation, $($arg)*), $error);
    };
}

/// Macro for logging progress
#[macro_export]
macro_rules! log_progress {
    ($message:expr) => {
        tracing::info!("⏳ {}", $message);
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::info!("⏳ {}", format!($message, $($arg)*));
    };
}

/// Macro for logging results
#[macro_export]
macro_rules! log_result {
    ($message:expr) => {
        tracing::info!("📊 {}", $message);
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::info!("📊 {}", format!($message, $($arg)*));
    };
}

/// Macro for logging critical errors
#[macro_export]
macro_rules! log_fatal {
    ($message:expr) => {
        tracing::error!("💀 FATAL: {}", $message);
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::error!("💀 FATAL: {}", format!($message, $($arg)*));
    };
}

/// Macro for logging information
#[macro_export]
macro_rules! log_info {
    ($message:expr) => {
        tracing::info!("ℹ️ {}", $message);
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::info!("ℹ️ {}", format!($message, $($arg)*));
    };
}

/// Macro for logging errors
#[macro_export]
macro_rules! log_error {
    ($message:expr) => {
        tracing::error!("❌ {}", $message)
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::error!("❌ {}", format!($message, $($arg)*))
    };
}

/// Macro for logging warnings
#[macro_export]
macro_rules! log_warn {
    ($message:expr) => {
        tracing::warn!("⚠️ {}", $message)
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::warn!("⚠️ {}", format!($message, $($arg)*))
    };
}

/// Macro for logging debug information
#[macro_export]
macro_rules! log_debug {
    ($message:expr) => {
        tracing::debug!("🐛 {}", $message)
    };
    ($message:expr, $($arg:tt)*) => {
        tracing::debug!("🐛 {}", format!($message, $($arg)*))
    };
}
