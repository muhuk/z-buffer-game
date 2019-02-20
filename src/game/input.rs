use crate::game::{Cursor, GameEvent};
use log::debug;
use specs::prelude::*;
use std::sync::mpsc::Receiver;

pub struct InputSystem {
    event_source: Receiver<GameEvent>,
}

impl InputSystem {
    pub fn new(event_source: Receiver<GameEvent>) -> InputSystem {
        InputSystem { event_source }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = Write<'a, Cursor>;

    fn run(&mut self, mut cursor: Self::SystemData) {
        for e in self.event_source.try_iter() {
            debug!("Received game event {:?}", e);
            match e {
                GameEvent::Move(direction) => {
                    cursor.location = cursor.location.move_towards(direction)
                }
            }
        }
    }
}
