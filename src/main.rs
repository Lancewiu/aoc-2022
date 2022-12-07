use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const FILENAME: &str = "data/input.txt";

fn is_buffer_unique(buffer: &[u8]) -> bool {
    let mut unique_buffer = buffer.to_vec();
    unique_buffer.sort();
    unique_buffer.dedup();
    unique_buffer.len() == buffer.len()
}

fn process_lines(reader: impl BufRead) -> anyhow::Result<usize> {
    let mut buffer = Vec::with_capacity(15);
    for (i, byte_res) in reader.bytes().enumerate() {
        let byte = byte_res?;
        buffer.push(byte);
        if buffer.len() < 14 {
            continue;
        }

        if buffer.len() > 14 {
            buffer.remove(0);
        }
        if is_buffer_unique(&buffer[..]) {
            return Ok(i + 1);
        }
    }
    Err(anyhow::Error::msg("could not find valid message index!"))
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(i_start_of_message) => {
                println!("start-of-message index: {}", i_start_of_message);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
