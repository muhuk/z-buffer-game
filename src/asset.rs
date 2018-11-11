use dirs;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Once;

const APP_NAME: &str = "z-buffer-game";
const ASSETS_DIR: &str = "assets";

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
        let mut path = get_data_dir();
        path.push(&file_name);
        println!("Extracting file: {:?}", &path);
        fs::write(&path, &data)?;
        Ok(path.into_boxed_path())
    }
}

fn get_data_dir() -> PathBuf {
    // ~/.z-buffer-game/assets/
    let mut dir = dirs::home_dir().unwrap();
    dir.push(format!(".{}", APP_NAME));
    dir.push(ASSETS_DIR);
    dir
}

fn initialize() {
    INIT.call_once(|| {
        let assets_dir = get_data_dir();
        if !assets_dir.exists() {
            println!("Creating assets directory: {:?}", &assets_dir);
            fs::create_dir_all(&assets_dir).unwrap();
        }
    });
}
