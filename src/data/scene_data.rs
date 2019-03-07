use crate::data::Location;
use crate::game::LogEntry;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

/// Data structure used to pass UI data from rendering system to the UI.
///
/// The game world is not supposed to know UI details.  Details like sizing
/// are passed into this struct as function parameters to allow doing
/// housekeeping.
#[derive(Debug, Default)]
pub struct SceneData {
    cursor_location: Cell<Location>,
    game_log: RefCell<VecDeque<LogEntry>>,
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

    /// Since [`SceneData`] has interior mutability, calling update does not
    /// require a mutable reference to the instance.
    pub fn update(
        &self,
        cursor_location: Location,
        new_entries: Vec<LogEntry>,
    ) {
        self.cursor_location.set(cursor_location);
        let mut game_log = self.game_log.borrow_mut();
        game_log.extend(new_entries);
    }
}
