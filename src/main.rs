//! z-buffer game

extern crate dirs;
extern crate tcod;

mod asset;
mod stage;
mod state;
mod ui;

use state::State;

fn main() {
    let mut st = State::init();
    st.main_loop();
}
