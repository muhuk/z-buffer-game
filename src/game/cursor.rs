use crate::data::{Direction, Location};

#[derive(Debug, Default)]
pub struct Cursor {
    location: Location,
}

impl Cursor {
    pub fn location(&self) -> Location {
        self.location
    }

    // TODO: Avoid going outside of the map boundaries.
    pub fn move_towards(&mut self, direction: Direction) {
        self.location = self.location.move_towards(direction);
    }
}
