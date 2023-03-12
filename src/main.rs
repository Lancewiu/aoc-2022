mod monkey;
mod parser;

use std::fs::File;
use std::io::BufReader;

const FILENAME: &str = "data/test_input.txt";

fn simulate_anxiety(monkeys: &[monkey::Monkey]) -> u64 {

}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match parser::parse_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(monkeys) => {
                println!("Multiple of Anxieties {}", simulate_anxiety(&monkeys[..]))
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
