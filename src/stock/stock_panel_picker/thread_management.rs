//! Thread management for background stock solution generation

use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use crate::{log_debug, log_error};

use crate::models::task::Task;
use crate::stock::StockSolution;
use crate::constants::StockConstants;
use crate::errors::{AppError, Result};
use super::StockPanelPicker;

impl StockPanelPicker {
    /// Initialize the background generation thread
    /// 
    /// This corresponds to the Java `init()` method that starts the background thread
    pub async fn init(&self) -> Result<()> {
        let mut generation_thread = self.generation_thread.lock().map_err(|_| {
            AppError::thread_error("Failed to acquire generation thread lock")
        })?;

        if generation_thread.is_some() {
            return Err(AppError::thread_error("Generation thread already initialized"));
        }

        let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
        
        // Store the shutdown sender
        {
            let mut sender_guard = self.shutdown_sender.lock().map_err(|_| {
                AppError::thread_error("Failed to acquire shutdown sender lock")
            })?;
            *sender_guard = Some(shutdown_tx);
        }

        // Clone necessary data for the background task
        let stock_solution_generator = Arc::new(Mutex::new(self.stock_solution_generator.clone()));
        let stock_solutions = Arc::clone(&self.stock_solutions);
        let max_retrieved_idx = Arc::clone(&self.max_retrieved_idx);
        let task = Arc::clone(&self.task);

        // Spawn the background generation task
        let handle = tokio::spawn(async move {
            Self::generation_loop(
                stock_solution_generator,
                stock_solutions,
                max_retrieved_idx,
                task,
                &mut shutdown_rx,
            ).await
        });

        *generation_thread = Some(handle);
        Ok(())
    }

    /// Main generation loop that runs in the background
    async fn generation_loop(
        stock_solution_generator: Arc<Mutex<crate::stock::StockSolutionGenerator>>,
        stock_solutions: Arc<Mutex<Vec<StockSolution>>>,
        max_retrieved_idx: Arc<Mutex<usize>>,
        task: Arc<Task>,
        shutdown_rx: &mut mpsc::UnboundedReceiver<()>,
    ) {
        let mut last_generated_solution: Option<StockSolution> = None;

        loop {
            // Check for shutdown signal
            if shutdown_rx.try_recv().is_ok() {
                log_debug!("Received shutdown signal, stopping generation loop");
                break;
            }

            // Atomically check if we need to generate more solutions
            let should_generate = {
                let solutions_guard = match stock_solutions.lock() {
                    Ok(guard) => guard,
                    Err(e) => {
                        log_error!("Failed to acquire stock solutions lock: {}", e);
                        break;
                    }
                };

                let max_idx_guard = match max_retrieved_idx.lock() {
                    Ok(guard) => guard,
                    Err(e) => {
                        log_error!("Failed to acquire max retrieved index lock: {}", e);
                        break;
                    }
                };

                let solutions_count = solutions_guard.len();
                let max_idx = *max_idx_guard;

                // Generate if we're running low on solutions or haven't reached minimum
                max_idx >= solutions_count.saturating_sub(1) || solutions_count <= StockConstants::MIN_INIT_STOCK_SOLUTIONS_TO_GENERATE
            };

            if should_generate {
                // Generate new solution
                let new_solution = {
                    let mut generator_guard = match stock_solution_generator.lock() {
                        Ok(guard) => guard,
                        Err(e) => {
                            log_error!("Failed to acquire stock solution generator lock: {}", e);
                            break;
                        }
                    };

                    match generator_guard.generate_stock_solution() {
                        crate::models::enums::StockSolutionResult::Solution(solution) => Some(solution),
                        crate::models::enums::StockSolutionResult::NoSolution => None,
                        crate::models::enums::StockSolutionResult::AllExcluded => None,
                    }
                };

                if let Some(solution) = new_solution {
                    // Add the solution to our collection atomically
                    let solutions_count = {
                        let mut solutions_guard = match stock_solutions.lock() {
                            Ok(guard) => guard,
                            Err(e) => {
                                log_error!("Failed to acquire stock solutions lock for adding: {}", e);
                                break;
                            }
                        };

                        solutions_guard.push(solution.clone());

                        // If the solution doesn't have unique panel sizes, add a sorted variant
                        if !solution.has_unique_panel_size() {
                            let mut sorted_solution = solution.clone();
                            sorted_solution.sort_panels_desc();
                            solutions_guard.push(sorted_solution);
                        }

                        let count = solutions_guard.len();
                        log_debug!(
                            "Added solution idx[{}] with [{}] panels, area[{}] to stack",
                            count - 1,
                            solution.get_stock_tile_dimensions().len(),
                            solution.get_total_area()
                        );
                        count
                    };

                    last_generated_solution = Some(solution);

                    // Check termination conditions after adding solution
                    if task.has_solution_all_fit() && solutions_count >= StockConstants::MIN_STOCK_SOLUTIONS_TO_GENERATE_WITH_ALL_FIT_SOLUTION {
                        log_debug!(
                            "Finishing stock picker thread: nbrGeneratedStockSolutions[{}] - Task has already an all fit solution",
                            solutions_count
                        );
                        break;
                    }
                } else {
                    last_generated_solution = None;
                    log_debug!("No more stock solutions can be generated");
                    break;
                }
            } else {
                let (solutions_count, max_idx) = {
                    let solutions_guard = match stock_solutions.lock() {
                        Ok(guard) => guard,
                        Err(e) => {
                            log_error!("Failed to acquire stock solutions lock for logging: {}", e);
                            break;
                        }
                    };

                    let max_idx_guard = match max_retrieved_idx.lock() {
                        Ok(guard) => guard,
                        Err(e) => {
                            log_error!("Failed to acquire max retrieved index lock for logging: {}", e);
                            break;
                        }
                    };

                    (solutions_guard.len(), *max_idx_guard)
                };

                log_debug!(
                    "No need to generate new candidate stock solution: maxRetrievedIdx[{}] stockSolutions[{}]",
                    max_idx, solutions_count
                );

                // Sleep if we have enough solutions
                if solutions_count > StockConstants::MIN_INIT_STOCK_SOLUTIONS_TO_GENERATE {
                    sleep(Duration::from_millis(StockConstants::SOLUTION_WAIT_SLEEP_MS)).await;
                }
            }

            // Check task termination conditions
            if !task.is_running() {
                let solutions_count = stock_solutions.lock()
                    .map(|guard| guard.len())
                    .unwrap_or(0);
                log_debug!(
                    "Finishing stock picker thread: nbrGeneratedStockSolutions[{}] - Task is no longer running",
                    solutions_count
                );
                break;
            }
        }

        // Final logging
        let final_count = stock_solutions.lock()
            .map(|guard| guard.len())
            .unwrap_or(0);
        
        if last_generated_solution.is_none() {
            log_debug!(
                "Finishing stock picker thread: nbrGeneratedStockSolutions[{}] - No more available stock solutions",
                final_count
            );
        }
    }

    /// Stop the background generation thread
    pub async fn stop_generation(&self) -> Result<()> {
        // Send shutdown signal
        {
            let sender_guard = self.shutdown_sender.lock().map_err(|_| {
                AppError::thread_error("Failed to acquire shutdown sender lock")
            })?;

            if let Some(sender) = sender_guard.as_ref() {
                let _ = sender.send(()); // Ignore send errors as the receiver might be dropped
            }
        }

        // Extract the handle without holding the lock across await
        let handle = {
            let mut generation_thread = self.generation_thread.lock().map_err(|_| {
                AppError::thread_error("Failed to acquire generation thread lock")
            })?;
            generation_thread.take()
        };

        // Now await the handle without holding any locks
        if let Some(handle) = handle {
            match handle.await {
                Ok(_) => log_debug!("Generation thread stopped successfully"),
                Err(e) => log_error!("Generation thread panicked: {:?}", e),
            }
        }

        Ok(())
    }

    /// Check if the generation thread is still running
    pub fn is_generation_active(&self) -> bool {
        let generation_thread = match self.generation_thread.lock() {
            Ok(guard) => guard,
            Err(_) => return false,
        };

        match generation_thread.as_ref() {
            Some(handle) => !handle.is_finished(),
            None => false,
        }
    }
}
