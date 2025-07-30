use std::time::Instant;

pub struct GraphVisualization {
    data: Vec<i32>,
    markers: Vec<bool>,
    visual_time: u128,
    real_time: Instant,
    compares: usize,
    swaps: usize,
    speed: f64,
}

impl GraphVisualization {
    pub fn new(data: Vec<i32>) -> Self {
        Self {
            data,
            markers: vec![false; 0], // будет инициализирован позже
            visual_time: 0,
            real_time: Instant::now(),
            compares: 0,
            swaps: 0,
            speed: 1.0,
        }
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn update_data(&mut self, data: Vec<i32>) {
        self.data = data;
    }

    pub fn set_markers(&mut self, markers: Vec<bool>) {
        self.markers = markers;
    }

    pub fn increment_compares(&mut self) {
        self.compares += 1;
    }

    pub fn increment_swaps(&mut self) {
        self.swaps += 1;
    }

    pub fn update_visual_time(&mut self, millis: u128) {
        self.visual_time = millis;
    }

    pub fn reset_stats(&mut self) {
        self.compares = 0;
        self.swaps = 0;
        self.visual_time = 0;
        self.real_time = Instant::now();
    }

    pub fn render(&self) {
        // Очищаем экран (простой способ для терминала)
        print!("{}[2J{}[H", 27 as char, 27 as char);

        let max_height = self.data.iter().max().copied().unwrap_or(0);
        let max_height = if max_height > 0 { max_height } else { 1 };
        let width = self.data.len();

        // Создаем сетку для визуализации
        let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; max_height as usize];

        // Рисуем столбцы
        for (i, &value) in self.data.iter().enumerate() {
            let height = value as usize;
            for j in 0..height {
                let row = max_height as usize - 1 - j;
                grid[row][i] = '█';
            }
        }

        // Добавляем маркеры, если есть
        if self.markers.len() == width {
            for (i, &marked) in self.markers.iter().enumerate() {
                if marked && max_height > 0 {
                    let marker_row = max_height as usize - 1;
                    if marker_row < grid.len() {
                        grid[marker_row][i] = '-';
                    }
                }
            }
        }

        // Выводим сетку
        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }

        // Выводим разделительную линию
        println!("{}", "-".repeat(width.max(40)));

        // Выводим статистику
        let real_elapsed = self.real_time.elapsed().as_millis();
        print!("visual time: {}ms | real time: {}ms | compares: {} | swaps: {}",
               self.visual_time,
               real_elapsed,
               self.compares,
               self.swaps);

        if self.speed != 1.0 {
            print!(" | speed: {:.2}x", self.speed);
        }
        println!();

        println!("{}", "-".repeat(width.max(40)));
        println!();
    }

    pub fn render_step(&mut self, data: Vec<i32>, markers: Vec<bool>) {
        self.update_data(data);
        self.set_markers(markers);
        self.render();
        
        // Искусственное замедление для наглядности
        if self.speed > 0.0 && self.speed < 1.0 {
            let delay = (100.0 / self.speed) as u64;
            std::thread::sleep(std::time::Duration::from_millis(delay));
        } else if self.speed > 1.0 {
            let delay = (100.0 / self.speed) as u64;
            std::thread::sleep(std::time::Duration::from_millis(delay));
        }
    }

    pub fn final_render(&self) {
        print!("{}[2J{}[H", 27 as char, 27 as char);
        
        let max_height = self.data.iter().max().copied().unwrap_or(0);
        let max_height = if max_height > 0 { max_height } else { 1 };
        let width = self.data.len();

        let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; max_height as usize];

        for (i, &value) in self.data.iter().enumerate() {
            let height = value as usize;
            for j in 0..height {
                let row = max_height as usize - 1 - j;
                grid[row][i] = '█';
            }
        }

        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }

        println!("{}", "-".repeat(width.max(40)));
        let real_elapsed = self.real_time.elapsed().as_millis();
        println!("FINAL: visual time: {}ms | real time: {}ms | compares: {} | swaps: {}",
                 self.visual_time,
                 real_elapsed,
                 self.compares,
                 self.swaps);
        println!("{}", "-".repeat(width.max(40)));
        println!("-> сортировка завершена!");
    }
}

impl Default for GraphVisualization {
    fn default() -> Self {
        Self::new(vec![])
    }
}
