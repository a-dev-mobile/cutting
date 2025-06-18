use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::collections::HashSet;
use crate::engine::model::{solution::Solution, tile::TileDimensions};
use crate::engine::stock::stock_solution::StockSolution;
use crate::engine::comparator::SolutionComparator;
use crate::error::CuttingError;

/// Направление первого разреза
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CutDirection {
    /// Горизонтальный разрез
    Horizontal,
    /// Вертикальный разрез
    Vertical,
    /// Оба направления (пробовать оба варианта)
    Both,
}

/// Статус выполнения потока
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadStatus {
    /// В очереди на выполнение
    Queued,
    /// Выполняется
    Running,
    /// Завершен успешно
    Finished,
    /// Прерван
    Terminated,
    /// Ошибка выполнения
    Error,
}

/// Поток для выполнения оптимизации раскроя
/// 
/// Реализует алгоритм размещения деталей на стоковых панелях
/// с учетом различных параметров оптимизации.
pub struct CutListThread {
    /// Группа потока для идентификации
    group: Option<String>,
    
    /// Дополнительная информация
    aux_info: Option<String>,
    
    /// Список всех решений (общий для всех потоков)
    all_solutions: Arc<Mutex<Vec<Solution>>>,
    
    /// Список деталей для размещения
    tiles: Vec<TileDimensions>,
    
    /// Учитывать ли направление волокон
    consider_grain_direction: bool,
    
    /// Толщина разреза
    cut_thickness: u32,
    
    /// Минимальный размер обрезки
    min_trim_dimension: u32,
    
    /// Направление первого разреза
    first_cut_orientation: CutDirection,
    
    /// Компараторы для сортировки решений в потоке
    thread_prioritized_comparators: Vec<Box<dyn SolutionComparator>>,
    
    /// Компараторы для финальной сортировки решений
    final_solution_prioritized_comparators: Vec<Box<dyn SolutionComparator>>,
    
    /// Фактор точности (количество лучших решений для сохранения)
    accuracy_factor: usize,
    
    /// Стоковое решение (исходные панели)
    stock_solution: StockSolution,
    
    /// Статус выполнения потока
    status: ThreadStatus,
    
    /// Процент выполнения
    percentage_done: u8,
    
    /// Время начала выполнения
    start_time: Option<Instant>,
    
    /// Локальные решения потока
    solutions: Vec<Solution>,
}

impl CutListThread {
    /// Создает новый поток с настройками по умолчанию
    pub fn new() -> Self {
        Self {
            group: None,
            aux_info: None,
            all_solutions: Arc::new(Mutex::new(Vec::new())),
            tiles: Vec::new(),
            consider_grain_direction: false,
            cut_thickness: 0,
            min_trim_dimension: 0,
            first_cut_orientation: CutDirection::Both,
            thread_prioritized_comparators: Vec::new(),
            final_solution_prioritized_comparators: Vec::new(),
            accuracy_factor: 100,
            stock_solution: StockSolution::new(Vec::new()),
            status: ThreadStatus::Queued,
            percentage_done: 0,
            start_time: None,
            solutions: Vec::new(),
        }
    }
    
    /// Выполняет вычисление решений
    /// 
    /// Основной алгоритм размещения деталей на стоковых панелях
    /// с генерацией и оценкой различных вариантов раскроя.
    pub fn compute_solutions(&mut self) -> Result<(), CuttingError> {
        self.status = ThreadStatus::Running;
        self.start_time = Some(Instant::now());
        
        // Создаем начальное решение со стоковыми панелями
        let mut current_solutions = vec![Solution::from_stock_solution(&self.stock_solution)];
        
        // Обрабатываем каждую деталь
        for (tile_index, tile) in self.tiles.iter().enumerate() {
            // Обновляем прогресс
            if tile_index % 3 == 0 {
                self.percentage_done = ((tile_index as f32 / self.tiles.len() as f32) * 100.0) as u8;
            }
            
            let mut new_solutions = Vec::new();
            
            // Пробуем разместить деталь в каждом текущем решении
            for mut solution in current_solutions {
                // Используем встроенный метод try_place_tile из Solution
                match solution.try_place_tile(tile) {
                    Ok(placement_solutions) => {
                        for mut new_solution in placement_solutions {
                            // Устанавливаем метаданные
                            if let Some(ref group) = self.group {
                                new_solution.set_creator_thread_group(group.clone());
                            }
                            if let Some(ref aux_info) = self.aux_info {
                                new_solution.set_aux_info(aux_info.clone());
                            }
                            new_solutions.push(new_solution);
                        }
                    }
                    Err(_) => {
                        // Если не удалось разместить, добавляем исходное решение
                        new_solutions.push(solution);
                    }
                }
            }
            
            // Обновляем текущие решения
            current_solutions = new_solutions;
            
            // Удаляем дубликаты
            self.remove_duplicated(&mut current_solutions);
            
            // Сортируем и ограничиваем количество решений
            self.sort_solutions(&mut current_solutions, &self.thread_prioritized_comparators);
            
            if current_solutions.len() > self.accuracy_factor {
                current_solutions.truncate(self.accuracy_factor);
            }
        }
        
        // Сохраняем локальные решения
        self.solutions = current_solutions.clone();
        
        // Добавляем решения в общий список
        if let Ok(mut all_solutions_guard) = self.all_solutions.lock() {
            all_solutions_guard.extend(current_solutions);
            
            // Сортируем общий список
            self.sort_solutions(&mut *all_solutions_guard, &self.final_solution_prioritized_comparators);
            
            // Ограничиваем размер общего списка
            if all_solutions_guard.len() > self.accuracy_factor {
                all_solutions_guard.truncate(self.accuracy_factor);
            }
        }
        
        self.status = ThreadStatus::Finished;
        self.percentage_done = 100;
        
        Ok(())
    }
    
    /// Удаляет дублирующиеся решения
    fn remove_duplicated(&self, solutions: &mut Vec<Solution>) -> usize {
        let mut seen = HashSet::new();
        let mut to_remove = Vec::new();
        
        for (index, solution) in solutions.iter().enumerate() {
            let identifier = solution.get_structure_identifier();
            if !seen.insert(identifier) {
                to_remove.push(index);
            }
        }
        
        // Удаляем в обратном порядке, чтобы индексы оставались валидными
        for &index in to_remove.iter().rev() {
            solutions.remove(index);
        }
        
        to_remove.len()
    }
    
    /// Сортирует решения с использованием компараторов
    fn sort_solutions(&self, solutions: &mut Vec<Solution>, comparators: &[Box<dyn SolutionComparator>]) {
        solutions.sort_by(|a, b| {
            for comparator in comparators {
                let result = comparator.compare(a, b);
                if result != std::cmp::Ordering::Equal {
                    return result;
                }
            }
            std::cmp::Ordering::Equal
        });
    }
    
    // Геттеры и сеттеры
    pub fn get_group(&self) -> Option<&String> {
        self.group.as_ref()
    }
    
    pub fn set_group(&mut self, group: Option<String>) {
        self.group = group;
    }
    
    pub fn get_aux_info(&self) -> Option<&String> {
        self.aux_info.as_ref()
    }
    
    pub fn set_aux_info(&mut self, aux_info: Option<String>) {
        self.aux_info = aux_info;
    }
    
    pub fn get_tiles(&self) -> &Vec<TileDimensions> {
        &self.tiles
    }
    
    pub fn set_tiles(&mut self, tiles: Vec<TileDimensions>) {
        self.tiles = tiles;
    }
    
    pub fn is_consider_grain_direction(&self) -> bool {
        self.consider_grain_direction
    }
    
    pub fn set_consider_grain_direction(&mut self, consider: bool) {
        self.consider_grain_direction = consider;
    }
    
    pub fn get_cut_thickness(&self) -> u32 {
        self.cut_thickness
    }
    
    pub fn set_cut_thickness(&mut self, thickness: u32) {
        self.cut_thickness = thickness;
    }
    
    pub fn get_min_trim_dimension(&self) -> u32 {
        self.min_trim_dimension
    }
    
    pub fn set_min_trim_dimension(&mut self, min_trim: u32) {
        self.min_trim_dimension = min_trim;
    }
    
    pub fn get_first_cut_orientation(&self) -> CutDirection {
        self.first_cut_orientation
    }
    
    pub fn set_first_cut_orientation(&mut self, orientation: CutDirection) {
        self.first_cut_orientation = orientation;
    }
    
    pub fn get_accuracy_factor(&self) -> usize {
        self.accuracy_factor
    }
    
    pub fn set_accuracy_factor(&mut self, factor: usize) {
        self.accuracy_factor = factor;
    }
    
    pub fn get_stock_solution(&self) -> &StockSolution {
        &self.stock_solution
    }
    
    pub fn set_stock_solution(&mut self, stock_solution: StockSolution) {
        self.stock_solution = stock_solution;
    }
    
    pub fn get_status(&self) -> ThreadStatus {
        self.status
    }
    
    pub fn get_percentage_done(&self) -> u8 {
        self.percentage_done
    }
    
    pub fn get_solutions(&self) -> &Vec<Solution> {
        &self.solutions
    }
    
    pub fn get_elapsed_time_millis(&self) -> u64 {
        if let Some(start_time) = self.start_time {
            start_time.elapsed().as_millis() as u64
        } else {
            0
        }
    }
    
    pub fn set_all_solutions(&mut self, all_solutions: Arc<Mutex<Vec<Solution>>>) {
        self.all_solutions = all_solutions;
    }
    
    pub fn set_thread_prioritized_comparators(&mut self, comparators: Vec<Box<dyn SolutionComparator>>) {
        self.thread_prioritized_comparators = comparators;
    }
    
    pub fn set_final_solution_prioritized_comparators(&mut self, comparators: Vec<Box<dyn SolutionComparator>>) {
        self.final_solution_prioritized_comparators = comparators;
    }
}

impl Default for CutListThread {
    fn default() -> Self {
        Self::new()
    }
}

/// Строитель для создания настроенных экземпляров CutListThread
/// 
/// Реализует паттерн Builder для удобного создания потоков
/// с различными конфигурациями параметров.
pub struct CutListThreadBuilder {
    group: Option<String>,
    aux_info: Option<String>,
    all_solutions: Option<Arc<Mutex<Vec<Solution>>>>,
    tiles: Option<Vec<TileDimensions>>,
    consider_grain_direction: bool,
    cut_thickness: u32,
    min_trim_dimension: u32,
    first_cut_orientation: CutDirection,
    thread_prioritized_comparators: Option<Vec<Box<dyn SolutionComparator>>>,
    final_solution_prioritized_comparators: Option<Vec<Box<dyn SolutionComparator>>>,
    accuracy_factor: usize,
    stock_solution: Option<StockSolution>,
}

impl CutListThreadBuilder {
    /// Создает новый строитель с настройками по умолчанию
    pub fn new() -> Self {
        Self {
            group: None,
            aux_info: None,
            all_solutions: None,
            tiles: None,
            consider_grain_direction: false,
            cut_thickness: 0,
            min_trim_dimension: 0,
            first_cut_orientation: CutDirection::Both,
            thread_prioritized_comparators: None,
            final_solution_prioritized_comparators: None,
            accuracy_factor: 100,
            stock_solution: None,
        }
    }
    
    /// Устанавливает группу потока
    pub fn set_group(mut self, group: String) -> Self {
        self.group = Some(group);
        self
    }
    
    /// Устанавливает дополнительную информацию
    pub fn set_aux_info(mut self, aux_info: String) -> Self {
        self.aux_info = Some(aux_info);
        self
    }
    
    /// Устанавливает общий список решений
    pub fn set_all_solutions(mut self, all_solutions: Arc<Mutex<Vec<Solution>>>) -> Self {
        self.all_solutions = Some(all_solutions);
        self
    }
    
    /// Устанавливает список деталей
    pub fn set_tiles(mut self, tiles: Vec<TileDimensions>) -> Self {
        self.tiles = Some(tiles);
        self
    }
    
    /// Устанавливает флаг учета направления волокон
    pub fn set_consider_grain_direction(mut self, consider: bool) -> Self {
        self.consider_grain_direction = consider;
        self
    }
    
    /// Устанавливает толщину разреза
    pub fn set_cut_thickness(mut self, thickness: u32) -> Self {
        self.cut_thickness = thickness;
        self
    }
    
    /// Устанавливает минимальный размер обрезки
    pub fn set_min_trim_dimension(mut self, min_trim: u32) -> Self {
        self.min_trim_dimension = min_trim;
        self
    }
    
    /// Устанавливает направление первого разреза
    pub fn set_first_cut_orientation(mut self, orientation: CutDirection) -> Self {
        self.first_cut_orientation = orientation;
        self
    }
    
    /// Устанавливает компараторы для сортировки в потоке
    pub fn set_thread_prioritized_comparators(mut self, comparators: Vec<Box<dyn SolutionComparator>>) -> Self {
        self.thread_prioritized_comparators = Some(comparators);
        self
    }
    
    /// Устанавливает компараторы для финальной сортировки
    pub fn set_final_solution_prioritized_comparators(mut self, comparators: Vec<Box<dyn SolutionComparator>>) -> Self {
        self.final_solution_prioritized_comparators = Some(comparators);
        self
    }
    
    /// Устанавливает фактор точности
    pub fn set_accuracy_factor(mut self, factor: usize) -> Self {
        self.accuracy_factor = factor;
        self
    }
    
    /// Устанавливает стоковое решение
    pub fn set_stock_solution(mut self, stock_solution: StockSolution) -> Self {
        self.stock_solution = Some(stock_solution);
        self
    }
    
    /// Создает настроенный экземпляр CutListThread
    /// 
    /// # Возвращает
    /// Result с созданным потоком или ошибкой при отсутствии обязательных параметров
    pub fn build(self) -> Result<CutListThread, CuttingError> {
        let mut thread = CutListThread::new();
        
        thread.set_group(self.group);
        thread.set_aux_info(self.aux_info);
        
        if let Some(all_solutions) = self.all_solutions {
            thread.set_all_solutions(all_solutions);
        }
        
        if let Some(tiles) = self.tiles {
            thread.set_tiles(tiles);
        }
        
        thread.set_consider_grain_direction(self.consider_grain_direction);
        thread.set_cut_thickness(self.cut_thickness);
        thread.set_min_trim_dimension(self.min_trim_dimension);
        thread.set_first_cut_orientation(self.first_cut_orientation);
        
        if let Some(comparators) = self.thread_prioritized_comparators {
            thread.set_thread_prioritized_comparators(comparators);
        }
        
        if let Some(comparators) = self.final_solution_prioritized_comparators {
            thread.set_final_solution_prioritized_comparators(comparators);
        }
        
        thread.set_accuracy_factor(self.accuracy_factor);
        
        if let Some(stock_solution) = self.stock_solution {
            thread.set_stock_solution(stock_solution);
        }
        
        Ok(thread)
    }
}

impl Default for CutListThreadBuilder {
    fn default() -> Self {
        Self::new()
    }
}
