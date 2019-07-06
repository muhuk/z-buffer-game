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

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Renderable {}
