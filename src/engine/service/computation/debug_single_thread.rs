//! Debug module for single-threaded algorithm verification
//!
//! This module provides a simplified, single-threaded version of the cutting optimization
//! algorithm for debugging and verification purposes. It removes all async/threading
//! complexity to make the algorithm behavior more predictable and easier to trace.

use std::collections::HashMap;
use crate::{
    errors::Result,
    models::{
        tile_dimensions::structs::TileDimensions,
        grouped_tile_dimensions::structs::GroupedTileDimensions,
        configuration::structs::Configuration,
        solution::structs::Solution,
        panel::structs::Panel,
        calculation_request::structs::CalculationRequest,
    },
    utils::arrangement,
    engine::service::computation::{
        dimension_utils::DimensionUtils,
        grouping::CollectionUtils,
    },
};
use tracing::{debug, info, warn};

/// Debug configuration for single-threaded execution
#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub max_permutations: usize,
    pub max_stock_iterations: usize,
    pub verbose_logging: bool,
    pub step_by_step: bool,
    pub print_intermediate_results: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            max_permutations: 10,  // Reduced for debugging
            max_stock_iterations: 5,  // Reduced for debugging
            verbose_logging: true,
            step_by_step: false,
            print_intermediate_results: true,
        }
    }
}

/// Debug result containing detailed information about the computation
#[derive(Debug, Clone)]
pub struct DebugResult {
    pub success: bool,
    pub error_message: Option<String>,
    pub tiles_processed: usize,
    pub stock_tiles_processed: usize,
    pub permutations_generated: usize,
    pub permutations_processed: usize,
    pub groups_created: usize,
    pub distinct_groups: usize,
    pub scaling_factor: f64,
    pub materials_found: Vec<String>,
    pub computation_steps: Vec<String>,
    pub solutions_found: usize,
    pub best_solution: Option<Solution>,
}

impl DebugResult {
    pub fn new() -> Self {
        Self {
            success: false,
            error_message: None,
            tiles_processed: 0,
            stock_tiles_processed: 0,
            permutations_generated: 0,
            permutations_processed: 0,
            groups_created: 0,
            distinct_groups: 0,
            scaling_factor: 1.0,
            materials_found: Vec::new(),
            computation_steps: Vec::new(),
            solutions_found: 0,
            best_solution: None,
        }
    }

    pub fn add_step(&mut self, step: String) {
        if self.computation_steps.len() < 100 {  // Limit to prevent memory issues
            self.computation_steps.push(step);
        }
    }

    pub fn print_summary(&self) {
        println!("\n=== DEBUG COMPUTATION SUMMARY ===");
        println!("Success: {}", self.success);
        if let Some(ref error) = self.error_message {
            println!("Error: {}", error);
        }
        println!("Tiles processed: {}", self.tiles_processed);
        println!("Stock tiles processed: {}", self.stock_tiles_processed);
        println!("Scaling factor: {}", self.scaling_factor);
        println!("Materials found: {:?}", self.materials_found);
        println!("Groups created: {}", self.groups_created);
        println!("Distinct groups: {}", self.distinct_groups);
        println!("Permutations generated: {}", self.permutations_generated);
        println!("Permutations processed: {}", self.permutations_processed);
        println!("Solutions found: {}", self.solutions_found);
        
        if self.computation_steps.len() > 0 {
            println!("\n=== COMPUTATION STEPS ===");
            for (i, step) in self.computation_steps.iter().enumerate() {
                println!("{:3}: {}", i + 1, step);
            }
        }
        
        if let Some(ref solution) = self.best_solution {
            println!("\n=== BEST SOLUTION ===");
            println!("Solution details: {:?}", solution);
        }
        
        println!("=== END SUMMARY ===\n");
    }
}

/// Single-threaded debug version of the complete computation pipeline
pub fn debug_compute_complete(
    request: CalculationRequest,
    debug_config: DebugConfig,
) -> Result<DebugResult> {
    let mut result = DebugResult::new();
    result.add_step("Starting debug computation".to_string());

    if debug_config.verbose_logging {
        info!("=== STARTING SINGLE-THREADED DEBUG COMPUTATION ===");
        info!("Debug config: {:?}", debug_config);
    }

    // Step 1: Input validation
    result.add_step("Validating input".to_string());
    if request.panels.is_empty() {
        result.error_message = Some("No panels provided".to_string());
        return Ok(result);
    }
    if request.stock_panels.is_empty() {
        result.error_message = Some("No stock panels provided".to_string());
        return Ok(result);
    }

    if debug_config.verbose_logging {
        info!("Input validation passed: {} panels, {} stock panels", 
              request.panels.len(), request.stock_panels.len());
    }

    // Step 2: Calculate scaling factor
    result.add_step("Calculating scaling factor".to_string());
    let scaling_factor = calculate_scaling_factor(&request)?;
    result.scaling_factor = scaling_factor;
    result.add_step(format!("Scaling factor calculated: {}", scaling_factor));

    if debug_config.verbose_logging {
        info!("Scaling factor calculated: {}", scaling_factor);
    }

    // Step 3: Convert panels to tiles
    result.add_step("Converting panels to tiles".to_string());
    let (tiles, stock_tiles) = convert_panels_to_tiles(&request, scaling_factor, &mut result)?;
    result.tiles_processed = tiles.len();
    result.stock_tiles_processed = stock_tiles.len();

    if debug_config.verbose_logging {
        info!("Converted to {} tiles and {} stock tiles", tiles.len(), stock_tiles.len());
        if debug_config.print_intermediate_results {
            print_tiles_summary(&tiles, "Regular tiles");
            print_tiles_summary(&stock_tiles, "Stock tiles");
        }
    }

    // Step 4: Group by materials
    result.add_step("Grouping by materials".to_string());
    let (tiles_per_material, stock_per_material) = group_by_materials(&tiles, &stock_tiles);
    result.materials_found = tiles_per_material.keys().cloned().collect();
    result.add_step(format!("Grouped tiles into {} materials", tiles_per_material.len()));

    if debug_config.verbose_logging {
        info!("Found {} materials: {:?}", result.materials_found.len(), result.materials_found);
        for (material, material_tiles) in &tiles_per_material {
            info!("Material '{}': {} tiles", material, material_tiles.len());
        }
    }

    // Step 5: Process each material
    result.add_step("Processing materials".to_string());
    let configuration = request.configuration.as_ref()
        .ok_or_else(|| crate::errors::CoreError::InvalidInput { 
            details: "No configuration provided".to_string() 
        })?;

    // Clone materials_found to avoid borrowing issues
    let materials_to_process = result.materials_found.clone();
    for material in &materials_to_process {
        if let (Some(material_tiles), Some(material_stock)) = (
            tiles_per_material.get(material),
            stock_per_material.get(material)
        ) {
            result.add_step(format!("Processing material: {}", material));
            
            if debug_config.verbose_logging {
                info!("=== PROCESSING MATERIAL: {} ===", material);
            }

            let material_result = debug_compute_material(
                material_tiles.clone(),
                material_stock.clone(),
                configuration,
                material,
                &debug_config,
            )?;

            result.solutions_found += material_result.solutions_found;
            if material_result.best_solution.is_some() && result.best_solution.is_none() {
                result.best_solution = material_result.best_solution;
            }

            // Merge computation steps
            for step in material_result.computation_steps {
                result.add_step(format!("[{}] {}", material, step));
            }

            if debug_config.step_by_step {
                println!("Press Enter to continue to next material...");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
            }
        } else {
            warn!("Material '{}' has tiles but no stock or vice versa", material);
            result.add_step(format!("Skipped material '{}' - missing tiles or stock", material));
        }
    }

    result.success = true;
    result.add_step("Computation completed successfully".to_string());

    if debug_config.verbose_logging {
        info!("=== DEBUG COMPUTATION COMPLETED ===");
        result.print_summary();
    }

    Ok(result)
}

/// Single-threaded debug version of material computation
pub fn debug_compute_material(
    tiles: Vec<TileDimensions>,
    stock_tiles: Vec<TileDimensions>,
    configuration: &Configuration,
    material: &str,
    debug_config: &DebugConfig,
) -> Result<DebugResult> {
    let mut result = DebugResult::new();
    result.tiles_processed = tiles.len();
    result.stock_tiles_processed = stock_tiles.len();

    if debug_config.verbose_logging {
        info!("Computing material: {} with {} tiles and {} stock tiles", 
              material, tiles.len(), stock_tiles.len());
    }

    // Step 1: Generate groups
    result.add_step("Generating groups".to_string());
    let grouped_tiles = debug_generate_groups(&tiles, &stock_tiles)?;
    result.groups_created = grouped_tiles.len();

    if debug_config.verbose_logging {
        info!("Generated {} groups", grouped_tiles.len());
        if debug_config.print_intermediate_results {
            print_groups_summary(&grouped_tiles);
        }
    }

    // Step 2: Get distinct groups
    result.add_step("Getting distinct groups".to_string());
    let distinct_groups = CollectionUtils::get_distinct_grouped_tile_dimensions(&grouped_tiles)?;
    result.distinct_groups = distinct_groups.len();

    if debug_config.verbose_logging {
        info!("Found {} distinct groups", distinct_groups.len());
    }

    // Step 3: Generate permutations
    result.add_step("Generating permutations".to_string());
    let permutations = debug_generate_permutations(&distinct_groups, &mut result)?;
    result.permutations_generated = permutations.len();

    let max_permutations = std::cmp::min(permutations.len(), debug_config.max_permutations);
    result.permutations_processed = max_permutations;

    if debug_config.verbose_logging {
        info!("Generated {} permutations, will process {}", 
              permutations.len(), max_permutations);
    }

    // Step 4: Convert permutations to tile lists
    result.add_step("Converting permutations to tile lists".to_string());
    let tile_permutations = debug_convert_permutations_to_tiles(&permutations, &grouped_tiles)?;

    // Step 5: Process permutations
    result.add_step("Processing permutations".to_string());
    for (perm_idx, permutation) in tile_permutations.iter().take(max_permutations).enumerate() {
        if debug_config.verbose_logging {
            info!("Processing permutation {}/{}", perm_idx + 1, max_permutations);
        }

        result.add_step(format!("Processing permutation {}", perm_idx + 1));

        // For debugging, we'll simulate the stock solution processing
        let stock_solutions_processed = debug_process_permutation(
            permutation,
            &stock_tiles,
            configuration,
            perm_idx,
            debug_config,
            &mut result,
        )?;

        if debug_config.verbose_logging {
            info!("Processed {} stock solutions for permutation {}", 
                  stock_solutions_processed, perm_idx + 1);
        }

        if debug_config.step_by_step {
            println!("Press Enter to continue to next permutation...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
    }

    result.success = true;
    Ok(result)
}

/// Calculate scaling factor (simplified version)
fn calculate_scaling_factor(request: &CalculationRequest) -> Result<f64> {
    let panels = &request.panels;
    let stock_panels = &request.stock_panels;
    
    let max_decimal_panels = DimensionUtils::get_max_nbr_decimal_places(panels);
    let max_decimal_stock = DimensionUtils::get_max_nbr_decimal_places(stock_panels);
    
    let mut max_decimal_places = std::cmp::max(max_decimal_panels, max_decimal_stock);
    
    if let Some(config) = &request.configuration {
        let cut_thickness_str = config.cut_thickness.to_string();
        max_decimal_places = std::cmp::max(max_decimal_places, 
            DimensionUtils::get_nbr_decimal_places(&cut_thickness_str));
        
        let min_trim_str = config.min_trim_dimension.to_string();
        max_decimal_places = std::cmp::max(max_decimal_places, 
            DimensionUtils::get_nbr_decimal_places(&min_trim_str));
    }
    
    const MAX_ALLOWED_DIGITS: usize = 6;
    let max_integer_panels = DimensionUtils::get_max_nbr_integer_places(panels);
    let max_integer_stock = DimensionUtils::get_max_nbr_integer_places(stock_panels);
    let max_integer_places = std::cmp::max(max_integer_panels, max_integer_stock);
    
    if max_decimal_places + max_integer_places > MAX_ALLOWED_DIGITS {
        max_decimal_places = MAX_ALLOWED_DIGITS.saturating_sub(max_integer_places);
    }
    
    let scaling_factor = 10.0_f64.powi(max_decimal_places as i32);
    
    Ok(scaling_factor)
}

/// Convert panels to tiles (simplified version)
fn convert_panels_to_tiles(
    request: &CalculationRequest,
    scaling_factor: f64,
    result: &mut DebugResult,
) -> Result<(Vec<TileDimensions>, Vec<TileDimensions>)> {
    let mut tiles = Vec::new();
    let mut stock_tiles = Vec::new();
    
    // Convert regular panels
    for panel in &request.panels {
        if panel.is_valid()? {
            for _ in 0..panel.count {
                let width_str = panel.width.as_ref()
                    .ok_or_else(|| crate::errors::CoreError::InvalidInput { 
                        details: "Panel width is None".to_string() 
                    })?;
                let height_str = panel.height.as_ref()
                    .ok_or_else(|| crate::errors::CoreError::InvalidInput { 
                        details: "Panel height is None".to_string() 
                    })?;
                
                let width_f64 = width_str.parse::<f64>()
                    .map_err(|e| crate::errors::CoreError::ParseFloat(e))?;
                let height_f64 = height_str.parse::<f64>()
                    .map_err(|e| crate::errors::CoreError::ParseFloat(e))?;
                
                let scaled_width = (width_f64 * scaling_factor).round() as i32;
                let scaled_height = (height_f64 * scaling_factor).round() as i32;
                
                let mut tile = TileDimensions::new(panel.id, scaled_width, scaled_height);
                tile.material = panel.material.clone();
                tile.orientation = DimensionUtils::convert_orientation(panel.orientation);
                tile.label = panel.label.clone();
                
                tiles.push(tile);
            }
        }
    }
    
    // Convert stock panels
    for panel in &request.stock_panels {
        if panel.is_valid()? {
            for _ in 0..panel.count {
                let width_str = panel.width.as_ref()
                    .ok_or_else(|| crate::errors::CoreError::InvalidInput { 
                        details: "Stock panel width is None".to_string() 
                    })?;
                let height_str = panel.height.as_ref()
                    .ok_or_else(|| crate::errors::CoreError::InvalidInput { 
                        details: "Stock panel height is None".to_string() 
                    })?;
                
                let width_f64 = width_str.parse::<f64>()
                    .map_err(|e| crate::errors::CoreError::ParseFloat(e))?;
                let height_f64 = height_str.parse::<f64>()
                    .map_err(|e| crate::errors::CoreError::ParseFloat(e))?;
                
                let scaled_width = (width_f64 * scaling_factor).round() as i32;
                let scaled_height = (height_f64 * scaling_factor).round() as i32;
                
                let mut tile = TileDimensions::new(panel.id, scaled_width, scaled_height);
                tile.material = panel.material.clone();
                tile.orientation = DimensionUtils::convert_orientation(panel.orientation);
                tile.label = panel.label.clone();
                
                stock_tiles.push(tile);
            }
        }
    }
    
    result.add_step(format!("Converted {} panels to {} tiles", request.panels.len(), tiles.len()));
    result.add_step(format!("Converted {} stock panels to {} stock tiles", 
                           request.stock_panels.len(), stock_tiles.len()));
    
    Ok((tiles, stock_tiles))
}

/// Group tiles by material
fn group_by_materials(
    tiles: &[TileDimensions],
    stock_tiles: &[TileDimensions],
) -> (HashMap<String, Vec<TileDimensions>>, HashMap<String, Vec<TileDimensions>>) {
    let mut tiles_per_material: HashMap<String, Vec<TileDimensions>> = HashMap::new();
    let mut stock_per_material: HashMap<String, Vec<TileDimensions>> = HashMap::new();
    
    for tile in tiles {
        tiles_per_material
            .entry(tile.material.clone())
            .or_insert_with(Vec::new)
            .push(tile.clone());
    }
    
    for tile in stock_tiles {
        stock_per_material
            .entry(tile.material.clone())
            .or_insert_with(Vec::new)
            .push(tile.clone());
    }
    
    (tiles_per_material, stock_per_material)
}

/// Generate groups (simplified version)
fn debug_generate_groups(
    tiles: &[TileDimensions],
    stock_tiles: &[TileDimensions],
) -> Result<Vec<GroupedTileDimensions>> {
    let mut tile_counts: HashMap<String, i32> = HashMap::new();
    for tile in tiles {
        let key = tile.dimensions_string();
        *tile_counts.entry(key).or_insert(0) += 1;
    }
    
    let max_group_size = std::cmp::max(tiles.len() / 100, 1);
    let _is_one_dim = debug_is_one_dimensional_optimization(tiles, stock_tiles)?;
    
    let mut groups = Vec::new();
    let mut current_group = 0;
    
    for tile in tiles {
        groups.push(GroupedTileDimensions {
            tile_dimensions: tile.clone(),
            group: current_group,
        });
        
        // Simple grouping logic for debugging
        if groups.len() % max_group_size == 0 {
            current_group += 1;
        }
    }
    
    Ok(groups)
}

/// Check if one-dimensional optimization
fn debug_is_one_dimensional_optimization(
    tiles: &[TileDimensions],
    stock_tiles: &[TileDimensions],
) -> Result<bool> {
    if tiles.is_empty() || stock_tiles.is_empty() {
        return Ok(false);
    }
    
    let mut common_dimensions = vec![tiles[0].width, tiles[0].height];
    
    for tile in tiles {
        common_dimensions.retain(|&dim| dim == tile.width || dim == tile.height);
        if common_dimensions.is_empty() {
            return Ok(false);
        }
    }
    
    for stock_tile in stock_tiles {
        common_dimensions.retain(|&dim| dim == stock_tile.width || dim == stock_tile.height);
        if common_dimensions.is_empty() {
            return Ok(false);
        }
    }
    
    Ok(!common_dimensions.is_empty())
}

/// Generate permutations (simplified version)
fn debug_generate_permutations(
    distinct_groups: &HashMap<GroupedTileDimensions, i32>,
    result: &mut DebugResult,
) -> Result<Vec<Vec<GroupedTileDimensions>>> {
    let mut groups: Vec<GroupedTileDimensions> = distinct_groups.keys().cloned().collect();
    
    if groups.is_empty() {
        return Ok(vec![]);
    }
    
    // Sort by area (largest first)
    groups.sort_by(|a, b| {
        let area_a = a.tile_dimensions.area();
        let area_b = b.tile_dimensions.area();
        area_b.cmp(&area_a)
    });
    
    result.add_step(format!("Sorted {} groups by area", groups.len()));
    
    // For debugging, limit to first 3 groups for permutation
    let (permutation_groups, fixed_groups) = if groups.len() > 3 {
        let permutation_groups = groups[0..3].to_vec();
        let fixed_groups = groups[3..].to_vec();
        (permutation_groups, fixed_groups)
    } else {
        (groups, Vec::new())
    };
    
    result.add_step(format!("Using {} groups for permutation, {} fixed groups", 
                           permutation_groups.len(), fixed_groups.len()));
    
    let mut permutations = arrangement::generate_permutations(permutation_groups);
    
    // Add fixed groups to each permutation
    for permutation in &mut permutations {
        permutation.extend(fixed_groups.clone());
    }
    
    result.add_step(format!("Generated {} permutations", permutations.len()));
    
    Ok(permutations)
}

/// Convert permutations to tile lists
fn debug_convert_permutations_to_tiles(
    permutations: &[Vec<GroupedTileDimensions>],
    grouped_tiles: &[GroupedTileDimensions],
) -> Result<Vec<Vec<TileDimensions>>> {
    let mut tile_permutations = Vec::new();
    
    for permutation in permutations {
        let mut tiles = Vec::new();
        
        for group in permutation {
            for grouped_tile in grouped_tiles {
                if grouped_tile.group == group.group && 
                   grouped_tile.tile_dimensions.id == group.tile_dimensions.id {
                    tiles.push(grouped_tile.tile_dimensions.clone());
                }
            }
        }
        
        tile_permutations.push(tiles);
    }
    
    Ok(tile_permutations)
}

/// Process a single permutation (simplified for debugging)
fn debug_process_permutation(
    permutation: &[TileDimensions],
    stock_tiles: &[TileDimensions],
    configuration: &Configuration,
    permutation_index: usize,
    debug_config: &DebugConfig,
    result: &mut DebugResult,
) -> Result<usize> {
    let max_stock_iterations = std::cmp::min(stock_tiles.len(), debug_config.max_stock_iterations);
    
    if debug_config.verbose_logging {
        info!("Processing permutation {} with {} tiles against {} stock solutions", 
              permutation_index, permutation.len(), max_stock_iterations);
    }
    
    result.add_step(format!("Processing {} stock solutions", max_stock_iterations));
    
    // For debugging, we'll simulate processing different stock combinations
    for stock_idx in 0..max_stock_iterations {
        if debug_config.verbose_logging {
            debug!("  Stock solution {}: using stock tile {} ({}x{})", 
                   stock_idx, 
                   stock_tiles[stock_idx % stock_tiles.len()].id,
                   stock_tiles[stock_idx % stock_tiles.len()].width,
                   stock_tiles[stock_idx % stock_tiles.len()].height);
        }
        
        // Simulate different cut orientations
        let orientations = ["AREA", "AREA_HCUTS_1ST", "AREA_VCUTS_1ST"];
        for orientation in &orientations {
            if debug_config.verbose_logging {
                debug!("    Testing orientation: {}", orientation);
            }
            
            // Simulate finding a solution (for debugging)
            if stock_idx == 0 && orientation == &"AREA" {
                result.solutions_found += 1;
                result.add_step(format!("Found solution with orientation {}", orientation));
                
                if debug_config.verbose_logging {
                    info!("    Found solution!");
                }
            }
        }
    }
    
    Ok(max_stock_iterations)
}

/// Print summary of tiles for debugging
fn print_tiles_summary(tiles: &[TileDimensions], title: &str) {
    println!("\n=== {} ===", title);
    println!("Total tiles: {}", tiles.len());
    
    let mut material_counts: HashMap<String, usize> = HashMap::new();
    let mut dimension_counts: HashMap<String, usize> = HashMap::new();
    
    for tile in tiles {
        *material_counts.entry(tile.material.clone()).or_insert(0) += 1;
        let dim_key = format!("{}x{}", tile.width, tile.height);
        *dimension_counts.entry(dim_key).or_insert(0) += 1;
    }
    
    println!("Materials: {:?}", material_counts);
    println!("Dimensions: {:?}", dimension_counts);
    
    if tiles.len() <= 10 {
        for (i, tile) in tiles.iter().enumerate() {
            println!("  {}: ID={}, {}x{}, material={}, label={:?}", 
                     i + 1, tile.id, tile.width, tile.height, tile.material, tile.label);
        }
    } else {
        println!("  (showing first 5 tiles)");
        for (i, tile) in tiles.iter().take(5).enumerate() {
            println!("  {}: ID={}, {}x{}, material={}, label={:?}", 
                     i + 1, tile.id, tile.width, tile.height, tile.material, tile.label);
        }
        println!("  ... and {} more", tiles.len() - 5);
    }
}

/// Print summary of groups for debugging
fn print_groups_summary(groups: &[GroupedTileDimensions]) {
    println!("\n=== GROUPS SUMMARY ===");
    println!("Total groups: {}", groups.len());
    
    let mut group_counts: HashMap<i32, usize> = HashMap::new();
    for group in groups {
        *group_counts.entry(group.group).or_insert(0) += 1;
    }
    
    println!("Group distribution: {:?}", group_counts);
    
    if groups.len() <= 10 {
        for (i, group) in groups.iter().enumerate() {
            println!("  {}: Group={}, ID={}, {}x{}, material={}", 
                     i + 1, group.group, group.tile_dimensions.id, 
                     group.tile_dimensions.width, group.tile_dimensions.height, 
                     group.tile_dimensions.material);
        }
    }
}

/// Create a simple test case for debugging
pub fn create_debug_test_case() -> CalculationRequest {
    let panels = vec![
        Panel {
            id: 1,
            width: Some("100.5".to_string()),
            height: Some("50.25".to_string()),
            count: 2,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Panel A".to_string()),
            edge: None,
        },
        Panel {
            id: 2,
            width: Some("75.0".to_string()),
            height: Some("25.5".to_string()),
            count: 1,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Panel B".to_string()),
            edge: None,
        },
        Panel {
            id: 3,
            width: Some("60.0".to_string()),
            height: Some("40.0".to_string()),
            count: 1,
            material: "Metal".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Panel C".to_string()),
            edge: None,
        },
    ];

    let stock_panels = vec![
        Panel {
            id: 101,
            width: Some("200.0".to_string()),
            height: Some("100.0".to_string()),
            count: 3,
            material: "Wood".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Stock Wood".to_string()),
            edge: None,
        },
        Panel {
            id: 102,
            width: Some("150.0".to_string()),
            height: Some("80.0".to_string()),
            count: 2,
            material: "Metal".to_string(),
            enabled: true,
            orientation: 0,
            label: Some("Stock Metal".to_string()),
            edge: None,
        },
    ];

    CalculationRequest {
        configuration: Some(Configuration::default()),
        panels,
        stock_panels,
    }
}
