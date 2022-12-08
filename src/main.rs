use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const FILENAME: &str = "data/input.txt";
const DELIMITER: &str = "/";

fn process_lines(reader: impl BufRead) -> anyhow::Result<usize> {
    let mut path_to_size: HashMap<String, usize> = HashMap::new();
    let mut current_path = vec!["".to_string()];

    for line_res in reader.lines() {
        let line = line_res?;
        let mut tokens = line.split_ascii_whitespace();
        let context = tokens.next().expect("unexpected empty line");

        match context {
            "$" => {
                let command = tokens.next().expect("invalid command");
                match command {
                    "cd" => {
                        let dir = tokens.next().expect("`cd` command missing argument");
                        if ".." == dir {
                            if 1 < current_path.len() {
                                current_path.pop();
                            }
                        } else {
                            let mut new_path: Vec<String> =
                                dir.split(DELIMITER).map(|s| s.to_string()).collect();
                            if new_path[0].is_empty() {
                                current_path = new_path;
                            } else {
                                current_path.append(&mut new_path);
                            }
                        }
                    }
                    "ls" => {}
                    tok => {
                        panic!("unexpected token `{}` encountered", tok);
                    }
                }
            }
            "dir" => {}
            size_str => {
                let file_size: usize = size_str.parse().expect("invalid file size value");
                let num_paths = current_path.len();
                for right_offset in (1..=num_paths).rev() {
                    path_to_size
                        .entry(current_path[..right_offset].join(DELIMITER))
                        .and_modify(|size| {
                            *size += file_size;
                        })
                        .or_insert(file_size);
                }
            }
        }
    }

    Ok(path_to_size
        .values()
        .filter(|dir_size| **dir_size <= 100_000)
        .sum())
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(sum) => {
                println!("sum: {}", sum);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
