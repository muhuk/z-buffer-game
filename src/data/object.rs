#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum TileId {
    Grass,
    Soil,
}

impl TileId {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "Grass" => Some(TileId::Grass),
            "Soil" => Some(TileId::Soil),
            _ => None,
        }
    }
}
