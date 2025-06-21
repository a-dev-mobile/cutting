//! Тесты для структур модуля логирования

use cutlist_optimizer_cli::logging::{LogConfig, LogLevel};

#[test]
fn test_log_config_default() {
    let config = LogConfig::default();
    assert_eq!(config.level, LogLevel::Info);
    assert!(!config.show_time);
    assert!(!config.show_target);
    assert!(config.show_level);
    assert!(config.compact);
}

#[test]
fn test_log_config_verbose() {
    let config = LogConfig::verbose();
    assert_eq!(config.level, LogLevel::Debug);
    assert!(config.show_time);
    assert!(config.show_target);
    assert!(config.show_level);
    assert!(!config.compact);
}

#[test]
fn test_log_config_quiet() {
    let config = LogConfig::quiet();
    assert_eq!(config.level, LogLevel::Error);
    assert!(!config.show_time);
    assert!(!config.show_target);
    assert!(!config.show_level);
    assert!(config.compact);
}

#[test]
fn test_with_time_config() {
    let config = LogConfig::with_time();
    assert_eq!(config.level, LogLevel::Info);
    assert!(config.show_time);
    assert!(!config.show_target);
    assert!(config.show_level);
    assert!(config.compact);
}

#[test]
fn test_log_config_debug() {
    let config = LogConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("LogConfig"));
    assert!(debug_str.contains("Info"));
}

#[test]
fn test_log_config_custom() {
    let config = LogConfig {
        level: LogLevel::Warn,
        show_time: true,
        show_target: false,
        show_level: true,
        compact: false,
    };
    
    assert_eq!(config.level, LogLevel::Warn);
    assert!(config.show_time);
    assert!(!config.show_target);
    assert!(config.show_level);
    assert!(!config.compact);
}
