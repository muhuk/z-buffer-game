use crate::data::Location;
use std::cell::{Cell, RefCell};

/// Data structure used to pass UI data from rendering system to the UI.
///
/// The game world is not supposed to know UI details.  Details like sizing
/// are passed into this struct as function parameters to allow doing
/// housekeeping.
#[derive(Debug, Default)]
pub struct SceneData {
    // TODO: Make these private and allow access through getters setters.
    //       Getters to do housekeeping.
    //       Setters to do validation.
    cursor_location: Cell<Location>,
    pub messages: RefCell<Vec<String>>,
}

impl SceneData {
    pub fn cursor_location(&self) -> Location {
        self.cursor_location.get()
    }

    /// Since [`SceneData`] has interior mutability, calling update does not
    /// require a mutable reference to the instance.
    pub fn update(&self, cursor_location: Location) {
        self.cursor_location.set(cursor_location);
    }
}
