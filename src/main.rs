//! z-buffer game

mod asset;
mod game;
mod input;
mod stage;
mod ui;

use crate::game::Game;

fn main() {
    let mut game_state = Game::new();
    game_state.main_loop();
}
