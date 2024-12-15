use std::collections::LinkedList;

use crate::direction::{ Direction, are_opposite };
use crate::grid::{ Cell, Grid };
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
        let curr_pos = self.segments.front().unwrap();
        let new_pos = curr_pos.move_in_direction(&self.dir);
        let width = grid.width();
        let height = grid.height();

        if let Ok(Position { mut x, mut y }) = new_pos {
            if x >= width || y >= height || grid[y*width + x] == Cell::Snake {
                self.status = Status::Died;
                return &self.status;
            }

            let head_pos = Position { x, y };
            for segment in self.segments.iter_mut() {
                (segment.x, x) = (x, segment.x);
                (segment.y, y) = (y, segment.y);
            }
            let tail_pos = Position { x, y };

            if self.status == Status::Ate {
                self.segments.push_back(tail_pos);
            } else {
                grid[tail_pos.y*width + tail_pos.x] = Cell::Empty;
            }

            if grid[head_pos.y*width + head_pos.x] == Cell::Apple {
                self.status = Status::Ate;
            } else {
                self.status = Status::Moved;
            }

            grid[head_pos.y * width + head_pos.x] = Cell::Snake;
        } else {
            self.status = Status::Died;
        }

        &self.status
    }

    pub fn set_dir(&mut self, new_dir: Direction) {
        if !are_opposite(&self.dir, &new_dir) {
            self.dir = new_dir;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as pretty_assert_eq;

    #[test]
    fn test_update_if_crossing_the_top_edge_then_dies() {
        let origin = Position { x: 2, y: 0 };
        let mut snake = Snake::new(origin);

        assert_eq!(snake.update(&mut Grid::new()), &Status::Died);
    }

    #[test]
    fn test_update_if_encountered_apple_then_returns_ate() {
        let origin = Position { x: 2, y: 2 };
        let mut grid = Grid::new();
        let width = grid.width();
        grid[1*width + 2] = Cell::Apple;
        let mut snake = Snake::new(origin);

        assert_eq!(snake.update(&mut grid), &Status::Ate);
    }

    #[test]
    fn test_update_if_no_obstacles_then_moves() {
        let origin = Position { x: 2, y: 11 };
        let mut snake = Snake::new(origin);

        assert_eq!(snake.update(&mut Grid::new()), &Status::Moved);
    }

    #[test]
    fn test_update_if_moves_constantly_in_one_dir_then_dies_eventually() {
        let origin = Position { x: 4, y: 4 };
        let mut grid = Grid::new();
        let mut snake = Snake::new(origin);

        for _ in 0..4 {
            snake.update(&mut grid);
        }

        assert_eq!(snake.update(&mut grid), &Status::Died);
    }

    #[test]
    fn test_set_dir() {
        let origin = Position { x: 2, y: 0 };
        let mut grid = Grid::new();
        let mut snake = Snake::new(origin);

        snake.set_dir(Direction::Left);

        assert_eq!(snake.update(&mut grid), &Status::Moved);
    }

    #[test]
    fn test_update_if_moved_then_the_grid_is_updated_correctly() {
        let origin = Position { x: 1, y: 2 };
        let mut grid = Grid::new();
        let width = grid.width();
        grid[6*width + 6] = Cell::Apple;
        let mut snake = Snake::new(origin);
        snake.update(&mut grid);

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
        let mut grid = Grid::new();
        let width = grid.width();
        grid[1*width + 1] = Cell::Apple;
        let mut snake = Snake::new(origin);

        snake.update(&mut grid); // (1, 2)
        snake.update(&mut grid); // (1, 1)
        let width = grid.width();
        grid[1*width + 2] = Cell::Apple;
        snake.set_dir(Direction::Right);
        snake.update(&mut grid); // (2, 1)
        snake.set_dir(Direction::Down);
        snake.update(&mut grid); // (2, 2)

        let expected = "\
┌───────────────────────────────────────────────────────────────────────┐
│ ┌─┬─┐                                                                 │
│ └─┼─┤                                                                 │
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

    #[test]
    fn test_update_if_collided_with_itself_then_dies() {
        let origin = Position { x: 1, y: 18 };
        let oy = origin.y;
        let mut grid = Grid::new();
        let width = grid.width();
        for y in (1..18).step_by(2) {
            grid[y*width + 1] = Cell::Apple;
        }
        let mut snake = Snake::new(origin);

        for _ in 1..oy {
            snake.update(&mut grid);
        }

        snake.set_dir(Direction::Right);
        snake.update(&mut grid);
        snake.update(&mut grid);

        snake.set_dir(Direction::Down);
        snake.update(&mut grid);
        snake.update(&mut grid);

        snake.set_dir(Direction::Left);
        snake.update(&mut grid);

        assert_eq!(snake.update(&mut grid), &Status::Died);
    }

    #[test]
    fn test_update_when_snake_eats_second_time_in_a_row() {
        let origin = Position { x: 1, y: 4 };
        let mut grid = Grid::new();
        let width = grid.width();
        grid[2*width + 1] = Cell::Apple;
        grid[3*width + 1] = Cell::Apple;
        let mut snake = Snake::new(origin);

        for _ in 0..3 {
            snake.update(&mut grid);
        }

        let expected = "\
┌───────────────────────────────────────────────────────────────────────┐
│ ┌─┐                                                                   │
│ ├─┤                                                                   │
│ ├─┤                                                                   │
│ └─┘                                                                   │
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
