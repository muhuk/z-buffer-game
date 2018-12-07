use std::collections::BTreeSet;
use std::collections::VecDeque;
use tcod::input::{self as tcod_input, Event as TcodEvent, KeyCode};

const DEFAULT_EVENT_QUEUE_SIZE: usize = 20;

#[derive(Debug, Clone)]
pub struct Modifiers {
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
}

#[derive(Debug, Clone)]
pub enum Event {
    KeyPress(KeyCode, Option<char>, Modifiers),
    KeyDown(KeyCode, Option<char>, Modifiers),
    KeyUp(KeyCode, Option<char>, Modifiers),
    Mouse,
}

pub struct Input {
    // TODO: Key type should be a KeyCode, but since it
    //       does not satisfy the constrains we will be
    //       casting to isize.
    key_states: BTreeSet<isize>,
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
                    let key_up_or_key_down: Event = Self::convert_key_event(&e);
                    let maybe_key_press = self.detect_keypress(&key_up_or_key_down);
                    // Emit up|down event before press event.
                    events.push_back(key_up_or_key_down);
                    maybe_key_press.map(|e| {
                        events.push_back(e);
                    });
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
        let code_isize: isize = *key_code as isize;
        match (e, self.key_states.contains(&code_isize)) {
            (Event::KeyDown(_, character, modifiers), true) => {
                Some(Event::KeyPress(*key_code, *character, (*modifiers).clone()))
            }
            (Event::KeyDown(..), false) => {
                self.key_states.insert(code_isize);
                None
            }
            (Event::KeyUp(_, character, modifiers), _) => {
                self.key_states.remove(&code_isize);
                Some(Event::KeyPress(*key_code, *character, (*modifiers).clone()))
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
