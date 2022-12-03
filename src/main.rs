use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const FILENAME: &str = "data/input.txt";

fn process_lines(reader: impl BufRead) -> anyhow::Result<u64> {
    let mut priorities: u64 = 0;
    let mut first_sack: Vec<char> = Vec::new();
    let mut second_sack: Vec<char> = Vec::new();
    for (i_line, line_result) in reader.lines().enumerate() {
        let mut rucksack: Vec<char> = line_result?.chars().collect();
        if 2 != i_line % 3 {
            if 0 == i_line % 3 {
                first_sack = rucksack;
            } else {
                second_sack = rucksack;
            }
            continue;
        }
        first_sack.sort();
        first_sack.dedup();
        second_sack.sort();
        second_sack.dedup();
        rucksack.sort();
        rucksack.dedup();

        let common_item = rucksack
            .into_iter()
            .find(|item| first_sack.contains(item) && second_sack.contains(item))
            .ok_or_else(|| anyhow::Error::msg("no common item found"))?;
        let item_priority = (common_item as u32)
            - if common_item.is_ascii_uppercase() {
                38
            } else {
                96
            };
        priorities += item_priority as u64;
    }
    Ok(priorities)
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(priorities) => {
                println!("priorities: {}", priorities);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
