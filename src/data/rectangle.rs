use crate::data::Location;
use std::cmp::{max, min};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rectangle {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl Rectangle {
    pub fn new(a: Location, b: Location) -> Result<Rectangle, RectangleError> {
        let r = Rectangle {
            min_x: min(a.x, b.x),
            min_y: min(a.y, b.y),
            max_x: max(a.x, b.x),
            max_y: max(a.y, b.y),
        };
        if r.width() == 0 {
            Err(RectangleError::ZeroWidth)
        } else if r.height() == 0 {
            Err(RectangleError::ZeroHeight)
        } else {
            Ok(r)
        }
    }

    fn area(self) -> i32 {
        self.width() * self.height()
    }

    fn height(self) -> i32 {
        self.max_y - self.min_y
    }

    fn width(self) -> i32 {
        self.max_x - self.min_x
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum RectangleError {
    ZeroHeight,
    ZeroWidth,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_a_canonical_rectangle_with_min_and_max_coordinates() {
        let r1 = Rectangle::new(Location::new(-2, 3), Location::new(4, -3))
            .unwrap();
        assert_eq!(r1.min_x, -2);
        assert_eq!(r1.min_y, -3);
        assert_eq!(r1.max_x, 4);
        assert_eq!(r1.max_y, 3);

        let r2 = Rectangle::new(Location::new(-5, -7), Location::new(4, 8))
            .unwrap();
        assert_eq!(r2.min_x, -5);
        assert_eq!(r2.min_y, -7);
        assert_eq!(r2.max_x, 4);
        assert_eq!(r2.max_y, 8);
    }

    #[test]
    fn width_cannot_be_zero() {
        assert_eq!(
            Err(RectangleError::ZeroWidth),
            Rectangle::new(Location::new(2, -10), Location::new(2, -4))
        );
    }

    #[test]
    fn height_cannot_be_zero() {
        assert_eq!(
            Err(RectangleError::ZeroHeight),
            Rectangle::new(Location::new(7, 5), Location::new(2, 5))
        );
    }

    #[test]
    fn area_is_width_times_height() {
        assert_eq!(
            Ok(25),
            Rectangle::new(Location::new(0, 0), Location::new(5, 5))
                .map(|r| r.area())
        );
        assert_eq!(
            Ok(36),
            Rectangle::new(Location::new(3, -3), Location::new(-3, 3))
                .map(|r| r.area())
        );
        assert_eq!(
            Ok(30000),
            Rectangle::new(Location::new(-10000, 0), Location::new(20000, 1))
                .map(|r| r.area())
        );
    }
}
