use std::ops::{AddAssign, Index};

const BITMAP_SIZE: usize = 5;

#[derive(Debug, PartialEq)]
pub enum State {
    E, // Empty
    N, // NonEmpty
}

pub struct Bitmap {
    bits: [State; BITMAP_SIZE],
}

impl AddAssign for Bitmap {
    fn add_assign(&mut self, other: Bitmap) {
        for i in 0..BITMAP_SIZE {
            if other.bits[i] != State::E {
                self.bits[i] = State::N;
            }
        }
    }
}

impl Index<usize> for Bitmap {
    type Output = State;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[index]
    }
}

impl Bitmap {
    pub fn new() -> Self {
        Self {
            bits: [const { State::E }; BITMAP_SIZE],
        }
    }

    pub fn from(bits: [State; BITMAP_SIZE]) -> Self {
        Self { bits }
    }

    pub fn render(&self) -> String {
        use State::*;

        let output = match self.bits {
            [
                   E,
                E, E, E,
                   E,
            ] => "  ",
            [
                   E,
                N, N, N,
                   E,
            ] => "──",
            [
                   N,
                E, N, E,
                   N,
            ] => "│ ",
            [
                   N,
                E, N, N,
                   E,
            ] => "└─",
            [
                   E,
                E, N, N,
                   N,
            ] => "┌─",
            [
                   E,
                N, N, E,
                   N,
            ] => "┐ ",
            [
                   N,
                N, N, E,
                   E,
            ] => "┘ ",
            [
                   N,
                N, N, N,
                   E,
            ] => "┴─",
            [
                   N,
                E, N, N,
                   N,
            ] => "├─",
            [
                   E,
                N, N, N,
                   N,
            ] => "┬─",
            [
                   N,
                N, N, E,
                   N,
            ] => "┤ ",
            [
                   N,
                N, N, N,
                   N,
            ] => "┼─",
            _ => {
                todo!("Probably not needed");
            }
        };

        output.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use State::*;

    #[test]
    fn test_index_when_empty() {
        let bitmap = Bitmap::new();
        let expected = [
               E,
            E, E, E,
               E,
        ];

        for i in 0..expected.len() {
            assert_eq!(bitmap[i], expected[i]);
        }
    }

    #[test]
    fn test_from() {
        let bitmap = Bitmap::from([
               E,
            E, N, N,
               N,
        ]);
        let expected = [
               E,
            E, N, N,
               N,
        ];

        for i in 0..expected.len() {
            assert_eq!(bitmap[i], expected[i]);
        }
    }

    #[test]
    fn test_add_assign() {
        let mut bitmap = Bitmap::from([
               E,
            E, N, N,
               N,
        ]);
        let rhs = Bitmap::from([
               E,
            N, N, E,
               N,
        ]);
        bitmap += rhs;
        let expected = [
               E,
            N, N, N,
               N,
        ];

        for i in 0..expected.len() {
            assert_eq!(bitmap[i], expected[i]);
        }
    }
}
