use crate::data::Location;
use std::cell::{Cell, RefCell};

/// Data structure used to pass UI data from rendering system to the UI.
///
/// The game world is not supposed to know UI details. Details like sizing are
/// passed into this struct as function parameters to allow doing housekeeping.
#[derive(Debug, Default)]
pub struct SceneData {
    // TODO: Make these private and allow access through getters setters.
    //       Getters to do housekeeping.
    //       Setters to do validation.
    pub cursor_location: Cell<Location>,
    pub messages: RefCell<Vec<String>>,
}
