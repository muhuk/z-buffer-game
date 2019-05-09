use crate::data::{Location, Rectangle, Time, VisibleObject};
use crate::game::LogEntry;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

/// Data structure used to pass UI data from rendering system to the UI.
///
/// The game world is not supposed to know UI details.  Details like window
/// sizing are passed into this struct as function parameters to allow doing
/// housekeeping.
#[derive(Debug, Default)]
pub struct SceneData {
    cursor_location: Cell<Location>,
    game_log: RefCell<VecDeque<LogEntry>>,
    time: Cell<Time>,
}

impl SceneData {
    pub fn cursor_location(&self) -> Location {
        self.cursor_location.get()
    }

    pub fn for_each_game_log<F>(&self, n: usize, f: F)
    where
        F: FnMut((usize, &LogEntry)),
    {
        assert!(n > 0);
        {
            // We are not able to use VecDeque::truncate as it drops elements
            // from the back.
            let mut msgs = self.game_log.borrow_mut();
            while msgs.len() > n {
                assert!(msgs.pop_front().is_some());
            }
        }
        self.game_log.borrow().iter().enumerate().for_each(f);
    }

    pub fn for_each_map_tile<F>(&self, mut f: F, boundaries: Rectangle)
    where
        F: FnMut(Location, &[VisibleObject]),
    {
        let map_bounds =
            Rectangle::centered_around(Location::new(0, 0), 64, 64);

        match boundaries.intersect(map_bounds) {
            Some(bounds) => {
                for loc in bounds {
                    f(
                        loc,
                        if (loc.x * loc.y) % 3 == 0 {
                            &[VisibleObject::Soil]
                        } else {
                            &[VisibleObject::Grass]
                        },
                    );
                }
            }
            None => {}
        }
    }

    pub fn t_millis(&self) -> u64 {
        self.time.get().t_millis()
    }

    /// Since [`SceneData`] has interior mutability, calling update does not
    /// require a mutable reference to the instance.
    pub fn update(
        &self,
        cursor_location: Location,
        new_entries: Vec<LogEntry>,
        time: Time,
    ) {
        self.cursor_location.set(cursor_location);
        self.game_log.borrow_mut().extend(new_entries);
        self.time.set(time);
    }
}
