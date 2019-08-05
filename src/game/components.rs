use crate::data::{Location as Loc, VisibleObject};
use specs::prelude::*;
use specs::storage::{DenseVecStorage, HashMapStorage};
use specs_derive::*;

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Tree {
    pub radius: u16,
}

impl Tree {
    pub fn new(radius: u16) -> Self {
        Self { radius }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Location {
    pub location: Loc,
}

impl Location {
    pub fn new(location: Loc) -> Self {
        Self { location }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Renderable {
    pub object: VisibleObject,
    pub z_index: u16,
}

impl Renderable {
    pub fn new(object: VisibleObject, z_index: u16) -> Self {
        Self { object, z_index }
    }
}

pub fn register_with(world: &mut World) {
    world.register::<Location>();
    world.register::<Renderable>();
    world.register::<Tree>();
}
