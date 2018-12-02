use event::Event;

pub enum Stage {
    Menu,
}

impl Stage {
    pub fn tick<T: IntoIterator<Item = Event>>(&self, dt_millis: u32, events: T) -> StageTransition {
        for e in events {
            println!("{:?}", e);
        }
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
