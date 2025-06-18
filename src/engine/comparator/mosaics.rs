//! Компараторы для сравнения решений по количеству мозаик и деталей

use super::SolutionComparator;
use crate::engine::model::solution::Solution;
use std::cmp::Ordering;

/// Компаратор для максимизации количества размещенных деталей
/// Сравнивает решения по количеству финальных деталей (чем больше, тем лучше)
pub struct SolutionMostNbrTilesComparator;

impl SolutionMostNbrTilesComparator {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionComparator for SolutionMostNbrTilesComparator {
    fn compare(&self, solution1: &Solution, solution2: &Solution) -> Ordering {
        let tiles1 = solution1.get_nbr_final_tiles();
        let tiles2 = solution2.get_nbr_final_tiles();
        
        // Больше деталей = лучше решение (обратный порядок)
        tiles2.cmp(&tiles1)
    }
}

/// Компаратор для минимизации количества листов материала (мозаик)
/// Сравнивает решения по количеству мозаик (чем меньше, тем лучше)
pub struct SolutionLeastNbrMosaicsComparator;

impl SolutionLeastNbrMosaicsComparator {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionComparator for SolutionLeastNbrMosaicsComparator {
    fn compare(&self, solution1: &Solution, solution2: &Solution) -> Ordering {
        let mosaics1 = solution1.get_nbr_mosaics();
        let mosaics2 = solution2.get_nbr_mosaics();
        
        // Меньше мозаик = лучше решение
        mosaics1.cmp(&mosaics2)
    }
}
