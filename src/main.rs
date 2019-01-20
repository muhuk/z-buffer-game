//! z-buffer game

use crate::game::Game;
use log::info;
use stderrlog;

pub mod asset;
pub mod conf;
pub mod game;
pub mod input;
pub mod menu;
pub mod stage;
pub mod ui;

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
