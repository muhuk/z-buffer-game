extern crate tcod;

use tcod::console::{self, Console};

const SCREEN_WIDTH_CHAR: i32 = 80;
const SCREEN_HEIGHT_CHAR: i32 = 50;
const FPS: i32 = 30;

fn main() {
    let mut root = console::Root::initializer()
        .title("z-buffer")
        .size(SCREEN_WIDTH_CHAR, SCREEN_HEIGHT_CHAR)
        .font(
            "assets/terminal16x16_gs_ro.png",
            console::FontLayout::AsciiInRow,
        ).init();

    tcod::system::set_fps(FPS);

    while !root.window_closed() {
        root.clear();
        root.set_char(SCREEN_WIDTH_CHAR / 2, SCREEN_HEIGHT_CHAR / 2 - 2, '');
        root.set_alignment(console::TextAlignment::Center);
        root.print_rect(
            SCREEN_WIDTH_CHAR / 2,
            SCREEN_HEIGHT_CHAR / 2 + 2,
            SCREEN_WIDTH_CHAR,
            1,
            "Hello, World!",
        );
        root.flush();
        root.wait_for_keypress(true);
    }
}
