//! Компаратор для сравнения решений по дисбалансу горизонтальных и вертикальных разрезов

use super::SolutionComparator;
use crate::engine::model::solution::Solution;
use std::cmp::Ordering;

/// Компаратор для максимизации дисбаланса горизонтальных и вертикальных разрезов
/// Предпочитает решения с большим дисбалансом (больше разница между H и V разрезами)
pub struct SolutionMostHVDiscrepancyComparator;

impl SolutionMostHVDiscrepancyComparator {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionComparator for SolutionMostHVDiscrepancyComparator {
    fn compare(&self, solution1: &Solution, solution2: &Solution) -> Ordering {
        let discrepancy1 = solution1.get_hv_diff();
        let discrepancy2 = solution2.get_hv_diff();
        
        // Больший дисбаланс считается лучше, поэтому сравниваем в обратном порядке
        discrepancy2.partial_cmp(&discrepancy1).unwrap_or(Ordering::Equal)
    }
}
