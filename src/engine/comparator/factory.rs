//! Фабрика для создания компараторов решений

use super::{
    SolutionComparator,
    priority::OptimizationPriority,
    area::{SolutionLeastWastedAreaComparator, SolutionBiggestUnusedTileAreaComparator},
    cuts::SolutionLeastNbrCutsComparator,
    mosaics::{SolutionLeastNbrMosaicsComparator, SolutionMostNbrTilesComparator},
    tiles::SolutionLeastNbrUnusedTilesComparator,
    discrepancy::SolutionMostHVDiscrepancyComparator,
    panel_area::SolutionMostUnusedPanelAreaComparator,
    center_mass::SolutionSmallestCenterOfMassDistToOriginComparator,
};

/// Фабрика для создания компараторов решений
pub struct SolutionComparatorFactory;

impl SolutionComparatorFactory {
    /// Создает компаратор для заданного приоритета оптимизации
    /// 
    /// # Аргументы
    /// * `priority` - Приоритет оптимизации
    /// 
    /// # Возвращает
    /// Boxed компаратор для данного приоритета
    pub fn create_comparator(priority: OptimizationPriority) -> Box<dyn SolutionComparator> {
        match priority {
            OptimizationPriority::MostTiles => {
                Box::new(SolutionMostNbrTilesComparator::new())
            }
            OptimizationPriority::LeastWastedArea => {
                Box::new(SolutionLeastWastedAreaComparator::new())
            }
            OptimizationPriority::LeastNbrCuts => {
                Box::new(SolutionLeastNbrCutsComparator::new())
            }
            OptimizationPriority::LeastNbrMosaics => {
                Box::new(SolutionLeastNbrMosaicsComparator::new())
            }
            OptimizationPriority::BiggestUnusedTileArea => {
                Box::new(SolutionBiggestUnusedTileAreaComparator::new())
            }
            OptimizationPriority::MostHVDiscrepancy => {
                Box::new(SolutionMostHVDiscrepancyComparator::new())
            }
            OptimizationPriority::SmallestCenterOfMassDistToOrigin => {
                Box::new(SolutionSmallestCenterOfMassDistToOriginComparator::new())
            }
            OptimizationPriority::LeastNbrUnusedTiles => {
                Box::new(SolutionLeastNbrUnusedTilesComparator::new())
            }
            OptimizationPriority::MostUnusedPanelArea => {
                Box::new(SolutionMostUnusedPanelAreaComparator::new())
            }
        }
    }
    
    /// Создает список компараторов для заданных приоритетов
    /// 
    /// # Аргументы
    /// * `priorities` - Список приоритетов в порядке важности
    /// 
    /// # Возвращает
    /// Вектор boxed компараторов
    pub fn create_comparators(
        priorities: Vec<OptimizationPriority>
    ) -> Vec<Box<dyn SolutionComparator>> {
        priorities
            .into_iter()
            .map(|priority| Self::create_comparator(priority))
            .collect()
    }
    
    /// Создает компаратор по имени (для десериализации)
    /// 
    /// # Аргументы
    /// * `name` - Имя компаратора
    /// 
    /// # Возвращает
    /// Option с компаратором если имя распознано
    pub fn create_comparator_by_name(name: &str) -> Option<Box<dyn SolutionComparator>> {
        match name.to_lowercase().as_str() {
            "most_tiles" | "mosttiles" => {
                Some(Box::new(SolutionMostNbrTilesComparator::new()))
            }
            "least_wasted_area" | "leastwastedarea" => {
                Some(Box::new(SolutionLeastWastedAreaComparator::new()))
            }
            "least_nbr_cuts" | "leastnbrcuts" => {
                Some(Box::new(SolutionLeastNbrCutsComparator::new()))
            }
            "least_nbr_mosaics" | "leastnbrmosaics" => {
                Some(Box::new(SolutionLeastNbrMosaicsComparator::new()))
            }
            "biggest_unused_tile_area" | "biggestunusedtilearea" => {
                Some(Box::new(SolutionBiggestUnusedTileAreaComparator::new()))
            }
            "most_hv_discrepancy" | "mosthvdiscrepancy" => {
                Some(Box::new(SolutionMostHVDiscrepancyComparator::new()))
            }
            "smallest_center_of_mass_dist_to_origin" | "smallestcenterofmassdisttoorigin" => {
                Some(Box::new(SolutionSmallestCenterOfMassDistToOriginComparator::new()))
            }
            "least_nbr_unused_tiles" | "leastnbrunusedtiles" => {
                Some(Box::new(SolutionLeastNbrUnusedTilesComparator::new()))
            }
            "most_unused_panel_area" | "mostunusedpanelarea" => {
                Some(Box::new(SolutionMostUnusedPanelAreaComparator::new()))
            }
            _ => None,
        }
    }
    
    /// Получает список всех доступных имен компараторов
    pub fn get_available_comparator_names() -> Vec<&'static str> {
        vec![
            "most_tiles",
            "least_wasted_area",
            "least_nbr_cuts",
            "least_nbr_mosaics",
            "biggest_unused_tile_area",
            "most_hv_discrepancy",
            "smallest_center_of_mass_dist_to_origin",
            "least_nbr_unused_tiles",
            "most_unused_panel_area",
        ]
    }
    
    /// Проверяет существование компаратора по имени
    pub fn is_valid_comparator_name(name: &str) -> bool {
        Self::create_comparator_by_name(name).is_some()
    }
}
