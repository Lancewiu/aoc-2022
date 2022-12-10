mod grid;
mod rope;

use grid::{Coordinate, Direction};
use rope::Rope;

use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "data/input.txt";

fn process_lines(reader: impl BufRead) -> anyhow::Result<usize> {
    let mut rope = Rope::new(Coordinate(0, 0), Coordinate(0, 0));
    let mut tail_positions: Vec<Coordinate> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let mut tokens = line.split_ascii_whitespace();
        let direction = tokens
            .next()
            .and_then(|dir_str| dir_str.chars().next())
            .and_then(Direction::try_from_char)
            .ok_or_else(|| anyhow::Error::msg("malformed direction input."))?;
        let distance: usize = tokens
            .next()
            .and_then(|dist_str| dist_str.parse().ok())
            .ok_or_else(|| anyhow::Error::msg("malformed distance input."))?;
        (0..distance).for_each(|_| {
            rope.shift(direction);
            tail_positions.push(rope.get_tail());
        });
    }
    tail_positions.sort();
    tail_positions.dedup();
    Ok(tail_positions.len())
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(n) => {
                println!("{}", n);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
