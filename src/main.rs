use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "data/input.txt";

fn process_lines(reader: impl BufRead) -> anyhow::Result<i64> {
    let mut register = 1_i64;
    let mut cycle = 0_usize;
    let cycle_query = [19_usize, 59, 99, 139, 179, 219];
    let mut signal_sum = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let cost = if "noop" == line { 1 } else { 2 };

        for i_cycle in cycle..(cycle + cost) {
            if cycle_query.binary_search(&i_cycle).is_ok() {
                signal_sum += (1 + i_cycle as i64) * register;
            }
        }

        cycle += cost;
        if "noop" != line {
            let mut tokens = line.split_ascii_whitespace().skip(1);
            let addend: i64 = tokens
                .next()
                .and_then(|add_str| add_str.parse().ok())
                .ok_or_else(|| anyhow::Error::msg("failed to read addend."))?;
            register += addend;
        }
    }

    Ok(signal_sum)
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
