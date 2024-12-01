use std::collections::LinkedList;

use crate::direction::Direction;
use crate::grid::{Cell, Grid};
use crate::position::Position;

#[derive(Debug, PartialEq)]
pub enum Status {
    Ate,
    Died,
    Moved,
}

pub struct Snake {
    dir: Direction,
    segments: LinkedList<Position>,
    status: Status,
}

impl Snake {
    pub fn new(origin: Position) -> Self {
        Self {
            dir: Direction::Up,
            segments: LinkedList::from([origin]),
            status: Status::Moved,
        }
    }

    pub fn update(&mut self, grid: &mut Grid) -> &Status {
        // TODO: Handle snake collision with itself
        let curr_pos = self.segments.front().unwrap();
        let pos_diff = self.dir.to_pos_diff();
        let (x, y) = (
            curr_pos.x + pos_diff.x,
            curr_pos.y + pos_diff.y,
        );
        let width: i32 = grid.width().try_into().unwrap();
        let height: i32 = grid.height().try_into().unwrap();

        if x < 0 || x >= width || y < 0 || y >= height {
            self.status = Status::Died;
            return &self.status;
        }

        let (mut sx, mut sy) = (x, y);
        for segment in self.segments.iter_mut() {
            (segment.x, sx) = (sx, segment.x);
            (segment.y, sy) = (sy, segment.y);
        }

        //if apple_pos == (Position { x, y }) {
        if grid[(y*width + x).try_into().unwrap()] == Cell::Apple {
            self.status = Status::Ate;
            grid[(sy*width + sx).try_into().unwrap()] = Cell::Empty;
        } else if self.status == Status::Ate {
            self.segments.push_back(Position { x: sx, y: sy });
            self.status = Status::Moved;
        } else {
            grid[(sy*width + sx).try_into().unwrap()] = Cell::Empty;
            self.status = Status::Moved;
        }

        grid[(y*width + x).try_into().unwrap()] = Cell::Snake;

        &self.status
    }

    pub fn set_dir(&mut self, new_dir: Direction) {
        self.dir = new_dir;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as pretty_assert_eq;

    #[test]
    fn test_update_if_crossing_the_top_edge_then_dies() {
        let origin = Position { x: 2, y: 0 };
        let apple = Apple { pos: Position { x: 0, y: 0 } };
        let mut snake = Snake::new(origin);

        assert_eq!(snake.update(&apple, &mut Grid::new()), &Status::Died);
    }

    #[test]
    fn test_update_if_encountered_apple_then_returns_ate() {
        let origin = Position { x: 2, y: 2 };
        let apple = Apple { pos: Position { x: 2, y: 1 } };
        let mut snake = Snake::new(origin);

        assert_eq!(snake.update(&apple, &mut Grid::new()), &Status::Ate);
    }

    #[test]
    fn test_update_if_no_obstacles_then_moves() {
        let origin = Position { x: 2, y: 11 };
        let apple = Apple { pos: Position { x: 0, y: 0 } };
        let mut snake = Snake::new(origin);

        assert_eq!(snake.update(&apple, &mut Grid::new()), &Status::Moved);
    }

    #[test]
    fn test_update_if_moves_constantly_in_one_dir_then_dies_eventually() {
        let origin = Position { x: 4, y: 4 };
        let apple = Apple { pos: Position { x: 0, y: 0 } };
        let mut grid = Grid::new();
        let mut snake = Snake::new(origin);

        for _ in 0..4 {
            snake.update(&apple, &mut grid);
        }

        assert_eq!(snake.update(&apple, &mut grid), &Status::Died);
    }

    #[test]
    fn test_set_dir() {
        let origin = Position { x: 2, y: 0 };
        let apple = Apple { pos: Position { x: 10, y: 10 } };
        let mut grid = Grid::new();
        let mut snake = Snake::new(origin);

        snake.set_dir(Direction::Left);

        assert_eq!(snake.update(&apple, &mut grid), &Status::Moved);
    }

    #[test]
    fn test_update_if_moved_then_the_grid_is_updated_correctly() {
        let origin = Position { x: 1, y: 2 };
        let apple = Apple { pos: Position { x: 6, y: 6 } };
        let mut grid = Grid::new();
        let grid_width = grid.width();
        let mut snake = Snake::new(origin);
        let (ax, ay): (usize, usize) = (
            apple.pos.x.try_into().unwrap(),
            apple.pos.y.try_into().unwrap(),
        );

        grid[ay*grid_width + ax] = Cell::Apple;
        snake.update(&apple, &mut grid);

        let expected = "\
┌───────────────────────────────────────────────────────────────────────┐
│ ┌─┐                                                                   │
│ └─┘                                                                   │
│                                                                       │
│                                                                       │
│                                                                       │
│           ┌─┐                                                         │
│           └─┘                                                         │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
└───────────────────────────────────────────────────────────────────────┘
";
        pretty_assert_eq!(grid.render(), expected);
    }

    #[test]
    fn test_update_if_ate_apples_then_the_grid_is_updated_correctly() {
        let origin = Position { x: 1, y: 3 };
        let mut apple = Apple { pos: Position { x: 1, y: 1 } };
        let mut grid = Grid::new();
        let grid_width = grid.width();
        let mut snake = Snake::new(origin);
        let (ax, ay): (usize, usize) = (
            apple.pos.x.try_into().unwrap(),
            apple.pos.y.try_into().unwrap(),
        );

        grid[ay*grid_width + ax] = Cell::Apple;

        snake.update(&apple, &mut grid); // (1, 2)
        snake.update(&apple, &mut grid); // (1, 1)
        apple.pos = Position { x: 2, y: 1 };
        grid[ay*grid_width + ax] = Cell::Apple;
        snake.set_dir(Direction::Right);
        snake.update(&apple, &mut grid); // (2, 1)
        snake.set_dir(Direction::Down);
        snake.update(&apple, &mut grid); // (2, 2)

        let expected = "\
┌───────────────────────────────────────────────────────────────────────┐
│   ┌─┐                                                                 │
│   ├─┤                                                                 │
│   └─┘                                                                 │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
└───────────────────────────────────────────────────────────────────────┘
";
        pretty_assert_eq!(grid.render(), expected);
    }
}
