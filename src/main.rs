use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "data/input.txt";

fn process_lines(reader: impl BufRead) -> anyhow::Result<()> {
    let mut sprite_column = 1_i64;
    let mut cycle = 0_usize;
    let mut is_pixel_bright = [false; 240];
    for line_result in reader.lines() {
        let line = line_result?;
        let cost = if "noop" == line { 1 } else { 2 };

        for i_cycle in cycle..(cycle + cost) {
            is_pixel_bright[i_cycle] = (-1..=1)
                .map(|i_sprite| i_sprite + sprite_column)
                .any(|i_sprite| (i_cycle as i64 % 40) == i_sprite);
        }

        cycle += cost;
        if "noop" != line {
            let mut tokens = line.split_ascii_whitespace().skip(1);
            let addend: i64 = tokens
                .next()
                .and_then(|add_str| add_str.parse().ok())
                .ok_or_else(|| anyhow::Error::msg("failed to read addend."))?;
            sprite_column += addend;
        }
    }

    let display_buffer: Vec<char> = is_pixel_bright
        .into_iter()
        .map(|is_bright| if is_bright { '#' } else { '.' })
        .collect();
    (0usize..6)
        .map(|i_row| {
            let i_start = i_row * 40;
            let i_end = i_start + 40;
            i_start..i_end
        })
        .for_each(|line_indices| {
            println!(
                "{}",
                display_buffer[line_indices].iter().collect::<String>()
            );
        });

    Ok(())
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(_n) => {}
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
