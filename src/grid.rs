use std::ops::Sub;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    pub fn try_from_char(c: char) -> Option<Direction> {
        match c {
            'N' | 'U' => Some(Direction::N),
            'S' | 'D' => Some(Direction::S),
            'E' | 'R' => Some(Direction::E),
            'W' | 'L' => Some(Direction::W),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate(pub isize, pub isize);

impl Coordinate {
    pub fn jump(&mut self, direction: Direction, distance: isize) {
        match direction {
            Direction::N => {
                self.1 = self.1.saturating_sub(distance);
            }
            Direction::S => {
                self.1 = self.1.saturating_add(distance);
            }
            Direction::E => {
                self.0 = self.0.saturating_add(distance);
            }
            Direction::W => {
                self.0 = self.0.saturating_sub(distance);
            }
        }
    }

    pub fn shift(&mut self, direction: Direction) {
        self.jump(direction, 1);
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate(self.0.saturating_sub(rhs.0), self.1.saturating_sub(rhs.1))
    }
}
