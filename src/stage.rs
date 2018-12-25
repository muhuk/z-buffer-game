use crate::input::{Event, EventIterator, KeyCode};
use crate::menu::Menu;
use crate::stage::main_menu::MainMenu;

pub mod main_menu;

pub enum Stage {
    MainMenu(MainMenu),
}

impl Stage {
    pub fn new() -> Self {
        Stage::MainMenu(MainMenu::new())
    }

    pub fn tick(&mut self, dt_millis: u32, events: EventIterator) -> StageTransition {
        match self {
            Stage::MainMenu { .. } => self.tick_main_menu(dt_millis, events),
        }
    }

    fn tick_main_menu(&mut self, _dt_millis: u32, events: EventIterator) -> StageTransition {
        let Stage::MainMenu(m) = self;
        for e in events {
            match e {
                Event::KeyPress(KeyCode::Up, ..) => m.select_previous(),
                Event::KeyPress(KeyCode::Down, ..) => m.select_next(),
                _ => (),
            }
        }
        StageTransition::Continue
    }
}

pub enum StageTransition {
    Continue,
    SwitchTo(Stage),
}
