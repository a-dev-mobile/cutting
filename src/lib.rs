//! CutList Optimizer - Core library for material cutting optimization
//!
//! This library provides algorithms and data structures for optimizing
//! the layout of cuts when processing sheet materials like wood, metal, etc.
//!
//! # Examples
//!
//! ```rust
//! use cutlist_optimizer_cli::{models::Configuration, Result};
//!
//! // Создание конфигурации для оптимизации
//! let config = Configuration::default();
//! ```

// Внутренние модули
pub mod cli;
pub mod comparator;
pub mod engine;
pub mod logging;
pub mod stock;

pub mod error;
pub mod models;
pub mod utils;

// Публичный API библиотеки
pub use error::{AppError, Result};
pub use models::{
    enums::{CutDirection, OptimizationPriority, Orientation, Status, StatusCode},
    Configuration,
    TileDimensions,
    DEFAULT_MATERIAL,
};

// Основные типы для работы с библиотекой
pub mod prelude {
    //! Основные типы и трейты для удобного импорта
    pub use crate::error::{AppError, Result};
    pub use crate::models::{
        enums::{CutDirection, OptimizationPriority, Orientation, Status, StatusCode},
        Configuration,
        TileDimensions,
        DEFAULT_MATERIAL,
    };
}
