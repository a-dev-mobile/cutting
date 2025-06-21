use cutlist_optimizer_cli::logging::*;
use std::sync::{Arc, Mutex};
use tracing_subscriber::fmt::MakeWriter;

// Простой writer для захвата логов в тестах
#[derive(Clone)]
struct TestWriter {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl TestWriter {
    fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_output(&self) -> String {
        let buffer = self.buffer.lock().unwrap();
        String::from_utf8_lossy(&buffer).to_string()
    }
}

impl std::io::Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for TestWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{error, warn, info, debug, trace};

    #[test]
    fn test_log_error_macro() {
        let test_writer = TestWriter::new();
        
        // Инициализируем логгер с тестовым writer
        let subscriber = tracing_subscriber::fmt()
            .with_writer(test_writer.clone())
            .with_max_level(tracing::Level::ERROR)
            .finish();
        
        let _guard = tracing::subscriber::set_default(subscriber);
        
        error!("Test error message");
        
        let output = test_writer.get_output();
        assert!(output.contains("ERROR"));
        assert!(output.contains("Test error message"));
    }

    #[test]
    fn test_log_warn_macro() {
        let test_writer = TestWriter::new();
        
        let subscriber = tracing_subscriber::fmt()
            .with_writer(test_writer.clone())
            .with_max_level(tracing::Level::WARN)
            .finish();
        
        let _guard = tracing::subscriber::set_default(subscriber);
        
        warn!("Test warning message");
        
        let output = test_writer.get_output();
        assert!(output.contains("WARN"));
        assert!(output.contains("Test warning message"));
    }

    #[test]
    fn test_log_info_macro() {
        let test_writer = TestWriter::new();
        
        let subscriber = tracing_subscriber::fmt()
            .with_writer(test_writer.clone())
            .with_max_level(tracing::Level::INFO)
            .finish();
        
        let _guard = tracing::subscriber::set_default(subscriber);
        
        info!("Test info message");
        
        let output = test_writer.get_output();
        assert!(output.contains("INFO"));
        assert!(output.contains("Test info message"));
    }

    #[test]
    fn test_log_debug_macro() {
        let test_writer = TestWriter::new();
        
        let subscriber = tracing_subscriber::fmt()
            .with_writer(test_writer.clone())
            .with_max_level(tracing::Level::DEBUG)
            .finish();
        
        let _guard = tracing::subscriber::set_default(subscriber);
        
        debug!("Test debug message");
        
        let output = test_writer.get_output();
        assert!(output.contains("DEBUG"));
        assert!(output.contains("Test debug message"));
    }

    #[test]
    fn test_log_trace_macro() {
        let test_writer = TestWriter::new();
        
        let subscriber = tracing_subscriber::fmt()
            .with_writer(test_writer.clone())
            .with_max_level(tracing::Level::TRACE)
            .finish();
        
        let _guard = tracing::subscriber::set_default(subscriber);
        
        trace!("Test trace message");
        
        let output = test_writer.get_output();
        assert!(output.contains("TRACE"));
        assert!(output.contains("Test trace message"));
    }

    #[test]
    fn test_macros_with_formatting() {
        let test_writer = TestWriter::new();
        
        let subscriber = tracing_subscriber::fmt()
            .with_writer(test_writer.clone())
            .with_max_level(tracing::Level::INFO)
            .finish();
        
        let _guard = tracing::subscriber::set_default(subscriber);
        
        let user_id = 42;
        let action = "login";
        
        info!("User {} performed action: {}", user_id, action);
        
        let output = test_writer.get_output();
        assert!(output.contains("User 42 performed action: login"));
    }

    #[test]
    fn test_macros_with_fields() {
        let test_writer = TestWriter::new();
        
        let subscriber = tracing_subscriber::fmt()
            .with_writer(test_writer.clone())
            .with_max_level(tracing::Level::INFO)
            .finish();
        
        let _guard = tracing::subscriber::set_default(subscriber);
        
        info!(user_id = 42, action = "login", "User action performed");
        
        let output = test_writer.get_output();
        assert!(output.contains("User action performed"));
    }

    #[test]
    fn test_log_level_filtering() {
        let test_writer = TestWriter::new();
        
        // Устанавливаем уровень только WARN и выше
        let subscriber = tracing_subscriber::fmt()
            .with_writer(test_writer.clone())
            .with_max_level(tracing::Level::WARN)
            .finish();
        
        let _guard = tracing::subscriber::set_default(subscriber);
        
        error!("This should appear");
        warn!("This should also appear");
        info!("This should NOT appear");
        debug!("This should NOT appear");
        
        let output = test_writer.get_output();
        assert!(output.contains("This should appear"));
        assert!(output.contains("This should also appear"));
        assert!(!output.contains("This should NOT appear"));
    }

    #[test]
    fn test_logging_config() {
        let config = LogConfig::default();
        assert_eq!(config.level, LogLevel::Info);
        assert!(!config.show_time);
        assert!(config.show_level);
        assert!(!config.show_target);
        assert!(config.compact);
    }

    #[test]
    fn test_verbose_config() {
        let config = LogConfig::verbose();
        assert_eq!(config.level, LogLevel::Debug);
        assert!(config.show_time);
        assert!(config.show_level);
        assert!(config.show_target);
        assert!(!config.compact);
    }

    #[test]
    fn test_quiet_config() {
        let config = LogConfig::quiet();
        assert_eq!(config.level, LogLevel::Error);
        assert!(!config.show_time);
        assert!(!config.show_level);
        assert!(!config.show_target);
        assert!(config.compact);
    }
}
