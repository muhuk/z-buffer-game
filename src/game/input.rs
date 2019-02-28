use crate::game::{Cursor, GameEvent, GameLog, LogEntry};
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
    type SystemData = (Write<'a, Cursor>, Read<'a, GameLog>);

    fn run(&mut self, system_data: Self::SystemData) {
        let (mut cursor, game_log) = system_data;
        for e in self.event_source.try_iter() {
            debug!("Received game event {:?}", e);
            match e {
                GameEvent::Move(direction) => {
                    cursor.move_towards(direction);
                    game_log.push(LogEntry::new(
                        format!(
                            "moved {:?}, new location is {:?}",
                            direction,
                            cursor.location()
                        )
                        .as_str(),
                    ))
                }
            }
        }
    }
}
