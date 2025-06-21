use cutlist_optimizer_cli::models::{CalculationRequest, Panel};

#[test]
fn test_new_calculation_request() {
    let request = CalculationRequest::new();
    assert!(request.configuration().is_none());
    assert!(request.panels().is_empty());
    assert!(request.stock_panels().is_empty());
}

#[test]
fn test_with_configuration() {
    use cutlist_optimizer_cli::models::Configuration;
    use cutlist_optimizer_cli::comparator::OptimizationPriority;
    use cutlist_optimizer_cli::models::PerformanceThresholds;
    
    let config = Configuration {
        cut_thickness: 3,
        min_trim_dimension: 10,
        consider_orientation: true,
        optimization_factor: 5,
        optimization_priority: OptimizationPriority::LeastWastedArea,
        use_single_stock_unit: false,
        units: "mm".to_string(),
        performance_thresholds: PerformanceThresholds::default(),
    };
    
    let request = CalculationRequest::with_configuration(config);
    assert!(request.configuration().is_some());
    assert!(request.panels().is_empty());
    assert!(request.stock_panels().is_empty());
}

#[test]
fn test_add_panel() {
    let mut request = CalculationRequest::new();
    
    let mut panel = Panel::default();
    panel.count = 2;
    panel.width = Some("100".to_string());
    panel.height = Some("200".to_string());
    
    request.add_panel(panel.clone());
    
    assert_eq!(request.panels().len(), 1);
    assert_eq!(request.panels()[0].count, 2);
}

#[test]
fn test_add_stock_panel() {
    let mut request = CalculationRequest::new();
    
    let mut stock_panel = Panel::default();
    stock_panel.count = 5;
    stock_panel.width = Some("1000".to_string());
    stock_panel.height = Some("2000".to_string());
    
    request.add_stock_panel(stock_panel.clone());
    
    assert_eq!(request.stock_panels().len(), 1);
    assert_eq!(request.stock_panels()[0].count, 5);
}

#[test]
fn test_tiles_to_string() {
    let mut request = CalculationRequest::new();
    
    // Create test panels
    let mut panel1 = Panel::default();
    panel1.count = 2;
    panel1.width = Some("100".to_string());
    panel1.height = Some("200".to_string());
    
    let mut panel2 = Panel::default();
    panel2.count = 0; // This should be filtered out
    panel2.width = Some("150".to_string());
    panel2.height = Some("250".to_string());
    
    let mut panel3 = Panel::default();
    panel3.count = 1;
    panel3.width = Some("300".to_string());
    panel3.height = Some("400".to_string());
    
    request.add_panel(panel1);
    request.add_panel(panel2);
    request.add_panel(panel3);
    
    let result = request.tiles_to_string();
    
    // Should contain panel1 and panel3, but not panel2 (count = 0)
    assert!(result.contains("[100x200]*2"));
    assert!(result.contains("[300x400]*1"));
    assert!(!result.contains("[150x250]*0"));
}

#[test]
fn test_base_tiles_to_string() {
    let mut request = CalculationRequest::new();
    
    let mut stock_panel = Panel::default();
    stock_panel.count = 5;
    stock_panel.width = Some("1000".to_string());
    stock_panel.height = Some("2000".to_string());
    
    request.add_stock_panel(stock_panel);
    
    let result = request.base_tiles_to_string();
    assert!(result.contains("[1000x2000]*5"));
}

#[test]
fn test_set_panels() {
    let mut request = CalculationRequest::new();
    
    let mut panel1 = Panel::default();
    panel1.count = 1;
    
    let mut panel2 = Panel::default();
    panel2.count = 2;
    
    let panels = vec![panel1, panel2];
    request.set_panels(panels);
    
    assert_eq!(request.panels().len(), 2);
    assert_eq!(request.panels()[0].count, 1);
    assert_eq!(request.panels()[1].count, 2);
}

#[test]
fn test_set_stock_panels() {
    let mut request = CalculationRequest::new();
    
    let mut stock_panel1 = Panel::default();
    stock_panel1.count = 3;
    
    let mut stock_panel2 = Panel::default();
    stock_panel2.count = 4;
    
    let stock_panels = vec![stock_panel1, stock_panel2];
    request.set_stock_panels(stock_panels);
    
    assert_eq!(request.stock_panels().len(), 2);
    assert_eq!(request.stock_panels()[0].count, 3);
    assert_eq!(request.stock_panels()[1].count, 4);
}

#[test]
fn test_take_configuration() {
    use cutlist_optimizer_cli::models::Configuration;
    use cutlist_optimizer_cli::comparator::OptimizationPriority;
    use cutlist_optimizer_cli::models::PerformanceThresholds;
    
    let config = Configuration {
        cut_thickness: 3,
        min_trim_dimension: 10,
        consider_orientation: true,
        optimization_factor: 5,
        optimization_priority: OptimizationPriority::LeastWastedArea,
        use_single_stock_unit: false,
        units: "mm".to_string(),
        performance_thresholds: PerformanceThresholds::default(),
    };
    
    let mut request = CalculationRequest::with_configuration(config);
    assert!(request.configuration().is_some());
    
    let taken_config = request.take_configuration();
    assert!(taken_config.is_some());
    assert!(request.configuration().is_none());
}
