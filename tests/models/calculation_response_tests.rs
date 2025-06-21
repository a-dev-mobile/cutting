//! Tests for CalculationResponse model

use cutlist_optimizer_cli::models::{CalculationResponse, CalculationRequest, FinalTile, NoFitTile, Mosaic};
use std::collections::HashMap;

#[test]
fn test_calculation_response_default() {
    let response = CalculationResponse::new();
    
    assert_eq!(response.version(), "1.2");
    assert_eq!(response.elapsed_time(), 0);
    assert_eq!(response.id(), None);
    assert_eq!(response.task_id(), None);
    assert_eq!(response.total_cut_length(), 0.0);
    assert_eq!(response.total_nbr_cuts(), 0);
    assert_eq!(response.total_used_area(), 0.0);
    assert_eq!(response.total_used_area_ratio(), 0.0);
    assert_eq!(response.total_wasted_area(), 0.0);
    assert!(response.panels().is_none());
    assert!(response.request().is_none());
    assert!(response.solution_elapsed_time().is_none());
    assert!(response.used_stock_panels().is_none());
    assert!(response.edge_bands().is_none());
    assert!(response.no_fit_panels().is_empty());
    assert!(response.mosaics().is_empty());
}

#[test]
fn test_calculation_response_new() {
    let response = CalculationResponse::new();
    
    assert_eq!(response.version(), "1.2");
    assert_eq!(response.elapsed_time(), 0);
}

#[test]
fn test_calculation_response_with_id() {
    let id = "test-id-123".to_string();
    let response = CalculationResponse::with_id(id.clone());
    
    assert_eq!(response.id(), Some("test-id-123"));
    assert_eq!(response.version(), "1.2");
}

#[test]
fn test_setters_and_getters() {
    let mut response = CalculationResponse::new();
    
    // Test ID
    response.set_id("calc-123".to_string());
    assert_eq!(response.id(), Some("calc-123"));
    
    // Test task ID
    response.set_task_id("task-456".to_string());
    assert_eq!(response.task_id(), Some("task-456"));
    
    // Test elapsed time
    response.set_elapsed_time(1500);
    assert_eq!(response.elapsed_time(), 1500);
    
    // Test solution elapsed time
    response.set_solution_elapsed_time(1200);
    assert_eq!(response.solution_elapsed_time(), Some(1200));
    
    // Test total used area
    response.set_total_used_area(150.5);
    assert_eq!(response.total_used_area(), 150.5);
    
    // Test total wasted area
    response.set_total_wasted_area(25.3);
    assert_eq!(response.total_wasted_area(), 25.3);
    
    // Test total used area ratio
    response.set_total_used_area_ratio(0.85);
    assert_eq!(response.total_used_area_ratio(), 0.85);
    
    // Test total number of cuts
    response.set_total_nbr_cuts(42);
    assert_eq!(response.total_nbr_cuts(), 42);
    
    // Test total cut length
    response.set_total_cut_length(123.45);
    assert_eq!(response.total_cut_length(), 123.45);
}

#[test]
fn test_panels_operations() {
    let mut response = CalculationResponse::new();
    
    let panel1 = FinalTile {
        request_obj_id: 1,
        width: 100.0,
        height: 50.0,
        label: Some("Panel 1".to_string()),
        count: 2,
    };
    
    let panel2 = FinalTile {
        request_obj_id: 2,
        width: 80.0,
        height: 60.0,
        label: Some("Panel 2".to_string()),
        count: 1,
    };
    
    let panels = vec![panel1.clone(), panel2.clone()];
    response.set_panels(panels);
    
    assert!(response.panels().is_some());
    assert_eq!(response.panels().unwrap().len(), 2);
    assert_eq!(response.panels().unwrap()[0], panel1);
    assert_eq!(response.panels().unwrap()[1], panel2);
}

#[test]
fn test_used_stock_panels_operations() {
    let mut response = CalculationResponse::new();
    
    let stock_panel = FinalTile {
        request_obj_id: 100,
        width: 200.0,
        height: 100.0,
        label: Some("Stock Panel".to_string()),
        count: 1,
    };
    
    let stock_panels = vec![stock_panel.clone()];
    response.set_used_stock_panels(stock_panels);
    
    assert!(response.used_stock_panels().is_some());
    assert_eq!(response.used_stock_panels().unwrap().len(), 1);
    assert_eq!(response.used_stock_panels().unwrap()[0], stock_panel);
}

#[test]
fn test_edge_bands_operations() {
    let mut response = CalculationResponse::new();
    
    let mut edge_bands = HashMap::new();
    edge_bands.insert("PVC".to_string(), 15.5);
    edge_bands.insert("ABS".to_string(), 8.2);
    
    response.set_edge_bands(edge_bands.clone());
    
    assert!(response.edge_bands().is_some());
    let stored_bands = response.edge_bands().unwrap();
    assert_eq!(stored_bands.get("PVC"), Some(&15.5));
    assert_eq!(stored_bands.get("ABS"), Some(&8.2));
}

#[test]
fn test_no_fit_panels_operations() {
    let mut response = CalculationResponse::new();
    
    let no_fit_panel1 = NoFitTile {
        id: 1,
        width: 30.0,
        height: 40.0,
        count: 1,
        label: Some("No Fit 1".to_string()),
        material: Some("Wood".to_string()),
    };
    
    let no_fit_panel2 = NoFitTile {
        id: 2,
        width: 25.0,
        height: 35.0,
        count: 2,
        label: Some("No Fit 2".to_string()),
        material: Some("MDF".to_string()),
    };
    
    // Test adding individual panels
    response.add_no_fit_panel(no_fit_panel1.clone());
    response.add_no_fit_panel(no_fit_panel2.clone());
    
    assert_eq!(response.no_fit_panels().len(), 2);
    assert_eq!(response.no_fit_panels()[0], no_fit_panel1);
    assert_eq!(response.no_fit_panels()[1], no_fit_panel2);
    
    // Test setting entire list
    let no_fit_panels = vec![no_fit_panel1.clone()];
    response.set_no_fit_panels(no_fit_panels);
    assert_eq!(response.no_fit_panels().len(), 1);
    assert_eq!(response.no_fit_panels()[0], no_fit_panel1);
    
    // Test clearing
    response.clear_no_fit_panels();
    assert!(response.no_fit_panels().is_empty());
}

#[test]
fn test_mosaics_operations() {
    let mut response = CalculationResponse::new();
    
    let mosaic1 = Mosaic::default();
    let mosaic2 = Mosaic::default();
    
    // Test adding individual mosaics
    response.add_mosaic(mosaic1.clone());
    response.add_mosaic(mosaic2.clone());
    
    assert_eq!(response.mosaics().len(), 2);
    
    // Test setting entire list
    let mosaics = vec![mosaic1.clone()];
    response.set_mosaics(mosaics);
    assert_eq!(response.mosaics().len(), 1);
    
    // Test clearing
    response.clear_mosaics();
    assert!(response.mosaics().is_empty());
}

#[test]
fn test_calculation_request_operations() {
    let mut response = CalculationResponse::new();
    
    let request = CalculationRequest::new();
    response.set_request(request.clone());
    
    assert!(response.request().is_some());
    // Note: CalculationRequest doesn't implement PartialEq, so we just check it's present
    let stored_request = response.request().unwrap();
    assert_eq!(stored_request.panels.len(), 0);
    assert_eq!(stored_request.stock_panels.len(), 0);
    assert!(stored_request.configuration.is_none());
}

#[test]
fn test_mutable_references() {
    let mut response = CalculationResponse::new();
    
    // Test mutable reference to panels
    let panels_mut = response.panels_mut();
    *panels_mut = Some(vec![FinalTile::default()]);
    assert!(response.panels().is_some());
    assert_eq!(response.panels().unwrap().len(), 1);
    
    // Test mutable reference to used stock panels
    let stock_panels_mut = response.used_stock_panels_mut();
    *stock_panels_mut = Some(vec![FinalTile::default()]);
    assert!(response.used_stock_panels().is_some());
    assert_eq!(response.used_stock_panels().unwrap().len(), 1);
    
    // Test mutable reference to edge bands
    let edge_bands_mut = response.edge_bands_mut();
    let mut bands = HashMap::new();
    bands.insert("Test".to_string(), 10.0);
    *edge_bands_mut = Some(bands);
    assert!(response.edge_bands().is_some());
    assert_eq!(response.edge_bands().unwrap().get("Test"), Some(&10.0));
    
    // Test mutable reference to no fit panels
    let no_fit_mut = response.no_fit_panels_mut();
    no_fit_mut.push(NoFitTile::default());
    assert_eq!(response.no_fit_panels().len(), 1);
    
    // Test mutable reference to mosaics
    let mosaics_mut = response.mosaics_mut();
    mosaics_mut.push(Mosaic::default());
    assert_eq!(response.mosaics().len(), 1);
}

#[test]
fn test_serialization() {
    let response = CalculationResponse::default();
    
    // Test that the struct can be serialized (this will fail if serde derives are missing)
    let serialized = serde_json::to_string(&response);
    assert!(serialized.is_ok());
    
    // Test that it can be deserialized back
    let json = serialized.unwrap();
    let deserialized: Result<CalculationResponse, _> = serde_json::from_str(&json);
    assert!(deserialized.is_ok());
    
    let deserialized_response = deserialized.unwrap();
    assert_eq!(deserialized_response.version(), "1.2");
}

#[test]
fn test_clone() {
    let mut original = CalculationResponse::new();
    original.set_id("test-123".to_string());
    original.set_elapsed_time(1000);
    
    let cloned = original.clone();
    assert_eq!(cloned.id(), Some("test-123"));
    assert_eq!(cloned.elapsed_time(), 1000);
}

#[test]
fn test_debug() {
    let response = CalculationResponse::default();
    let debug_str = format!("{:?}", response);
    assert!(debug_str.contains("CalculationResponse"));
    assert!(debug_str.contains("version"));
}
