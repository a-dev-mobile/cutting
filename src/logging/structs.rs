//! Структуры для модуля логирования

use crate::logging::enums::LogLevel;

/// Конфигурация логирования
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Уровень логирования
    pub level: LogLevel,
    /// Показывать время в логах
    pub show_time: bool,
    /// Показывать цели (targets) в логах
    pub show_target: bool,
    /// Показывать уровень в логах
    pub show_level: bool,
    /// Компактный формат вывода
    pub compact: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            show_time: false,
            show_target: false,
            show_level: true,
            compact: true,
        }
    }
}

impl LogConfig {
    /// Создать конфигурацию для verbose режима
    pub fn verbose() -> Self {
        Self {
            level: LogLevel::Debug,
            show_time: true,
            show_target: true,
            show_level: true,
            compact: false,
        }
    }

    /// Создать конфигурацию для quiet режима
    pub fn quiet() -> Self {
        Self {
            level: LogLevel::Error,
            show_time: false,
            show_target: false,
            show_level: false,
            compact: true,
        }
    }

    /// Создать конфигурацию с временными метками (Info уровень)
    pub fn with_time() -> Self {
        Self {
            level: LogLevel::Info,
            show_time: true,
            show_target: false,
            show_level: true,
            compact: true,
        }
    }
}
