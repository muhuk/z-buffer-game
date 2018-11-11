//! z-buffer game

extern crate dirs;
extern crate tcod;

mod asset;
mod stage;
mod state;
mod ui;

fn main() {
    let mut root = state::initialize().ui.root_console;

    while !root.window_closed() {
        ui::draw_hello_world(&mut root);
        root.flush();
        root.wait_for_keypress(true);
    }
}
