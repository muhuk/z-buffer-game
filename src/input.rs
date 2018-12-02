use std::collections::VecDeque;
use tcod::input::{self as tcod_input, Event as TcodEvent, KeyCode};

const DEFAULT_EVENT_QUEUE_SIZE: usize = 20;

#[derive(Debug)]
pub struct Modifiers {
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
}

#[derive(Debug)]
pub enum Event {
    // KeyPress(KeyCode, Modifiers),
    KeyDown(KeyCode, Modifiers),
    KeyUp(KeyCode, Modifiers),
    Mouse,
}

pub struct Input {}

impl Input {
    pub fn new() -> Input {
        Input {}
    }

    pub fn events(&self) -> VecDeque<Event> {
        let mut events = VecDeque::with_capacity(DEFAULT_EVENT_QUEUE_SIZE);
        for (_, e) in tcod_input::events() {
            match e {
                TcodEvent::Key(k) => {
                    let modifiers = Modifiers {
                        shift: k.shift,
                        alt: k.alt,
                        ctrl: k.ctrl,
                    };
                    let e = if k.pressed {
                        Event::KeyDown(k.code, modifiers)
                    } else {
                        Event::KeyUp(k.code, modifiers)
                    };
                    events.push_back(e)
                }
                TcodEvent::Mouse(_) => events.push_back(Event::Mouse),
            }
        }
        events
    }
}
