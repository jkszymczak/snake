mod bitmap;
mod direction;
mod grid;
mod position;
mod snake;
mod game;

use crate::game::{ Game, GameError };

fn main() {
    let mut game = Game::new();
    if let Err(e) = game.run() {
        let msg = match e {
            GameError::FlushScreen => {
                "Failed to flush screen"
            },
            GameError::GetTerminalSize => {
                "Failed to get terminal size"
            }
            GameError::HideCursor => {
                "Failed to hide cursor"
            },
            GameError::SetCursorPos => {
                "Failed to set cursor position"
            },
            GameError::ShowCursor => {
                "Failed to show cursor"
            },
            GameError::SwitchIntoAlternateScreen => {
                "Failed to switch into alternate screen"
            },
            GameError::SwitchIntoRawMode => {
                "Failed to switch into raw mode"
            },
            GameError::TerminalWidthTooSmall => {
                "Terminal width is too small"
            },
            GameError::TerminalHeightTooSmall => {
                "Terminal height is too small"
            },
        };
        eprintln!("{}", msg);
    }
}
