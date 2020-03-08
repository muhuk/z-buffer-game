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

use crate::data::{Pause, Time};
use specs::prelude::*;

const DEFAULT_TIME_SCALE: u32 = 100;

pub struct GameTimeSystem {
    time_scale: u32, // game_time / time_scale = real_time
}

impl GameTimeSystem {
    pub fn new() -> GameTimeSystem {
        GameTimeSystem {
            time_scale: DEFAULT_TIME_SCALE,
        }
    }
}

impl<'a> System<'a> for GameTimeSystem {
    type SystemData = (Read<'a, Pause>, Write<'a, Time>);

    fn run(&mut self, sys_data: Self::SystemData) {
        let (pause, mut time) = sys_data;
        if !pause.is_paused {
            let mut days = time.game_time_days();
            let mut hours = time.game_time_hours();
            let mut minutes = time.game_time_minutes();
            let mut millis: u32 =
                time.game_time_millis() + time.dt_millis() * self.time_scale;
            // 1 minute is 60000 milliseconds.
            if millis >= 60000 {
                millis -= 60000;
                minutes += 1;
            }
            if minutes >= 60 {
                minutes -= 60;
                hours += 1;
            }
            if hours >= 24 {
                hours -= 24;
                days += 1;
            }
            time.set_game_time(days, hours, minutes, millis);
        }
    }
}
