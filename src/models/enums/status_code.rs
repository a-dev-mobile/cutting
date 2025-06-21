use serde::{Deserialize, Serialize};

/// Status codes for operation results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum StatusCode {
    Ok = 0,
    InvalidTiles = 1,
    InvalidStockTiles = 2,
    TaskAlreadyRunning = 3,
    ServerUnavailable = 4,
    TooManyPanels = 5,
    TooManyStockPanels = 6,
}

impl StatusCode {
    /// Get the numeric value of the status code
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Get the string representation of the numeric value
    pub fn string_value(self) -> String {
        self.value().to_string()
    }

    /// Create StatusCode from numeric value
    pub const fn from_value(value: u8) -> Option<StatusCode> {
        match value {
            0 => Some(StatusCode::Ok),
            1 => Some(StatusCode::InvalidTiles),
            2 => Some(StatusCode::InvalidStockTiles),
            3 => Some(StatusCode::TaskAlreadyRunning),
            4 => Some(StatusCode::ServerUnavailable),
            5 => Some(StatusCode::TooManyPanels),
            6 => Some(StatusCode::TooManyStockPanels),
            _ => None,
        }
    }

    /// Check if the status code indicates success
    pub const fn is_ok(self) -> bool {
        matches!(self, StatusCode::Ok)
    }

    /// Check if the status code indicates an error
    pub const fn is_error(self) -> bool {
        !self.is_ok()
    }

    /// Get a human-readable description of the status code
    pub const fn description(self) -> &'static str {
        match self {
            StatusCode::Ok => "Operation completed successfully",
            StatusCode::InvalidTiles => "Invalid tiles provided",
            StatusCode::InvalidStockTiles => "Invalid stock tiles provided",
            StatusCode::TaskAlreadyRunning => "Task is already running",
            StatusCode::ServerUnavailable => "Server is unavailable",
            StatusCode::TooManyPanels => "Too many panels specified",
            StatusCode::TooManyStockPanels => "Too many stock panels specified",
        }
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::Ok
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.value(), self.description())
    }
}

impl From<StatusCode> for u8 {
    fn from(status: StatusCode) -> Self {
        status.value()
    }
}

impl TryFrom<u8> for StatusCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        StatusCode::from_value(value).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_values() {
        assert_eq!(StatusCode::Ok.value(), 0);
        assert_eq!(StatusCode::InvalidTiles.value(), 1);
        assert_eq!(StatusCode::TooManyStockPanels.value(), 6);
    }

    #[test]
    fn test_string_value() {
        assert_eq!(StatusCode::Ok.string_value(), "0");
        assert_eq!(StatusCode::InvalidTiles.string_value(), "1");
    }

    #[test]
    fn test_from_value() {
        assert_eq!(StatusCode::from_value(0), Some(StatusCode::Ok));
        assert_eq!(StatusCode::from_value(1), Some(StatusCode::InvalidTiles));
        assert_eq!(StatusCode::from_value(99), None);
    }

    #[test]
    fn test_is_ok_and_is_error() {
        assert!(StatusCode::Ok.is_ok());
        assert!(!StatusCode::Ok.is_error());
        
        assert!(!StatusCode::InvalidTiles.is_ok());
        assert!(StatusCode::InvalidTiles.is_error());
    }

    #[test]
    fn test_display() {
        let status = StatusCode::Ok;
        assert_eq!(format!("{}", status), "0: Operation completed successfully");
    }

    #[test]
    fn test_conversions() {
        let status = StatusCode::InvalidTiles;
        let value: u8 = status.into();
        assert_eq!(value, 1);

        let back: StatusCode = StatusCode::try_from(value).unwrap();
        assert_eq!(back, status);
    }
}
