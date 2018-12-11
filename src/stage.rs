use crate::input::EventIterator;

pub enum Stage {
    Menu,
}

impl Stage {
    pub fn tick(&self, dt_millis: u32, events: impl EventIterator) -> StageTransition {
        match self {
            Stage::Menu => Stage::tick_menu(dt_millis, events),
        }
    }

    fn tick_menu(_dt_millis: u32, events: impl EventIterator) -> StageTransition {
        for e in events {
            println!("{:?}", e);
        }
        StageTransition::Continue
    }
}

pub enum StageTransition {
    Continue,
    SwitchTo(Stage),
}
