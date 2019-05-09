//! Stage where the game runs on.
//!
//! [Game] is the entry point.

use crate::data::{Location, SceneData, Time};
use crate::game::{
    map_boundaries, Cursor, GameEvent, GameLog, InputSystem, LogEntry,
    RenderingSystem,
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
        world.add_resource(Cursor::new(
            Location::origin(),
            Some(map_boundaries()),
        ));
        world.add_resource(GameLog::default());
        world.add_resource(Time::default());
        // TODO: Register components
        let mut dispatcher = DispatcherBuilder::new()
            .with(InputSystem::new(event_source), "input_system", &[])
            .with_thread_local(RenderingSystem::new(scene_data.clone()))
            .build();
        dispatcher.setup(&mut world.res);
        world
            .write_resource::<GameLog>()
            .push(LogEntry::new("Game world initialized"));

        Game {
            dispatcher,
            event_sink,
            scene_data,
            world,
        }
    }

    pub fn publish_event(&self, event: GameEvent) {
        // TODO: Handle send result
        self.event_sink.send(event).unwrap();
    }

    pub fn scene_data(&self) -> Weak<SceneData> {
        Rc::downgrade(&self.scene_data)
    }

    pub fn update_world(&mut self, dt_millis: u32) {
        self.world.write_resource::<Time>().advance_dt(dt_millis);
        self.dispatcher.dispatch(&self.world.res);
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Game")
    }
}

impl StageData for Game {}
