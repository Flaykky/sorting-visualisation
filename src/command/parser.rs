use std::str::FromStr;
use thiserror::Error;

/// Constants for command strings to avoid magic strings
const CMD_RANDOMIZE: &str = ".randomize";
const CMD_LIST: &str = ".list";
const CMD_GRAPHS: &str = ".graphs";
const CMD_SPEED: &str = ".speed";
const CMD_GENERATE: &str = ".generate";
const CMD_SORT: &str = ".sort";
const CMD_COMPARE: &str = ".compare";
const CMD_READLIST: &str = ".readlist";
const CMD_QUICKSORT: &str = ".quicksort";
const CMD_MERGESORT: &str = ".mergesort";
const CMD_TIMSORT: &str = ".timsort";
const CMD_RADIX: &str = ".radix";
const CMD_HEAPSORT: &str = ".heapsort";

/// Represents all possible commands in the application
#[derive(Debug, Clone)]
pub enum Command {
    Randomize,
    Generate {
        count: Option<usize>,
        range: (i32, i32),
        no_repeats: bool,
    },
    List,
    Graphs,
    Speed(f64),
    Sort(String),
    Compare(String, String),
    ReadList(String),
    QuickSort,
    MergeSort,
    TimSort,
    RadixSort,
    HeapSort,
}

/// Possible errors that can occur during command parsing
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Missing argument for command: {0}")]
    MissingArgument(String),
    #[error("Invalid range: {0} must be less than {1}")]
    InvalidRange(i32, i32),
}

pub struct CommandParser;

impl CommandParser {
    /// Parses a string input into a Command
    ///
    /// # Arguments
    /// * `input` - The command string to parse
    ///
    /// # Returns
    /// * `Result<Command, ParseError>` - The parsed command or an error
    pub fn parse(input: &str) -> Result<Command, ParseError> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            return Err(ParseError::UnknownCommand("Empty command".to_string()));
        }

        let cmd = parts[0];
        let args = &parts[1..];

        match cmd {
            CMD_RANDOMIZE => Ok(Command::Randomize),
            CMD_LIST => Ok(Command::List),
            CMD_GRAPHS => Ok(Command::Graphs),
            CMD_QUICKSORT => Ok(Command::QuickSort),
            CMD_MERGESORT => Ok(Command::MergeSort),
            CMD_TIMSORT => Ok(Command::TimSort),
            CMD_RADIX => Ok(Command::RadixSort),
            CMD_HEAPSORT => Ok(Command::HeapSort),
            CMD_SPEED => Self::parse_speed(args),
            CMD_GENERATE => Self::parse_generate(args),
            CMD_SORT => Self::parse_sort(args),
            CMD_COMPARE => Self::parse_compare(args),
            CMD_READLIST => Self::parse_readlist(args),
            _ => Err(ParseError::UnknownCommand(cmd.to_string())),
        }
    }

    fn parse_speed(args: &[&str]) -> Result<Command, ParseError> {
        let speed = args
            .first()
            .ok_or_else(|| ParseError::MissingArgument(CMD_SPEED.to_string()))?
            .parse()
            .map_err(|_| ParseError::InvalidArgument("Speed must be a valid number".to_string()))?;
        Ok(Command::Speed(speed))
    }

    fn parse_generate(args: &[&str]) -> Result<Command, ParseError> {
        let mut count: Option<usize> = None;
        let mut range: (i32, i32) = (0, 10);
        let mut no_repeats = false;

        for arg in args {
            match *arg {
                "nr" => no_repeats = true,
                arg if arg.contains('-') => {
                    let (start, end) = Self::parse_range(arg)?;
                    range = (start, end);
                }
                _ => {
                    count = Some(arg.parse().map_err(|_| {
                        ParseError::InvalidArgument("Count must be a positive number".to_string())
                    })?);
                }
            }
        }

        if range.0 >= range.1 {
            return Err(ParseError::InvalidRange(range.0, range.1));
        }

        Ok(Command::Generate {
            count,
            range,
            no_repeats,
        })
    }

    fn parse_range(range_str: &str) -> Result<(i32, i32), ParseError> {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            return Err(ParseError::InvalidArgument(format!(
                "Invalid range format: {}",
                range_str
            )));
        }

        let start = parts[0].parse().map_err(|_| {
            ParseError::InvalidArgument(format!("Invalid range start: {}", parts[0]))
        })?;
        let end = parts[1].parse().map_err(|_| {
            ParseError::InvalidArgument(format!("Invalid range end: {}", parts[1]))
        })?;

        Ok((start, end))
    }

    fn parse_sort(args: &[&str]) -> Result<Command, ParseError> {
        let algorithm = args
            .first()
            .ok_or_else(|| ParseError::MissingArgument(CMD_SORT.to_string()))?;
        Ok(Command::Sort(algorithm.to_string()))
    }

    fn parse_compare(args: &[&str]) -> Result<Command, ParseError> {
        if args.len() < 2 {
            return Err(ParseError::MissingArgument(
                "Compare requires two algorithms".to_string(),
            ));
        }
        Ok(Command::Compare(args[0].to_string(), args[1].to_string()))
    }

    fn parse_readlist(args: &[&str]) -> Result<Command, ParseError> {
        let filename = args
            .first()
            .ok_or_else(|| ParseError::MissingArgument(CMD_READLIST.to_string()))?;
        Ok(Command::ReadList(filename.to_string()))
    }
}
