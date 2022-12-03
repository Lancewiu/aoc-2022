use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const FILENAME: &str = "data/input.txt";

fn process_lines(reader: impl BufRead) -> anyhow::Result<u64> {
    let mut priorities: u64 = 0;
    for line_result in reader.lines() {
        let rucksack: Vec<char> = line_result?.chars().collect();
        let second_compartment_index = rucksack.len() / 2;
        let mut first_rucksack_types = (&rucksack[..second_compartment_index]).to_vec();
        first_rucksack_types.sort();
        first_rucksack_types.dedup();
        let common_item = rucksack[second_compartment_index..]
            .iter()
            .find(|&item| first_rucksack_types.contains(item))
            .copied()
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
