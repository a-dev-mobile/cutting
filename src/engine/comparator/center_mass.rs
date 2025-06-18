//! Компаратор для сравнения решений по расстоянию центра масс до начала координат

use super::SolutionComparator;
use crate::engine::model::solution::Solution;
use std::cmp::Ordering;

/// Компаратор для минимизации расстояния центра масс до начала координат
/// Предпочитает решения с меньшим расстоянием центра масс до начала координат
pub struct SolutionSmallestCenterOfMassDistToOriginComparator;

impl SolutionSmallestCenterOfMassDistToOriginComparator {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionComparator for SolutionSmallestCenterOfMassDistToOriginComparator {
    fn compare(&self, solution1: &Solution, solution2: &Solution) -> Ordering {
        let distance1 = solution1.get_center_of_mass_distance_to_origin();
        let distance2 = solution2.get_center_of_mass_distance_to_origin();
        
        // Меньшее расстояние считается лучше
        distance1.partial_cmp(&distance2).unwrap_or(Ordering::Equal)
    }
}
