//! Компараторы для сравнения решений по площадным характеристикам

use super::SolutionComparator;
use crate::engine::model::solution::Solution;
use std::cmp::Ordering;

/// Компаратор для минимизации отходов материала
/// Сравнивает решения по неиспользуемой площади (чем меньше, тем лучше)
pub struct SolutionLeastWastedAreaComparator;

impl SolutionLeastWastedAreaComparator {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionComparator for SolutionLeastWastedAreaComparator {
    fn compare(&self, solution1: &Solution, solution2: &Solution) -> Ordering {
        let unused_area1 = solution1.get_unused_area();
        let unused_area2 = solution2.get_unused_area();
        
        // Меньше отходов = лучше решение
        unused_area1.partial_cmp(&unused_area2).unwrap_or(Ordering::Equal)
    }
}

/// Компаратор для максимизации площади самого большого неиспользуемого блока
/// Сравнивает решения по максимальной свободной площади (чем больше, тем лучше)
pub struct SolutionBiggestUnusedTileAreaComparator;

impl SolutionBiggestUnusedTileAreaComparator {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionComparator for SolutionBiggestUnusedTileAreaComparator {
    fn compare(&self, solution1: &Solution, solution2: &Solution) -> Ordering {
        let max_unused1 = solution1.get_biggest_unused_tile_area();
        let max_unused2 = solution2.get_biggest_unused_tile_area();
        
        // Больше максимальная свободная площадь = лучше решение
        max_unused2.partial_cmp(&max_unused1).unwrap_or(Ordering::Equal)
    }
}
