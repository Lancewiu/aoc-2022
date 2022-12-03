mod rps;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const FILENAME: &str = "data/input.txt";

fn process_lines(reader: impl BufRead) -> anyhow::Result<u64> {
    let mut score: u64 = 0;
    let get_first_char = |token_opt: Option<&str>| -> anyhow::Result<char> {
        token_opt
            .and_then(|s| s.chars().next())
            .ok_or_else(|| anyhow::Error::msg("invalid token string"))
    };
    for line_result in reader.lines() {
        let line = line_result?;
        let mut tokens = line.split_ascii_whitespace();
        let play = rps::Shape::try_from_play(get_first_char(tokens.next())?)
            .ok_or_else(|| anyhow::Error::msg("invalid play character"))?;
        let outcome = rps::Outcome::try_from_strategy(get_first_char(tokens.next())?)
            .ok_or_else(|| anyhow::Error::msg("invalid outcome character"))?;

        let strategy = rps::Shape::from_strategy(&play, &outcome);
        score += strategy.score() + outcome.score();
    }
    Ok(score)
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(score) => {
                println!("score: {}", score);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
