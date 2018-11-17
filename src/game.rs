use stage::{Stage, StageTransition};
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
            self.dt = 10;
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
