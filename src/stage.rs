use crate::input::{Event, EventIterator, KeyCode};
use crate::menu::Menu;
use crate::stage::main_menu::MainMenu;
use std::process::exit;

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
        let transition = StageTransition::Continue;
        let exit_code_ok: i32 = 0;
        for e in events {
            match e {
                Event::KeyPress(KeyCode::Up, ..) => m.select_previous(),
                Event::KeyPress(KeyCode::Down, ..) => m.select_next(),
                Event::KeyPress(KeyCode::Enter, ..) => {
                    println!("Main menu - chosen {}", &m.selected);
                    if m.is_selected(&main_menu::Choice::Exit) {
                        println!("Bye!");
                        exit(exit_code_ok);
                    }
                }
                _ => (),
            }
        }
        transition
    }
}

pub enum StageTransition {
    Continue,
    SwitchTo(Stage),
}
