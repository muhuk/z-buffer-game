//! Various defaults for the game.
//!
//! Public access is through functions are to allow reading configuration
//! externally later.

use dirs;
use std::path::PathBuf;

const ASSETS_DIR: &str = "assets";
const MAX_FPS: u32 = 30;
const NAME: &str = "z-buffer-game";
const SCREEN_WIDTH_CHAR: u32 = 80;
const SCREEN_HEIGHT_CHAR: u32 = 50;

pub fn data_directory() -> PathBuf {
    let mut dir = dirs::home_dir().unwrap();
    dir.push(format!(".{}", NAME));
    dir
}

pub fn assets_directory() -> PathBuf {
    let mut dir = data_directory();
    dir.push(ASSETS_DIR);
    dir
}

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
    NAME
}
