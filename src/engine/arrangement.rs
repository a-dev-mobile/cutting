use std::collections::VecDeque;

/// Генератор перестановок для оптимизации порядка размещения деталей
pub struct Arrangement;

impl Arrangement {
    /// Генерирует все возможные перестановки списка элементов
    /// 
    /// Использует рекурсивный алгоритм для генерации всех перестановок.
    /// Сложность: O(n! * n) по времени, O(n! * n) по памяти.
    /// 
    /// # Аргументы
    /// * `list` - исходный список элементов для перестановки
    /// 
    /// # Возвращает
    /// Вектор всех возможных перестановок
    /// 
    /// # Примеры
    /// ```
    /// use cutting_cli::engine::arrangement::Arrangement;
    /// 
    /// let items = vec![1, 2, 3];
    /// let permutations = Arrangement::generate_permutations(items);
    /// assert_eq!(permutations.len(), 6); // 3! = 6
    /// ```
    pub fn generate_permutations<T>(list: Vec<T>) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        if list.is_empty() {
            return vec![vec![]];
        }
        
        if list.len() == 1 {
            return vec![list];
        }
        
        Self::generate_permutations_recursive(list)
    }
    
    /// Рекурсивная генерация перестановок
    fn generate_permutations_recursive<T>(mut list: Vec<T>) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        if list.len() <= 1 {
            return vec![list];
        }
        
        let mut result = Vec::new();
        
        // Для каждого элемента в списке
        for i in 0..list.len() {
            // Извлекаем элемент
            let element = list.remove(i);
            
            // Генерируем перестановки оставшихся элементов
            let sub_permutations = Self::generate_permutations_recursive(list.clone());
            
            // Добавляем текущий элемент в начало каждой перестановки
            for mut perm in sub_permutations {
                perm.insert(0, element.clone());
                result.push(perm);
            }
            
            // Возвращаем элемент обратно в список
            list.insert(i, element);
        }
        
        result
    }
    
    /// Генерирует перестановки с ограничением количества
    /// 
    /// Полезно для больших списков, когда нужно ограничить количество
    /// генерируемых перестановок для экономии времени и памяти.
    /// 
    /// # Аргументы
    /// * `list` - исходный список элементов
    /// * `max_permutations` - максимальное количество перестановок
    /// 
    /// # Возвращает
    /// Вектор перестановок (не более max_permutations)
    pub fn generate_limited_permutations<T>(list: Vec<T>, max_permutations: usize) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        if list.is_empty() || max_permutations == 0 {
            return vec![];
        }
        
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        
        // Начинаем с исходного списка
        queue.push_back((list, Vec::new()));
        
        while !queue.is_empty() && result.len() < max_permutations {
            let (remaining, current_perm) = queue.pop_front().unwrap();
            
            if remaining.is_empty() {
                // Перестановка готова
                result.push(current_perm);
            } else {
                // Генерируем следующий уровень
                for i in 0..remaining.len() {
                    let mut new_remaining = remaining.clone();
                    let element = new_remaining.remove(i);
                    
                    let mut new_perm = current_perm.clone();
                    new_perm.push(element);
                    
                    queue.push_back((new_remaining, new_perm));
                }
            }
        }
        
        result
    }
    
    /// Генерирует случайные перестановки
    /// 
    /// Использует алгоритм Fisher-Yates для генерации случайных перестановок.
    /// Полезно когда нужно быстро получить разнообразные варианты размещения.
    /// 
    /// # Аргументы
    /// * `list` - исходный список элементов
    /// * `count` - количество случайных перестановок
    /// 
    /// # Возвращает
    /// Вектор случайных перестановок
    pub fn generate_random_permutations<T>(list: Vec<T>, count: usize) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        if list.is_empty() || count == 0 {
            return vec![];
        }
        
        let mut result = Vec::new();
        let mut seed = 12345u64; // Простой PRNG seed
        
        for _ in 0..count {
            let mut perm = list.clone();
            
            // Fisher-Yates shuffle с простым PRNG
            for i in (1..perm.len()).rev() {
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                let j = (seed as usize) % (i + 1);
                perm.swap(i, j);
            }
            
            result.push(perm);
        }
        
        result
    }
    
    /// Генерирует эвристически оптимизированные перестановки
    /// 
    /// Применяет эвристики для генерации перестановок, которые
    /// с большей вероятностью дадут хорошие результаты размещения.
    /// 
    /// # Аргументы
    /// * `list` - исходный список элементов
    /// * `area_fn` - функция для вычисления площади элемента
    /// * `max_permutations` - максимальное количество перестановок
    /// 
    /// # Возвращает
    /// Вектор эвристически оптимизированных перестановок
    pub fn generate_heuristic_permutations<T, F>(
        list: Vec<T>,
        area_fn: F,
        max_permutations: usize,
    ) -> Vec<Vec<T>>
    where
        T: Clone,
        F: Fn(&T) -> u64,
    {
        if list.is_empty() || max_permutations == 0 {
            return vec![];
        }
        
        let mut result = Vec::new();
        
        // 1. Сортировка по убыванию площади (largest first)
        let mut sorted_by_area = list.clone();
        sorted_by_area.sort_by(|a, b| area_fn(b).cmp(&area_fn(a)));
        result.push(sorted_by_area);
        
        if result.len() >= max_permutations {
            return result;
        }
        
        // 2. Сортировка по возрастанию площади (smallest first)
        let mut sorted_by_area_asc = list.clone();
        sorted_by_area_asc.sort_by(|a, b| area_fn(a).cmp(&area_fn(b)));
        result.push(sorted_by_area_asc);
        
        if result.len() >= max_permutations {
            return result;
        }
        
        // 3. Чередование больших и маленьких элементов
        let mut alternating = Vec::new();
        let mut large_items = list.clone();
        large_items.sort_by(|a, b| area_fn(b).cmp(&area_fn(a)));
        
        let mid = large_items.len() / 2;
        let mut large_iter = large_items[..mid].iter();
        let mut small_iter = large_items[mid..].iter().rev();
        
        loop {
            match (large_iter.next(), small_iter.next()) {
                (Some(large), Some(small)) => {
                    alternating.push(large.clone());
                    alternating.push(small.clone());
                }
                (Some(large), None) => {
                    alternating.push(large.clone());
                }
                (None, Some(small)) => {
                    alternating.push(small.clone());
                }
                (None, None) => break,
            }
        }
        result.push(alternating);
        
        if result.len() >= max_permutations {
            return result;
        }
        
        // 4. Добавляем случайные перестановки для разнообразия
        let remaining_count = max_permutations - result.len();
        let random_perms = Self::generate_random_permutations(list, remaining_count);
        result.extend(random_perms);
        
        result
    }
    
    /// Вычисляет факториал числа
    /// 
    /// # Аргументы
    /// * `n` - число для вычисления факториала
    /// 
    /// # Возвращает
    /// Факториал числа или None при переполнении
    pub fn factorial(n: usize) -> Option<usize> {
        if n > 20 {
            // Предотвращаем переполнение для больших чисел
            return None;
        }
        
        let mut result: usize = 1;
        for i in 2..=n {
            result = result.checked_mul(i)?;
        }
        Some(result)
    }
    
    /// Оценивает количество перестановок, которые будут сгенерированы
    /// 
    /// # Аргументы
    /// * `list_size` - размер списка
    /// 
    /// # Возвращает
    /// Количество перестановок или None, если слишком много
    pub fn estimate_permutation_count(list_size: usize) -> Option<usize> {
        Self::factorial(list_size)
    }
    
    /// Проверяет, разумно ли генерировать все перестановки
    /// 
    /// # Аргументы
    /// * `list_size` - размер списка
    /// * `max_reasonable_count` - максимальное разумное количество перестановок
    /// 
    /// # Возвращает
    /// true, если генерация всех перестановок разумна
    pub fn is_full_generation_reasonable(list_size: usize, max_reasonable_count: usize) -> bool {
        if let Some(count) = Self::estimate_permutation_count(list_size) {
            count <= max_reasonable_count
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_permutations_empty() {
        let empty: Vec<i32> = vec![];
        let permutations = Arrangement::generate_permutations(empty);
        assert_eq!(permutations.len(), 1);
        assert_eq!(permutations[0].len(), 0);
    }
    
    #[test]
    fn test_generate_permutations_single() {
        let single = vec![42];
        let permutations = Arrangement::generate_permutations(single);
        assert_eq!(permutations.len(), 1);
        assert_eq!(permutations[0], vec![42]);
    }
    
    #[test]
    fn test_generate_permutations_three_elements() {
        let items = vec!['A', 'B', 'C'];
        let permutations = Arrangement::generate_permutations(items);
        
        assert_eq!(permutations.len(), 6); // 3! = 6
        
        // Проверяем, что все перестановки уникальны
        let mut unique_perms = std::collections::HashSet::new();
        for perm in &permutations {
            unique_perms.insert(perm.clone());
        }
        assert_eq!(unique_perms.len(), 6);
        
        // Проверяем, что каждая перестановка содержит все исходные элементы
        for perm in &permutations {
            assert_eq!(perm.len(), 3);
            assert!(perm.contains(&'A'));
            assert!(perm.contains(&'B'));
            assert!(perm.contains(&'C'));
        }
    }
    
    #[test]
    fn test_generate_limited_permutations() {
        let items = vec![1, 2, 3, 4];
        let permutations = Arrangement::generate_limited_permutations(items, 10);
        
        assert!(permutations.len() <= 10);
        
        // Проверяем, что все перестановки валидны
        for perm in &permutations {
            assert_eq!(perm.len(), 4);
            let mut sorted_perm = perm.clone();
            sorted_perm.sort();
            assert_eq!(sorted_perm, vec![1, 2, 3, 4]);
        }
    }
    
    #[test]
    fn test_generate_random_permutations() {
        let items = vec![1, 2, 3, 4, 5];
        let permutations = Arrangement::generate_random_permutations(items, 5);
        
        assert_eq!(permutations.len(), 5);
        
        // Проверяем, что все перестановки валидны
        for perm in &permutations {
            assert_eq!(perm.len(), 5);
            let mut sorted_perm = perm.clone();
            sorted_perm.sort();
            assert_eq!(sorted_perm, vec![1, 2, 3, 4, 5]);
        }
    }
    
    #[test]
    fn test_generate_heuristic_permutations() {
        let items = vec![10, 5, 20, 15]; // Разные "площади"
        let area_fn = |x: &i32| *x as u64;
        let permutations = Arrangement::generate_heuristic_permutations(items, area_fn, 3);
        
        assert!(permutations.len() <= 3);
        assert!(permutations.len() > 0);
        
        // Первая перестановка должна быть отсортирована по убыванию
        assert_eq!(permutations[0], vec![20, 15, 10, 5]);
        
        // Вторая перестановка должна быть отсортирована по возрастанию
        if permutations.len() > 1 {
            assert_eq!(permutations[1], vec![5, 10, 15, 20]);
        }
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(Arrangement::factorial(0), Some(1));
        assert_eq!(Arrangement::factorial(1), Some(1));
        assert_eq!(Arrangement::factorial(3), Some(6));
        assert_eq!(Arrangement::factorial(4), Some(24));
        assert_eq!(Arrangement::factorial(5), Some(120));
        
        // Большие числа должны возвращать None
        assert_eq!(Arrangement::factorial(25), None);
    }
    
    #[test]
    fn test_is_full_generation_reasonable() {
        assert!(Arrangement::is_full_generation_reasonable(3, 10)); // 3! = 6 <= 10
        assert!(Arrangement::is_full_generation_reasonable(4, 100)); // 4! = 24 <= 100
        assert!(!Arrangement::is_full_generation_reasonable(10, 1000)); // 10! > 1000
    }
    
    #[test]
    fn test_estimate_permutation_count() {
        assert_eq!(Arrangement::estimate_permutation_count(3), Some(6));
        assert_eq!(Arrangement::estimate_permutation_count(4), Some(24));
        assert_eq!(Arrangement::estimate_permutation_count(5), Some(120));
        assert_eq!(Arrangement::estimate_permutation_count(25), None);
    }
}
