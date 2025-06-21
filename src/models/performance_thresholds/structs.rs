use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum number of simultaneous tasks
    pub max_simultaneous_tasks: usize,
    
    /// Maximum number of threads per task
    pub max_simultaneous_threads: usize,
    
    /// Interval between thread status checks (milliseconds)
    pub thread_check_interval: u64,
}
