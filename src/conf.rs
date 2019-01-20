//! Various defaults for the game.
//!
//! Public access is through functions are to allow reading configuration
//! externally later.

const MAX_FPS: u32 = 30;
const SCREEN_WIDTH_CHAR: u32 = 80;
const SCREEN_HEIGHT_CHAR: u32 = 50;
const WINDOW_TITLE: &str = "z-buffer";

pub fn max_fps() -> u32 {
    MAX_FPS
}

pub fn screen_width_char() -> u32 {
    SCREEN_WIDTH_CHAR
}

pub fn screen_height_char() -> u32 {
    SCREEN_HEIGHT_CHAR
}

pub fn window_title() -> &'static str {
    WINDOW_TITLE
}
