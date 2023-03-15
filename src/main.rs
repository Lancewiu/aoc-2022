mod monkey;
mod parser;

use std::fs::File;
use std::io::BufReader;

const FILENAME: &str = "data/input.txt";

fn simulate_anxiety(monkeys: &mut [monkey::Monkey]) -> u64 {
    let mut inspection_counter: Vec<u64> = vec![0; monkeys.len()];

    for i_round in 0..20 {
        for i_monkey in 0..monkeys.len() {
            'turn: loop {
                let new_worry = if let Some(worry) = monkeys[i_monkey].throw_current_item() {
                    monkeys[i_monkey].inspect_item(worry) / 3
                } else {
                    break 'turn;
                };
                inspection_counter[i_monkey] += 1;
                let next_monkey = monkeys[i_monkey].test_anxiety(new_worry);
                monkeys[next_monkey].catch_item(new_worry);
            }
        }
        println!("After round {}:", i_round + 1);
        monkeys
            .iter()
            .enumerate()
            .for_each(|(i, m)| println!("Monkey: {}: {:?}", i, m));
    }

    inspection_counter.sort_unstable();
    inspection_counter.reverse();
    inspection_counter[0] * inspection_counter[1]
}

fn main() {
    match File::open(FILENAME) {
        Ok(file) => match parser::parse_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", FILENAME, err);
            }
            Ok(mut monkeys) => {
                println!("Anxiety Score {}", simulate_anxiety(&mut monkeys[..]))
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", FILENAME, err);
        }
    }
}
