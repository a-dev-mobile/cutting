use cutlist_optimizer_cli::models::PerformanceThresholds;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_performance_thresholds() {
        let thresholds = PerformanceThresholds::default();
        
        assert_eq!(thresholds.max_simultaneous_tasks(), 1);
        assert!(thresholds.max_simultaneous_threads() > 0);
        assert_eq!(thresholds.thread_check_interval(), 1000);
    }

    #[test]
    fn test_new_constructor() {
        let thresholds = PerformanceThresholds::new(4, 2000);
        
        assert_eq!(thresholds.max_simultaneous_tasks(), 1); // Default value
        assert_eq!(thresholds.max_simultaneous_threads(), 4);
        assert_eq!(thresholds.thread_check_interval(), 2000);
    }

    #[test]
    fn test_with_all_params_constructor() {
        let thresholds = PerformanceThresholds::with_all_params(5, 8, 1500);
        
        assert_eq!(thresholds.max_simultaneous_tasks(), 5);
        assert_eq!(thresholds.max_simultaneous_threads(), 8);
        assert_eq!(thresholds.thread_check_interval(), 1500);
    }

    #[test]
    fn test_setters() {
        let mut thresholds = PerformanceThresholds::default();
        
        thresholds.set_max_simultaneous_tasks(3);
        thresholds.set_max_simultaneous_threads(6);
        thresholds.set_thread_check_interval(500);
        
        assert_eq!(thresholds.max_simultaneous_tasks(), 3);
        assert_eq!(thresholds.max_simultaneous_threads(), 6);
        assert_eq!(thresholds.thread_check_interval(), 500);
    }

    #[test]
    fn test_validation_success() {
        let thresholds = PerformanceThresholds::with_all_params(2, 4, 1000);
        assert!(thresholds.validate().is_ok());
    }

    #[test]
    fn test_validation_zero_tasks() {
        let thresholds = PerformanceThresholds::with_all_params(0, 4, 1000);
        assert!(thresholds.validate().is_err());
    }

    #[test]
    fn test_validation_zero_threads() {
        let thresholds = PerformanceThresholds::with_all_params(2, 0, 1000);
        assert!(thresholds.validate().is_err());
    }

    #[test]
    fn test_validation_zero_interval() {
        let thresholds = PerformanceThresholds::with_all_params(2, 4, 0);
        assert!(thresholds.validate().is_err());
    }

    #[test]
    fn test_serialization() {
        let thresholds = PerformanceThresholds::with_all_params(3, 6, 2000);
        let json = serde_json::to_string(&thresholds).unwrap();
        let deserialized: PerformanceThresholds = serde_json::from_str(&json).unwrap();
        
        assert_eq!(thresholds.max_simultaneous_tasks(), deserialized.max_simultaneous_tasks());
        assert_eq!(thresholds.max_simultaneous_threads(), deserialized.max_simultaneous_threads());
        assert_eq!(thresholds.thread_check_interval(), deserialized.thread_check_interval());
    }
}