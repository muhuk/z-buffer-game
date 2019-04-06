use crate::asset::Assets;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::result::Result;
use toml;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Tile {
    name: String,
    glyph_id: u16,
}

#[derive(Debug)]
pub struct Tiles {
    tiles: HashMap<String, Tile>,
}

#[derive(Deserialize)]
struct TilesConfig {
    tiles: Vec<Tile>,
}

impl Tiles {
    pub fn read() -> Tiles {
        Self::from_path(&*Assets::TilesToml.extract().unwrap()).unwrap()
    }

    pub(self) fn from_str(s: &str) -> Result<Tiles, String> {
        let conf: TilesConfig = toml::from_str::<TilesConfig>(s)
            .map_err(|e| format!("Failed to parse tiles: {:?}", e))?;
        let tiles = Tiles {
            tiles: conf
                .tiles
                .iter()
                .map(|tile| (tile.name.clone(), tile.clone()))
                .collect(),
        };
        Result::Ok(tiles)
    }

    fn from_path(path: &Path) -> Result<Tiles, String> {
        Self::from_str(&fs::read_to_string(path).map_err(|e| {
            format!("Failed to read {}, {:?}", path.to_str().unwrap(), e)
        })?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_tiles_from_a_string() {
        let input = r#"
            [[tiles]]
            name = "Test"
            glyph_id = 0x0001
        "#;

        let tiles: Tiles = Tiles::from_str(input).unwrap();
        assert_eq!(1, tiles.tiles.iter().count());
        assert_eq!(
            Some(&Tile {
                name: String::from("Test"),
                glyph_id: 0x0001
            }),
            tiles.tiles.get("Test")
        );
    }
}
