use crate::data::{Direction, Location, Rectangle};

#[derive(Debug, Default)]
pub struct Cursor {
    boundaries: Option<Rectangle>,
    location: Location,
}

impl Cursor {
    pub fn new(location: Location, boundaries: Option<Rectangle>) -> Cursor {
        Cursor {
            location,
            boundaries,
        }
    }

    pub fn boundaries(&self) -> Option<Rectangle> {
        self.boundaries
    }

    pub fn clear_boundaries(&mut self) -> Option<Rectangle> {
        self.boundaries.take()
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn set_boundaries(
        &mut self,
        bounds: Rectangle,
    ) -> Result<Option<Rectangle>, CursorError> {
        if bounds.contains(self.location) {
            Ok(self.boundaries.replace(bounds))
        } else {
            Err(CursorError::LocationIsOutOfBounds)
        }
    }

    pub fn move_towards(&mut self, direction: Direction) {
        let new_location = self.location.move_towards(direction);
        if self
            .boundaries
            .map(|bounds| bounds.contains(new_location))
            .unwrap_or(true)
        {
            self.location = new_location
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CursorError {
    LocationIsOutOfBounds,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Direction, Location, Rectangle};

    #[test]
    fn cursor_is_unbounded_by_default() {
        assert_eq!(None, Cursor::default().boundaries());
    }

    #[test]
    fn set_boundaries_sets_the_boundaries_of_cursor() {
        let mut cursor = Cursor::default();
        let bounds = Rectangle::centered_around(Location::origin(), 20, 20);
        assert_eq!(Ok(None), cursor.set_boundaries(bounds));
        assert_eq!(Some(bounds), cursor.boundaries());
    }

    #[test]
    fn set_boundaries_requires_current_location_to_be_within_the_boundaries() {
        let mut cursor = Cursor::default();
        let bounds =
            Rectangle::new(Location::new(10, 10), Location::new(30, 30));
        assert_eq!(
            Err(CursorError::LocationIsOutOfBounds),
            cursor.set_boundaries(bounds)
        );
        assert_eq!(None, cursor.boundaries());
    }

    #[test]
    fn clear_boundaries_removes_and_returns_current_boundaries() {
        let mut cursor = Cursor::default();
        let bounds = Rectangle::centered_around(Location::origin(), 5, 5);
        assert!(cursor.set_boundaries(bounds).is_ok());
        assert_eq!(Some(bounds), cursor.clear_boundaries());
        assert_eq!(None, cursor.boundaries());
    }

    #[test]
    fn move_towards_limits_movements_if_boundaries_is_set() {
        let mut cursor = Cursor::default();
        assert!(cursor
            .set_boundaries(Rectangle::centered_around(
                Location::origin(),
                1,
                1,
            ))
            .is_ok());
        assert_eq!(Location::origin(), cursor.location);
        cursor.move_towards(Direction::West);
        assert_eq!(Location::origin(), cursor.location);
        cursor.move_towards(Direction::South);
        cursor.move_towards(Direction::East);
        assert_eq!(Location::origin(), cursor.location);
    }
}
