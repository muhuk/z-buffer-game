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

use crate::data::{Location, Rectangle, Time, VisibleObject};
use crate::game::LogEntry;
use std::collections::{BTreeMap, VecDeque};
use std::sync::Mutex;

/// Data structure used to pass UI data from rendering system to the UI.
///
/// The game world is not supposed to know UI details.  Details like window
/// sizing are passed into this struct as function parameters to allow doing
/// housekeeping.
#[derive(Debug, Default)]
pub struct SceneData {
    cursor_location: Location,
    game_log: Mutex<VecDeque<LogEntry>>,
    objects: BTreeMap<Location, Vec<(u16, VisibleObject)>>,
    time: Time,
}

impl SceneData {
    pub fn add_object_to_location(
        &mut self,
        location: Location,
        object: VisibleObject,
        z_index: u16,
    ) {
        if self.objects.get(&location).is_none() {
            self.set_objects_for_location(location, vec![]);
        }
        {
            let mut idx: usize = 0;
            for z in self
                .objects
                .get(&location)
                .unwrap()
                .iter()
                .map(|(z_index, _)| z_index)
            {
                if *z > z_index {
                    break;
                }
                idx += 1
            }
            self.objects
                .get_mut(&location)
                .map(|objects| objects.insert(idx, (z_index, object)));
        }
    }

    pub fn cursor_location(&self) -> Location {
        self.cursor_location
    }

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    pub fn for_each_game_log<F>(&self, n: usize, f: F)
    where
        F: FnMut((usize, &LogEntry)),
    {
        assert!(n > 0);
        let mut game_log = self.game_log.lock().unwrap();
        {
            // We are not able to use VecDeque::truncate as it drops elements
            // from the back.
            while game_log.len() > n {
                assert!(game_log.pop_front().is_some());
            }
        }
        game_log.iter().enumerate().for_each(f);
    }

    /// Call `f` once for each location in `boundaries`, pass in the
    /// coordinates and an array of [`VisibleObject`]'s.
    ///
    /// The order of visible objects in the array is ascending z-order.  The
    /// object at the bottom is the first element, the object on top of it is
    /// the second and so on.
    pub fn for_each_map_tile<F>(&self, mut f: F, boundaries: Rectangle)
    where
        F: FnMut(Location, &[VisibleObject]),
    {
        for loc in boundaries {
            f(loc, &self.get_objects_for_location(&loc));
        }
    }

    pub fn get_objects_for_location(
        &self,
        location: &Location,
    ) -> Vec<VisibleObject> {
        self.objects
            .get(location)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|(_, obj)| *obj)
            .collect()
    }

    pub fn t_millis(&self) -> u64 {
        self.time.t_millis()
    }

    /// Since [`SceneData`] has interior mutability, calling update does not
    /// require a mutable reference to the instance.
    pub fn update(
        &mut self,
        cursor_location: Location,
        new_entries: Vec<LogEntry>,
        time: Time,
    ) {
        self.cursor_location = cursor_location;
        let mut game_log = self.game_log.lock().unwrap();
        game_log.extend(new_entries);
        self.time = time;
    }

    fn set_objects_for_location(
        &mut self,
        location: Location,
        objects: Vec<(u16, VisibleObject)>,
    ) {
        self.objects.insert(location, objects);
    }
}
