use std::cmp;

#[derive(PartialEq, Eq)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn try_from_strategy(code: char) -> Option<Outcome> {
        match code {
            'X' => Some(Outcome::Loss),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => None,
        }
    }

    pub fn score(&self) -> u64 {
        match self {
            Outcome::Draw => 3,
            Outcome::Loss => 0,
            Outcome::Win => 6,
        }
    }
}

// cannot be Ord as this fails the transitive property
#[derive(PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl cmp::PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(match self {
            Shape::Rock => match other {
                Shape::Rock => cmp::Ordering::Equal,
                Shape::Paper => cmp::Ordering::Less,
                Shape::Scissors => cmp::Ordering::Greater,
            },
            Shape::Paper => match other {
                Shape::Rock => cmp::Ordering::Greater,
                Shape::Paper => cmp::Ordering::Equal,
                Shape::Scissors => cmp::Ordering::Less,
            },
            Shape::Scissors => match other {
                Shape::Rock => cmp::Ordering::Less,
                Shape::Paper => cmp::Ordering::Greater,
                Shape::Scissors => cmp::Ordering::Equal,
            },
        })
    }
}

impl Shape {
    pub fn try_from_play(code: char) -> Option<Shape> {
        match code {
            'A' => Some(Shape::Rock),
            'B' => Some(Shape::Paper),
            'C' => Some(Shape::Scissors),
            _ => None,
        }
    }

    pub fn from_strategy(play: &Shape, outcome: &Outcome) -> Shape {
        match play {
            Shape::Rock => match outcome {
                Outcome::Draw => Shape::Rock,
                Outcome::Loss => Shape::Scissors,
                Outcome::Win => Shape::Paper,
            },
            Shape::Paper => match outcome {
                Outcome::Draw => Shape::Paper,
                Outcome::Loss => Shape::Rock,
                Outcome::Win => Shape::Scissors,
            },
            Shape::Scissors => match outcome {
                Outcome::Draw => Shape::Scissors,
                Outcome::Loss => Shape::Paper,
                Outcome::Win => Shape::Rock,
            },
        }
    }

    pub fn score(&self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}
