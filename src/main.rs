mod sort;
mod visualization;
mod command;
mod utils;

use crate::sort::*;
use crate::visualization::*;
use crate::command::*;
use crate::utils::*;

use std::io;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct App {
     Vec<i32>,
    visualization_mode: VisualizationMode,
    speed: Duration,
}

#[derive(Debug, Clone, Copy)]
enum VisualizationMode {
    List,
    Graphs,
}

fn main() {
    let mut app = App {
         vec![5, 2, 4, 6, 3, 10, 7, 1],
        visualization_mode: VisualizationMode::List,
        speed: Duration::from_millis(100),
    };

    loop {
        match app.visualization_mode {
            VisualizationMode::List => list_visualization(&app.data, &[]),
            VisualizationMode::Graphs => graph_visualization(&app.data, &[]),
        }

        println!("Enter command (e.g., .quicksort, .randomize, .help):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let command = parse_command(&input);
        match command {
            Some(Command::Help) => println!("Available commands: .randomize, .generate, .list, .graphs, .speed, .quicksort, .mergesort, .timsort, .radix, .heapsort, .readList"),
            Some(Command::Randomize) => randomize_data(&mut app.data),
            Some(Command::GenerateFull(range)) => {
                app.data = generate_full(range);
            },
            Some(Command::GenerateLength(length, range, no_repeat)) => {
                app.data = generate_with_length(length, range, no_repeat);
            },
            Some(Command::List) => app.visualization_mode = VisualizationMode::List,
            Some(Command::Graphs) => app.visualization_mode = VisualizationMode::Graphs,
            Some(Command::Speed(s)) => {
                app.speed = Duration::from_secs_f64(s);
            },
            Some(Command::Quicksort) => {
                quicksort(&mut app.data, &|arr, indices| {
                    visualize(&app, arr, indices);
                });
            },
            Some(Command::Mergesort) => {
                mergesort(&mut app.data, &|arr, indices| {
                    visualize(&app, arr, indices);
                });
            },
            Some(Command::Timsort) => {
                timsort(&mut app.data, &|arr, indices| {
                    visualize(&app, arr, indices);
                });
            },
            Some(Command::Radix) => {
                radixsort(&mut app.data, &|arr, indices| {
                    visualize(&app, arr, indices);
                });
            },
            Some(Command::Heapsort) => {
                heapsort(&mut app.data, &|arr, indices| {
                    visualize(&app, arr, indices);
                });
            },
            Some(Command::ReadList(filename)) => {
                if let Ok(data) = read_list_from_file(&filename) {
                    app.data = data;
                } else {
                    eprintln!("Failed to read file {}", filename);
                }
            },
            None => println!("Unknown command"),
        }
    }
}

fn visualize(app: &App,  &[i32], indices: &[usize]) {
    match app.visualization_mode {
        VisualizationMode::List => list_visualization(data, indices),
        VisualizationMode::Graphs => graph_visualization(data, indices),
    }
    thread::sleep(app.speed);
}

fn randomize_data( &mut Vec<i32>) {
    data.shuffle(&mut rand::thread_rng());
}
