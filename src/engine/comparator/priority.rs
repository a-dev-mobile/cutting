//! Приоритеты оптимизации и фабрика для создания списков приоритетов

use std::fmt;

/// Перечисление приоритетов оптимизации для сравнения решений
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationPriority {
    /// Максимум размещенных деталей (приоритет #1)
    MostTiles,
    /// Минимум отходов материала
    LeastWastedArea,
    /// Минимум количества разрезов
    LeastNbrCuts,
    /// Минимум листов материала
    LeastNbrMosaics,
    /// Максимальная площадь остатка
    BiggestUnusedTileArea,
    /// Баланс горизонтальных/вертикальных разрезов
    MostHVDiscrepancy,
    /// Минимальное расстояние центра масс до начала координат
    SmallestCenterOfMassDistToOrigin,
    /// Минимум неиспользуемых узлов
    LeastNbrUnusedTiles,
    /// Максимальная неиспользуемая площадь панели
    MostUnusedPanelArea,
}

impl fmt::Display for OptimizationPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            OptimizationPriority::MostTiles => "Most Tiles",
            OptimizationPriority::LeastWastedArea => "Least Wasted Area",
            OptimizationPriority::LeastNbrCuts => "Least Number of Cuts",
            OptimizationPriority::LeastNbrMosaics => "Least Number of Mosaics",
            OptimizationPriority::BiggestUnusedTileArea => "Biggest Unused Tile Area",
            OptimizationPriority::MostHVDiscrepancy => "Most H/V Discrepancy",
            OptimizationPriority::SmallestCenterOfMassDistToOrigin => "Smallest Center of Mass Distance to Origin",
            OptimizationPriority::LeastNbrUnusedTiles => "Least Number of Unused Tiles",
            OptimizationPriority::MostUnusedPanelArea => "Most Unused Panel Area",
        };
        write!(f, "{}", name)
    }
}

/// Фабрика для создания приоритетных списков компараторов
pub struct PriorityListFactory;

impl PriorityListFactory {
    /// Создает список приоритетов для финальной сортировки решений
    /// 
    /// # Аргументы
    /// * `optimization_priority` - Основной приоритет оптимизации (0 = площадь, 1 = разрезы)
    /// 
    /// # Возвращает
    /// Вектор приоритетов в порядке важности
    pub fn get_final_solution_prioritized_comparator_list(
        optimization_priority: i32,
    ) -> Vec<OptimizationPriority> {
        let mut priorities = Vec::new();
        
        // Количество размещенных деталей всегда первый приоритет
        priorities.push(OptimizationPriority::MostTiles);
        
        // Второй и третий приоритеты зависят от настройки
        match optimization_priority {
            0 => {
                // Приоритет на минимизацию отходов
                priorities.push(OptimizationPriority::LeastWastedArea);
                priorities.push(OptimizationPriority::LeastNbrCuts);
            }
            1 => {
                // Приоритет на минимизацию разрезов
                priorities.push(OptimizationPriority::LeastNbrCuts);
                priorities.push(OptimizationPriority::LeastWastedArea);
            }
            _ => {
                // По умолчанию приоритет на площадь
                priorities.push(OptimizationPriority::LeastWastedArea);
                priorities.push(OptimizationPriority::LeastNbrCuts);
            }
        }
        
        // Добавляем дополнительные критерии в фиксированном порядке
        priorities.push(OptimizationPriority::LeastNbrMosaics);
        priorities.push(OptimizationPriority::BiggestUnusedTileArea);
        priorities.push(OptimizationPriority::MostHVDiscrepancy);
        priorities.push(OptimizationPriority::SmallestCenterOfMassDistToOrigin);
        priorities.push(OptimizationPriority::LeastNbrUnusedTiles);
        priorities.push(OptimizationPriority::MostUnusedPanelArea);
        
        priorities
    }
    
    /// Создает список приоритетов для промежуточной сортировки
    /// 
    /// # Аргументы
    /// * `optimization_priority` - Основной приоритет оптимизации
    /// 
    /// # Возвращает
    /// Упрощенный список приоритетов для быстрой сортировки
    pub fn get_intermediate_solution_prioritized_comparator_list(
        optimization_priority: i32,
    ) -> Vec<OptimizationPriority> {
        let mut priorities = Vec::new();
        
        priorities.push(OptimizationPriority::MostTiles);
        
        match optimization_priority {
            0 => {
                priorities.push(OptimizationPriority::LeastWastedArea);
                priorities.push(OptimizationPriority::LeastNbrCuts);
            }
            1 => {
                priorities.push(OptimizationPriority::LeastNbrCuts);
                priorities.push(OptimizationPriority::LeastWastedArea);
            }
            _ => {
                priorities.push(OptimizationPriority::LeastWastedArea);
                priorities.push(OptimizationPriority::LeastNbrCuts);
            }
        }
        
        priorities.push(OptimizationPriority::LeastNbrMosaics);
        
        priorities
    }
    
    /// Создает пользовательский список приоритетов
    /// 
    /// # Аргументы
    /// * `primary` - Основной критерий
    /// * `secondary` - Вторичный критерий
    /// * `additional` - Дополнительные критерии
    /// 
    /// # Возвращает
    /// Пользовательский список приоритетов
    pub fn create_custom_priority_list(
        primary: OptimizationPriority,
        secondary: Option<OptimizationPriority>,
        additional: Vec<OptimizationPriority>,
    ) -> Vec<OptimizationPriority> {
        let mut priorities = Vec::new();
        
        priorities.push(primary);
        
        if let Some(secondary) = secondary {
            priorities.push(secondary);
        }
        
        for priority in additional {
            if !priorities.contains(&priority) {
                priorities.push(priority);
            }
        }
        
        priorities
    }
    
    /// Получает все доступные приоритеты
    pub fn get_all_priorities() -> Vec<OptimizationPriority> {
        vec![
            OptimizationPriority::MostTiles,
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts,
            OptimizationPriority::LeastNbrMosaics,
            OptimizationPriority::BiggestUnusedTileArea,
            OptimizationPriority::MostHVDiscrepancy,
            OptimizationPriority::SmallestCenterOfMassDistToOrigin,
            OptimizationPriority::LeastNbrUnusedTiles,
            OptimizationPriority::MostUnusedPanelArea,
        ]
    }
    
    /// Проверяет валидность приоритета оптимизации
    pub fn is_valid_optimization_priority(priority: i32) -> bool {
        matches!(priority, 0 | 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_optimization_priority_display() {
        assert_eq!(
            OptimizationPriority::MostTiles.to_string(),
            "Most Tiles"
        );
        assert_eq!(
            OptimizationPriority::LeastWastedArea.to_string(),
            "Least Wasted Area"
        );
        assert_eq!(
            OptimizationPriority::LeastNbrCuts.to_string(),
            "Least Number of Cuts"
        );
    }
    
    #[test]
    fn test_final_solution_priority_list_area_priority() {
        let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(0);
        
        assert_eq!(priorities[0], OptimizationPriority::MostTiles);
        assert_eq!(priorities[1], OptimizationPriority::LeastWastedArea);
        assert_eq!(priorities[2], OptimizationPriority::LeastNbrCuts);
        assert_eq!(priorities[3], OptimizationPriority::LeastNbrMosaics);
        
        // Проверяем что все приоритеты включены
        assert_eq!(priorities.len(), 9);
    }
    
    #[test]
    fn test_final_solution_priority_list_cuts_priority() {
        let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(1);
        
        assert_eq!(priorities[0], OptimizationPriority::MostTiles);
        assert_eq!(priorities[1], OptimizationPriority::LeastNbrCuts);
        assert_eq!(priorities[2], OptimizationPriority::LeastWastedArea);
        assert_eq!(priorities[3], OptimizationPriority::LeastNbrMosaics);
    }
    
    #[test]
    fn test_final_solution_priority_list_invalid_priority() {
        let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(999);
        
        // Должен использовать значения по умолчанию (как для 0)
        assert_eq!(priorities[0], OptimizationPriority::MostTiles);
        assert_eq!(priorities[1], OptimizationPriority::LeastWastedArea);
        assert_eq!(priorities[2], OptimizationPriority::LeastNbrCuts);
    }
    
    #[test]
    fn test_intermediate_solution_priority_list() {
        let priorities_area = PriorityListFactory::get_intermediate_solution_prioritized_comparator_list(0);
        let priorities_cuts = PriorityListFactory::get_intermediate_solution_prioritized_comparator_list(1);
        
        // Промежуточный список должен быть короче
        assert!(priorities_area.len() < 9);
        assert!(priorities_cuts.len() < 9);
        
        // Первый приоритет всегда MostTiles
        assert_eq!(priorities_area[0], OptimizationPriority::MostTiles);
        assert_eq!(priorities_cuts[0], OptimizationPriority::MostTiles);
        
        // Проверяем различие в порядке
        assert_eq!(priorities_area[1], OptimizationPriority::LeastWastedArea);
        assert_eq!(priorities_cuts[1], OptimizationPriority::LeastNbrCuts);
    }
    
    #[test]
    fn test_create_custom_priority_list() {
        let priorities = PriorityListFactory::create_custom_priority_list(
            OptimizationPriority::LeastNbrCuts,
            Some(OptimizationPriority::MostTiles),
            vec![
                OptimizationPriority::LeastWastedArea,
                OptimizationPriority::LeastNbrMosaics,
            ],
        );
        
        assert_eq!(priorities[0], OptimizationPriority::LeastNbrCuts);
        assert_eq!(priorities[1], OptimizationPriority::MostTiles);
        assert_eq!(priorities[2], OptimizationPriority::LeastWastedArea);
        assert_eq!(priorities[3], OptimizationPriority::LeastNbrMosaics);
        assert_eq!(priorities.len(), 4);
    }
    
    #[test]
    fn test_create_custom_priority_list_no_duplicates() {
        let priorities = PriorityListFactory::create_custom_priority_list(
            OptimizationPriority::MostTiles,
            Some(OptimizationPriority::LeastWastedArea),
            vec![
                OptimizationPriority::MostTiles, // дубликат
                OptimizationPriority::LeastWastedArea, // дубликат
                OptimizationPriority::LeastNbrCuts,
            ],
        );
        
        // Дубликаты должны быть исключены
        assert_eq!(priorities.len(), 3);
        assert_eq!(priorities[0], OptimizationPriority::MostTiles);
        assert_eq!(priorities[1], OptimizationPriority::LeastWastedArea);
        assert_eq!(priorities[2], OptimizationPriority::LeastNbrCuts);
    }
    
    #[test]
    fn test_get_all_priorities() {
        let all_priorities = PriorityListFactory::get_all_priorities();
        assert_eq!(all_priorities.len(), 9);
        
        // Проверяем что все варианты enum включены
        assert!(all_priorities.contains(&OptimizationPriority::MostTiles));
        assert!(all_priorities.contains(&OptimizationPriority::LeastWastedArea));
        assert!(all_priorities.contains(&OptimizationPriority::LeastNbrCuts));
        assert!(all_priorities.contains(&OptimizationPriority::LeastNbrMosaics));
        assert!(all_priorities.contains(&OptimizationPriority::BiggestUnusedTileArea));
        assert!(all_priorities.contains(&OptimizationPriority::MostHVDiscrepancy));
        assert!(all_priorities.contains(&OptimizationPriority::SmallestCenterOfMassDistToOrigin));
        assert!(all_priorities.contains(&OptimizationPriority::LeastNbrUnusedTiles));
        assert!(all_priorities.contains(&OptimizationPriority::MostUnusedPanelArea));
    }
    
    #[test]
    fn test_is_valid_optimization_priority() {
        assert!(PriorityListFactory::is_valid_optimization_priority(0));
        assert!(PriorityListFactory::is_valid_optimization_priority(1));
        assert!(!PriorityListFactory::is_valid_optimization_priority(-1));
        assert!(!PriorityListFactory::is_valid_optimization_priority(2));
        assert!(!PriorityListFactory::is_valid_optimization_priority(999));
    }
}
