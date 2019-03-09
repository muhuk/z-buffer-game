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
    pub fn read() -> HashMap<String, Tile> {
        Self::from_path(&*Assets::TilesToml.extract().unwrap()).unwrap()
    }

    pub(self) fn from_str(s: &str) -> Result<HashMap<String, Tile>, String> {
        let conf: Tiles = toml::from_str::<Tiles>(s)
            .map_err(|e| format!("Failed to parse tiles: {:?}", e))?;

        let mut result: HashMap<String, Tile> = HashMap::new();
        conf.tiles
            .iter()
            .map(|t| (t.name.clone(), t))
            .for_each(|(k, v)| {
                result.insert(k, v.clone());
            });
        Result::Ok(result)
    }

    fn from_path(path: &Path) -> Result<HashMap<String, Tile>, String> {
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

        let deserialized: HashMap<String, Tile> =
            Tiles::from_str(input).unwrap();
        assert_eq!(
            vec!["Test"],
            deserialized.keys().collect::<Vec<&String>>()
        );
        let key = "Test";
        assert_eq!(
            Tile {
                name: key.to_owned(),
                glyph_id: 1u16
            },
            *deserialized.get(key).unwrap()
        );
    }
}
