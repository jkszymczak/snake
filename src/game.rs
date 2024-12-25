use std::io::{stdout, Stdout, Write};
use std::thread;
use std::time::{Duration, Instant};
use termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    raw::RawTerminal,
    screen::AlternateScreen,
    screen::IntoAlternateScreen,
};

use crate::direction::Direction;
use crate::grid::{
    Cell,
    Grid,
    GRID_WIDTH_IN_CHARS,
    GRID_HEIGHT_IN_CHARS,
};
use crate::position::Position;
use crate::snake::{ Snake, Status };

const FRAME_DURATION: Duration = Duration::from_millis(100);

#[derive(PartialEq)]
enum State {
    Playing,
    GameOver,
}

pub struct Game {
    grid: Grid,
    snake: Snake,
    points: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut grid = Grid::new(); 
        let width = grid.width();
        let height = grid.height();
        let origin = Position {
            x: width / 2,
            y: height / 2,
        };
        grid[origin.y*width + origin.x] = Cell::Snake;
        grid.gen_apple();
        let snake = Snake::new(origin);
        Self {
            grid,
            snake,
            points: 0,
        }
    }

    pub fn run(&mut self) -> usize {
        let mut stdin = termion::async_stdin().keys();
        let stdout = stdout().into_raw_mode().unwrap();
        let mut screen = stdout.into_alternate_screen().unwrap();

        write!(
            screen,
            "{}",
            termion::cursor::Hide,
        ).expect("Failed to hide cursor");

        let mut time = Instant::now();
        let mut state = State::Playing;
        while state == State::Playing {
            state = self.update();
            self.render(&mut screen);

            let elapsed = Instant::now().duration_since(time);
            if let Some(t) = FRAME_DURATION.checked_sub(elapsed) {
                thread::sleep(t);
            }
            time = Instant::now();

            let mut input = stdin.next();
            while let Some(Ok(key)) = input {
                match key {
                    Key::Left  | Key::Char('h') => {
                        self.snake.set_dir(Direction::Left);
                    },
                    Key::Down  | Key::Char('j') => {
                        self.snake.set_dir(Direction::Down);
                    },
                    Key::Up    | Key::Char('k') => {
                        self.snake.set_dir(Direction::Up);
                    },
                    Key::Right | Key::Char('l') => {
                        self.snake.set_dir(Direction::Right);
                    },
                    Key::Char('q') => {
                        state = State::GameOver;
                    },
                    _ => (),
                }
                input = stdin.next();
            }
        }

        write!(
            screen,
            "{}",
            termion::cursor::Show,
        ).expect("Failed to show cursor");

        self.points
    }

    fn update(&mut self) -> State {
        match self.snake.update(&mut self.grid) {
            Status::Ate => {
                self.grid.gen_apple();
                State::Playing
            },
            Status::Died => State::GameOver,
            Status::Moved => State::Playing,
        }
    }

    fn render(&self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        // TODO: Show points
        write!(
            screen,
            "{}",
            termion::cursor::Goto(1, 1)
        ).expect("Failed to set cursor position");

        let (col_count, row_count) = termion::terminal_size().expect(
            "Failed to get terminal size"
        );

        if GRID_HEIGHT_IN_CHARS + 1 > (row_count as usize) {
            panic!("Terminal window height is too small");
        }
        if GRID_WIDTH_IN_CHARS > (col_count as usize) {
            panic!("Terminal window width is too small");
        }

        let top_margin = (row_count as usize)/2 - GRID_HEIGHT_IN_CHARS/2;
        let left_margin = (col_count as usize)/2 - GRID_WIDTH_IN_CHARS/2;

        for _ in 0..top_margin - 1 {
            print!("\r\n");
        }

        for line in self.grid.render().lines() {
            print!("\r\n");
            print!("{}", String::from(" ").repeat(left_margin));
            print!("{}", line);
        }

        screen.flush().unwrap();
    }
}
