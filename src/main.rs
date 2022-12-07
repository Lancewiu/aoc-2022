use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const FILENAME: &str = "data/input.txt";

fn process_lines(reader: impl BufRead) -> anyhow::Result<String> {
    let mut line_iter = reader.lines();

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(9);
    (0..9).for_each(|_| stacks.push(Vec::with_capacity(8)));
    for _ in 0..8 {
        // parse initial stack state
        let line = line_iter
            .next()
            .ok_or_else(|| anyhow::Error::msg("incomplete stack text!"))??;
        let crate_iter = line.chars().enumerate().filter(|(_, c)| c.is_alphabetic());

        for (char_id, crate_id) in crate_iter {
            let stack_index = char_id
                .checked_sub(1)
                .map(|id| id / 4)
                .ok_or_else(|| anyhow::Error::msg("invalid stack text!"))?;
            stacks[stack_index].push(crate_id);
        }
    }
    // reverse all stacks
    stacks.iter_mut().for_each(|stack| stack.reverse());

    for line_result in line_iter.skip(2) {
        // process commands
        let line = line_result?;
        let values: Vec<u64> = line
            .split_ascii_whitespace()
            .skip(1)
            .step_by(2)
            .take(3)
            .map(|n_str| n_str.parse())
            .collect::<Result<Vec<u64>, _>>()?;
        let n = values[0];
        let source_stack_index = (values[1] - 1) as usize;
        let dest_stack_index = (values[2] - 1) as usize;
        for _ in 0..n {
            let crate_id = stacks[source_stack_index]
                .pop()
                .ok_or_else(|| anyhow::Error::msg("crane had nothing to grab!"))?;
            stacks[dest_stack_index].push(crate_id);
        }
    }

    stacks
        .into_iter()
        .map(|mut stack| stack.pop())
        .collect::<Option<String>>()
        .ok_or_else(|| anyhow::Error::msg("empty stacks encountered!"))
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
