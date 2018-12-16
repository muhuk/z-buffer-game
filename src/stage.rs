use crate::input::{Event, EventIterator, KeyCode};
use crate::stage::main_menu::MainMenu;

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
                Event::KeyPress(KeyCode::Up, ..) => m.previous(),
                Event::KeyPress(KeyCode::Down, ..) => m.next(),
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

mod main_menu {
    use std::fmt::{Display, Formatter, Result};

    pub struct MainMenu {
        pub selected: Choice,
    }

    impl MainMenu {
        pub fn new() -> MainMenu {
            MainMenu {
                selected: Choice::NewGame,
            }
        }

        pub fn next(&mut self) {
            if let Some(choice) = self.selected.next() {
                self.selected = choice
            }
        }

        pub fn previous(&mut self) {
            if let Some(choice) = self.selected.previous() {
                self.selected = choice
            }
        }
    }

    #[derive(Debug)]
    pub enum Choice {
        NewGame,
        Credits,
        Exit,
    }

    impl Choice {
        fn next(&self) -> Option<Choice> {
            match self {
                Choice::NewGame => Some(Choice::Credits),
                Choice::Credits => Some(Choice::Exit),
                Choice::Exit => None,
            }
        }

        fn previous(&self) -> Option<Choice> {
            match self {
                Choice::NewGame => None,
                Choice::Credits => Some(Choice::NewGame),
                Choice::Exit => Some(Choice::Credits),
            }
        }
    }

    impl Display for Choice {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "{:?}", self)
        }
    }
}
