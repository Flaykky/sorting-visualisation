use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

/// Генерирует случайный вектор чисел
pub fn generate_random_list(count: Option<usize>, min: i32, max: i32, no_repeats: bool) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let count = count.unwrap_or((max - min + 1) as usize);
    
    if no_repeats {
        let mut used = HashSet::new();
        let mut result = Vec::with_capacity(count);
        let range_size = (max - min + 1) as usize;
        
        if count > range_size {
            // Если запрошено больше уникальных чисел, чем доступно в диапазоне
            for i in min..=max {
                result.push(i);
            }
            // Добавляем случайные числа из диапазона до нужного количества
            while result.len() < count {
                let num = rng.gen_range(min..=max);
                result.push(num);
            }
        } else {
            while result.len() < count {
                let num = rng.gen_range(min..=max);
                if used.insert(num) {
                    result.push(num);
                }
            }
        }
        result
    } else {
        (0..count)
            .map(|_| rng.gen_range(min..=max))
            .collect()
    }
}

/// Читает список чисел из файла
pub fn read_list_from_file(filename: &str) -> Result<Vec<i32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        for num_str in line.split_whitespace() {
            if let Ok(num) = num_str.parse::<i32>() {
                numbers.push(num);
            }
        }
    }
    
    Ok(numbers)
}

/// Проверяет, отсортирован ли вектор
pub fn is_sorted(data: &[i32]) -> bool {
    data.windows(2).all(|w| w[0] <= w[1])
}

/// Создает вектор маркеров для визуализации
pub fn create_markers(len: usize, active_indices: &[usize]) -> Vec<bool> {
    let mut markers = vec![false; len];
    for &index in active_indices {
        if index < len {
            markers[index] = true;
        }
    }
    markers
}

/// Клонирует вектор с возможностью изменения отдельных элементов
pub fn clone_with_modifications(data: &[i32], modifications: &[(usize, i32)]) -> Vec<i32> {
    let mut result = data.to_vec();
    for &(index, value) in modifications {
        if index < result.len() {
            result[index] = value;
        }
    }
    result
}

/// Находит минимальный и максимальный элементы в векторе
pub fn find_min_max(data: &[i32]) -> (i32, i32) {
    if data.is_empty() {
        return (0, 0);
    }
    
    let mut min = data[0];
    let mut max = data[0];
    
    for &value in data.iter().skip(1) {
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }
    
    (min, max)
}

/// Проверяет, содержит ли вектор только уникальные элементы
pub fn has_unique_elements(data: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for &item in data {
        if !seen.insert(item) {
            return false;
        }
    }
    true
}

/// Преобразует вектор в строку для отладки
pub fn vec_to_string(data: &[i32]) -> String {
    format!("[{}]", data.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(", "))
}

/// Получает подвектор по индексам
pub fn get_subvector(data: &[i32], start: usize, end: usize) -> Vec<i32> {
    if start >= data.len() || end > data.len() || start >= end {
        return Vec::new();
    }
    data[start..end].to_vec()
}

/// Объединяет два вектора
pub fn merge_vectors(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = left;
    result.extend(right);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_list() {
        let list = generate_random_list(Some(10), 1, 100, false);
        assert_eq!(list.len(), 10);
    }

    #[test]
    fn test_is_sorted() {
        assert!(is_sorted(&[1, 2, 3, 4, 5]));
        assert!(!is_sorted(&[5, 2, 3, 1, 4]));
    }

    #[test]
    fn test_create_markers() {
        let markers = create_markers(5, &[0, 2, 4]);
        assert_eq!(markers, vec![true, false, true, false, true]);
    }
}
