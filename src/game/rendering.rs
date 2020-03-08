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

use crate::data::Time;
use crate::game::{components, Cursor, GameLog, SceneData};
use shred_derive::*;
use specs::prelude::*;

pub struct RenderingSystem {}

impl RenderingSystem {
    pub fn new() -> RenderingSystem {
        RenderingSystem {}
    }
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = RenderingSystemData<'a>;

    fn run(&mut self, mut sys_data: Self::SystemData) {
        let mut scene_data = sys_data.scene_data;
        scene_data.clear_objects();
        for (loc, rend) in (&sys_data.locations, &sys_data.renderables).join()
        {
            scene_data.add_object_to_location(
                loc.location,
                rend.object,
                rend.z_index,
            );
        }
        scene_data.update(
            sys_data.cursor.location(),
            sys_data.game_log.take(),
            sys_data.time.clone(),
        );

        let time = sys_data.time;
        let game_time_str = format!(
            "Day {} - {:>2}:{:0>2}",
            time.game_time_days(),
            time.game_time_hours(),
            time.game_time_minutes()
        );
        scene_data.set_game_time_str(game_time_str);
    }
}

#[derive(SystemData)]
pub struct RenderingSystemData<'a> {
    cursor: Read<'a, Cursor>,
    game_log: Write<'a, GameLog>,
    scene_data: Write<'a, SceneData>,
    time: Read<'a, Time>,
    locations: ReadStorage<'a, components::Location>,
    renderables: ReadStorage<'a, components::Renderable>,
}
