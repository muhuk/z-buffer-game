//! Game state

use crate::data::Direction;
use crate::input::{Event, EventIterator, KeyCode};
use crate::stage::game::Game;
use crate::stage::main_menu::MainMenu;
use log::info;

pub mod game;
pub mod main_menu;

/// Stages represent game state in a self contained manner.
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

    pub fn is_running(&self) -> bool {
        match self {
            Stage::MainMenu(menu) => !menu.should_exit,
            Stage::Game(_game) => true,
        }
    }

    fn tick_game(
        game: &mut Game,
        dt_millis: u32,
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
        game.update_world(dt_millis);
        StageTransition::Continue
    }

    fn tick_main_menu(
        menu: &mut MainMenu,
        _dt_millis: u32,
        events: EventIterator,
    ) -> StageTransition {
        match menu.handle_events(events) {
            Some(main_menu::Choice::NewGame) => {
                info!("Starting new game.");
                StageTransition::SwitchTo(Stage::Game(Game::new()))
            }
            Some(main_menu::Choice::Credits) => unimplemented!(),
            Some(main_menu::Choice::Exit) => {
                menu.should_exit = true;
                StageTransition::Continue
            }
            None => StageTransition::Continue,
        }
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
