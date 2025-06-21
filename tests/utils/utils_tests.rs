use cutlist_optimizer_cli::utils::{format_duration, percentage};
use std::time::Duration;

#[test]
fn test_format_duration() {
    assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
    assert_eq!(format_duration(Duration::from_secs(65)), "1m 5s");
    assert_eq!(format_duration(Duration::from_secs(3665)), "1h 1m 5s");
}

#[test]
fn test_percentage() {
    assert_eq!(percentage(25.0, 100.0), 25.0);
    assert_eq!(percentage(0.0, 100.0), 0.0);
    assert_eq!(percentage(50.0, 0.0), 0.0);
}
