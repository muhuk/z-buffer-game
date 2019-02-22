use crate::data::Location;
use std::cell::{Cell, RefCell, RefMut};
use std::collections::VecDeque;

/// Data structure used to pass UI data from rendering system to the UI.
///
/// The game world is not supposed to know UI details.  Details like sizing
/// are passed into this struct as function parameters to allow doing
/// housekeeping.
#[derive(Debug, Default)]
pub struct SceneData {
    cursor_location: Cell<Location>,
    messages: RefCell<VecDeque<String>>,
}

impl SceneData {
    pub fn add_message(&self, message: String) {
        self.messages.borrow_mut().push_back(message);
    }

    pub fn cursor_location(&self) -> Location {
        self.cursor_location.get()
    }

    pub fn messages(&self, n: usize) -> impl IntoIterator<Item = String> {
        assert!(n > 0);
        {
            let mut msgs: RefMut<VecDeque<String>> =
                self.messages.borrow_mut();
            while msgs.len() > n {
                assert!(msgs.pop_front().is_some());
            }
        }
        // TODO: Try to void cloning both the Vec and the elements here.
        self.messages.borrow().iter().cloned().collect::<Vec<_>>()
    }

    /// Since [`SceneData`] has interior mutability, calling update does not
    /// require a mutable reference to the instance.
    pub fn update(&self, cursor_location: Location) {
        self.cursor_location.set(cursor_location);
    }
}
