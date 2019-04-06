use crate::asset::Assets;
use crate::data::VisibleObject;
use serde::Deserialize;
use std::char::decode_utf16;
use std::collections::HashMap;
use std::fs;
use std::iter;
use std::path::Path;
use std::result::Result;
use toml;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tile {
    object_id: VisibleObject,
    glyph_id: u16,
}

impl Tile {
    pub fn glyph(&self) -> char {
        decode_utf16(iter::once(self.glyph_id))
            .next()
            .unwrap()
            .unwrap()
    }
}

#[derive(Debug)]
pub struct Tiles {
    tiles: HashMap<VisibleObject, Tile>,
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

    pub fn get(&self, key: VisibleObject) -> Option<Tile> {
        self.tiles.get(&key).cloned()
    }

    pub(self) fn from_str(s: &str) -> Result<Tiles, String> {
        let conf: TilesConfig = toml::from_str::<TilesConfig>(s)
            .map_err(|e| format!("Failed to parse tiles: {:?}", e))?;
        let tiles = Tiles {
            tiles: conf
                .tiles
                .iter()
                .map(|tile| {
                    let object_id = VisibleObject::from_str(&tile.name)
                        .expect(
                            format!(
                                "Unrecognized tile name \"{}\"",
                                tile.name
                            )
                            .as_str(),
                        );
                    (
                        object_id,
                        Tile {
                            object_id,
                            glyph_id: tile.glyph_id,
                        },
                    )
                })
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
            name = "Grass"
            glyph_id = 0x0001
        "#;

        let tiles: Tiles = Tiles::from_str(input).unwrap();
        assert_eq!(1, tiles.tiles.iter().count());
        assert_eq!(
            Some(&Tile {
                object_id: VisibleObject::Grass,
                glyph_id: 0x0001
            }),
            tiles.tiles.get(&VisibleObject::Grass)
        );
    }
}
