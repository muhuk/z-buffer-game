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

use crate::data::{Location as Loc, VisibleObject};
use specs::prelude::*;
use specs::storage::{DenseVecStorage, HashMapStorage};
use specs_derive::*;

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Tree {
    pub size: u16,
}

impl Tree {
    pub fn new(size: u16) -> Self {
        Self { size }
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
