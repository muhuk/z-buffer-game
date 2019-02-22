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
        self.scene_data.update(cursor.location);
        *self.scene_data.messages.borrow_mut() = vec![
            String::from("Message 1"),
            String::from("Message 2"),
            String::from("Message 3"),
            String::from("Message 4"),
            String::from("Message 5"),
            String::from("Message 6"),
            String::from("Message 7"),
            String::from("Message 8"),
        ];
    }
}
