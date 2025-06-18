use std::sync::atomic::{AtomicI32, Ordering};

/// Глобальный счетчик для генерации уникальных ID
static NEXT_ID: AtomicI32 = AtomicI32::new(0);

/// Генерирует следующий уникальный ID
pub fn next_id() -> i32 {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

/// Материал по умолчанию
pub const DEFAULT_MATERIAL: &str = "DEFAULT_MATERIAL";

/// Максимальное значение для внешнего ID (эквивалент RoomDatabase.MAX_BIND_PARAMETER_CNT)
pub const MAX_EXTERNAL_ID: i32 = 999;
