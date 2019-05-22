use crate::data::{SceneData, Time};
use crate::game::{Cursor, GameLog, Map};
use specs::prelude::*;
use std::rc::Rc;

pub struct RenderingSystem {
    scene_data: Rc<SceneData>,
}

impl RenderingSystem {
    pub fn new(scene_data: Rc<SceneData>) -> RenderingSystem {
        RenderingSystem { scene_data }
    }
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        Write<'a, Cursor>,
        Write<'a, GameLog>,
        Read<'a, Map>,
        Read<'a, Time>,
    );

    fn run(&mut self, sys_data: Self::SystemData) {
        let (mut cursor, mut game_log, map, time) = sys_data;
        // TODO: Ideally we shouldn't need to do this at
        //       every frame.  Add a system to handle this.
        //       Then we can make Cursor read-only here again.
        cursor.set_boundaries(map.boundaries()).unwrap();
        self.scene_data.update(
            cursor.location(),
            game_log.take(),
            time.clone(),
        );
    }
}
