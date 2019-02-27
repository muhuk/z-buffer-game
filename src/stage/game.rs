//! Stage where the game runs on.
//!
//! [Game] is the entry point.

use crate::data::{Direction, SceneData};
use crate::game::{
    Cursor, GameEvent, GameLog, InputSystem, LogEntry, RenderingSystem,
};
use crate::stage::StageData;
use specs::prelude::*;
use std::fmt::{Debug, Error, Formatter};
use std::rc::{Rc, Weak};
use std::sync::mpsc::{self, Sender};

pub struct Game {
    dispatcher: Dispatcher<'static, 'static>,
    event_sink: Sender<GameEvent>,
    scene_data: Rc<SceneData>,
    world: World,
}

impl Game {
    pub fn new() -> Game {
        let scene_data = Rc::new(SceneData::default());
        let (event_sink, event_source) = mpsc::channel::<GameEvent>();

        let mut world = World::new();
        world.add_resource(Cursor::default());
        world.add_resource(GameLog::default());
        // TODO: Register components
        let mut dispatcher = DispatcherBuilder::new()
            .with(InputSystem::new(event_source), "input_system", &[])
            .with_thread_local(RenderingSystem::new(scene_data.clone()))
            .build();
        dispatcher.setup(&mut world.res);
        world
            .write_resource::<GameLog>()
            .push(LogEntry::new(String::from("Game world initialized")));

        Game {
            dispatcher,
            event_sink,
            scene_data,
            world,
        }
    }

    // TODO: Consider inlining this function
    pub fn player_move(&self, direction: Direction) {
        // TODO: Handle send result
        self.event_sink.send(GameEvent::Move(direction)).unwrap();
    }

    pub fn scene_data(&self) -> Weak<SceneData> {
        Rc::downgrade(&self.scene_data)
    }

    pub fn update_world(&mut self, _dt_millis: u32) {
        self.dispatcher.dispatch(&self.world.res);
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Game")
    }
}

impl StageData for Game {}
