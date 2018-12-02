//! z-buffer game

extern crate dirs;
extern crate tcod;

mod asset;
mod game;
mod input;
mod stage;
mod ui;

use game::Game;

fn main() {
    let mut game_state = Game::new();
    game_state.main_loop();
}
