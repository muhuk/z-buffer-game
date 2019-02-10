//! Stage where the game runs on.
//!
//! [Game] is the entry point.

use crate::data::{Direction, Location};

#[derive(Debug)]
pub struct Game {
    location: Location,
}

impl Game {
    pub fn new() -> Game {
        Game {
            location: Location::new(0, 0),
        }
    }

    pub fn player_coordinates(&self) -> (i32, i32) {
        (self.location.x, self.location.y)
    }

    pub fn player_move(&mut self, direction: Direction) {
        self.location = self.location.move_towards(direction);
    }
}
