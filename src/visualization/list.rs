use std::time::Instant;

pub struct ListVisualization {
    data: Vec<i32>,
    markers: Vec<bool>,
    visual_time: u128,
    real_time: Instant,
    compares: usize,
    swaps: usize,
    speed: f64,
}

impl ListVisualization {
    pub fn new(data: Vec<i32>) -> Self {
        Self {
            data,
            markers: vec![false; 0],
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
        // Очищаем экран
        print!("{}[2J{}[H", 27 as char, 27 as char);

        // Выводим массив
        print!("[");
        for (i, &value) in self.data.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", value);
        }
        println!("]");

        // Выводим маркеры
        if !self.markers.is_empty() && self.markers.len() == self.data.len() {
            print!(" ");
            for (i, &marked) in self.markers.iter().enumerate() {
                if i > 0 {
                    print!("  ");
                }
                if marked {
                    print!("-");
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        // Выводим статистику
        println!("{}", "-".repeat(40));
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
        println!("{}", "-".repeat(40));
        println!("-> ");
    }

    pub fn render_step(&mut self, data: Vec<i32>, markers: Vec<bool>) {
        self.update_data(data);
        self.set_markers(markers);
        self.render();
        
        // Искусственное замедление
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
        print!("[");
        for (i, &value) in self.data.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", value);
        }
        println!("]");
        println!("{}", "-".repeat(40));
        let real_elapsed = self.real_time.elapsed().as_millis();
        println!("FINAL: visual time: {}ms | real time: {}ms | compares: {} | swaps: {}",
                 self.visual_time,
                 real_elapsed,
                 self.compares,
                 self.swaps);
        println!("{}", "-".repeat(40));
        println!("-> сортировка завершена!");
    }
}

impl Default for ListVisualization {
    fn default() -> Self {
        Self::new(vec![])
    }
}
