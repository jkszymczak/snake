use crate::direction::Direction;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn move_in_direction(&self, dir: &Direction) -> Result<Position, ()> {
        let (x, y) = match dir {
            Direction::Left => {
                if self.x == 0 {
                    return Err(());
                }
                (self.x - 1, self.y)
            },
            Direction::Down => {
                (self.x, self.y + 1)
            },
            Direction::Up => {
                if self.y == 0 {
                    return Err(());
                }
                (self.x, self.y - 1)
            },
            Direction::Right => {
                (self.x + 1, self.y)
            },
        };

        Ok(Position { x, y })
    }
}
