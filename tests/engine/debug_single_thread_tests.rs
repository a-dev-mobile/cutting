//! Tests for the single-threaded debug module
//!
//! These tests verify that the debug version of the algorithm works correctly
//! and provides detailed information about the computation process.

use cutlist_optimizer_cli::{
    engine::service::computation::debug_single_thread::{
        debug_compute_complete, create_debug_test_case, DebugConfig, DebugResult
    },
    models::{
        calculation_request::structs::CalculationRequest,
        configuration::structs::Configuration,
        panel::structs::Panel,
    },
};

#[test]
fn test_debug_compute_complete_basic() {
    // Create a simple test case
    let request = create_debug_test_case();
    let debug_config = DebugConfig::default();
    
    // Run the debug computation
    let result = debug_compute_complete(request, debug_config);
    
    // Verify the result
    assert!(result.is_ok(), "Debug computation should succeed");
    let debug_result = result.unwrap();
    
    // Check basic success
    assert!(debug_result.success, "Computation should be successful");
    assert!(debug_result.error_message.is_none(), "Should not have error message");
    
    // Check that tiles were processed
    assert!(debug_result.tiles_processed > 0, "Should process some tiles");
    assert!(debug_result.stock_tiles_processed > 0, "Should process some stock tiles");
    
    // Check that materials were found
    assert!(!debug_result.materials_found.is_empty(), "Should find some materials");
    
    // Check that computation steps were recorded
    assert!(!debug_result.computation_steps.is_empty(), "Should record computation steps");
    
    // Print summary for manual inspection
    println!("=== TEST RESULT SUMMARY ===");
    debug_result.print_summary();
}

#[test]
fn test_debug_compute_with_verbose_logging() {
    let request = create_debug_test_case();
    let debug_config = DebugConfig {
        max_permutations: 5,
        max_stock_iterations: 3,
        verbose_logging: true,
        step_by_step: false,
        print_intermediate_results: true,
    };
    
    let result = debug_compute_complete(request, debug_config);
    assert!(result.is_ok());
    
    let debug_result = result.unwrap();
    assert!(debug_result.success);
    
    // With verbose logging, we should have detailed steps
    assert!(debug_result.computation_steps.len() > 10, 
            "Verbose mode should record many steps");
}

#[test]
fn test_debug_compute_with_limited_permutations() {
    let request = create_debug_test_case();
    let debug_config = DebugConfig {
        max_permutations: 2,  // Very limited
        max_stock_iterations: 2,
        verbose_logging: false,
        step_by_step: false,
        print_intermediate_results: false,
    };
    
    let result = debug_compute_complete(request, debug_config);
    assert!(result.is_ok());
    
    let debug_result = result.unwrap();
    assert!(debug_result.success);
    
    // Should respect the permutation limit
    assert!(debug_result.permutations_processed <= 2, 
            "Should not process more than max_permutations");
}

#[test]
fn test_debug_compute_empty_panels() {
    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels: vec![],  // Empty panels
        stock_panels: vec![
            Panel {
                id: 101,
                width: Some("200.0".to_string()),
                height: Some("100.0".to_string()),
                count: 1,
                material: "Wood".to_string(),
                enabled: true,
                orientation: 0,
                label: Some("Stock".to_string()),
                edge: None,
            },
        ],
    };
    
    let debug_config = DebugConfig::default();
    let result = debug_compute_complete(request, debug_config);
    
    assert!(result.is_ok());
    let debug_result = result.unwrap();
    
    // Should fail gracefully with empty panels
    assert!(!debug_result.success);
    assert!(debug_result.error_message.is_some());
    assert!(debug_result.error_message.unwrap().contains("No panels provided"));
}

#[test]
fn test_debug_compute_empty_stock() {
    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels: vec![
            Panel {
                id: 1,
                width: Some("100.0".to_string()),
                height: Some("50.0".to_string()),
                count: 1,
                material: "Wood".to_string(),
                enabled: true,
                orientation: 0,
                label: Some("Panel".to_string()),
                edge: None,
            },
        ],
        stock_panels: vec![],  // Empty stock
    };
    
    let debug_config = DebugConfig::default();
    let result = debug_compute_complete(request, debug_config);
    
    assert!(result.is_ok());
    let debug_result = result.unwrap();
    
    // Should fail gracefully with empty stock
    assert!(!debug_result.success);
    assert!(debug_result.error_message.is_some());
    assert!(debug_result.error_message.unwrap().contains("No stock panels provided"));
}

#[test]
fn test_debug_compute_mixed_materials() {
    let panels = vec![
        Panel {
            id: 1,
            width: Some("100.0".to_string()),
            height: Some("50.0".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Wood Panel".to_string()),
            edge: None,
        },
        Panel {
            id: 2,
            width: Some("80.0".to_string()),
            height: Some("40.0".to_string()),
            count: 1,
            material: "Metal".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Metal Panel".to_string()),
            edge: None,
        },
        Panel {
            id: 3,
            width: Some("60.0".to_string()),
            height: Some("30.0".to_string()),
            count: 1,
            material: "Plastic".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Plastic Panel".to_string()),
            edge: None,
        },
    ];

    let stock_panels = vec![
        Panel {
            id: 101,
            width: Some("200.0".to_string()),
            height: Some("100.0".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Wood Stock".to_string()),
            edge: None,
        },
        Panel {
            id: 102,
            width: Some("150.0".to_string()),
            height: Some("80.0".to_string()),
            count: 1,
            material: "Metal".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Metal Stock".to_string()),
            edge: None,
        },
        // Note: No plastic stock - should be handled gracefully
    ];

    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels,
        stock_panels,
    };
    
    let debug_config = DebugConfig::default();
    let result = debug_compute_complete(request, debug_config);
    
    assert!(result.is_ok());
    let debug_result = result.unwrap();
    
    // Should succeed even with missing material stock
    assert!(debug_result.success);
    
    // Should find multiple materials
    assert!(debug_result.materials_found.len() >= 2, 
            "Should find at least Wood and Metal materials");
    
    // Should contain Wood and Metal
    assert!(debug_result.materials_found.contains(&"Wood".to_string()));
    assert!(debug_result.materials_found.contains(&"Metal".to_string()));
    
    // May or may not contain Plastic (depends on implementation)
    println!("Materials found: {:?}", debug_result.materials_found);
    debug_result.print_summary();
}

#[test]
fn test_debug_result_step_tracking() {
    let mut debug_result = DebugResult::new();
    
    // Test adding steps
    debug_result.add_step("Step 1".to_string());
    debug_result.add_step("Step 2".to_string());
    debug_result.add_step("Step 3".to_string());
    
    assert_eq!(debug_result.computation_steps.len(), 3);
    assert_eq!(debug_result.computation_steps[0], "Step 1");
    assert_eq!(debug_result.computation_steps[1], "Step 2");
    assert_eq!(debug_result.computation_steps[2], "Step 3");
    
    // Test step limit (should not exceed 100)
    for i in 4..=105 {
        debug_result.add_step(format!("Step {}", i));
    }
    
    assert_eq!(debug_result.computation_steps.len(), 100, 
               "Should not exceed 100 steps");
}

#[test]
fn test_debug_config_defaults() {
    let config = DebugConfig::default();
    
    assert_eq!(config.max_permutations, 10);
    assert_eq!(config.max_stock_iterations, 5);
    assert!(config.verbose_logging);
    assert!(!config.step_by_step);
    assert!(config.print_intermediate_results);
}

#[test]
fn test_create_debug_test_case() {
    let request = create_debug_test_case();
    
    // Verify the test case structure
    assert!(!request.panels.is_empty(), "Should have panels");
    assert!(!request.stock_panels.is_empty(), "Should have stock panels");
    assert!(request.configuration.is_some(), "Should have configuration");
    
    // Check panel details
    assert_eq!(request.panels.len(), 3, "Should have 3 panels");
    assert_eq!(request.stock_panels.len(), 2, "Should have 2 stock panels");
    
    // Check materials
    let wood_panels = request.panels.iter()
        .filter(|p| p.material == "Wood")
        .count();
    let metal_panels = request.panels.iter()
        .filter(|p| p.material == "Metal")
        .count();
    
    assert_eq!(wood_panels, 2, "Should have 2 wood panels");
    assert_eq!(metal_panels, 1, "Should have 1 metal panel");
    
    // Check stock materials
    let wood_stock = request.stock_panels.iter()
        .filter(|p| p.material == "Wood")
        .count();
    let metal_stock = request.stock_panels.iter()
        .filter(|p| p.material == "Metal")
        .count();
    
    assert_eq!(wood_stock, 1, "Should have 1 wood stock");
    assert_eq!(metal_stock, 1, "Should have 1 metal stock");
}

#[test]
fn test_debug_compute_scaling_factor() {
    let request = create_debug_test_case();
    let debug_config = DebugConfig::default();
    
    let result = debug_compute_complete(request, debug_config);
    assert!(result.is_ok());
    
    let debug_result = result.unwrap();
    assert!(debug_result.success);
    
    // Scaling factor should be reasonable (typically 10^n where n is decimal places)
    assert!(debug_result.scaling_factor > 0.0, "Scaling factor should be positive");
    assert!(debug_result.scaling_factor >= 1.0, "Scaling factor should be at least 1.0");
    
    println!("Scaling factor: {}", debug_result.scaling_factor);
}

// /// Integration test that runs the complete debug pipeline
// #[test]
// fn test_debug_integration_complete_pipeline() {
//     println!("\n=== RUNNING COMPLETE PIPELINE INTEGRATION TEST ===");
    
//     let request = create_debug_test_case();
//     let debug_config = DebugConfig {
//         max_permutations: 8,
//         max_stock_iterations: 4,
//         verbose_logging: true,
//         step_by_step: false,
//         print_intermediate_results: true,
//     };
    
//     let result = debug_compute_complete(request, debug_config);
    
//     assert!(result.is_ok(), "Integration test should succeed");
//     let debug_result = result.unwrap();
    
//     // Comprehensive checks
//     assert!(debug_result.success, "Pipeline should complete successfully");
//     assert!(debug_result.tiles_processed > 0, "Should process tiles");
//     assert!(debug_result.stock_tiles_processed > 0, "Should process stock tiles");
//     assert!(debug_result.scaling_factor > 0.0, "Should calculate scaling factor");
//     assert!(!debug_result.materials_found.is_empty(), "Should find materials");
//     assert!(debug_result.groups_created > 0, "Should create groups");
//     assert!(debug_result.permutations_generated > 0, "Should generate permutations");
//     assert!(!debug_result.computation_steps.is_empty(), "Should record steps");
    
//     // Print detailed results
//     debug_result.print_summary();
    
//     println!("=== INTEGRATION TEST COMPLETED SUCCESSFULLY ===\n");
// }
