use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// Уровни логирования
#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// Запись в логе
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: String,
    pub thread_id: Option<String>,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: String, thread_id: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        
        Self {
            timestamp,
            level,
            message,
            thread_id,
        }
    }
}

/// Трейт для логгера (соответствует CutListLogger.java)
pub trait CutListLogger: Send + Sync {
    fn log(&self, level: LogLevel, message: &str);
    fn log_with_thread(&self, level: LogLevel, message: &str, thread_id: &str);
    fn debug(&self, message: &str);
    fn info(&self, message: &str);
    fn warning(&self, message: &str);
    fn error(&self, message: &str);
    fn get_logs(&self) -> Vec<LogEntry>;
    fn clear_logs(&self);
    fn set_max_entries(&self, max_entries: usize);
}

/// Реализация логгера (соответствует CutListLoggerImpl.java)
#[derive(Debug)]
pub struct CutListLoggerImpl {
    entries: Arc<Mutex<VecDeque<LogEntry>>>,
    max_entries: Arc<Mutex<usize>>,
    enabled: Arc<Mutex<bool>>,
}

impl CutListLoggerImpl {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(VecDeque::new())),
            max_entries: Arc::new(Mutex::new(1000)), // По умолчанию 1000 записей
            enabled: Arc::new(Mutex::new(true)),
        }
    }

    pub fn with_max_entries(max_entries: usize) -> Self {
        Self {
            entries: Arc::new(Mutex::new(VecDeque::new())),
            max_entries: Arc::new(Mutex::new(max_entries)),
            enabled: Arc::new(Mutex::new(true)),
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        if let Ok(mut guard) = self.enabled.lock() {
            *guard = enabled;
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.lock().map(|guard| *guard).unwrap_or(true)
    }

    fn add_entry(&self, entry: LogEntry) {
        if !self.is_enabled() {
            return;
        }

        if let (Ok(mut entries), Ok(max_entries)) = (self.entries.lock(), self.max_entries.lock()) {
            entries.push_back(entry);
            
            // Удаляем старые записи, если превышен лимит
            while entries.len() > *max_entries {
                entries.pop_front();
            }
        }
    }

    pub fn get_entries_by_level(&self, level: LogLevel) -> Vec<LogEntry> {
        if let Ok(entries) = self.entries.lock() {
            entries.iter()
                .filter(|entry| entry.level == level)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_entries_by_thread(&self, thread_id: &str) -> Vec<LogEntry> {
        if let Ok(entries) = self.entries.lock() {
            entries.iter()
                .filter(|entry| {
                    entry.thread_id.as_ref().map_or(false, |id| id == thread_id)
                })
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_recent_entries(&self, count: usize) -> Vec<LogEntry> {
        if let Ok(entries) = self.entries.lock() {
            entries.iter()
                .rev()
                .take(count)
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for CutListLoggerImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl CutListLogger for CutListLoggerImpl {
    fn log(&self, level: LogLevel, message: &str) {
        let entry = LogEntry::new(level, message.to_string(), None);
        self.add_entry(entry);
    }

    fn log_with_thread(&self, level: LogLevel, message: &str, thread_id: &str) {
        let entry = LogEntry::new(level, message.to_string(), Some(thread_id.to_string()));
        self.add_entry(entry);
    }

    fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    fn get_logs(&self) -> Vec<LogEntry> {
        if let Ok(entries) = self.entries.lock() {
            entries.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }

    fn clear_logs(&self) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.clear();
        }
    }

    fn set_max_entries(&self, max_entries: usize) {
        if let Ok(mut max) = self.max_entries.lock() {
            *max = max_entries;
        }
    }
}

/// Глобальный логгер для удобства использования
pub struct GlobalLogger {
    logger: Arc<dyn CutListLogger>,
}

impl GlobalLogger {
    pub fn new(logger: Arc<dyn CutListLogger>) -> Self {
        Self { logger }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        self.logger.log(level, message);
    }

    pub fn debug(&self, message: &str) {
        self.logger.debug(message);
    }

    pub fn info(&self, message: &str) {
        self.logger.info(message);
    }

    pub fn warning(&self, message: &str) {
        self.logger.warning(message);
    }

    pub fn error(&self, message: &str) {
        self.logger.error(message);
    }
}

/// Макросы для удобного логирования
#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $($arg:tt)*) => {
        $logger.debug(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $($arg:tt)*) => {
        $logger.info(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warning {
    ($logger:expr, $($arg:tt)*) => {
        $logger.warning(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $($arg:tt)*) => {
        $logger.error(&format!($($arg)*));
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_basic_functionality() {
        let logger = CutListLoggerImpl::new();
        
        logger.info("Test message");
        logger.debug("Debug message");
        logger.warning("Warning message");
        logger.error("Error message");
        
        let logs = logger.get_logs();
        assert_eq!(logs.len(), 4);
        
        assert_eq!(logs[0].level, LogLevel::Info);
        assert_eq!(logs[0].message, "Test message");
        
        assert_eq!(logs[1].level, LogLevel::Debug);
        assert_eq!(logs[1].message, "Debug message");
        
        assert_eq!(logs[2].level, LogLevel::Warning);
        assert_eq!(logs[2].message, "Warning message");
        
        assert_eq!(logs[3].level, LogLevel::Error);
        assert_eq!(logs[3].message, "Error message");
    }

    #[test]
    fn test_logger_with_thread_id() {
        let logger = CutListLoggerImpl::new();
        
        logger.log_with_thread(LogLevel::Info, "Thread message", "thread-1");
        
        let logs = logger.get_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].thread_id, Some("thread-1".to_string()));
    }

    #[test]
    fn test_logger_max_entries() {
        let logger = CutListLoggerImpl::with_max_entries(2);
        
        logger.info("Message 1");
        logger.info("Message 2");
        logger.info("Message 3");
        
        let logs = logger.get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].message, "Message 2");
        assert_eq!(logs[1].message, "Message 3");
    }

    #[test]
    fn test_logger_clear() {
        let logger = CutListLoggerImpl::new();
        
        logger.info("Message 1");
        logger.info("Message 2");
        
        assert_eq!(logger.get_logs().len(), 2);
        
        logger.clear_logs();
        assert_eq!(logger.get_logs().len(), 0);
    }

    #[test]
    fn test_logger_filter_by_level() {
        let logger = CutListLoggerImpl::new();
        
        logger.info("Info message");
        logger.debug("Debug message");
        logger.error("Error message");
        
        let info_logs = logger.get_entries_by_level(LogLevel::Info);
        assert_eq!(info_logs.len(), 1);
        assert_eq!(info_logs[0].message, "Info message");
        
        let debug_logs = logger.get_entries_by_level(LogLevel::Debug);
        assert_eq!(debug_logs.len(), 1);
        assert_eq!(debug_logs[0].message, "Debug message");
    }

    #[test]
    fn test_logger_filter_by_thread() {
        let logger = CutListLoggerImpl::new();
        
        logger.log_with_thread(LogLevel::Info, "Thread 1 message", "thread-1");
        logger.log_with_thread(LogLevel::Info, "Thread 2 message", "thread-2");
        logger.log_with_thread(LogLevel::Info, "Another thread 1 message", "thread-1");
        
        let thread1_logs = logger.get_entries_by_thread("thread-1");
        assert_eq!(thread1_logs.len(), 2);
        assert_eq!(thread1_logs[0].message, "Thread 1 message");
        assert_eq!(thread1_logs[1].message, "Another thread 1 message");
        
        let thread2_logs = logger.get_entries_by_thread("thread-2");
        assert_eq!(thread2_logs.len(), 1);
        assert_eq!(thread2_logs[0].message, "Thread 2 message");
    }

    #[test]
    fn test_logger_recent_entries() {
        let logger = CutListLoggerImpl::new();
        
        logger.info("Message 1");
        logger.info("Message 2");
        logger.info("Message 3");
        logger.info("Message 4");
        
        let recent = logger.get_recent_entries(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].message, "Message 3");
        assert_eq!(recent[1].message, "Message 4");
    }

    #[test]
    fn test_logger_enabled_disabled() {
        let logger = CutListLoggerImpl::new();
        
        logger.info("Message 1");
        assert_eq!(logger.get_logs().len(), 1);
        
        logger.set_enabled(false);
        logger.info("Message 2");
        assert_eq!(logger.get_logs().len(), 1); // Не должно добавиться
        
        logger.set_enabled(true);
        logger.info("Message 3");
        assert_eq!(logger.get_logs().len(), 2);
    }
}
