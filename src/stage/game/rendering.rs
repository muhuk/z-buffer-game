use crate::data::SceneData;
use crate::stage::game::cursor::Cursor;
use log::debug;
use specs::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

pub struct RenderingSystem {
    scene_data: Rc<SceneData>,
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = Read<'a, Cursor>;

    fn run(&mut self, sys_data: Self::SystemData) {
        let (cursor) = sys_data;
        self.scene_data.cursor_location.set(cursor.location);
        debug!("{:?}", &*self.scene_data);
    }
}

impl RenderingSystem {
    pub fn new(scene_data: Rc<SceneData>) -> RenderingSystem {
        RenderingSystem { scene_data }
    }
}
