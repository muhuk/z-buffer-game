use self::generator::generate_map;
use crate::data::{Location, Rectangle};
use crate::game::{Cursor, MapTile, Renderable};
use log::debug;
use specs::prelude::*;

mod generator;

const MAP_WIDTH: u16 = 64;
const MAP_HEIGHT: u16 = 64;

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
            generate_map(boundaries, |loc, obj| {
                let entity = entities.create();
                assert!(map_tiles
                    .insert(entity, MapTile::new(loc, obj))
                    .unwrap()
                    .is_none());
                assert!(renderables
                    .insert(entity, Renderable {})
                    .unwrap()
                    .is_none());
            });
            cursor.set_boundaries(boundaries).unwrap();
            self.status = MapStatus::Initialized
        }
    }
}
