//! z-buffer game

use crate::game::Game;
use log::info;
use stderrlog;

mod asset;
mod game;
mod input;
mod menu;
mod stage;
mod ui;

// 4 is log everything, 3 is DEBUG.
const LOG_VERBOSITY: usize = 4;

fn main() {
    init_logger();
    info!("Starting z-buffer-game.");

    let mut game_state = Game::new();
    game_state.main_loop();
}

fn init_logger() {
    stderrlog::new()
        .verbosity(LOG_VERBOSITY)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .module(module_path!())
        .init()
        .unwrap()
}
