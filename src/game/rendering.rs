use crate::data::SceneData;
use crate::game::Cursor;
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
    type SystemData = Read<'a, Cursor>;

    fn run(&mut self, sys_data: Self::SystemData) {
        let cursor = sys_data;
        self.scene_data.cursor_location.set(cursor.location);
    }
}
