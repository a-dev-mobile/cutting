//! Компаратор для сравнения решений по количеству неиспользуемых деталей

use super::SolutionComparator;
use crate::engine::model::solution::Solution;
use std::cmp::Ordering;

/// Компаратор для минимизации количества неиспользуемых узлов/деталей
/// Сравнивает решения по количеству неиспользуемых узлов (чем меньше, тем лучше)
pub struct SolutionLeastNbrUnusedTilesComparator;

impl SolutionLeastNbrUnusedTilesComparator {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionComparator for SolutionLeastNbrUnusedTilesComparator {
    fn compare(&self, solution1: &Solution, solution2: &Solution) -> Ordering {
        let unused_tiles1 = solution1.get_nbr_unused_tiles();
        let unused_tiles2 = solution2.get_nbr_unused_tiles();
        
        // Меньше неиспользуемых деталей = лучше решение
        unused_tiles1.cmp(&unused_tiles2)
    }
}
