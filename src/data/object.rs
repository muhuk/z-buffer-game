// © Copyright 2019-2020, Atamert Ölçgen
//
// This file is part of z-buffer-game.
//
// z-buffer-game is free software: you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// z-buffer-game is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with z-buffer-game.  If not, see <https://www.gnu.org/licenses/>.

use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum VisibleObject {
    Grass,
    Rock,
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
            "Rock" => Ok(VisibleObject::Rock),
            "Soil" => Ok(VisibleObject::Soil),
            "TreeTrunk" => Ok(VisibleObject::TreeTrunk),
            "TreeFoilage" => Ok(VisibleObject::TreeFoilage),
            _ => Err(UnrecognizedTileName(s.to_owned())),
        }
    }
}
