mod apple;
mod bitmap;
mod direction;
mod grid;
mod position;
mod snake;
mod game;

use crate::game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
