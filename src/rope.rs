use crate::grid::{Coordinate, Direction};

#[derive(Debug)]
pub struct Rope {
    nodes: [Coordinate; 10],
}

impl Rope {
    pub fn new(head: Coordinate) -> Self {
        Self { nodes: [head; 10] }
    }

    pub fn shift(&mut self, direction: Direction) {
        self.nodes[0].shift(direction);

        for i_head in 0..9 {
            let i_tail = i_head + 1;
            let offset = self.nodes[i_head] - self.nodes[i_tail];
            let x_dist = offset.0.abs();
            let y_dist = offset.1.abs();

            if x_dist <= 1 && y_dist <= 1 {
                break;
            }

            self.nodes[i_tail].shift(if x_dist > 1 {
                if offset.0.is_positive() {
                    Direction::E
                } else {
                    Direction::W
                }
            } else if offset.1.is_negative() {
                Direction::N
            } else {
                Direction::S
            });

            if x_dist > 1 && y_dist > 0 {
                self.nodes[i_tail].shift(if offset.1.is_negative() {
                    Direction::N
                } else {
                    Direction::S
                })
            } else if y_dist > 1 && x_dist > 0 {
                self.nodes[i_tail].shift(if offset.0.is_positive() {
                    Direction::E
                } else {
                    Direction::W
                });
            }
        }
    }

    pub fn get_tail(&self) -> Coordinate {
        self.nodes[9]
    }
}
