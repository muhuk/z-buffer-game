use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum TileId {
    Grass,
    Soil,
}

#[derive(Debug)]
pub struct UnrecognizedTileName(String);

impl FromStr for TileId {
    type Err = UnrecognizedTileName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Grass" => Ok(TileId::Grass),
            "Soil" => Ok(TileId::Soil),
            _ => Err(UnrecognizedTileName(s.to_owned())),
        }
    }
}
