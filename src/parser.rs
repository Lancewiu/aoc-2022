mod inspection;

use super::monkey;

use std::io::BufRead;

fn parse_items(line: &str) -> anyhow::Result<Vec<monkey::WorryLevel>> {
    let colon_i = line
        .find(':')
        .ok_or_else(|| anyhow::Error::msg("malformed monkey line. missing colon separator"))?;
    line[(colon_i + 1)..]
        .trim()
        .split(',')
        .map(|item_str| item_str.trim().parse::<monkey::WorryLevel>())
        .collect::<Result<Vec<monkey::WorryLevel>, _>>()
        .map_err(anyhow::Error::from)
}

fn parse_op(line: &str) -> anyhow::Result<monkey::InspectionFunction> {
    let colon_i = line.find(':').ok_or_else(|| {
        anyhow::Error::msg("malformed monkey operation. Colon discriminator not found.")
    })?;
    inspection::parse_inspection(&line[(colon_i + 1)..])
}

fn parse_tests(lines: &[String]) -> anyhow::Result<monkey::TestBehavior> {
    let condition_divisor: monkey::WorryLevel = lines[0]
        .trim()
        .split_ascii_whitespace()
        .last()
        .ok_or_else(|| anyhow::Error::msg("malformed monkey test. condition divisor not found"))?
        .parse()?;
    let true_destination: monkey::MonkeyIndex = lines[1]
        .trim()
        .split_ascii_whitespace()
        .last()
        .ok_or_else(|| {
            anyhow::Error::msg("malformed monkey test. true throw destination not found.")
        })?
        .parse()?;
    let false_destination: monkey::MonkeyIndex = lines[2]
        .trim()
        .split_ascii_whitespace()
        .last()
        .ok_or_else(|| {
            anyhow::Error::msg("malformed monkey test. false throw destination not found.")
        })?
        .parse()?;
    Ok(monkey::TestBehavior::new(
        Box::new(|worry| 0 == worry % condition_divisor),
        true_destination,
        false_destination,
    ))
}

fn parse_monkey(lines: &[String]) -> anyhow::Result<monkey::Monkey> {
    if 6 > lines.len() {
        return Err(anyhow::Error::msg(
            "malformed monkey. Invalid number of lines.",
        ));
    }

    Ok(monkey::MonkeyFactory::initialize()
        .with_items(parse_items(lines[1].as_str())?.as_slice())
        .with_inspection(parse_op(lines[2].as_str())?)
        .with_test_behavior(parse_tests(&lines[3..6])?)
        .spawn_monkey()
        .unwrap())
}

pub fn parse_lines(reader: impl BufRead) -> anyhow::Result<Vec<monkey::Monkey>> {
    let mut monkeys: Vec<monkey::Monkey> = Vec::new();
    let mut monkey_lines: Vec<String> = Vec::with_capacity(6);
    for maybe_line in reader.lines() {
        let line = maybe_line?;
        if line.is_empty() {
            monkeys.push(parse_monkey(&monkey_lines[..])?);
            monkey_lines.clear();
        } else {
            monkey_lines.push(line);
        }
    }
    monkeys.push(parse_monkey(&monkey_lines[..])?);
    Ok(monkeys)
}
