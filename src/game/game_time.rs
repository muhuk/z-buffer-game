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
use log::debug;
use specs::prelude::*;

pub struct GameTimeSystem {}

impl GameTimeSystem {
    pub fn new() -> GameTimeSystem {
        GameTimeSystem {}
    }
}

impl<'a> System<'a> for GameTimeSystem {
    type SystemData = (Read<'a, Pause>, Write<'a, Time>);

    fn run(&mut self, mut sys_data: Self::SystemData) {
        let (pause, time) = sys_data;
        debug!(
            "Pause = {}, time is {}|{}:{}",
            pause.is_paused,
            time.game_time_days(),
            time.game_time_hours(),
            time.game_time_days()
        );
    }
}
