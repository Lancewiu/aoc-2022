#[derive(Clone, Copy, Debug)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Clone, Copy, Debug)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    pub fn to_linear_index(self, width: usize) -> usize {
        self.0 + width * self.1
    }

    pub fn shift(&self, direction: Direction, distance: usize) -> Coordinate {
        match direction {
            Direction::N => Coordinate(self.0, self.1.saturating_sub(distance)),
            Direction::S => Coordinate(self.0, self.1.saturating_add(distance)),
            Direction::E => Coordinate(self.0.saturating_add(distance), self.1),
            Direction::W => Coordinate(self.0.saturating_sub(distance), self.1),
        }
    }
}

pub struct Field {
    raw: Vec<u32>,
    width: usize,
    height: usize,
}

impl Field {
    pub fn from_raw(raw: Vec<u32>, width: usize, height: usize) -> Self {
        Self { raw, width, height }
    }

    pub fn values_to_edge(&self, from: Coordinate, direction: Direction) -> Vec<u32> {
        if !self.is_coordinate_valid(from) {
            return Vec::new();
        }

        let num_values = match direction {
            Direction::N => from.1,
            Direction::S => self.height - from.1 - 1,
            Direction::E => self.width - from.0 - 1,
            Direction::W => from.0,
        };

        (1..=num_values)
            .map(|i_distance| from.shift(direction, i_distance))
            .map(|coordinate| self.get_value(coordinate))
            .collect::<Option<Vec<u32>>>()
            .expect("invalid coordinates provided")
    }

    pub fn is_coordinate_valid(&self, coord: Coordinate) -> bool {
        coord.0 < self.width && coord.1 < self.height
    }

    pub fn get_value(&self, from: Coordinate) -> Option<u32> {
        if !self.is_coordinate_valid(from) {
            return None;
        }

        Some(self.raw[from.to_linear_index(self.width)])
    }
}
