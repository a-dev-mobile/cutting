use cutlist_optimizer_cli::utils::timing::*;
use cutlist_optimizer_cli::utils::math::percentage;
use std::time::Duration;

#[test]
fn test_format_duration_milliseconds() {
    let duration = Duration::from_millis(500);
    assert_eq!(format_duration(duration), "500ms");
}

#[test]
fn test_format_duration_seconds() {
    let duration = Duration::from_secs(45);
    assert_eq!(format_duration(duration), "45.0s");
}

#[test]
fn test_format_duration_minutes_and_seconds() {
    let duration = Duration::from_secs(65); // 1 minute 5 seconds
    assert_eq!(format_duration(duration), "1m 5s");
}

#[test]
fn test_format_duration_hours_minutes_seconds() {
    let duration = Duration::from_secs(3665); // 1 hour 1 minute 5 seconds
    assert_eq!(format_duration(duration), "1h 1m 5s");
}

#[test]
fn test_format_duration_zero() {
    let duration = Duration::from_secs(0);
    assert_eq!(format_duration(duration), "0ms");
}

#[test]
fn test_percentage_basic() {
    assert_eq!(percentage(25.0, 100.0), 25.0);
}

#[test]
fn test_percentage_zero_numerator() {
    assert_eq!(percentage(0.0, 100.0), 0.0);
}

#[test]
fn test_percentage_zero_denominator() {
    assert_eq!(percentage(50.0, 0.0), 0.0);
}

#[test]
fn test_percentage_equal_values() {
    assert_eq!(percentage(100.0, 100.0), 100.0);
}

#[test]
fn test_percentage_decimal_result() {
    assert_eq!(percentage(33.0, 100.0), 33.0);
}
