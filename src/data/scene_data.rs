use crate::data::Location;
use std::cell::Cell;

#[derive(Debug, Default)]
pub struct SceneData {
    pub cursor_location: Cell<Location>,
}
