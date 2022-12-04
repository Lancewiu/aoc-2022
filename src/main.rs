use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const FILENAME: &str = "data/input.txt";

fn str_to_range(dashed_range: &str) -> anyhow::Result<(u64, u64)> {
    let (start, end) = dashed_range
        .split_once('-')
        .ok_or_else(|| anyhow::Error::msg("invalid range format"))?;
    Ok((start.parse()?, end.parse()?))
}

fn is_range_overlapping(a: (u64, u64), b: (u64, u64)) -> bool {
    (a.0 <= b.0 && a.1 >= b.0) || (b.0 <= a.0 && b.1 >= a.0)
}

fn process_lines(reader: impl BufRead) -> anyhow::Result<u64> {
    let mut count = 0u64;
    for line_result in reader.lines() {
        let line = line_result?;
        let (first, second) = line
            .split_once(',')
            .ok_or_else(|| anyhow::Error::msg("invalid pair"))?;
        let first_range = str_to_range(first)?;
        let second_range = str_to_range(second)?;
        if is_range_overlapping(first_range, second_range) {
            count += 1;
        }
    }
    Ok(count)
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(num_subset) => {
                println!("# of subset pairs: {}", num_subset);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
