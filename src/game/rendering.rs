use crate::data::Time;
use crate::game::{Cursor, GameLog, MapTile, Renderable, SceneData};
use specs::prelude::*;

pub struct RenderingSystem {}

impl RenderingSystem {
    pub fn new() -> RenderingSystem {
        RenderingSystem {}
    }
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        Read<'a, Cursor>,
        Write<'a, GameLog>,
        Write<'a, SceneData>,
        Read<'a, Time>,
        // FIXME: Filter components by renderable
        ReadStorage<'a, MapTile>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, sys_data: Self::SystemData) {
        // FIXME: Read map data from entities.
        let (
            cursor,
            mut game_log,
            mut scene_data,
            time,
            map_tiles,
            renderables,
        ) = sys_data;
        for (tile, _) in (&map_tiles, &renderables).join() {
            scene_data
                .set_objects_for_location(tile.location, vec![tile.object]);
        }
        scene_data.update(cursor.location(), game_log.take(), time.clone());
    }
}
