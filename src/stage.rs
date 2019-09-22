//! Game state

use crate::data::Direction;
use crate::game::GameEvent;
use crate::input::{Event, KeyCode};
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
    pub fn tick<E>(&mut self, dt_millis: u32, events: E) -> StageTransition
    where
        E: Iterator<Item = Event>,
    {
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

    fn tick_game<E>(
        game: &mut Game,
        dt_millis: u32,
        events: E,
    ) -> StageTransition
    where
        E: Iterator<Item = Event>,
    {
        for e in events {
            match e {
                Event::KeyPress(KeyCode::Up, ..) => {
                    game.publish_event(GameEvent::Move(Direction::North))
                }
                Event::KeyPress(KeyCode::Right, ..) => {
                    game.publish_event(GameEvent::Move(Direction::East))
                }
                Event::KeyPress(KeyCode::Down, ..) => {
                    game.publish_event(GameEvent::Move(Direction::South))
                }
                Event::KeyPress(KeyCode::Left, ..) => {
                    game.publish_event(GameEvent::Move(Direction::West))
                }
                _ => (),
            }
        }
        game.update_world(dt_millis);
        StageTransition::Continue
    }

    fn tick_main_menu<E>(
        menu: &mut MainMenu,
        _dt_millis: u32,
        events: E,
    ) -> StageTransition
    where
        E: Iterator<Item = Event>,
    {
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

pub trait StageData {}
