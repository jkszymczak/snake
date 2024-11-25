use crate::position::Position;

pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

impl Direction {
    pub fn to_pos_diff(&self) -> Position {
        let (x, y) = match self {
            Self::Left  => (-1, 0),
            Self::Down  => (0, 1),
            Self::Up    => (0, -1),
            Self::Right => (1, 0),
        };
        Position { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pos_diff_for_left_dir() {
        assert_eq!(
            Direction::Left.to_pos_diff(),
            Position { x: -1, y: 0 },
        );
    }

    #[test]
    fn test_to_pos_diff_for_down_dir() {
        assert_eq!(
            Direction::Down.to_pos_diff(),
            Position { x: 0, y: 1 },
        );
    }

    #[test]
    fn test_to_pos_diff_for_up_dir() {
        assert_eq!(
            Direction::Up.to_pos_diff(),
            Position { x: 0, y: -1 },
        );
    }

    #[test]
    fn test_to_pos_diff_for_right_dir() {
        assert_eq!(
            Direction::Right.to_pos_diff(),
            Position { x: 1, y: 0 },
        );
    }
}
