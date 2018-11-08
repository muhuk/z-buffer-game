//! z-buffer game

extern crate tcod;

pub mod ui;

use ui::UI;

/// Application state.
pub struct State {
    ui: UI,
}

fn initialize() -> State {
    State {
        ui: ui::initialize(),
    }
}

fn main() {
    let mut root = initialize().ui.root_console;

    while !root.window_closed() {
        ui::draw_hello_world(&mut root);
        root.flush();
        root.wait_for_keypress(true);
    }
}
