pub enum Stage {
    Menu,
}

impl Stage {
    pub fn tick(&self, dt_millis: u32) -> StageTransition {
        match self {
            Stage::Menu => Stage::tick_menu(dt_millis),
        }
    }

    fn tick_menu(dt_millis: u32) -> StageTransition {
        StageTransition::Continue
    }
}

pub enum StageTransition {
    Continue,
    SwitchTo(Stage),
}
