#[derive(PartialEq)]
pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

pub fn are_opposite(dir1: &Direction, dir2: &Direction) -> bool {
    use Direction::*;

    match dir1 {
        Left  => dir2 == &Right,
        Down  => dir2 == &Up,
        Up    => dir2 == &Down,
        Right => dir2 == &Left,
    }
}
