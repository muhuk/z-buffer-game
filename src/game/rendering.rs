use crate::data::{SceneData, Time};
use crate::game::{Cursor, GameLog};
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
    type SystemData = (Read<'a, Cursor>, Write<'a, GameLog>, Read<'a, Time>);

    fn run(&mut self, sys_data: Self::SystemData) {
        let (cursor, mut game_log, time) = sys_data;
        self.scene_data.update(
            cursor.location(),
            game_log.take(),
            time.clone(),
        );
    }
}
