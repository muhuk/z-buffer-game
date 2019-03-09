use serde::Deserialize;
use std::collections::HashMap;
use std::result::Result;
use toml;

#[derive(Debug, Deserialize)]
struct Tiles {
    tiles: Vec<Tile>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct Tile {
    name: String,
    glyph_id: u16,
}

impl Tile {
    pub fn read(s: &str) -> Result<HashMap<String, Tile>, String> {
        let mut result: HashMap<String, Tile> = HashMap::new();
        // TODO: Process error from from_str
        let deserialized = toml::from_str::<Tiles>(s).unwrap();
        deserialized
            .tiles
            .iter()
            .map(|t| (t.name.clone(), t))
            .for_each(|(k, v)| {
                result.insert(k, v.clone());
            });
        Result::Ok(result)
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

        let deserialized: HashMap<String, Tile> = Tile::read(input).unwrap();
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
