//! Тесты для перечислений модуля логирования

use cutlist_optimizer_cli::logging::enums::LogLevel;
use tracing::Level;

#[test]
fn test_log_level_to_tracing_level_conversion() {
    assert_eq!(Level::from(LogLevel::Error), Level::ERROR);
    assert_eq!(Level::from(LogLevel::Warn), Level::WARN);
    assert_eq!(Level::from(LogLevel::Info), Level::INFO);
    assert_eq!(Level::from(LogLevel::Debug), Level::DEBUG);
    assert_eq!(Level::from(LogLevel::Trace), Level::TRACE);
}

#[test]
fn test_log_level_to_string_conversion() {
    assert_eq!(String::from(LogLevel::Error), "error");
    assert_eq!(String::from(LogLevel::Warn), "warn");
    assert_eq!(String::from(LogLevel::Info), "info");
    assert_eq!(String::from(LogLevel::Debug), "debug");
    assert_eq!(String::from(LogLevel::Trace), "trace");
}

#[test]
fn test_log_level_equality() {
    assert_eq!(LogLevel::Error, LogLevel::Error);
    assert_ne!(LogLevel::Error, LogLevel::Warn);
    assert_ne!(LogLevel::Info, LogLevel::Debug);
}

#[test]
fn test_log_level_clone() {
    let level = LogLevel::Info;
    let cloned = level.clone();
    assert_eq!(level, cloned);
}

#[test]
fn test_log_level_debug() {
    let level = LogLevel::Debug;
    let debug_str = format!("{:?}", level);
    assert_eq!(debug_str, "Debug");
}
