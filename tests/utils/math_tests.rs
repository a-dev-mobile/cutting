use cutlist_optimizer_cli::utils::math::*;

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
fn test_efficiency_ratio_perfect() {
    assert_eq!(efficiency_ratio(100.0, 100.0), 100.0);
}

#[test]
fn test_efficiency_ratio_half() {
    assert_eq!(efficiency_ratio(50.0, 100.0), 50.0);
}

#[test]
fn test_efficiency_ratio_zero_used() {
    assert_eq!(efficiency_ratio(0.0, 100.0), 0.0);
}

#[test]
fn test_efficiency_ratio_zero_total() {
    assert_eq!(efficiency_ratio(50.0, 0.0), 0.0);
}

#[test]
fn test_waste_percentage_no_waste() {
    assert_eq!(waste_percentage(100.0, 100.0), 0.0);
}

#[test]
fn test_waste_percentage_half_waste() {
    assert_eq!(waste_percentage(50.0, 100.0), 50.0);
}

#[test]
fn test_waste_percentage_zero_used() {
    assert_eq!(waste_percentage(0.0, 100.0), 100.0);
}

#[test]
fn test_waste_percentage_zero_total() {
    assert_eq!(waste_percentage(50.0, 0.0), 0.0);
}

#[test]
fn test_round_to_decimal_places() {
    assert_eq!(round_to_decimal_places(3.14159, 2), 3.14);
    assert_eq!(round_to_decimal_places(2.5, 0), 3.0);
    assert_eq!(round_to_decimal_places(1.999, 1), 2.0);
}

#[test]
fn test_approx_equal() {
    assert!(approx_equal(1.0, 1.0000000001, 1e-9));
    assert!(!approx_equal(1.0, 1.1, 1e-9));
    assert!(approx_equal_default(1.0, 1.0000000001));
}

#[test]
fn test_clamp() {
    assert_eq!(clamp(5, 0, 10), 5);
    assert_eq!(clamp(-5, 0, 10), 0);
    assert_eq!(clamp(15, 0, 10), 10);
}

#[test]
fn test_lerp() {
    assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
    assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
    assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
}

#[test]
fn test_rectangle_area() {
    assert_eq!(rectangle_area(5.0, 4.0), 20.0);
    assert_eq!(rectangle_area(0.0, 4.0), 0.0);
}

#[test]
fn test_rectangle_perimeter() {
    assert_eq!(rectangle_perimeter(5.0, 4.0), 18.0);
    assert_eq!(rectangle_perimeter(0.0, 4.0), 8.0);
}
