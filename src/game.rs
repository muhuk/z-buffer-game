use stage::{Stage, StageTransition};
use std::time::Duration;
use ui::{self, UI};

/// Game state.
pub struct Game {
    pub ui: UI,
    pub stage: Stage,
    pub dt: Duration,
    pub time: Duration,
}

impl Game {
    pub fn init() -> Game {
        Game {
            ui: ui::initialize(),
            stage: Stage::Menu,
            dt: Duration::from_millis(0),
            time: Duration::from_millis(0),
        }
    }

    pub fn main_loop(&mut self) {
        while !self.ui.root_console.window_closed() {
            self.dt = Duration::from_millis(10);
            self.time += self.dt;
            match self.stage.tick(self.dt) {
                StageTransition::Continue => (),
                StageTransition::SwitchTo(_) => unimplemented!(),
            }
            ui::draw(self);
            self.ui.root_console.flush();
            self.ui.root_console.wait_for_keypress(true);
        }
    }
}
