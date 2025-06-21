//! Функции инициализации логирования

use crate::logging::structs::LogConfig;
use tracing_subscriber::{
    fmt::time::SystemTime,
    EnvFilter,
};

/// Инициализация логирования
pub fn init_logging(config: LogConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(String::from(config.level)));

    let subscriber = tracing_subscriber::fmt()
        .with_ansi(true)
        .with_level(config.show_level)
        .with_target(config.show_target)
        .with_env_filter(env_filter);

    if config.compact {
        if config.show_time {
            subscriber.compact().with_timer(SystemTime).try_init()?;
        } else {
            subscriber.compact().without_time().try_init()?;
        }
    } else {
        if config.show_time {
            subscriber.with_timer(SystemTime).try_init()?;
        } else {
            subscriber.without_time().try_init()?;
        }
    }

    Ok(())
}

/// Простая инициализация с настройками по умолчанию
pub fn init_default() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging(LogConfig::default())
}

/// Инициализация для CLI с учетом verbose флага
pub fn init_cli(verbose: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = if verbose {
        LogConfig::verbose()
    } else {
        LogConfig::default()
    };
    init_logging(config)
}
