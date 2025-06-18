//! Компаратор для сравнения решений по количеству разрезов

use super::SolutionComparator;
use crate::engine::model::solution::Solution;
use std::cmp::Ordering;

/// Компаратор для минимизации количества разрезов
/// Сравнивает решения по общему количеству разрезов (чем меньше, тем лучше)
pub struct SolutionLeastNbrCutsComparator;

impl SolutionLeastNbrCutsComparator {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionComparator for SolutionLeastNbrCutsComparator {
    fn compare(&self, solution1: &Solution, solution2: &Solution) -> Ordering {
        let cuts1 = solution1.get_nbr_cuts();
        let cuts2 = solution2.get_nbr_cuts();

        // Меньше разрезов = лучше решение
        cuts1.cmp(&cuts2)
    }
}
