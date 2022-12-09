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

    let mut max_scene_score = 0;
    for i_x in 1..(width - 1) {
        for i_y in 1..(height - 1) {
            let cursor = Coordinate(i_x, i_y);
            let cursor_height = field.get_value(cursor).expect("impossible cursor");

            //eprintln!("cursor {:?} [{}]", cursor, cursor_height);
            let scenery: usize = [Direction::N, Direction::S, Direction::E, Direction::W]
                .into_iter()
                .map(|direction| field.values_to_edge(cursor, direction))
                .map(|edge_values| {
                    edge_values
                        .iter()
                        .position(|height| cursor_height <= *height)
                        .map_or(edge_values.len(), |index| 1 + index)
                })
                .product();
            max_scene_score = max_scene_score.max(scenery);
        }
    }
    Ok(max_scene_score)
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
