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

use crate::conf;
use log::debug;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::sync::Once;

const TERMINAL16X16_GS_RO_PATH: &str = "terminal16x16_gs_ro.png";
static TERMINAL16X16_GS_RO_DATA: &[u8] =
    include_bytes!("../assets/terminal16x16_gs_ro.png");

static INIT: Once = Once::new();

pub enum Assets {
    FontTerminal16x16GsRo,
}

impl Assets {
    pub fn extract(&self) -> Result<Box<Path>> {
        initialize();
        let (file_name, data) = match self {
            Assets::FontTerminal16x16GsRo => {
                (TERMINAL16X16_GS_RO_PATH, TERMINAL16X16_GS_RO_DATA)
            }
        };
        let mut path = conf::assets_directory();
        path.push(&file_name);
        debug!("Extracting file: {:?}", &path);
        fs::write(&path, &data)?;
        Ok(path.into_boxed_path())
    }
}

fn initialize() {
    INIT.call_once(|| {
        let assets_dir = conf::assets_directory();
        if !assets_dir.exists() {
            debug!("Creating assets directory: {:?}", &assets_dir);
            fs::create_dir_all(&assets_dir).unwrap();
        }
    });
}
