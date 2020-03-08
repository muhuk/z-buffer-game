// © Copyright 2019-2020, Atamert Ölçgen
//
// This file is part of z-buffer-game.
//
// z-buffer-game is free software: you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// z-buffer-game is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with z-buffer-game.  If not, see <https://www.gnu.org/licenses/>.

//! Stage where the game runs on.
//!
//! [Game] is the entry point.

use crate::data::{Pause, Time};
use crate::game::{
    components, Cursor, GameEvent, GameLog, GameTimeSystem, InputSystem,
    LogEntry, MapSystem, RenderingSystem, SceneData,
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
        // TODO: Register resources like components::register
        world.add_resource(Cursor::default());
        world.add_resource(GameLog::default());
        world.add_resource(SceneData::default());
        world.add_resource(Pause::default());
        world.add_resource(Time::default());
        components::register_with(&mut world);
        let mut dispatcher = DispatcherBuilder::new()
            .with(GameTimeSystem::new(), "game_time_system", &[])
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
        self.world.maintain();
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Game")
    }
}

impl StageData for Game {}
