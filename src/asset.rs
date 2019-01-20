use crate::conf;
use log::debug;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::sync::Once;

const TERMINAL16X16_GS_RO_PATH: &str = "terminal16x16_gs_ro.png";
static TERMINAL16X16_GS_RO_DATA: &[u8] = include_bytes!("../assets/terminal16x16_gs_ro.png");

static INIT: Once = Once::new();

pub enum Assets {
    FontTerminal16x16GsRo,
}

impl Assets {
    pub fn extract(&self) -> Result<Box<Path>> {
        initialize();
        let (file_name, data) = match self {
            Assets::FontTerminal16x16GsRo => (TERMINAL16X16_GS_RO_PATH, TERMINAL16X16_GS_RO_DATA),
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
