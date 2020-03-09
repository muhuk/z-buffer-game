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
                    ));
                }
                GameEvent::Spacebar => {
                    game_log.push(LogEntry::new("Spacebar pressed"));
                }
            }
        }
    }
}
