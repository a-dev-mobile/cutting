//! Material computation operations
//!
//! This module handles the second compute method for processing individual materials
//! and contains the lambda logic from the Java implementation.
//! 
//! Based on Java CutListOptimizerServiceImpl.compute() method (lines 300-500)

use std::{collections::HashMap, sync::Arc};
use crate::{
    errors::Result,
    models::{
        tile_dimensions::structs::TileDimensions,
        grouped_tile_dimensions::structs::GroupedTileDimensions,
        configuration::structs::Configuration,
        task::structs::Task,
        performance_thresholds::structs::PerformanceThresholds,
        solution::structs::Solution,
        enums::status::Status,
    },
    logging::macros::{debug, info, trace, warn, error},
    utils::arrangement,
    engine::stock::{
        stock_panel_picker::StockPanelPicker,
        stock_solution::StockSolution,
    },
};

use super::{
    grouping::CollectionUtils,
};

// Constants from Java implementation
const MAX_PERMUTATION_ITERATIONS: usize = 1000;
const MAX_STOCK_ITERATIONS: usize = 1000;
const MAX_PERMUTATIONS_WITH_SOLUTION: usize = 150;

/// Compute optimization for a specific material
/// 
/// This is the Rust equivalent of the Java compute method for individual materials
/// Based on Java CutListOptimizerServiceImpl.compute() lines 300-500
/// 
/// This implements the complex Java pattern including:
/// - Performance threshold management
/// - Complex permutation generation and processing
/// - Stock solution optimization with multiple iterations
/// - Thread spawning with different cut orientations
/// - Solution ranking and comparison
pub async fn compute_material(
    tiles: Vec<TileDimensions>,
    stock_tiles: Vec<TileDimensions>,
    configuration: &Configuration,
    task_arc: std::sync::Arc<parking_lot::RwLock<Task>>,
    material: &str,
) -> Result<()> {
    
    debug!("Computing material: {} with {} tiles and {} stock tiles", 
           material, tiles.len(), stock_tiles.len());

    // Validate inputs
    if tiles.is_empty() {
        return Err(crate::errors::CoreError::InvalidInput { 
            details: "Tiles array cannot be empty".to_string() 
        }.into());
    }

    if stock_tiles.is_empty() {
        return Err(crate::errors::CoreError::InvalidInput { 
            details: "Stock tiles array cannot be empty".to_string() 
        }.into());
    }

    // Get task ID for logging
    let task_id = {
        let task = task_arc.read();
        task.id.clone()
    };

    // Step 1: Setup performance thresholds (Java PerformanceThresholds setup)
    let performance_thresholds = setup_performance_thresholds(configuration)?;
    
    // Step 2: Get solutions collection for this material (Java: final List<Solution> solutions = task.getSolutions(str))
    let solutions = {
        let task = task_arc.read();
        let solutions_map = task.solutions.lock().unwrap();
        solutions_map.get(material).cloned().unwrap_or_default()
    };

    // Step 3: Generate groups (Java: generateGroups method)
    let grouped_tiles = generate_groups(&tiles, &stock_tiles, &task_arc)?;
    
    // Step 4: Get distinct grouped tile dimensions (Java: getDistinctGroupedTileDimensions)
    let distinct_groups = get_distinct_grouped_tile_dimensions(&grouped_tiles, configuration)?;
    
    // Step 5: Generate permutations (Java: Arrangement.generatePermutations)
    debug!("Task[{}] Calculating permutations...", task_id);
    let mut permutations = generate_complex_permutations(&distinct_groups)?;
    
    // Step 6: Sort tiles according to permutations (Java: groupedTileDimensionsList2TileDimensionsList)
    debug!("Task[{}] Sorting tiles according to permutations...", task_id);
    let tile_permutations = convert_permutations_to_tiles(&permutations, &grouped_tiles)?;
    
    // Step 7: Remove duplicated permutations (Java: removeDuplicatedPermutations)
    debug!("Removing duplicated permutations...");
    let removed_count = remove_duplicated_permutations(&mut permutations);
    debug!("Removed {} duplicated permutations", removed_count);
    
    // Step 8: Set task to running status (Java: task.setRunningStatus())
    {
        let task = task_arc.read();
        let _ = task.set_running_status();
    }
    
    // Step 9: Initialize StockPanelPicker (Java: StockPanelPicker stockPanelPicker = new StockPanelPicker(...))
    // Note: We need to create a new Arc<Task> from the RwLock content for StockPanelPicker
    let task_for_picker = {
        let task = task_arc.read();
        Arc::new(task.clone())
    };
    let stock_panel_picker = StockPanelPicker::new(
        tiles.clone(),
        stock_tiles.clone(),
        task_for_picker,
        if configuration.use_single_stock_unit { Some(1) } else { None }
    )?;
    stock_panel_picker.init().await?;
    
    // Step 10: Calculate optimization factor (Java: optimizationFactor calculation)
    let mut optimization_factor = if configuration.optimization_factor > 0 {
        (100.0 * configuration.optimization_factor as f64) as i32
    } else {
        100
    };
    
    if tiles.len() > 100 {
        optimization_factor = (optimization_factor as f64 * (0.5 / (tiles.len() as f64 / 100.0))) as i32;
        info!("Limiting solution pool elements to [{}]", optimization_factor);
    }
    
    // Step 11: Process permutations (Java: main permutation loop)
    let mut permutation_index = 0;
    let total_permutations = std::cmp::min(permutations.len(), MAX_PERMUTATION_ITERATIONS);
    
    while permutation_index < total_permutations {
        // Check if task is still running (Java: if (!task.isRunning()))
        {
            let task = task_arc.read();
            if !task.is_running() {
                debug!("Task no longer has running status. Stopping permutation spawner at permutationIdx[{}]", permutation_index);
                break;
            }
        }
        
        // Check if we have solution and reached max permutations (Java: task.hasSolutionAllFit() check)
        {
            let task = task_arc.read();
            if task.has_solution_all_fit() && permutation_index > MAX_PERMUTATIONS_WITH_SOLUTION {
                task.set_material_percentage_done(material.to_string(), 100);
                debug!("Task has solution and spawned max permutations threads");
                break;
            }
        }
        
        // Process this permutation (Java: lambda function call)
        if let Some(permutation) = tile_permutations.get(permutation_index) {
            process_permutation_complex(
                &stock_panel_picker,
                permutation_index,
                &task_arc,
                &solutions,
                &tile_permutations,
                configuration,
                permutation,
                optimization_factor,
                &performance_thresholds,
                material,
            ).await?;
        }
        
        permutation_index += 1;
        
        // Update progress
        let progress = (permutation_index as f64 / total_permutations as f64 * 100.0) as i32;
        {
            let task = task_arc.read();
            task.set_material_percentage_done(material.to_string(), progress);
        }
    }
    
    // Step 12: Wait for all threads to complete (Java: while loop waiting for threads)
    wait_for_computation_completion(&task_arc, material).await?;
    
    // Step 13: Mark material as complete (Java: task.setMaterialPercentageDone(str, 100))
    {
        let task = task_arc.read();
        if task.status() == Status::Running {
            task.set_material_percentage_done(material.to_string(), 100);
        }
    }

    info!("Completed material computation for: {} with {} permutations", material, permutation_index);
    Ok(())
}

/// Setup performance thresholds (Java: PerformanceThresholds setup)
fn setup_performance_thresholds(configuration: &Configuration) -> Result<PerformanceThresholds> {
    let mut thresholds = configuration.performance_thresholds.clone();
    
    // Set defaults if not specified
    if thresholds.max_simultaneous_threads == 0 {
        warn!("No performance thresholds specified, using defaults");
        thresholds.max_simultaneous_threads = 5;
    }
    if thresholds.thread_check_interval == 0 {
        thresholds.thread_check_interval = 1000;
    }
    
    Ok(thresholds)
}

/// Generate groups (Java: generateGroups method)
/// 
/// This implements the complex Java generateGroups logic including:
/// - Material grouping and counting
/// - One-dimensional optimization detection
/// - Group splitting for large datasets
fn generate_groups(
    tiles: &[TileDimensions],
    stock_tiles: &[TileDimensions],
    task_arc: &std::sync::Arc<parking_lot::RwLock<Task>>,
) -> Result<Vec<GroupedTileDimensions>> {
    // Step 1: Count tiles by dimensions (Java: HashMap map = new HashMap())
    let mut tile_counts: HashMap<String, i32> = HashMap::new();
    for tile in tiles {
        let key = tile.dimensions_string();
        *tile_counts.entry(key).or_insert(0) += 1;
    }
    
    // Step 2: Log tile information (Java: StringBuilder sb logging)
    let mut log_info = String::new();
    for (key, count) in &tile_counts {
        log_info.push_str(&format!("{}*{} ", key, count));
    }
    
    let task_id = {
        let task = task_arc.read();
        task.id.clone()
    };
    
    trace!("Task[{}] TotalNbrTiles[{}] Tiles: {}", task_id, tiles.len(), log_info);
    
    // Step 3: Calculate group split threshold (Java: int iMax = Math.max(list.size() / 100, 1))
    let mut max_group_size = std::cmp::max(tiles.len() / 100, 1);
    
    // Step 4: Check for one-dimensional optimization (Java: isOneDimensionalOptimization)
    if is_one_dimensional_optimization(tiles, stock_tiles)? {
        info!("Task is one dimensional optimization");
        max_group_size = 1;
    }
    
    // Step 5: Create grouped tiles with splitting logic (Java: main grouping loop)
    let mut groups = Vec::new();
    let mut group_counts: HashMap<String, i32> = HashMap::new();
    let mut current_group = 0;
    
    for tile in tiles {
        let base_key = tile.dimensions_string();
        let group_key = format!("{}{}", base_key, current_group);
        
        *group_counts.entry(group_key.clone()).or_insert(0) += 1;
        
        groups.push(GroupedTileDimensions {
            tile_dimensions: tile.clone(),
            group: current_group,
        });
        
        // Check if we should split this group (Java: group splitting logic)
        if let Some(&total_count) = tile_counts.get(&base_key) {
            if total_count > max_group_size as i32 {
                if let Some(&current_count) = group_counts.get(&group_key) {
                    if current_count > total_count / 4 {
                        debug!("Task[{}] Splitting panel set [{}] with [{}] units into two groups", 
                               task_id, tile.dimensions_string(), total_count);
                        current_group += 1;
                    }
                }
            }
        }
    }
    
    Ok(groups)
}

/// Check if this is one-dimensional optimization (Java: isOneDimensionalOptimization)
fn is_one_dimensional_optimization(
    tiles: &[TileDimensions],
    stock_tiles: &[TileDimensions],
) -> Result<bool> {
    if tiles.is_empty() || stock_tiles.is_empty() {
        return Ok(false);
    }
    
    // Start with dimensions from first tile
    let mut common_dimensions = vec![tiles[0].width, tiles[0].height];
    
    // Check all tiles
    for tile in tiles {
        common_dimensions.retain(|&dim| dim == tile.width || dim == tile.height);
        if common_dimensions.is_empty() {
            return Ok(false);
        }
    }
    
    // Check all stock tiles
    for stock_tile in stock_tiles {
        common_dimensions.retain(|&dim| dim == stock_tile.width || dim == stock_tile.height);
        if common_dimensions.is_empty() {
            return Ok(false);
        }
    }
    
    Ok(!common_dimensions.is_empty())
}

/// Get distinct grouped tile dimensions
/// 
/// This is the Rust equivalent of the Java method getDistinctGroupedTileDimensions()
fn get_distinct_grouped_tile_dimensions(
    groups: &[GroupedTileDimensions],
    _configuration: &Configuration,
) -> Result<HashMap<GroupedTileDimensions, i32>> {
    CollectionUtils::get_distinct_grouped_tile_dimensions(groups)
}

/// Generate complex permutations (Java: Arrangement.generatePermutations)
/// 
/// This implements the complex Java permutation generation including:
/// - Sorting by area (largest first)
/// - Limiting to first 7 groups for permutation
/// - Adding remaining groups to each permutation
fn generate_complex_permutations(
    distinct_groups: &HashMap<GroupedTileDimensions, i32>,
) -> Result<Vec<Vec<GroupedTileDimensions>>> {
    let mut groups: Vec<GroupedTileDimensions> = distinct_groups.keys().cloned().collect();
    
    if groups.is_empty() {
        return Ok(vec![]);
    }
    
    // Sort by area (largest first) - Java: Collections.sort with area comparator
    groups.sort_by(|a, b| {
        let area_a = a.tile_dimensions.area();
        let area_b = b.tile_dimensions.area();
        area_b.cmp(&area_a) // Reverse order for largest first
    });
    
    // Split into permutation groups and fixed groups (Java: if (arrayList2.size() > 7))
    let (permutation_groups, fixed_groups) = if groups.len() > 7 {
        let permutation_groups = groups[0..7].to_vec();
        let fixed_groups = groups[7..].to_vec();
        (permutation_groups, fixed_groups)
    } else {
        (groups, Vec::new())
    };
    
    // Generate permutations (Java: Arrangement.generatePermutations(arrayList2))
    let mut permutations = arrangement::generate_permutations(permutation_groups);
    
    // Add fixed groups to each permutation (Java: ((List) it.next()).addAll(arrayList))
    for permutation in &mut permutations {
        permutation.extend(fixed_groups.clone());
    }
    
    Ok(permutations)
}

/// Convert permutations to tile lists (Java: groupedTileDimensionsList2TileDimensionsList)
fn convert_permutations_to_tiles(
    permutations: &[Vec<GroupedTileDimensions>],
    grouped_tiles: &[GroupedTileDimensions],
) -> Result<Vec<Vec<TileDimensions>>> {
    let mut tile_permutations = Vec::new();
    
    for permutation in permutations {
        let mut tiles = Vec::new();
        
        // Sort grouped_tiles according to the permutation order
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

/// Remove duplicated permutations (Java: removeDuplicatedPermutations)
fn remove_duplicated_permutations(permutations: &mut Vec<Vec<GroupedTileDimensions>>) -> usize {
    let mut seen_hashes = Vec::new();
    let mut removed_count = 0;
    
    permutations.retain(|permutation| {
        // Calculate hash based on dimensions (Java: dimensionsBasedHashCode logic)
        let mut hash: i32 = 0;
        for group in permutation {
            hash = hash.wrapping_mul(31).wrapping_add(group.tile_dimensions.dimensions_hash() as i32);
        }
        
        if seen_hashes.contains(&hash) {
            removed_count += 1;
            false
        } else {
            seen_hashes.push(hash);
            true
        }
    });
    
    removed_count
}

/// Process a single permutation with complex Java lambda logic
/// 
/// This implements the complex Java lambda function that processes each permutation
/// with stock solutions and spawns computation threads with different orientations.
/// 
/// Java reference: lambda function in compute method around lines 400-500
async fn process_permutation_complex(
    stock_panel_picker: &StockPanelPicker,
    permutation_index: usize,
    task_arc: &std::sync::Arc<parking_lot::RwLock<Task>>,
    solutions: &[Solution],
    all_permutations: &[Vec<TileDimensions>],
    configuration: &Configuration,
    permutation: &[TileDimensions],
    optimization_factor: i32,
    performance_thresholds: &PerformanceThresholds,
    material: &str,
) -> Result<()> {
    let task_id = {
        let task = task_arc.read();
        task.id.clone()
    };
    
    // Process stock solutions (Java: for (int i4 = 0; i4 < 1000; i4++))
    for stock_index in 0..MAX_STOCK_ITERATIONS {
        // Get stock solution (Java: StockSolution stockSolution = stockPanelPicker.getStockSolution(i4))
        let stock_solution = match stock_panel_picker.get_stock_solution(stock_index) {
            Ok(Some(solution)) => solution,
            Ok(None) => {
                debug!("No more possible stock solutions: stockSolution[{}] permutationIdx[{}]", 
                       stock_index, permutation_index);
                break;
            }
            Err(_) => {
                debug!("Error getting stock solution: stockSolution[{}] permutationIdx[{}]", 
                       stock_index, permutation_index);
                break;
            }
        };
        
        // Check if task is still running (Java: if (!task.isRunning()))
        {
            let task = task_arc.read();
            if !task.is_running() {
                debug!("Task no longer has running status. Stopping stock loop for permutationIdx[{}]", 
                       permutation_index);
                return Ok(());
            }
        }
        
        // Check solution optimization conditions (Java: complex if condition)
        let should_process = {
            let task = task_arc.read();
            !task.has_solution_all_fit() || 
            solutions.is_empty() || 
            solutions[0].get_mosaics().len() != 1 || 
            solutions[0].get_total_area() >= stock_solution.get_total_area()
        };
        
        if should_process {
            debug!("Starting permutationIdx[{}/{}] with stock solution [{}] {{nbrPanels[{}] area[{}] {}}}", 
                   permutation_index, all_permutations.len(), stock_index, 
                   stock_solution.get_stock_tile_dimensions().len(), 
                   stock_solution.get_total_area(), stock_solution.to_string());
            
            // Setup cut thickness and trim dimension (Java: parsing configuration values)
            let cut_thickness = parse_configuration_value(configuration.cut_thickness, &task_arc, "cut thickness")?;
            let min_trim_dimension = parse_configuration_value(configuration.min_trim_dimension, &task_arc, "minimum trim dimension")?;
            
            // Wait for thread availability (Java: while loop checking thread limits)
            wait_for_thread_availability(task_arc, performance_thresholds).await?;
            
            // Spawn threads for different cut orientations (Java: isThreadEligibleToStart checks)
            spawn_cut_orientation_threads(
                task_arc,
                material,
                configuration,
                permutation,
                &stock_solution,
                cut_thickness,
                min_trim_dimension,
                optimization_factor,
                permutation_index,
                stock_index,
                solutions,
            ).await?;
            
        } else {
            debug!("Stopping stock loop for permutationIdx[{}/{}] at stock solution {} with area [{}] because there's already an all fit solution using stock solution with area [{}]", 
                   permutation_index, all_permutations.len(), stock_solution.to_string(), 
                   stock_solution.get_total_area(), solutions[0].get_total_area());
        }
    }
    
    Ok(())
}

/// Parse configuration value with error handling (Java: parsing logic with try-catch)
fn parse_configuration_value(
    value: i32,
    task_arc: &std::sync::Arc<parking_lot::RwLock<Task>>,
    value_name: &str,
) -> Result<i32> {
    let factor = {
        let task = task_arc.read();
        task.factor()
    };
    
    Ok((value as f64 * factor).round() as i32)
}

/// Wait for thread availability (Java: while loop checking thread limits)
async fn wait_for_thread_availability(
    task_arc: &std::sync::Arc<parking_lot::RwLock<Task>>,
    performance_thresholds: &PerformanceThresholds,
) -> Result<()> {
    use tokio::time::{sleep, Duration};
    
    loop {
        let (running_threads, queued_threads) = {
            let task = task_arc.read();
            (task.nbr_running_threads(), task.nbr_queued_threads())
        };
        
        if running_threads + queued_threads < performance_thresholds.max_simultaneous_threads {
            break;
        }
        
        trace!("Maximum number of active threads per task reached: running[{}] queued[{}]", 
               running_threads, queued_threads);
        
        sleep(Duration::from_millis(performance_thresholds.thread_check_interval)).await;
    }
    
    Ok(())
}

/// Spawn threads for different cut orientations (Java: isThreadEligibleToStart and thread spawning)
async fn spawn_cut_orientation_threads(
    task_arc: &std::sync::Arc<parking_lot::RwLock<Task>>,
    material: &str,
    configuration: &Configuration,
    permutation: &[TileDimensions],
    stock_solution: &StockSolution,
    cut_thickness: i32,
    min_trim_dimension: i32,
    optimization_factor: i32,
    permutation_index: usize,
    stock_index: usize,
    solutions: &[Solution],
) -> Result<()> {
    // For now, just simulate the thread spawning logic
    // TODO: Implement actual CutListThreadBuilder and thread execution
    
    // Check thread eligibility for AREA orientation (Java: isThreadEligibleToStart("AREA", task, str))
    if is_thread_eligible_to_start("AREA", task_arc, material)? {
        debug!("Spawning AREA thread for permutation[{}] stock[{}]", permutation_index, stock_index);
        // TODO: Spawn actual CutListThread with AREA orientation
    }
    
    // Check thread eligibility for AREA_HCUTS_1ST (Java: horizontal cuts first)
    if is_thread_eligible_to_start("AREA_HCUTS_1ST", task_arc, material)? {
        debug!("Spawning AREA_HCUTS_1ST thread for permutation[{}] stock[{}]", permutation_index, stock_index);
        // TODO: Spawn actual CutListThread with horizontal cuts first
    }
    
    // Check thread eligibility for AREA_VCUTS_1ST (Java: vertical cuts first)
    if is_thread_eligible_to_start("AREA_VCUTS_1ST", task_arc, material)? {
        debug!("Spawning AREA_VCUTS_1ST thread for permutation[{}] stock[{}]", permutation_index, stock_index);
        // TODO: Spawn actual CutListThread with vertical cuts first
    }
    
    Ok(())
}

/// Check if thread is eligible to start (Java: isThreadEligibleToStart)
fn is_thread_eligible_to_start(
    group: &str,
    task_arc: &std::sync::Arc<parking_lot::RwLock<Task>>,
    material: &str,
) -> Result<bool> {
    // For now, always return true
    // TODO: Implement actual thread eligibility logic based on thread group rankings
    // Java logic: check thread group rankings and finished threads count
    
    let task = task_arc.read();
    let finished_threads = task.nbr_finished_threads_for_material(material);
    
    // Simple eligibility check - allow if we haven't finished too many threads
    Ok(finished_threads < 10)
}

/// Wait for computation completion (Java: while loop waiting for threads)
async fn wait_for_computation_completion(
    task_arc: &std::sync::Arc<parking_lot::RwLock<Task>>,
    material: &str,
) -> Result<()> {
    use tokio::time::{sleep, Duration};
    
    loop {
        let (running_threads, queued_threads) = {
            let task = task_arc.read();
            (task.nbr_running_threads(), task.nbr_queued_threads())
        };
        
        debug!("Waiting for computation completion: running[{}] queued[{}]", 
               running_threads, queued_threads);
        
        if running_threads + queued_threads <= 0 {
            break;
        }
        
        sleep(Duration::from_secs(1)).await;
    }
    
    Ok(())
}
