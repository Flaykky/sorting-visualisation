use std::ops::RangeInclusive;

#[derive(Debug)]
pub enum Command {
    Help,
    Randomize,
    GenerateFull(RangeInclusive<i32>),
    GenerateLength(usize, RangeInclusive<i32>, bool),
    List,
    Graphs,
    Speed(f64),
    Quicksort,
    Mergesort,
    Timsort,
    Radix,
    Heapsort,
    ReadList(String),
}

pub fn parse_command(input: &str) -> Option<Command> {
    let input = input.trim();
    if input.starts_with('.') {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }
        match parts[0] {
            ".help" => Some(Command::Help),
            ".randomize" => Some(Command::Randomize),
            ".list" => Some(Command::List),
            ".graphs" => Some(Command::Graphs),
            ".quicksort" => Some(Command::Quicksort),
            ".mergesort" => Some(Command::Mergesort),
            ".timsort" => Some(Command::Timsort),
            ".radix" => Some(Command::Radix),
            ".heapsort" => Some(Command::Heapsort),
            ".speed" => {
                if parts.len() < 2 {
                    return None;
                }
                let speed = parts[1].parse().ok()?;
                Some(Command::Speed(speed))
            },
            ".generate" => {
                if parts.len() < 2 {
                    return None;
                }
                let first = parts[1];
                if first == "full" {
                    if parts.len() < 3 {
                        return None;
                    }
                    let range_parts: Vec<&str> = parts[2].split('-').collect();
                    if range_parts.len() != 2 {
                        return None;
                    }
                    let start = range_parts[0].parse().ok()?;
                    let end = range_parts[1].parse().ok()?;
                    Some(Command::GenerateFull(start..=end))
                } else {
                    let length = first.parse().ok()?;
                    if parts.len() < 3 {
                        return None;
                    }
                    let range_parts: Vec<&str> = parts[2].split('-').collect();
                    if range_parts.len() != 2 {
                        return None;
                    }
                    let start = range_parts[0].parse().ok()?;
                    let end = range_parts[1].parse().ok()?;
                    let no_repeat = parts.get(3).map_or(false, |&s| s == "nr");
                    Some(Command::GenerateLength(length, start..=end, no_repeat))
                }
            },
            ".readList" => {
                if parts.len() < 2 {
                    None
                } else {
                    Some(Command::ReadList(parts[1].to_string()))
                }
            },
            _ => None,
        }
    } else {
        None
    }
}
