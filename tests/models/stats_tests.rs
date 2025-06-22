//! Tests for Stats model functionality

use cutlist_optimizer_cli::models::stats::Stats;

#[test]
fn test_stats_default() {
    let stats = Stats::default();
    assert_eq!(stats.total_tasks(), 0);
    assert_eq!(stats.total_threads(), 0);
    assert!(!stats.is_busy());
    assert_eq!(stats.success_rate(), 100.0);
}

#[test]
fn test_stats_calculations() {
    let stats = Stats {
        nbr_idle_tasks: 1,
        nbr_running_tasks: 2,
        nbr_finished_tasks: 3,
        nbr_stopped_tasks: 1,
        nbr_terminated_tasks: 0,
        nbr_error_tasks: 1,
        nbr_running_threads: 5,
        nbr_queued_threads: 3,
        nbr_finished_threads: 100,
        task_reports: vec![],
    };

    assert_eq!(stats.total_tasks(), 8);
    assert_eq!(stats.total_threads(), 8);
    assert!(stats.is_busy());
    assert_eq!(stats.success_rate(), 60.0); // 3 finished out of 5 completed (3+1+1)
}

#[test]
fn test_stats_display() {
    let stats = Stats {
        nbr_running_tasks: 2,
        nbr_finished_tasks: 5,
        nbr_running_threads: 3,
        nbr_queued_threads: 1,
        ..Default::default()
    };

    let display = format!("{}", stats);
    assert!(display.contains("2/7 running")); // 2 running out of 7 total (2 running + 5 finished)
    assert!(display.contains("3/4 active"));   // 3 running out of 4 total threads (3 running + 1 queued)
}
