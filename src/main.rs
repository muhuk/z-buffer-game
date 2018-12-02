//! z-buffer game

extern crate dirs;
extern crate tcod;

mod asset;
mod event;
mod game;
mod stage;
mod ui;

use game::Game;

fn main() {
    let mut game_state = Game::init();
    game_state.main_loop();
}
