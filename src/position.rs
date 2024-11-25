use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_assign() {
        let mut pos = Position { x: 1, y: 3 };
        let rhs = Position { x: -1, y: 1 };

        pos += rhs;

        assert_eq!(pos, Position { x: 0, y: 4});
    }
}
