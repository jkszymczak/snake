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

pub enum GameError {
    FlushScreen,
    GetTerminalSize,
    HideCursor,
    SetCursorPos,
    ShowCursor,
    SwitchIntoAlternateScreen,
    SwitchIntoRawMode,
    TerminalHeightTooSmall,
    TerminalWidthTooSmall,
}

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

    pub fn run(&mut self) -> Result<usize, GameError> {
        let mut stdin = termion::async_stdin().keys();
        let stdout = match stdout().into_raw_mode() {
            Ok(stdout) => stdout,
            Err(_) => return Err(GameError::SwitchIntoRawMode),
        };
        let mut screen = match stdout.into_alternate_screen() {
            Ok(screen) => screen,
            Err(_) => return Err(GameError::SwitchIntoAlternateScreen),
        };

        if let Err(_) = write!(screen, "{}", termion::cursor::Hide) {
            return Err(GameError::HideCursor);
        }

        let mut time = Instant::now();
        let mut state = State::Playing;
        while state == State::Playing {
            state = self.update();
            if let Err(e) = self.render(&mut screen) {
                write!(screen, "{}", termion::cursor::Show).unwrap();
                return Err(e);
            }

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

        if let Err(_) = write!(screen, "{}", termion::cursor::Show) {
            return Err(GameError::ShowCursor);
        }

        Ok(self.points)
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

    fn render(
        &self,
        screen: &mut AlternateScreen<RawTerminal<Stdout>>,
    ) -> Result<(), GameError> {
        // TODO: Show points

        if let Err(_) = write!(screen, "{}", termion::cursor::Goto(1, 1)) {
            return Err(GameError::SetCursorPos);
        }

        let (col_count, row_count) = match termion::terminal_size() {
            Ok(size) => size,
            Err(_) => return Err(GameError::GetTerminalSize),
        };

        if GRID_HEIGHT_IN_CHARS + 1 > (row_count as usize) {
            return Err(GameError::TerminalHeightTooSmall);
        }
        if GRID_WIDTH_IN_CHARS > (col_count as usize) {
            return Err(GameError::TerminalWidthTooSmall);
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

        if let Err(_) = screen.flush() {
            return Err(GameError::FlushScreen);
        }

        Ok(())
    }
}
