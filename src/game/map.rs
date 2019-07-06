use crate::data::{Location, Rectangle, VisibleObject};
use crate::game::{Cursor, MapTile, Renderable};
use log::debug;
use specs::prelude::*;

const MAP_WIDTH: i32 = 64;
const MAP_HEIGHT: i32 = 64;

#[derive(Debug, PartialEq)]
pub enum MapStatus {
    Unitialized,
    Initialized,
}

pub struct MapSystem {
    status: MapStatus,
}

impl MapSystem {
    pub fn new() -> Self {
        // Initialize the map.
        Self {
            status: MapStatus::Unitialized,
        }
    }
}

impl<'a> System<'a> for MapSystem {
    type SystemData = (
        Write<'a, Cursor>,
        Entities<'a>,
        WriteStorage<'a, MapTile>,
        WriteStorage<'a, Renderable>,
    );

    fn run(&mut self, sys_data: Self::SystemData) {
        let (mut cursor, entities, mut map_tiles, mut renderables) = sys_data;

        if self.status == MapStatus::Unitialized {
            debug!("Generating new map");
            let boundaries = Rectangle::centered_around(
                Location::origin(),
                MAP_WIDTH,
                MAP_HEIGHT,
            );
            for loc in boundaries.into_iter() {
                let obj = if loc.x % 8 == 0 && loc.y % 8 == 0 {
                    VisibleObject::Soil
                } else {
                    VisibleObject::Grass
                };
                let entity = entities.create();
                assert!(map_tiles
                    .insert(entity, MapTile::new(loc, obj))
                    .unwrap()
                    .is_none());
                assert!(renderables
                    .insert(entity, Renderable {})
                    .unwrap()
                    .is_none());
            }
            cursor.set_boundaries(boundaries).unwrap();
            self.status = MapStatus::Initialized
        }
    }
}
