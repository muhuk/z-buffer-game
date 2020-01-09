// © Copyright 2019-2020, Atamert Ölçgen
//
// This file is part of z-buffer-game.
//
// z-buffer-game is free software: you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// z-buffer-game is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with z-buffer-game.  If not, see <https://www.gnu.org/licenses/>.

//! Stage where the main menu is.
//!
//! [MainMenu] is the entry point.

use crate::input::{Event, KeyCode};
use crate::menu::Menu;
use crate::stage::StageData;
use std::fmt::{Display, Formatter, Result};
use std::slice::Iter;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Choice {
    NewGame,
    Credits,
    Exit,
}

impl Choice {
    pub const ALL: &'static [Choice] =
        &[Choice::NewGame, Choice::Credits, Choice::Exit];

    pub fn next(self) -> Option<Choice> {
        match self {
            Choice::NewGame => Some(Choice::Credits),
            Choice::Credits => Some(Choice::Exit),
            Choice::Exit => None,
        }
    }

    pub fn previous(self) -> Option<Choice> {
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

#[derive(Debug)]
pub struct MainMenu {
    pub selected: Choice,
    pub should_exit: bool,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            selected: Choice::NewGame,
            should_exit: false,
        }
    }

    pub fn handle_events<E>(&mut self, events: E) -> Option<Choice>
    where
        E: Iterator<Item = Event>,
    {
        let mut selected: Option<Choice> = None;
        for e in events {
            match e {
                Event::KeyPress(KeyCode::Up, ..) => self.select_previous(),
                Event::KeyPress(KeyCode::Down, ..) => self.select_next(),
                Event::KeyPress(KeyCode::Enter, ..) => {
                    selected = Some(self.selected)
                }
                _ => (),
            }
        }
        selected
    }
}

impl Default for MainMenu {
    fn default() -> Self {
        Self::new()
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
            i: Choice::ALL.iter(),
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
        self.selected
    }
}

impl StageData for MainMenu {}
