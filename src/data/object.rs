use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum VisibleObject {
    Grass,
    Soil,
}

#[derive(Debug)]
pub struct UnrecognizedTileName(String);

impl FromStr for VisibleObject {
    type Err = UnrecognizedTileName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Grass" => Ok(VisibleObject::Grass),
            "Soil" => Ok(VisibleObject::Soil),
            _ => Err(UnrecognizedTileName(s.to_owned())),
        }
    }
}
