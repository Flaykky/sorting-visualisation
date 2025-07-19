use std::fs;
use std::io;
use rand::seq::SliceRandom;

pub fn generate_full(range: RangeInclusive<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = range.collect();
    v.shuffle(&mut rand::thread_rng());
    v
}

pub fn generate_with_length(length: usize, range: RangeInclusive<i32>, no_repeat: bool) -> Vec<i32> {
    if no_repeat {
        let mut all: Vec<i32> = (range.start()..=*range.end()).collect();
        if all.len() < length {
            panic!("Not enough unique numbers");
        }
        all.shuffle(&mut rand::thread_rng());
        all.truncate(length);
        all
    } else {
        (0..length).map(|_| rand::random::<i32>().abs() % (range.end() - range.start() + 1) + range.start()).collect()
    }
}

pub fn read_list_from_file(filename: &str) -> Result<Vec<i32>, io::Error> {
    let content = fs::read_to_string(filename)?;
    let numbers: Vec<i32> = content
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    Ok(numbers)
}
