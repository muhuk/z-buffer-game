use crate::input::Input;
use crate::stage::{Stage, StageTransition};

use std::time::Duration;
use tcod::system::get_elapsed_time;
use crate::ui::{self, UI};

/// Game state.
pub struct Game {
    pub stage: Stage,
    pub ui: UI,
    dt: u32,
    input: Input,
    time: u64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            ui: ui::initialize(),
            stage: Stage::Menu,
            dt: 0,
            time: 0,
            input: Input::new(),
        }
    }

    pub fn dt(&self) -> u32 {
        self.dt
    }

    pub fn main_loop(&mut self) {
        self.ui.root_console.flush();
        while !self.ui.root_console.window_closed() {
            self.update_time();
            let events = self.input.events();
            match self.stage.tick(self.dt, events) {
                StageTransition::Continue => (),
                StageTransition::SwitchTo(_) => unimplemented!(),
            }
            ui::draw(self);
        }
    }

    pub fn time(&self) -> u64 {
        self.time
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
