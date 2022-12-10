use crate::grid::{Coordinate, Direction};

#[derive(Debug)]
pub struct Rope {
    head: Coordinate,
    tail: Coordinate,
}

impl Rope {
    pub fn new(head: Coordinate, tail: Coordinate) -> Self {
        Self { head, tail }
    }

    pub fn shift(&mut self, direction: Direction) {
        self.head.shift(direction);
        let (x_offset, y_offset) = self.head - self.tail;
        let x_dist = x_offset.abs();
        let y_dist = y_offset.abs();

        if x_dist <= 1 && y_dist <= 1 {
            return;
        }

        self.tail.shift(direction);
        if x_dist > 1 && y_dist > 0 {
            self.tail.shift(if y_offset.is_negative() {
                Direction::N
            } else {
                Direction::S
            })
        } else if y_dist > 1 && x_dist > 0 {
            self.tail.shift(if x_offset.is_positive() {
                Direction::E
            } else {
                Direction::W
            });
        }
    }

    pub fn get_tail(&self) -> Coordinate {
        self.tail
    }
}
