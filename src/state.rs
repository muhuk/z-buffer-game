use stage::{Stage, StageTransition};
use std::time::Duration;
use ui::{self, UI};

/// Application state.
pub struct State {
    pub ui: UI,
    pub stage: Stage,
}

impl State {
    pub fn init() -> State {
        State {
            ui: ui::initialize(),
            stage: Stage::Menu,
        }
    }

    pub fn main_loop(&mut self) {
        while !self.ui.root_console.window_closed() {
            let dt = Duration::from_millis(10);
            match self.stage.tick(dt, &mut self.ui) {
                StageTransition::Continue => (),
                StageTransition::SwitchTo(_) => unimplemented!(),
            }
            self.ui.root_console.flush();
            self.ui.root_console.wait_for_keypress(true);
        }
    }
}
