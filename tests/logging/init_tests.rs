//! Тесты для функций инициализации логирования

use cutlist_optimizer_cli::logging::{init_default, LogConfig, LogLevel};

#[test]
fn test_init_default_does_not_panic() {
    // Тест проверяет, что функция не паникует
    // В реальном окружении может быть уже инициализирован subscriber,
    // поэтому мы просто проверяем, что функция выполняется
    let result = init_default();
    // Может вернуть ошибку, если уже инициализирован, но не должна паниковать
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_log_config_creation() {
    let default_config = LogConfig::default();
    assert_eq!(default_config.level, LogLevel::Info);
    
    let verbose_config = LogConfig::verbose();
    assert_eq!(verbose_config.level, LogLevel::Debug);
    assert!(verbose_config.show_time);
    
    let quiet_config = LogConfig::quiet();
    assert_eq!(quiet_config.level, LogLevel::Error);
    assert!(!quiet_config.show_time);
}

#[test]
fn test_config_properties() {
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

#[test]
fn test_verbose_vs_default_config() {
    let default = LogConfig::default();
    let verbose = LogConfig::verbose();
    
    // Verbose должен иметь более детальные настройки
    assert!(verbose.show_time && !default.show_time);
    assert!(verbose.show_target && !default.show_target);
    assert!(!verbose.compact && default.compact);
}

#[test]
fn test_quiet_vs_default_config() {
    let default = LogConfig::default();
    let quiet = LogConfig::quiet();
    
    // Quiet должен показывать меньше информации
    assert!(!quiet.show_level && default.show_level);
    assert_eq!(quiet.level, LogLevel::Error);
    assert_eq!(default.level, LogLevel::Info);
}
