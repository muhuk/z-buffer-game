use input::read_events;
use stage::{Stage, StageTransition};

use std::time::Duration;
use tcod::system::get_elapsed_time;
use ui::{self, UI};

/// Game state.
pub struct Game {
    pub ui: UI,
    pub stage: Stage,
    pub dt: u32,
    pub time: u64,
}

impl Game {
    pub fn init() -> Game {
        Game {
            ui: ui::initialize(),
            stage: Stage::Menu,
            dt: 0,
            time: 0,
        }
    }

    pub fn main_loop(&mut self) {
        self.ui.root_console.flush();
        while !self.ui.root_console.window_closed() {
            self.update_time();
            let events = read_events();
            match self.stage.tick(self.dt, events) {
                StageTransition::Continue => (),
                StageTransition::SwitchTo(_) => unimplemented!(),
            }
            ui::draw(self);
        }
    }

    fn to_millis(t: &Duration) -> u64 {
        t.as_secs() * 1000 + t.subsec_millis() as u64
    }

    fn update_time(&mut self) {
        let old_time = self.time;
        self.time = Self::to_millis(&get_elapsed_time());
        self.dt = (self.time - old_time) as u32;
    }
}
