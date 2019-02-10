//! Game state

use crate::data::Direction;
use crate::input::{Event, EventIterator, KeyCode};
use crate::menu::Menu;
use crate::stage::game::Game;
use crate::stage::main_menu::MainMenu;
use log::{debug, info};
use std::process::exit;

pub mod game;
pub mod main_menu;

/// Stages represent game state in a self contained manner.
///
/// See `crate::game::Game::main_loop` for the usage of `Stage`.
#[derive(Debug)]
pub enum Stage {
    Game(Game),
    MainMenu(MainMenu),
}

impl Stage {
    pub fn new() -> Self {
        Stage::MainMenu(MainMenu::new())
    }

    /// Update the status within the stage and signal the container if a stage
    /// transition is necessary.
    pub fn tick(
        &mut self,
        dt_millis: u32,
        events: EventIterator,
    ) -> StageTransition {
        match self {
            Stage::MainMenu(menu) => {
                Stage::tick_main_menu(menu, dt_millis, events)
            }
            Stage::Game(game) => Stage::tick_game(game, dt_millis, events),
        }
    }

    fn tick_game(
        game: &mut Game,
        _dt_millis: u32,
        events: EventIterator,
    ) -> StageTransition {
        for e in events {
            match e {
                Event::KeyPress(KeyCode::Up, ..) => {
                    game.player_move(Direction::North)
                }
                Event::KeyPress(KeyCode::Right, ..) => {
                    game.player_move(Direction::East)
                }
                Event::KeyPress(KeyCode::Down, ..) => {
                    game.player_move(Direction::South)
                }
                Event::KeyPress(KeyCode::Left, ..) => {
                    game.player_move(Direction::West)
                }
                _ => (),
            }
        }
        StageTransition::Continue
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
        const EXIT_CODE_OK: i32 = 0;
        for e in events {
            match e {
                Event::KeyPress(KeyCode::Up, ..) => menu.select_previous(),
                Event::KeyPress(KeyCode::Down, ..) => menu.select_next(),
                Event::KeyPress(KeyCode::Enter, ..) => {
                    debug!("Menu item selected: {}", menu.selected);
                    match menu.selected {
                        main_menu::Choice::NewGame => {
                            info!("Starting new game.");
                            transition = StageTransition::SwitchTo(
                                Stage::Game(Game::new()),
                            );
                        }
                        main_menu::Choice::Credits => unimplemented!(),
                        main_menu::Choice::Exit => {
                            info!("Bye!");
                            exit(EXIT_CODE_OK);
                        }
                    }
                }
                _ => (),
            }
        }
        transition
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self::new()
    }
}

pub enum StageTransition {
    Continue,
    SwitchTo(Stage),
}
