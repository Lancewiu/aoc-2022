mod calculator;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::{self, FromStr};

const FILENAME: &str = "data/test_input.txt";

struct Monkey {
    id: usize,
    items: Vec<i64>,
    operation: calculator::Phrase,
    test: Box<dyn FnOnce(i64) -> bool>,
    test_pass_destination: usize,
    test_fail_destination: usize,
}

fn process_test(test: &str) -> anyhow::Result<Box<dyn FnOnce(i64) -> bool>> {
    let divisor: i64 = test
        .split_whitespace()
        .nth(3)
        .and_then(|div_str| div_str.parse().ok())
        .ok_or_else(|| anyhow::Error::msg("malformed test: no divisor found"))?;
    Ok(if 0 == divisor {
        Box::new(|_| false)
    } else {
        Box::new(move |var: i64| 0 == var % divisor)
    })
}

fn process_starting_items(items: &str) -> anyhow::Result<Vec<i64>> {
    items
        .trim()
        .split(',')
        .map(|item| {
            item.trim()
                .parse()
                .map_err(|_| anyhow::Error::msg("failed to parse starting items"))
        })
        .collect()
}

impl Monkey {
    fn try_from_lines(lines: &[String]) -> anyhow::Result<Self> {
        if lines.len() < 6 {
            return Err(anyhow::Error::msg(format!(
                "invalid # of lines {} != 6",
                lines.len()
            )));
        }
        let (id_str, _) = lines[0]
            .split_once(':')
            .ok_or_else(|| anyhow::Error::msg("no id found"))?;
        let (_, item_str) = lines[1]
            .split_once(':')
            .ok_or_else(|| anyhow::Error::msg("missing starting items list"))?;
        let (_, operation_str) = lines[2]
            .split_once(':')
            .ok_or_else(|| anyhow::Error::msg("missing operation function"))?;
        Ok(Monkey {
            id: id_str
                .split_whitespace()
                .nth(1)
                .and_then(|id_token| id_token.parse().ok())
                .ok_or_else(|| anyhow::Error::msg("invalid id found"))?,
            items: process_starting_items(item_str)?,
            operation: calculator::Phrase::from_str(operation_str)?,
            test: process_test(lines[3].as_str())?,
            test_pass_destination: lines[4]
                .split_whitespace()
                .nth(5)
                .and_then(|id_str| id_str.parse().ok())
                .ok_or_else(|| anyhow::Error::msg("invalid success destination"))?,
            test_fail_destination: lines[5]
                .split_whitespace()
                .nth(5)
                .and_then(|id_str| id_str.parse().ok())
                .ok_or_else(|| anyhow::Error::msg("invalid failure destination"))?,
        })
    }
}

fn process_lines(reader: impl BufRead) -> anyhow::Result<Vec<Monkey>> {
    let mut monkey_lines: Vec<Vec<String>> = Vec::new();
    let mut monkey_raw: Vec<String> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        if line.is_empty() {
            monkey_lines.push(monkey_raw.clone());
            monkey_raw.clear();
        } else {
            monkey_raw.push(line);
        }
    }
    monkey_lines.push(monkey_raw);

    monkey_lines
        .into_iter()
        .map(|lines| Monkey::try_from_lines(&lines[..]))
        .collect()
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(_n) => {}
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
