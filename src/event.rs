use std::collections::VecDeque;
use tcod::input::{self as tcod_input, Event as TcodEvent};

pub type Event = TcodEvent;

const DEFAULT_EVENT_QUEUE_SIZE: usize = 20;

pub fn read_events() -> VecDeque<Event> {
    let mut events = VecDeque::with_capacity(DEFAULT_EVENT_QUEUE_SIZE);
    for (_, e) in tcod_input::events() {
        events.push_back(e);
    }
    events
}
