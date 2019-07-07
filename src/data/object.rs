use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum VisibleObject {
    Grass,
    Soil,
    TreeTrunk,
    TreeFoilage,
}

#[derive(Debug)]
pub struct UnrecognizedTileName(String);

impl FromStr for VisibleObject {
    type Err = UnrecognizedTileName;

    // NOTE: This is useful when we implement reading
    //       save files.  Until the save is working
    //       this doesn't serve any purpose.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Grass" => Ok(VisibleObject::Grass),
            "Soil" => Ok(VisibleObject::Soil),
            "TreeTrunk" => Ok(VisibleObject::TreeTrunk),
            "TreeFoilage" => Ok(VisibleObject::TreeFoilage),
            _ => Err(UnrecognizedTileName(s.to_owned())),
        }
    }
}
