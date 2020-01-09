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

//! z-buffer game

use crate::input::{Event, Input, Modifiers};
use crate::stage::{Stage, StageTransition};
use crate::ui::UI;
use log::info;
use std::collections::VecDeque;
use std::time::Duration;
use stderrlog;
use tcod::input::KeyCode;
use tcod::system::get_elapsed_time;

mod asset;
mod conf;
mod data;
mod game;
mod input;
mod menu;
mod stage;
mod ui;

// 4 is log everything, 3 is DEBUG.
const LOG_VERBOSITY: usize = 4;

/// Application entry point.
fn main() {
    init_logger();
    info!("Starting z-buffer-game.");
    Application::new().main_loop();
    info!("Bye!");
}

/// Application data that brings together display, input & game state.
#[derive(Default)]
struct Application {
    stage: Stage,
    ui: UI,
    dt: u32,
    input: Input,
    time: u64,
}

impl Application {
    fn new() -> Application {
        Application {
            ui: UI::new(),
            stage: Stage::new(),
            dt: 0,
            time: 0,
            input: Input::new(),
        }
    }

    /// Filter application events and return remaining events.
    fn game_events(&mut self) -> VecDeque<Event> {
        let mut result = VecDeque::new();
        for e in self.input.events() {
            match e {
                Event::KeyPress(
                    KeyCode::Enter,
                    _,
                    Modifiers {
                        shift: false,
                        alt: true,
                        ctrl: false,
                    },
                ) => self.ui.toggle_fullscreen(),
                _ => result.push_back(e),
            }
        }
        result
    }

    /// Application main loop, blocks until UI terminates.
    fn main_loop(&mut self) {
        while self.ui.is_running() && self.stage.is_running() {
            self.update_time();
            let events: VecDeque<Event> = self.game_events();
            match self.stage.tick(self.dt, events.into_iter()) {
                StageTransition::Continue => (),
                StageTransition::SwitchTo(new_stage) => self.stage = new_stage,
            }
            self.ui.draw(&self.stage);
        }
    }

    fn update_time(&mut self) {
        let old_time = self.time;
        self.time = duration_to_millis(&get_elapsed_time());
        self.dt = (self.time - old_time) as u32;
    }
}

fn duration_to_millis(t: &Duration) -> u64 {
    t.as_secs() * 1000 + u64::from(t.subsec_millis())
}

fn init_logger() {
    stderrlog::new()
        .verbosity(LOG_VERBOSITY)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .module(module_path!())
        .init()
        .unwrap()
}
