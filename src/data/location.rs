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

use crate::data::Direction;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    pub fn new(x: i32, y: i32) -> Location {
        Location { x, y }
    }

    pub fn origin() -> Location {
        Location::new(0, 0)
    }

    pub fn move_by(self, dx: i32, dy: i32) -> Location {
        Location::new(self.x + dx, self.y + dy)
    }

    pub fn move_towards(self, direction: Direction) -> Location {
        let (dx, dy): (i32, i32) = direction.to_vector();
        Location {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_by_adds_differences_to_the_coordinates() {
        assert_eq!(Location::new(10, 0), Location::new(5, 0).move_by(5, 0));
        assert_eq!(Location::new(13, 23), Location::new(3, 3).move_by(10, 20));
        assert_eq!(Location::new(4, 5), Location::new(-4, -2).move_by(8, 7));
        assert_eq!(
            Location::new(35, 53),
            Location::new(40, 50).move_by(-5, 3)
        );
    }

    #[test]
    fn move_towards_adds_direction_vector_to_current_location() {
        assert_eq!(
            Location::new(0, -1),
            Location::origin().move_towards(Direction::North)
        );
        assert_eq!(
            Location::new(1, 3),
            Location::new(1, 2).move_towards(Direction::South)
        );
        assert_eq!(
            Location::new(9, 5),
            Location::new(10, 5).move_towards(Direction::West)
        );
        assert_eq!(
            Location::new(1, -2),
            Location::new(0, -2).move_towards(Direction::East)
        );
    }
}
