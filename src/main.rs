use std::io::{self, Write};
use std::time::Instant;

mod command;
mod sort;
mod visualization;
mod utils;

use command::parser::{CommandParser, Command};
use visualization::{GraphVisualization, ListVisualization};
use utils::ArrayUtils;
use sort::*;

struct AppState {
    data: Vec<i32>,
    speed: f64,
    visualization_mode: VisualizationMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum VisualizationMode {
    List,
    Graph,
}

impl Default for VisualizationMode {
    fn default() -> Self {
        VisualizationMode::List
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            data: ArrayUtils::default_array(),
            speed: 1.0,
            visualization_mode: VisualizationMode::default(),
        }
    }
}

impl AppState {
    fn new() -> Self {
        Self::default()
    }

    fn execute_command(&mut self, command: Command) -> Result<(), String> {
        match command {
            Command::Randomize => {
                ArrayUtils::shuffle_array(&mut self.data);
                println!("Массив перемешан случайным образом");
                self.show_current_array();
            }
            Command::Generate { count, range, no_repeats } => {
                let count = count.unwrap_or(10);
                let (min, max) = range;
                self.data = ArrayUtils::generate_random_array(count, min, max, no_repeats);
                println!("Сгенерирован массив из {} элементов в диапазоне {}-{}", 
                        count, min, max);
                self.show_current_array();
            }
            Command::List => {
                self.visualization_mode = VisualizationMode::List;
                println!("Режим визуализации: список");
                self.show_current_array();
            }
            Command::Graphs => {
                self.visualization_mode = VisualizationMode::Graph;
                println!("Режим визуализации: графики");
                self.show_current_array();
            }
            Command::Speed(speed) => {
                self.speed = speed;
                println!("Скорость визуализации установлена на {:.2}x", speed);
            }
            Command::Sort(sort_name) => {
                self.run_sort_by_name(&sort_name)?;
            }
            Command::Compare(sort1, sort2) => {
                self.compare_sorts(&sort1, &sort2)?;
            }
            Command::ReadList(filename) => {
                match utils::ArrayUtils::read_array_from_file(&filename) {
                    Ok(data) => {
                        self.data = data;
                        println!("Массив загружен из файла: {}", filename);
                        self.show_current_array();
                    }
                    Err(e) => {
                        return Err(format!("Ошибка чтения файла {}: {}", filename, e));
                    }
                }
            }
            Command::QuickSort => {
                self.run_sort_by_name("quicksort")?;
            }
            Command::MergeSort => {
                self.run_sort_by_name("mergesort")?;
            }
            Command::TimSort => {
                self.run_sort_by_name("timsort")?;
            }
            Command::RadixSort => {
                self.run_sort_by_name("radix")?;
            }
            Command::HeapSort => {
                self.run_sort_by_name("heapsort")?;
            }
        }
        Ok(())
    }

    fn show_current_array(&self) {
        println!("Текущий массив: {}", utils::ArrayUtils::array_to_string(&self.data));
    }

    fn run_sort_by_name(&mut self, name: &str) -> Result<(), String> {
        let cloned_data = utils::ArrayUtils::clone_array(&self.data);
        
        if utils::ArrayUtils::is_trivial_array(&cloned_data) {
            println!("Массив слишком мал для сортировки");
            return Ok(());
        }

        match name.to_lowercase().as_str() {
            "quicksort" => {
                self.run_sort_with_visualization(cloned_data, |arr, vis| {
                    quicksort::quicksort_with_visualization(arr, vis)
                }, "QuickSort")?;
            }
            "mergesort" => {
                self.run_sort_with_visualization(cloned_data, |arr, vis| {
                    mergesort::mergesort_with_visualization(arr, vis)
                }, "MergeSort")?;
            }
            "timsort" => {
                self.run_sort_with_visualization(cloned_data, |arr, vis| {
                    timsort::timsort_with_visualization(arr, vis)
                }, "TimSort")?;
            }
            "radix" => {
                self.run_sort_with_visualization(cloned_data, |arr, vis| {
                    radix::radix_sort_with_visualization(arr, vis)
                }, "Radix Sort")?;
            }
            "heapsort" => {
                self.run_sort_with_visualization(cloned_data, |arr, vis| {
                    heapsort::heapsort_with_visualization(arr, vis)
                }, "HeapSort")?;
            }
            "bubblesort" => {
                self.run_sort_with_visualization(cloned_data, |arr, vis| {
                    bubblesort::bubblesort_with_visualization(arr, vis)
                }, "BubbleSort")?;
            }
            "insertionsort" => {
                self.run_sort_with_visualization(cloned_data, |arr, vis| {
                    insertionsort::insertion_sort_with_visualization(arr, vis)
                }, "InsertionSort")?;
            }
            _ => {
                return Err(format!("Неизвестный алгоритм сортировки: {}", name));
            }
        }
        Ok(())
    }

    fn run_sort_with_visualization<F>(
        &mut self,
        mut data: Vec<i32>,
        sort_func: F,
        sort_name: &str,
    ) -> Result<(), String>
    where
        F: FnOnce(&mut Vec<i32>, &mut dyn visualization::Visualization),
    {
        println!("Запуск сортировки: {}", sort_name);
        
        match self.visualization_mode {
            VisualizationMode::List => {
                let mut viz = ListVisualization::new(data.clone());
                viz.set_speed(self.speed);
                viz.reset_stats();
                
                let start_time = Instant::now();
                sort_func(&mut data, &mut viz);
                let duration = start_time.elapsed();
                
                viz.update_visual_time(duration.as_millis());
                viz.final_render();
            }
            VisualizationMode::Graph => {
                let mut viz = GraphVisualization::new(data.clone());
                viz.set_speed(self.speed);
                viz.reset_stats();
                
                let start_time = Instant::now();
                sort_func(&mut data, &mut viz);
                let duration = start_time.elapsed();
                
                viz.update_visual_time(duration.as_millis());
                viz.final_render();
            }
        }
        
        // Обновляем основной массив
        self.data = data;
        Ok(())
    }

    fn compare_sorts(&self, sort1: &str, sort2: &str) -> Result<(), String> {
        println!("Сравнение {} и {}:", sort1, sort2);
        
        let data1 = utils::ArrayUtils::clone_array(&self.data);
        let data2 = utils::ArrayUtils::clone_array(&self.data);
        
        if utils::ArrayUtils::is_trivial_array(&data1) {
            println!("Массив слишком мал для сравнения");
            return Ok(());
        }

        // Тест первого алгоритма
        let (time1, compares1, swaps1) = self.test_sort_performance(sort1, data1)?;
        
        // Тест второго алгоритма
        let (time2, compares2, swaps2) = self.test_sort_performance(sort2, data2)?;

        // Вывод результатов
        println!("\nРезультаты сравнения:");
        println!("{}", "=".repeat(50));
        println!("{:<15} {:<12} {:<10} {:<10}", "Алгоритм", "Время", "Сравнения", "Обмены");
        println!("{}", "-".repeat(50));
        println!("{:<15} {:<12} {:<10} {:<10}", sort1, format!("{}ms", time1), compares1, swaps1);
        println!("{:<15} {:<12} {:<10} {:<10}", sort2, format!("{}ms", time2), compares2, swaps2);
        println!("{}", "=".repeat(50));
        
        if time1 < time2 {
            println!("{} быстрее на {}ms", sort1, time2 - time1);
        } else if time2 < time1 {
            println!("{} быстрее на {}ms", sort2, time1 - time2);
        } else {
            println!("Алгоритмы показали примерно одинаковое время");
        }

        Ok(())
    }

    fn test_sort_performance(&self, sort_name: &str, mut data: Vec<i32>) -> Result<(u128, usize, usize), String> {
        let start_time = Instant::now();
        let mut compares = 0;
        let mut swaps = 0;

        match sort_name.to_lowercase().as_str() {
            "quicksort" => {
                quicksort::quicksort(&mut data, &mut compares);
            }
            "mergesort" => {
                mergesort::mergesort(&mut data, &mut compares);
            }
            "timsort" => {
                timsort::timsort(&mut data, &mut compares);
            }
            "radix" => {
                radix::radixsort(&mut data, &mut compares);
            }
            "heapsort" => {
                heapsort::heapsort(&mut data, &mut compares);
            }
            "bubblesort" => {
                sort::bubblesort::sort(&mut data, &mut compares);
            }
            "insertionsort" => {
                sort::insertionsort::sort(&mut data, &mut compares);
            }
            _ => {
                return Err(format!("Неизвестный алгоритм: {}", sort_name));
            }
        }

        let duration = start_time.elapsed();
        Ok((duration.as_millis(), compares, swaps))
    }
}

fn show_help() {
    println!("Доступные команды:");
    println!("  .randomize              - перемешать текущий массив");
    println!("  .generate [count] [min-max] [nr] - генерировать случайный массив");
    println!("  .list                   - режим визуализации: список");
    println!("  .graphs                 - режим визуализации: столбцы");
    println!("  .speed <value>          - установить скорость визуализации");
    println!("  .sort <algorithm>       - сортировать выбранным алгоритмом");
    println!("  .compare <alg1> <alg2>  - сравнить два алгоритма");
    println!("  .readlist <filename>    - загрузить массив из файла");
    println!("  .quicksort              - быстрая сортировка");
    println!("  .mergesort              - сортировка слиянием");
    println!("  .timsort                - сортировка Тима");
    println!("  .radix                  - поразрядная сортировка");
    println!("  .heapsort               - пирамидальная сортировка");
    println!("  .help                   - показать эту справку");
    println!("  .exit                   - выход из программы");
    println!();
    println!("Алгоритмы: quicksort, mergesort, timsort, radix, heapsort, bubblesort, insertionsort");
}

fn main() {
    println!("=== Визуализатор алгоритмов сортировки ===");
    println!("Введите .help для получения списка команд");
    println!();

    let mut app = AppState::new();
    app.show_current_array();

    loop {
        print!("-> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }

                match input {
                    ".exit" | ".quit" => {
                        println!("До свидания!");
                        break;
                    }
                    ".help" => {
                        show_help();
                        continue;
                    }
                    _ => {}
                }

                match CommandParser::parse(input) {
                    Ok(command) => {
                        if let Err(e) = app.execute_command(command) {
                            eprintln!("Ошибка: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Ошибка парсинга команды: {:?}", e);
                        println!("Введите .help для получения списка команд");
                    }
                }
            }
            Err(error) => {
                eprintln!("Ошибка чтения ввода: {}", error);
                break;
            }
        }
    }
}
