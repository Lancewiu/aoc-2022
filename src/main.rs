use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const FILENAME: &str = "data/input.txt";

fn process_lines(reader: impl BufRead) -> anyhow::Result<u64> {
    let mut elf_sum: Vec<u64> = Vec::new();
    let mut elf: Vec<u64> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        if line.is_empty() {
            elf_sum.push(elf.drain(..).sum());
            continue;
        }

        elf.push(line.parse()?);
    }
    Ok(elf_sum
        .into_iter()
        .max()
        .ok_or(anyhow::Error::msg("no elves?"))?)
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(calories) => {
                println!("max calories: {}", calories);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
