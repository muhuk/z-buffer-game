//! Stage where the game runs on.
//!
//! [Game] is the entry point.

use crate::data::Time;
use crate::game::{
    Cursor, GameEvent, GameLog, InputSystem, LogEntry, MapSystem, MapTile,
    RenderingSystem, SceneData,
};
use crate::stage::StageData;
use specs::prelude::*;
use std::fmt::{Debug, Error, Formatter};
use std::sync::mpsc::{self, Sender};

pub struct Game {
    dispatcher: Dispatcher<'static, 'static>,
    event_sink: Sender<GameEvent>,
    world: World,
}

impl Game {
    pub fn new() -> Game {
        let (event_sink, event_source) = mpsc::channel::<GameEvent>();

        let mut world = World::new();
        world.add_resource(Cursor::default());
        world.add_resource(GameLog::default());
        world.add_resource(SceneData::default());
        world.add_resource(Time::default());
        world.register::<MapTile>();
        let mut dispatcher = DispatcherBuilder::new()
            .with(MapSystem::new(), "map_system", &[])
            .with(InputSystem::new(event_source), "input_system", &[])
            .with_thread_local(RenderingSystem::new())
            .build();
        dispatcher.setup(&mut world.res);
        world
            .write_resource::<GameLog>()
            .push(LogEntry::new("Game world initialized"));

        Game {
            dispatcher,
            event_sink,
            world,
        }
    }

    pub fn publish_event(&self, event: GameEvent) {
        // TODO: Handle send result
        self.event_sink.send(event).unwrap();
    }

    pub fn with_scene_data<F>(&self, f: F)
    where
        F: FnOnce(&SceneData),
    {
        f(&self.world.read_resource())
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
