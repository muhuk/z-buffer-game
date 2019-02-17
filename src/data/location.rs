use super::direction::Direction;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

    pub fn move_towards(&self, direction: Direction) -> Location {
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
    fn move_towards_adds_direction_vector_to_current_location() {
        assert_eq!(
            Location::new(-1, 0),
            Location::origin().move_towards(Direction::North)
        );
        assert_eq!(
            Location::new(3, 1),
            Location::new(2, 1).move_towards(Direction::South)
        );
        assert_eq!(
            Location::new(5, 9),
            Location::new(5, 10).move_towards(Direction::West)
        );
        assert_eq!(
            Location::new(-2, 1),
            Location::new(-2, 0).move_towards(Direction::East)
        );
    }
}
