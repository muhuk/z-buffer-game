use crate::asset::Assets;
use crate::data::TileId;
use serde::Deserialize;
use std::char::decode_utf16;
use std::collections::HashMap;
use std::fs;
use std::iter;
use std::path::Path;
use std::result::Result;
use std::str::FromStr;
use toml;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tile {
    tile_id: TileId,
    glyph: char,
}

impl Tile {
    pub fn glyph(self) -> char {
        self.glyph
    }
}

#[derive(Debug)]
pub struct Tiles {
    tiles: HashMap<TileId, Tile>,
}

#[derive(Deserialize)]
pub struct TileEntry {
    name: String,
    glyph_id: u16,
}

#[derive(Deserialize)]
struct TilesConfig {
    tiles: Vec<TileEntry>,
}

impl Tiles {
    pub fn read() -> Tiles {
        Self::from_path(&*Assets::TilesToml.extract().unwrap()).unwrap()
    }

    pub fn get(&self, key: TileId) -> Option<Tile> {
        self.tiles.get(&key).cloned()
    }

    pub(self) fn parse_str(s: &str) -> Result<Tiles, String> {
        let conf: TilesConfig = toml::from_str::<TilesConfig>(s)
            .map_err(|e| format!("Failed to parse tiles: {:?}", e))?;
        let tiles = Tiles {
            tiles: conf
                .tiles
                .iter()
                .map(|tile| {
                    let tile_id = TileId::from_str(&tile.name).unwrap();
                    let glyph: char = decode_utf16(iter::once(tile.glyph_id))
                        .next()
                        .unwrap()
                        .unwrap();
                    (tile_id, Tile { tile_id, glyph })
                })
                .collect(),
        };
        Result::Ok(tiles)
    }

    fn from_path(path: &Path) -> Result<Tiles, String> {
        Self::parse_str(&fs::read_to_string(path).map_err(|e| {
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
            name = "Grass"
            glyph_id = 0x0001
        "#;

        let tiles: Tiles = Tiles::parse_str(input).unwrap();
        assert_eq!(1, tiles.tiles.iter().count());
        assert_eq!(
            Some(&Tile {
                tile_id: TileId::Grass,
                glyph: '\u{01}'
            }),
            tiles.tiles.get(&TileId::Grass)
        );
    }
}
