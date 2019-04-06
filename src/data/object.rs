#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum VisibleObject {
    Grass,
    Soil,
}

impl VisibleObject {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "Grass" => Some(VisibleObject::Grass),
            "Soil" => Some(VisibleObject::Soil),
            _ => None,
        }
    }
}
