use crate::data::Direction;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
