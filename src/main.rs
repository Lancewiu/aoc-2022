use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const FILENAME: &str = "data/input.txt";

fn process_lines(reader: impl BufRead) -> anyhow::Result<usize> {
    let mut buffer: Vec<u8> = Vec::with_capacity(5);
    for (i, byte_res) in reader.bytes().enumerate() {
        buffer.push(byte_res?);
        if buffer.len() < 4 {
            continue;
        }

        if buffer.len() > 4 {
            buffer.remove(0);
        }

        let mut unique_buffer = buffer.clone();
        unique_buffer.sort();
        unique_buffer.dedup();
        if 4 == unique_buffer.len() {
            return Ok(i + 1);
        }
    }
    Err(anyhow::Error::msg("failed to find a valid index!"))
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(i_start_of_packet) => {
                println!("start-of-packet index: {}", i_start_of_packet);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
