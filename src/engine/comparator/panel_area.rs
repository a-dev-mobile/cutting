//! Компаратор для сравнения решений по максимальной неиспользуемой площади панели

use super::SolutionComparator;
use crate::engine::model::solution::Solution;
use std::cmp::Ordering;

/// Компаратор для максимизации неиспользуемой площади панели
/// Предпочитает решения с большей максимальной неиспользуемой площадью панели
pub struct SolutionMostUnusedPanelAreaComparator;

impl SolutionMostUnusedPanelAreaComparator {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionComparator for SolutionMostUnusedPanelAreaComparator {
    fn compare(&self, solution1: &Solution, solution2: &Solution) -> Ordering {
        let unused_area1 = solution1.get_most_unused_panel_area();
        let unused_area2 = solution2.get_most_unused_panel_area();
        
        // Большая неиспользуемая площадь считается лучше
        unused_area2.cmp(&unused_area1)
    }
}
