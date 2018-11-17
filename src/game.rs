use stage::{Stage, StageTransition};
use std::time::Duration;
use tcod::system::get_elapsed_time;
use ui::{self, UI};

/// Game state.
pub struct Game {
    pub ui: UI,
    pub stage: Stage,
    pub dt: u64,
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
        while !self.ui.root_console.window_closed() {
            self.update_time();
            match self.stage.tick(self.dt) {
                StageTransition::Continue => (),
                StageTransition::SwitchTo(_) => unimplemented!(),
            }
            ui::draw(self);
            self.ui.root_console.flush();
            self.ui.root_console.wait_for_keypress(true);
        }
    }

    fn update_time(&mut self) {
        let old_time = self.time;
        self.time = Self::to_millis(&get_elapsed_time());
        self.dt = self.time - old_time;
    }

    fn to_millis(t: &Duration) -> u64 {
        t.as_secs() * 1000 + t.subsec_millis() as u64
    }
}
