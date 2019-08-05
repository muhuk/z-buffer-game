use crate::data::Time;
use crate::game::{components, Cursor, GameLog, SceneData};
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
        for (loc, rend) in (&sys_data.locations, &sys_data.renderables).join()
        {
            scene_data
                .set_objects_for_location(loc.location, vec![rend.object]);
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
    locations: ReadStorage<'a, components::Location>,
    renderables: ReadStorage<'a, components::Renderable>,
}
