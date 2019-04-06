use crate::asset::Assets;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::result::Result;
use toml;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct Tile {
    name: String,
    glyph_id: u16,
}

#[derive(Debug, Deserialize)]
struct Tiles {
    tiles: Vec<Tile>,
}

impl Tiles {
    pub fn read() -> Tiles {
        Self::from_path(&*Assets::TilesToml.extract().unwrap()).unwrap()
    }

    pub fn get(&self, name: &str) -> Option<&Tile> {
        self.tiles.iter().find(|tile| tile.name == name)
    }

    pub(self) fn from_str(s: &str) -> Result<Tiles, String> {
        let conf: Tiles = toml::from_str::<Tiles>(s)
            .map_err(|e| format!("Failed to parse tiles: {:?}", e))?;
        Result::Ok(conf)
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

        let deserialized: Vec<Tile> = Tiles::from_str(input).unwrap().tiles;
        assert_eq!(
            vec![Tile {
                name: String::from("Test"),
                glyph_id: 0x0001
            }],
            deserialized
        );
    }
}
