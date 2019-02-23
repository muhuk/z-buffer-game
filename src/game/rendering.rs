use crate::data::SceneData;
use crate::game::{Cursor, Messages};
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
    type SystemData = (Read<'a, Cursor>, Write<'a, Messages>);

    fn run(&mut self, sys_data: Self::SystemData) {
        let (cursor, mut messages) = sys_data;
        self.scene_data.update(cursor.location);
        messages.take().iter().for_each(|msg| {
            self.scene_data.add_message(format!("{:?}", msg));
        });
    }
}
