use std::time::Duration;
use ui::{self, UI};

pub enum Stage {
    Menu,
}

impl Stage {
    pub fn tick(&self, dt: Duration, ui: &mut UI) -> StageTransition {
        match self {
            Stage::Menu => Stage::tick_menu(dt, ui),
        }
    }

    fn tick_menu(dt: Duration, ui: &mut UI) -> StageTransition {
        ui::draw_hello_world(&mut ui.root_console);
        ui.root_console.flush();
        ui.root_console.wait_for_keypress(true);
        StageTransition::Continue
    }
}

pub enum StageTransition {
    Continue,
    SwitchTo(Stage),
}
