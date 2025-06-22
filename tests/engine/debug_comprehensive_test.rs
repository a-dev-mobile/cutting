//! Comprehensive debug tests for single-threaded algorithm verification
//!
//! This module contains advanced tests to thoroughly verify the correctness
//! of the single-threaded cutting optimization algorithm.

use cutlist_optimizer_cli::{
    engine::service::computation::debug_single_thread::{
        debug_compute_complete, DebugConfig
    },
    models::{
        calculation_request::structs::CalculationRequest,
        configuration::structs::Configuration,
        panel::structs::Panel,
    },
};

/// Test with high precision decimal values
#[test]
fn test_debug_high_precision_decimals() {
    println!("\n=== TESTING HIGH PRECISION DECIMALS ===");
    
    let panels = vec![
        Panel {
            id: 1,
            width: Some("123.456".to_string()),
            height: Some("78.123".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("High Precision Panel".to_string()),
            edge: None,
        },
        Panel {
            id: 2,
            width: Some("99.999".to_string()),
            height: Some("55.555".to_string()),
            count: 2,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Precision Panel 2".to_string()),
            edge: None,
        },
    ];

    let stock_panels = vec![
        Panel {
            id: 101,
            width: Some("250.000".to_string()),
            height: Some("150.000".to_string()),
            count: 2,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Precision Stock".to_string()),
            edge: None,
        },
    ];

    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels,
        stock_panels,
    };
    
    let debug_config = DebugConfig {
        max_permutations: 15,
        max_stock_iterations: 5,
        verbose_logging: true,
        step_by_step: false,
        print_intermediate_results: true,
    };
    
    let result = debug_compute_complete(request, debug_config);
    assert!(result.is_ok(), "High precision test should succeed");
    
    let debug_result = result.unwrap();
    assert!(debug_result.success, "Computation should be successful");
    
    // Check scaling factor handles decimals correctly
    assert!(debug_result.scaling_factor >= 100.0, "Should scale for 3 decimal places");
    
    println!("High precision test completed successfully");
    debug_result.print_summary();
}

/// Test with large number of panels
#[test]
fn test_debug_large_panel_count() {
    println!("\n=== TESTING LARGE PANEL COUNT ===");
    
    let mut panels = Vec::new();
    
    // Create 20 panels of varying sizes
    for i in 1..=20 {
        panels.push(Panel {
            id: i,
            width: Some(format!("{}.0", 50 + i * 5)),
            height: Some(format!("{}.0", 30 + i * 3)),
            count: 1,
            material: if i % 3 == 0 { "Metal".to_string() } else { "Wood".to_string() },
            enabled: true,
            orientation: 0,
            label: Some(format!("Panel {}", i)),
            edge: None,
        });
    }

    let stock_panels = vec![
        Panel {
            id: 101,
            width: Some("300.0".to_string()),
            height: Some("200.0".to_string()),
            count: 5,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Wood Stock".to_string()),
            edge: None,
        },
        Panel {
            id: 102,
            width: Some("250.0".to_string()),
            height: Some("180.0".to_string()),
            count: 3,
            material: "Metal".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Metal Stock".to_string()),
            edge: None,
        },
    ];

    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels,
        stock_panels,
    };
    
    let debug_config = DebugConfig {
        max_permutations: 8,  // Limited to keep test reasonable
        max_stock_iterations: 4,
        verbose_logging: false,  // Reduce output for large test
        step_by_step: false,
        print_intermediate_results: false,
    };
    
    let result = debug_compute_complete(request, debug_config);
    assert!(result.is_ok(), "Large panel count test should succeed");
    
    let debug_result = result.unwrap();
    assert!(debug_result.success, "Computation should be successful");
    
    // Should process all panels
    assert_eq!(debug_result.tiles_processed, 20, "Should process all 20 panels");
    
    // Should find both materials
    assert!(debug_result.materials_found.contains(&"Wood".to_string()));
    assert!(debug_result.materials_found.contains(&"Metal".to_string()));
    
    println!("Large panel count test completed successfully");
    println!("Processed {} tiles, found {} materials", 
             debug_result.tiles_processed, debug_result.materials_found.len());
}

/// Test with edge cases and boundary conditions
#[test]
fn test_debug_edge_cases() {
    println!("\n=== TESTING EDGE CASES ===");
    
    let panels = vec![
        // Very small panel
        Panel {
            id: 1,
            width: Some("1.0".to_string()),
            height: Some("1.0".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Tiny Panel".to_string()),
            edge: None,
        },
        // Large panel (but reasonable size)
        Panel {
            id: 2,
            width: Some("500.0".to_string()),
            height: Some("300.0".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Large Panel".to_string()),
            edge: None,
        },
        // Square panel
        Panel {
            id: 3,
            width: Some("100.0".to_string()),
            height: Some("100.0".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Square Panel".to_string()),
            edge: None,
        },
        // Very thin panel
        Panel {
            id: 4,
            width: Some("1000.0".to_string()),
            height: Some("1.0".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Thin Panel".to_string()),
            edge: None,
        },
    ];

    let stock_panels = vec![
        Panel {
            id: 101,
            width: Some("1200.0".to_string()),
            height: Some("800.0".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Large Stock".to_string()),
            edge: None,
        },
    ];

    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels,
        stock_panels,
    };
    
    let debug_config = DebugConfig {
        max_permutations: 10,
        max_stock_iterations: 3,
        verbose_logging: true,
        step_by_step: false,
        print_intermediate_results: true,
    };
    
    let result = debug_compute_complete(request, debug_config);
    assert!(result.is_ok(), "Edge cases test should succeed");
    
    let debug_result = result.unwrap();
    assert!(debug_result.success, "Computation should be successful");
    
    // Should handle all edge case panels
    assert_eq!(debug_result.tiles_processed, 4, "Should process all 4 edge case panels");
    
    println!("Edge cases test completed successfully");
    debug_result.print_summary();
}

/// Test algorithm consistency - run same input multiple times
#[test]
fn test_debug_algorithm_consistency() {
    println!("\n=== TESTING ALGORITHM CONSISTENCY ===");
    
    let panels = vec![
        Panel {
            id: 1,
            width: Some("100.0".to_string()),
            height: Some("50.0".to_string()),
            count: 3,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Test Panel".to_string()),
            edge: None,
        },
    ];

    let stock_panels = vec![
        Panel {
            id: 101,
            width: Some("200.0".to_string()),
            height: Some("100.0".to_string()),
            count: 2,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Test Stock".to_string()),
            edge: None,
        },
    ];

    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels,
        stock_panels,
    };
    
    let debug_config = DebugConfig {
        max_permutations: 5,
        max_stock_iterations: 3,
        verbose_logging: false,
        step_by_step: false,
        print_intermediate_results: false,
    };
    
    // Run the same computation 3 times
    let mut results = Vec::new();
    for i in 1..=3 {
        println!("Running consistency test iteration {}", i);
        let result = debug_compute_complete(request.clone(), debug_config.clone());
        assert!(result.is_ok(), "Consistency test iteration {} should succeed", i);
        
        let debug_result = result.unwrap();
        assert!(debug_result.success, "Iteration {} should be successful", i);
        results.push(debug_result);
    }
    
    // Verify consistency across runs
    let first_result = &results[0];
    for (i, result) in results.iter().enumerate().skip(1) {
        assert_eq!(result.tiles_processed, first_result.tiles_processed, 
                   "Iteration {} should process same number of tiles", i + 1);
        assert_eq!(result.stock_tiles_processed, first_result.stock_tiles_processed, 
                   "Iteration {} should process same number of stock tiles", i + 1);
        assert_eq!(result.scaling_factor, first_result.scaling_factor, 
                   "Iteration {} should have same scaling factor", i + 1);
        assert_eq!(result.materials_found, first_result.materials_found, 
                   "Iteration {} should find same materials", i + 1);
    }
    
    println!("Algorithm consistency test completed successfully");
    println!("All {} iterations produced consistent results", results.len());
}

/// Test performance with time measurement
#[test]
fn test_debug_performance_measurement() {
    println!("\n=== TESTING PERFORMANCE MEASUREMENT ===");
    
    let panels = vec![
        Panel {
            id: 1,
            width: Some("150.0".to_string()),
            height: Some("75.0".to_string()),
            count: 5,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Performance Test Panel".to_string()),
            edge: None,
        },
        Panel {
            id: 2,
            width: Some("120.0".to_string()),
            height: Some("60.0".to_string()),
            count: 3,
            material: "Metal".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Metal Panel".to_string()),
            edge: None,
        },
    ];

    let stock_panels = vec![
        Panel {
            id: 101,
            width: Some("300.0".to_string()),
            height: Some("200.0".to_string()),
            count: 3,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Wood Stock".to_string()),
            edge: None,
        },
        Panel {
            id: 102,
            width: Some("250.0".to_string()),
            height: Some("150.0".to_string()),
            count: 2,
            material: "Metal".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Metal Stock".to_string()),
            edge: None,
        },
    ];

    let request = CalculationRequest {
        configuration: Some(Configuration::default()),
        panels,
        stock_panels,
    };
    
    let debug_config = DebugConfig {
        max_permutations: 12,
        max_stock_iterations: 6,
        verbose_logging: false,
        step_by_step: false,
        print_intermediate_results: false,
    };
    
    // Measure execution time
    let start_time = std::time::Instant::now();
    let result = debug_compute_complete(request, debug_config);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Performance test should succeed");
    
    let debug_result = result.unwrap();
    assert!(debug_result.success, "Computation should be successful");
    
    println!("Performance test completed in {:?}", duration);
    println!("Processed {} tiles and {} stock tiles", 
             debug_result.tiles_processed, debug_result.stock_tiles_processed);
    println!("Generated {} computation steps", debug_result.computation_steps.len());
    
    // Performance should be reasonable (less than 1 second for this test)
    assert!(duration.as_secs() < 1, "Algorithm should complete within 1 second");
    
    println!("Performance test completed successfully");
}
