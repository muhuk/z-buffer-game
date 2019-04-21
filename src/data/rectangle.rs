use crate::data::Location;
use std::cmp::{max, min};

pub struct Rectangle {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl Rectangle {
    fn new(a: Location, b: Location) -> Rectangle {
        Rectangle {
            min_x: min(a.x, b.x),
            min_y: min(a.y, b.y),
            max_x: max(a.x, b.x),
            max_y: max(a.y, b.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_a_canonical_rectangle_with_min_and_max_coordinates() {
        let r1 = Rectangle::new(Location::new(-2, 3), Location::new(4, -3));
        assert_eq!(r1.min_x, -2);
        assert_eq!(r1.min_y, -3);
        assert_eq!(r1.max_x, 4);
        assert_eq!(r1.max_y, 3);

        let r2 = Rectangle::new(Location::new(-5, -7), Location::new(4, 8));
        assert_eq!(r2.min_x, -5);
        assert_eq!(r2.min_y, -7);
        assert_eq!(r2.max_x, 4);
        assert_eq!(r2.max_y, 8);
    }
}
