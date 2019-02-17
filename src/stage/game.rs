//! Stage where the game runs on.
//!
//! [Game] is the entry point.

use crate::data::{Direction, SceneData};
use crate::stage::game::{cursor::Cursor, rendering::RenderingSystem};
use log::debug;
use specs::prelude::*;
use std::fmt::{Debug, Error, Formatter};
use std::rc::{Rc, Weak};

mod cursor;
mod rendering;

pub struct Game {
    dispatcher: Dispatcher<'static, 'static>,
    scene_data: Rc<SceneData>,
    world: World,
}

impl Game {
    pub fn new() -> Game {
        let mut world = World::new();
        world.add_resource(Cursor::default());
        let scene_data = Rc::new(SceneData::default());
        //world.add_resource(scene_data.clone());
        // TODO: Register components
        let mut dispatcher = DispatcherBuilder::new()
            .with_thread_local(RenderingSystem::new(scene_data.clone()))
            .build();
        dispatcher.setup(&mut world.res);

        Game {
            dispatcher,
            scene_data,
            world,
        }
    }

    // TODO: Remove this.
    pub fn player_move(&mut self, direction: Direction) {
        debug!("Player move towards {:?}", direction);
    }

    pub fn scene_data(&self) -> Weak<SceneData> {
        Rc::downgrade(&self.scene_data)
    }

    pub fn update_world(&mut self, _dt_millis: u32) {
        self.dispatcher.dispatch(&mut self.world.res);
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Game")
    }
}
