use crate::data::SceneData;
use crate::game::Cursor;
use specs::prelude::*;
use std::rc::Rc;

pub struct RenderingSystem {
    scene_data: Rc<SceneData>,
    // TODO: Remove i
    i: u32,
}

impl RenderingSystem {
    pub fn new(scene_data: Rc<SceneData>) -> RenderingSystem {
        RenderingSystem { scene_data, i: 0 }
    }
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = Read<'a, Cursor>;

    fn run(&mut self, sys_data: Self::SystemData) {
        let cursor = sys_data;
        self.scene_data.update(cursor.location);
        // TODO: Remove the message below.
        self.scene_data.add_message(format!("message #{}", self.i));
        self.i += 1;
    }
}
