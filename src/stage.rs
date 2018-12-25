use crate::input::{Event, EventIterator, KeyCode};
use crate::menu::Menu;
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

pub mod main_menu {
    use crate::menu::Menu;
    use std::fmt::{Display, Formatter, Result};
    use std::slice::Iter;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Choice {
        NewGame,
        Credits,
        Exit,
    }

    // TODO: Try to remive these two; next & previous.
    impl Choice {
        pub fn next(&self) -> Option<Choice> {
            match self {
                Choice::NewGame => Some(Choice::Credits),
                Choice::Credits => Some(Choice::Exit),
                Choice::Exit => None,
            }
        }

        pub fn previous(&self) -> Option<Choice> {
            match self {
                Choice::NewGame => None,
                Choice::Credits => Some(Choice::NewGame),
                Choice::Exit => Some(Choice::Credits),
            }
        }
    }

    // TODO: Consider replacing this with
    //       something more TCOD aware.
    impl Display for Choice {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "{:?}", self)
        }
    }

    pub struct MainMenu {
        pub selected: Choice,
    }

    impl MainMenu {
        const ALL: &'static [Choice] = &[Choice::NewGame, Choice::Credits, Choice::Exit];

        pub fn new() -> MainMenu {
            MainMenu {
                selected: Choice::NewGame,
            }
        }
    }

    pub struct MainMenuIterator<'a> {
        i: Iter<'a, Choice>,
    }

    impl<'a> Iterator for MainMenuIterator<'a> {
        type Item = &'a Choice;

        fn next(&mut self) -> Option<&'a Choice> {
            self.i.next()
        }
    }

    impl<'a> Menu<'a> for MainMenu {
        type Item = Choice;
        type IterMenu = MainMenuIterator<'a>;

        fn iter(&self) -> Self::IterMenu {
            MainMenuIterator {
                i: Self::ALL.iter(),
            }
        }

        fn select_next(&mut self) {
            if let Some(choice) = self.selected.next() {
                self.selected = choice
            }
        }

        fn select_previous(&mut self) {
            if let Some(choice) = self.selected.previous() {
                self.selected = choice
            }
        }

        fn is_selected(&self, item: &Choice) -> bool {
            self.selected == *item
        }

        fn selected(&self) -> Choice {
            self.selected.clone()
        }
    }
}
