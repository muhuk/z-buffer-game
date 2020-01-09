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

use std::collections::BTreeSet;
use std::collections::VecDeque;
pub use tcod::input::KeyCode;
use tcod::input::{self as tcod_input, Event as TcodEvent};

const DEFAULT_EVENT_QUEUE_SIZE: usize = 20;

#[derive(Debug, Clone, Copy)]
pub struct Modifiers {
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    KeyPress(KeyCode, Option<char>, Modifiers),
    KeyDown(KeyCode, Option<char>, Modifiers),
    KeyUp(KeyCode, Option<char>, Modifiers),
    Mouse,
}

#[derive(Default)]
pub struct Input {
    // Key type should be a KeyCode, but since it does not
    // satisfy Ord constraint we will be casting to u32.
    //
    // In this scheme we are just storing the KeyCode::Char
    // but not the printable char. Practically this seems
    // to make no difference as when multiple printable chars
    // are pressed only the last one repeat.
    key_states: BTreeSet<u32>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            key_states: BTreeSet::new(),
        }
    }

    pub fn events(&mut self) -> VecDeque<Event> {
        let mut events = VecDeque::with_capacity(DEFAULT_EVENT_QUEUE_SIZE);
        for (_, e) in tcod_input::events() {
            match e {
                TcodEvent::Key(_) => {
                    let key_up_or_key_down: Event =
                        Self::convert_key_event(&e);
                    let maybe_key_press =
                        self.detect_keypress(&key_up_or_key_down);
                    // Emit up|down event before press event.
                    events.push_back(key_up_or_key_down);
                    if let Some(e2) = maybe_key_press {
                        events.push_back(e2);
                    }
                }
                TcodEvent::Mouse(_) => events.push_back(Event::Mouse),
            }
        }
        events
    }

    fn detect_keypress(&mut self, e: &Event) -> Option<Event> {
        let key_code: &KeyCode = match e {
            Event::KeyDown(key_code, ..) => key_code,
            Event::KeyUp(key_code, ..) => key_code,
            _ => panic!(),
        };
        let key_code_num: u32 = *key_code as u32;
        match (e, self.key_states.contains(&key_code_num)) {
            (Event::KeyDown(_, character, modifiers), true) => {
                Some(Event::KeyPress(*key_code, *character, *modifiers))
            }
            (Event::KeyDown(..), false) => {
                self.key_states.insert(key_code_num);
                None
            }
            (Event::KeyUp(_, character, modifiers), _) => {
                self.key_states.remove(&key_code_num);
                Some(Event::KeyPress(*key_code, *character, *modifiers))
            }
            _ => None,
        }
    }

    #[inline]
    fn convert_key_event(event: &TcodEvent) -> Event {
        match event {
            TcodEvent::Key(k) => {
                let modifiers = Modifiers {
                    shift: k.shift,
                    alt: k.alt,
                    ctrl: k.ctrl,
                };
                let character: Option<char> = Self::character_for(&event);
                if k.pressed {
                    Event::KeyDown(k.code, character, modifiers)
                } else {
                    Event::KeyUp(k.code, character, modifiers)
                }
            }
            _ => unreachable!(),
        }
    }

    #[inline]
    fn character_for(event: &TcodEvent) -> Option<char> {
        match event {
            TcodEvent::Key(k) => {
                if k.code == KeyCode::Char {
                    Some(k.printable)
                } else {
                    None
                }
            }
            _ => unreachable!(),
        }
    }
}
