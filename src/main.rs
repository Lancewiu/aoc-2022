mod field;

use field::{Coordinate, Direction, Field};
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "data/input.txt";

fn process_lines(reader: impl BufRead) -> anyhow::Result<usize> {
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>()?;
    let height = lines.len();
    let width = lines
        .get(0)
        .ok_or_else(|| anyhow::Error::msg("empty lines encountered?"))?
        .len();
    let field_raw: Vec<u32> = lines
        .iter()
        .flat_map(|line| line.chars())
        .map(|height_str| height_str.to_digit(10))
        .collect::<Option<Vec<u32>>>()
        .ok_or_else(|| anyhow::Error::msg("non-digit encountered"))?;
    let field = Field::from_raw(field_raw, width, height);

    let mut num_internal_visible = 0;
    for i_x in 1..(width - 1) {
        for i_y in 1..(height - 1) {
            let cursor = Coordinate(i_x, i_y);
            let cursor_height = field.get_value(cursor).expect("impossible cursor");

            //eprintln!("cursor {:?} [{}]", cursor, cursor_height);
            let is_visible = [Direction::N, Direction::S, Direction::E, Direction::W]
                .into_iter()
                //.inspect(|direction| eprint!("direction: {:?} values: ", direction))
                .map(|direction| field.values_to_edge(cursor, direction))
                //.inspect(|edge_values| eprintln!("{:?}", edge_values))
                .any(|edge_values| edge_values.into_iter().all(|height| cursor_height > height));
            if is_visible {
                num_internal_visible += 1;
            }
        }
    }
    Ok(2 * width + 2 * (height - 2) + num_internal_visible)
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(n) => {
                println!("# visible trees: {}", n);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
