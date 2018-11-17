use stage::{Stage, StageTransition};
use std::time::Duration;
use ui::{self, UI};

/// Game state.
pub struct Game {
    pub ui: UI,
    pub stage: Stage,
}

impl Game {
    pub fn init() -> Game {
        Game {
            ui: ui::initialize(),
            stage: Stage::Menu,
        }
    }

    pub fn main_loop(&mut self) {
        while !self.ui.root_console.window_closed() {
            let dt = Duration::from_millis(10);
            match self.stage.tick(dt) {
                StageTransition::Continue => (),
                StageTransition::SwitchTo(_) => unimplemented!(),
            }
            ui::draw(&self.stage, &mut self.ui.root_console);
            self.ui.root_console.flush();
            self.ui.root_console.wait_for_keypress(true);
        }
    }
}
