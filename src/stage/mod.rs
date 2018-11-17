use std::time::Duration;

pub enum Stage {
    Menu,
}

impl Stage {
    pub fn tick(&self, dt: Duration) -> StageTransition {
        match self {
            Stage::Menu => Stage::tick_menu(dt),
        }
    }

    fn tick_menu(dt: Duration) -> StageTransition {
        StageTransition::Continue
    }
}

pub enum StageTransition {
    Continue,
    SwitchTo(Stage),
}
