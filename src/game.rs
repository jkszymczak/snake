use rand;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use std::io::{stdout, Stdout, Write};
use std::thread;
use std::time::{Duration, Instant};
use termion::{
    screen::IntoAlternateScreen,
    raw::IntoRawMode,
    event::Key,
    input::TermRead,
};

use crate::apple::Apple;
use crate::direction::Direction;
use crate::grid::{ Cell, Grid };
use crate::position::Position;
use crate::snake::{ Snake, Status };

const FRAME_DURATION: Duration = Duration::from_millis(200);

#[derive(PartialEq)]
enum State {
    Playing,
    GameOver,
}

pub struct Game {
    apple: Apple,
    grid: Grid,
    snake: Snake,
    points: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut grid = Grid::new(); 
        let width: i32 = grid.width().try_into().unwrap();
        let height: i32 = grid.height().try_into().unwrap();
        let origin = Position {
            x: width / 2,
            y: height / 2,
        };
        // TODO: Ensure the apple position does not collide with one of the
        // snake's segments.
        let apple = Apple {
            pos: Position {
                x: (rand::random::<i32>() % width).abs(),
                y: (rand::random::<i32>() % height).abs(),
            },
        };
        // TODO: Remove the `Apple` structure. It is not needed, since we
        // set the cell's type manually anyway.
        grid[(apple.pos.y*width + apple.pos.x).try_into().unwrap()]
            = Cell::Apple;
        grid[(origin.y*width + origin.x).try_into().unwrap()] = Cell::Snake;
        let snake = Snake::new(origin);
        Self {
            apple,
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
        // TODO: Make `Position` fields be of type `i32`, so we do not need
        // to cast `grid.width` to `i32` everywhere.
        let width: i32 = self.grid.width().try_into().unwrap();
        let height: i32 = self.grid.height().try_into().unwrap();

        match self.snake.update(&self.apple, &mut self.grid) {
            Status::Ate => {
                // TODO: Ensure the apple position does not collide with one
                // of the snake's segments.
                self.apple.pos.x = (rand::random::<i32>() % width).abs();
                self.apple.pos.y = (rand::random::<i32>() % height).abs();
                self.grid[
                    (self.apple.pos.y*width + self.apple.pos.x)
                        .try_into().unwrap()
                ] = Cell::Apple;

                State::Playing
            },
            Status::Died => State::GameOver,
            Status::Moved => State::Playing,
        }
    }

    fn render(&self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        // TODO: Show points
        // TODO: Center the grid in the terminal window
        write!(
            screen,
            "{}",
            termion::cursor::Goto(1, 1)
        ).expect("Failed to set cursor position");

        screen.flush().unwrap();

        for line in self.grid.render().lines() {
            print!("{}\r\n", line);
        }
    }
}
