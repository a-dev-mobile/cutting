//! Logging functionality for Task struct
//! 
//! This module contains methods for managing task logging and log content.

use super::Task;

impl Task {
    // ===== Logging =====

    /// Get the current log
    pub fn log(&self) -> String {
        self.log.lock().unwrap().clone()
    }

    /// Set the log content
    pub fn set_log(&self, log_content: String) {
        *self.log.lock().unwrap() = log_content;
    }

    /// Append a line to the log
    pub fn append_line_to_log(&self, line: &str) {
        let mut log = self.log.lock().unwrap();
        if !log.is_empty() {
            log.push('\n');
        }
        log.push_str(line);
    }
}
