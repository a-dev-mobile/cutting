//! Компараторы и сортировка решений для оптимизации раскроя
//! 
//! Этот модуль содержит различные компараторы для сравнения решений
//! по разным критериям оптимизации, а также фабрики для создания
//! приоритетных списков компараторов.

pub mod priority;
pub mod factory;
pub mod area;
pub mod cuts;
pub mod mosaics;
pub mod tiles;
pub mod discrepancy;
pub mod panel_area;
pub mod center_mass;

pub use priority::{OptimizationPriority, PriorityListFactory};
pub use factory::SolutionComparatorFactory;

use crate::engine::model::solution::Solution;
use std::cmp::Ordering;

/// Базовый трейт для всех компараторов решений
pub trait SolutionComparator {
    /// Сравнивает два решения и возвращает результат сравнения
    /// 
    /// # Аргументы
    /// * `solution1` - Первое решение для сравнения
    /// * `solution2` - Второе решение для сравнения
    /// 
    /// # Возвращает
    /// * `Ordering::Less` - если solution1 лучше solution2
    /// * `Ordering::Greater` - если solution2 лучше solution1  
    /// * `Ordering::Equal` - если решения равны по данному критерию
    fn compare(&self, solution1: &Solution, solution2: &Solution) -> Ordering;
}

/// Составной компаратор для многокритериальной сортировки
pub struct MultiCriteriaComparator {
    comparators: Vec<Box<dyn SolutionComparator>>,
}

impl MultiCriteriaComparator {
    /// Создает новый составной компаратор
    pub fn new(comparators: Vec<Box<dyn SolutionComparator>>) -> Self {
        Self { comparators }
    }
    
    /// Создает составной компаратор из списка приоритетов
    pub fn from_priorities(priorities: Vec<OptimizationPriority>) -> Self {
        let comparators = priorities
            .into_iter()
            .map(|priority| SolutionComparatorFactory::create_comparator(priority))
            .collect();
        Self::new(comparators)
    }
}

impl SolutionComparator for MultiCriteriaComparator {
    fn compare(&self, solution1: &Solution, solution2: &Solution) -> Ordering {
        for comparator in &self.comparators {
            let result = comparator.compare(solution1, solution2);
            if result != Ordering::Equal {
                return result;
            }
        }
        Ordering::Equal
    }
}

/// Утилиты для работы с решениями
pub struct SolutionUtils;

impl SolutionUtils {
    /// Сортирует список решений по заданным приоритетам
    pub fn sort_solutions(
        solutions: &mut Vec<Solution>,
        priorities: Vec<OptimizationPriority>,
    ) {
        let comparator = MultiCriteriaComparator::from_priorities(priorities);
        solutions.sort_by(|a, b| comparator.compare(a, b));
    }
    
    /// Удаляет дубликаты решений на основе их структурного идентификатора
    pub fn remove_duplicates(solutions: &mut Vec<Solution>) -> usize {
        use std::collections::HashSet;
        
        let mut seen = HashSet::new();
        let initial_count = solutions.len();
        
        solutions.retain(|solution| {
            let identifier = solution.get_structure_identifier();
            seen.insert(identifier)
        });
        
        initial_count - solutions.len()
    }
    
    /// Ограничивает количество решений до заданного лимита
    pub fn limit_solutions(solutions: &mut Vec<Solution>, limit: usize) {
        if solutions.len() > limit {
            solutions.truncate(limit);
        }
    }
    
    /// Полная обработка списка решений: сортировка, удаление дубликатов, ограничение
    pub fn process_solutions(
        solutions: &mut Vec<Solution>,
        priorities: Vec<OptimizationPriority>,
        limit: Option<usize>,
    ) -> usize {
        // Сортируем решения
        Self::sort_solutions(solutions, priorities);
        
        // Удаляем дубликаты
        let removed_count = Self::remove_duplicates(solutions);
        
        // Ограничиваем количество если задан лимит
        if let Some(limit) = limit {
            Self::limit_solutions(solutions, limit);
        }
        
        removed_count
    }
}
