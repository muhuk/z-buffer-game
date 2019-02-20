//! z-buffer game

use crate::input::Input;
use crate::stage::{Stage, StageTransition};
use crate::ui::UI;
use log::info;
use std::time::Duration;
use stderrlog;
use tcod::system::get_elapsed_time;

pub mod asset;
pub mod conf;
pub mod data;
pub mod game;
pub mod input;
pub mod menu;
pub mod stage;
pub mod ui;

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

    /// Application main loop, blocks until UI terminates.
    fn main_loop(&mut self) {
        while self.ui.is_running() && self.stage.is_running() {
            self.update_time();
            let events = self.input.events();
            match self.stage.tick(self.dt, events) {
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
