use crate::input::{Event, EventIterator, KeyCode};
use crate::menu::Menu;
use crate::stage::game::Game;
use crate::stage::main_menu::MainMenu;
use std::process::exit;

pub mod game;
pub mod main_menu;

pub enum Stage {
    Game(Game),
    MainMenu(MainMenu),
}

impl Stage {
    pub fn new() -> Self {
        Stage::MainMenu(MainMenu::new())
    }

    pub fn tick(&mut self, dt_millis: u32, events: EventIterator) -> StageTransition {
        match self {
            Stage::MainMenu(menu) => Stage::tick_main_menu(menu, dt_millis, events),
            Stage::Game { .. } => StageTransition::Continue,
        }
    }

    fn tick_main_menu(
        menu: &mut MainMenu,
        _dt_millis: u32,
        events: EventIterator,
    ) -> StageTransition {
        // transition is mutable here because we are
        // processing multiple events and they may
        // require differen transitions. This can be
        // improved.
        let mut transition = StageTransition::Continue;
        let exit_code_ok: i32 = 0;
        for e in events {
            match e {
                Event::KeyPress(KeyCode::Up, ..) => menu.select_previous(),
                Event::KeyPress(KeyCode::Down, ..) => menu.select_next(),
                // TODO: Remove dbg! macro, use a proper logger.
                Event::KeyPress(KeyCode::Enter, ..) => match dbg!(menu.selected) {
                    main_menu::Choice::NewGame => {
                        transition = StageTransition::SwitchTo(Stage::Game(Game {}));
                    }
                    main_menu::Choice::Credits => unimplemented!(),
                    main_menu::Choice::Exit => {
                        println!("Bye!");
                        exit(exit_code_ok);
                    }
                },
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
