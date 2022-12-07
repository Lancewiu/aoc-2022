mod elffs;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

type DirectoryMap = HashMap<String, elffs::Directory>;

const FILENAME: &str = "data/input.txt";

fn get_directory_size(directory_map: &DirectoryMap, directory_name: &str) -> usize {
    directory_map
        .get(&directory_name.to_string())
        .map(|dir| {
            dir.get_file_sizes()
                + dir
                    .get_subdirectory_names()
                    .iter()
                    .map(|subdir| get_directory_size(directory_map, subdir.as_str()))
                    .sum::<usize>()
        })
        .unwrap_or(0)
}

fn process_lines(reader: impl BufRead) -> anyhow::Result<usize> {
    let mut name_to_directory: DirectoryMap = HashMap::new();
    let mut name_to_parent_name: HashMap<String, String> = HashMap::new();
    let mut current_dir_name = "/".to_string();
    name_to_directory
        .entry(current_dir_name.clone())
        .or_default();
    for line_res in reader.lines() {
        let line = line_res?;
        let mut tokens = line.split_ascii_whitespace();
        let context = tokens.next().expect("unexpected empty line");

        match context {
            "$" => {
                let command = tokens.next().expect("invalid command");
                match command {
                    "cd" => {
                        let dir = tokens
                            .next()
                            .expect("`cd` command missing argument")
                            .to_string();
                        current_dir_name = if ".." == dir {
                            name_to_parent_name[&current_dir_name].clone()
                        } else {
                            dir.clone()
                        };
                    }
                    "ls" => {}
                    tok => {
                        panic!("unexpected token `{}` encountered", tok);
                    }
                }
            }
            "dir" => {
                let dir_name = tokens.next().expect("missing dir name").to_string();
                name_to_parent_name.insert(dir_name.clone(), current_dir_name.clone());
                name_to_directory.entry(dir_name.clone()).or_default();
                name_to_directory
                    .get_mut(&current_dir_name)
                    .expect("current directory not defined! could not add directory")
                    .add_directory(dir_name.as_str());
            }
            size_str => {
                let file_name = tokens.next().expect("missing file name");
                let file_size: usize = size_str.parse().expect("invalid file size value");
                name_to_directory
                    .get_mut(&current_dir_name)
                    .expect("current directory not defined! could not add file")
                    .add_file(elffs::File::new(file_name.to_string(), file_size));
            }
        }
    }

    Ok(name_to_directory
        .keys()
        .map(|name| get_directory_size(&name_to_directory, name.as_str()))
        .filter(|dir_size| *dir_size <= 100_000)
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
