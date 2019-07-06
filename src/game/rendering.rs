use crate::data::Time;
use crate::game::{Cursor, GameLog, MapTile, Renderable, SceneData};
use shred_derive::*;
use specs::prelude::*;

pub struct RenderingSystem {}

impl RenderingSystem {
    pub fn new() -> RenderingSystem {
        RenderingSystem {}
    }
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = RenderingSystemData<'a>;

    fn run(&mut self, mut sys_data: Self::SystemData) {
        let mut scene_data = sys_data.scene_data;
        // TODO: Add other renderable object to scene data.
        for (tile, _) in (&sys_data.map_tiles, &sys_data.renderables).join() {
            scene_data
                .set_objects_for_location(tile.location, vec![tile.object]);
        }
        scene_data.update(
            sys_data.cursor.location(),
            sys_data.game_log.take(),
            sys_data.time.clone(),
        );
    }
}

#[derive(SystemData)]
pub struct RenderingSystemData<'a> {
    cursor: Read<'a, Cursor>,
    game_log: Write<'a, GameLog>,
    scene_data: Write<'a, SceneData>,
    time: Read<'a, Time>,
    map_tiles: ReadStorage<'a, MapTile>,
    renderables: ReadStorage<'a, Renderable>,
}
