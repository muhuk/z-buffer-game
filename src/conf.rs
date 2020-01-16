// © Copyright 2019-2020, Atamert Ölçgen
//
// This file is part of z-buffer-game.
//
// z-buffer-game is free software: you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// z-buffer-game is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with z-buffer-game.  If not, see <https://www.gnu.org/licenses/>.

//! Configuration access and defaults.
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

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn data_directory() -> PathBuf {
    let mut dir = dirs::home_dir().unwrap();
    dir.push(format!(".{}", NAME));
    dir
}

#[cfg(target_os = "windows")]
pub fn data_directory() -> PathBuf {
    let mut dir = dirs::home_dir().unwrap();
    dir.push("My Documents");
    dir.push(NAME);
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
