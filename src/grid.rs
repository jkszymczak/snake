use std::ops::{Index, IndexMut};

use crate::bitmap::{Bitmap, State};

const GRID_WIDTH: usize = 36;
const GRID_HEIGHT: usize = 20;
const GRID_SIZE: usize = GRID_WIDTH*GRID_HEIGHT;

#[derive(Debug, PartialEq)]
pub enum Cell {
    Empty,
    Apple,
    Snake,
}

pub struct Grid {
    cells: [Cell; GRID_SIZE],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [const { Cell::Empty }; GRID_SIZE],
        }
    }

    pub fn width(&self) -> usize {
        GRID_WIDTH
    }

    pub fn height(&self) -> usize {
        GRID_HEIGHT
    }

    pub fn size(&self) -> usize {
        GRID_WIDTH*GRID_HEIGHT
    }

    pub fn render(&self) -> String {
        let width = self.width();
        let height = self.height();
        let mut output = String::new();

        for y in 0..=height {
            for x in 0..=width {
                output.push_str(&self.render_cell(x, y));
            }
            output.pop(); // Pop the whitespace
            output.push('\n');
        }

        output
    }

    fn render_cell(&self, x: usize, y: usize) -> String {
        use State::*;

        let mut bitmap = Bitmap::new();
        let width = self.width();
        let height = self.height();

        if x == 0 && y == 0 {
            // Top-left corner
            bitmap += Bitmap::from([
                   E,
                E, N, N,
                   N,
            ]);
        } else if x == width && y == 0 {
            // Top-right corner
            bitmap += Bitmap::from([
                   E,
                N, N, E,
                   N,
            ]);
        } else if x == 0 && y == height {
            // Bottom-left corner
            bitmap += Bitmap::from([
                   N,
                E, N, N,
                   E,
            ]);
        } else if x == width && y == height {
            // Bottom-right corner
            bitmap += Bitmap::from([
                   N,
                N, N, E,
                   E,
            ]);
        } else if y == 0 || y == height {
            // Top or bottom border
            bitmap += Bitmap::from([
                   E,
                N, N, N,
                   E,
            ]);
        } else if x == 0 || x == width {
            // Left or right border
            bitmap += Bitmap::from([
                   N,
                E, N, E,
                   N,
            ]);
        }

        if x > 0 && y > 0 && self[(y-1)*width + (x-1)] != Cell::Empty {
            // Non-empty top-left cell
            bitmap += Bitmap::from([
                   N,
                N, N, E,
                   E,
            ]);
        }
        if y > 0 && x < width && self[(y-1)*width + x] != Cell::Empty {
            // Non-empty top cell
            bitmap += Bitmap::from([
                   N,
                E, N, N,
                   E,
            ]);
        }
        if x > 0 && self[y*width + (x-1)] != Cell::Empty {
            // Non-empty left cell
            bitmap += Bitmap::from([
                   E,
                N, N, E,
                   N,
            ]);
        }
        if x < width && y < height && self[y*width + x] != Cell::Empty {
            // Non-empty cell
            bitmap += Bitmap::from([
                   E,
                E, N, N,
                   N,
            ]);
        }

        bitmap.render()
    }
}

impl Index<usize> for Grid {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size() {
            return &Cell::Empty;
        }
        &self.cells[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as pretty_assert_eq;

    #[test]
    fn test_index() {
        let grid = Grid::new();
        let size = grid.size();

        for i in 0..size {
            assert_eq!(grid[i], Cell::Empty);
        }
    }

    #[test]
    fn test_index_mut() {
        let mut grid = Grid::new();

        grid[2] = Cell::Apple;

        assert_eq!(grid[2], Cell::Apple);
    }

    #[test]
    fn test_render_when_all_cells_are_empty() {
        let grid = Grid::new();
        let expected = "\
┌───────────────────────────────────────────────────────────────────────┐
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
│                                                                       │
│                                                                       │
│                                                                       │
└───────────────────────────────────────────────────────────────────────┘
";
        pretty_assert_eq!(grid.render(), expected);
    }

    #[test]
    fn test_render_when_there_is_an_apple_cell() {
        let mut grid = Grid::new();
        let width = grid.width();
        let height = grid.height();
        grid[(height/2 - 1)*width + width/2 - 1] = Cell::Apple;
        let expected = "\
┌───────────────────────────────────────────────────────────────────────┐
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                 ┌─┐                                   │
│                                 └─┘                                   │
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
    fn test_render_when_there_is_a_snake() {
        let mut grid = Grid::new();
        let snake_segment_pos = vec![
            74, 75, 76, 112, 148, 184, 220, 221, 222, 223,
        ];
        for i in snake_segment_pos {
            grid[i] = Cell::Snake;
        }
        let expected = "\
┌───────────────────────────────────────────────────────────────────────┐
│                                                                       │
│   ┌─┬─┬─┐                                                             │
│   └─┴─┼─┤                                                             │
│       ├─┤                                                             │
│       ├─┤                                                             │
│       ├─┼─┬─┬─┐                                                       │
│       └─┴─┴─┴─┘                                                       │
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
    fn test_render_when_snake_collides_with_walls() {
        let mut grid = Grid::new();
        let snake_segment_pos = vec![
            72, 73, 74, 38, 2, 3, 4, 40, 76, 112, 148, 184, 220, 221, 222,
            223, 224, 225, 226, 227, 228, 264, 300, 336, 372, 408, 444, 480,
            516, 552, 588, 624, 660, 696, 697, 698, 699, 700, 701, 702, 703,
            704, 705, 706, 707, 708, 709, 710, 711, 675, 639, 603, 567, 531,
            532, 533, 534, 535, 536, 537, 538, 539, 503, 467, 431
        ];
        for i in snake_segment_pos {
            grid[i] = Cell::Snake;
        }
        let expected = "\
┌───┬─┬─┬─┬─────────────────────────────────────────────────────────────┐
│   ├─┼─┼─┤                                                             │
├─┬─┼─┤ ├─┤                                                             │
├─┴─┴─┘ ├─┤                                                             │
│       ├─┤                                                             │
│       ├─┤                                                             │
│       ├─┼─┬─┬─┬─┬─┬─┬─┬─┐                                             │
│       └─┴─┴─┴─┴─┴─┴─┴─┼─┤                                             │
│                       ├─┤                                             │
│                       ├─┤                                             │
│                       ├─┤                                             │
│                       ├─┤                                           ┌─┤
│                       ├─┤                                           ├─┤
│                       ├─┤                                           ├─┤
│                       ├─┤                           ┌─┬─┬─┬─┬─┬─┬─┬─┼─┤
│                       ├─┤                           ├─┼─┴─┴─┴─┴─┴─┴─┴─┤
│                       ├─┤                           ├─┤               │
│                       ├─┤                           ├─┤               │
│                       ├─┤                           ├─┤               │
│                       ├─┼─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┼─┤               │
└───────────────────────┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴───────────────┘
";
        pretty_assert_eq!(grid.render(), expected);
    }
}
