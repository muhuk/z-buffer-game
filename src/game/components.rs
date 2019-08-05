use crate::data::{Location, VisibleObject};
use specs::prelude::*;
use specs::storage::BTreeStorage;
use specs_derive::*;

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct MapTile {
    pub location: Location,
    pub object: VisibleObject,
}

impl MapTile {
    pub fn new(location: Location, object: VisibleObject) -> Self {
        Self { location, object }
    }
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Renderable {}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct Tree {
    pub location: Location,
    pub radius: u16,
}

impl Tree {
    pub fn new(location: Location, radius: u16) -> Self {
        Self { location, radius }
    }
}
