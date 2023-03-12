use crate::monkey::InspectionFunction;

use std::ops::Add;
use std::ops::Mul;

fn map_op(op: &str) -> Option<Box<dyn Fn(u64, u64) -> u64>> {
    match op {
        "+" => Some(Box::new(u64::add)),
        "*" => Some(Box::new(u64::mul)),
        _ => None,
    }
}

pub fn parse_inspection(text: &str) -> anyhow::Result<InspectionFunction> {
    let equals_i = text
        .find('=')
        .ok_or_else(|| anyhow::Error::msg("missing equals sign"))?;
    let (_, function) = text.split_at(equals_i + 1);
    let tokens = function.trim().split_ascii_whitespace();
    let left = tokens
        .next()
        .ok_or_else(|| anyhow::Error::msg("missing left operand"))?;
    let op_str = tokens
        .next()
        .ok_or_else(|| anyhow::Error::msg("missing op character"))?;
    let right = tokens
        .next()
        .ok_or_else(|| anyhow::Error::msg("missing right operand"))?;

    let op = map_op(op_str).ok_or_else(|| anyhow::Error::msg("invalid op character"))?;

    Ok(match left {
        "old" => match right {
            "old" => Box::new(|var| (*op)(var, var)),
            r_str => {
                let r_value: u64 = r_str.parse()?;
                Box::new(|var| (*op)(var, r_value))
            }
        },
        l_str => {
            let l_value: u64 = l_str.parse()?;
            match right {
                "old" => Box::new(|var| (*op)(l_value, var)),
                r_str => {
                    let r_value: u64 = r_str.parse()?;
                    Box::new(|var| (*op)(l_value, r_value))
                }
            }
        }
    })
}
