use crate::data::Location;
use std::cmp::{max, min};
use std::iter::{IntoIterator, Iterator};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rectangle {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl Rectangle {
    pub fn new(a: Location, b: Location) -> Rectangle {
        Rectangle {
            min_x: min(a.x, b.x),
            min_y: min(a.y, b.y),
            max_x: max(a.x, b.x),
            max_y: max(a.y, b.y),
        }
    }

    fn area(self) -> i32 {
        self.width() * self.height()
    }

    fn height(self) -> i32 {
        self.max_y - self.min_y + 1
    }

    fn width(self) -> i32 {
        self.max_x - self.min_x + 1
    }
}

pub struct RectangleIter {
    rect: Rectangle,
    idx: i32,
}

impl Iterator for RectangleIter {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.rect.area() {
            None
        } else {
            let x = self.rect.min_x + self.idx % self.rect.width();
            let y = self.rect.min_y + self.idx / self.rect.width();
            self.idx += 1;
            Some(Location::new(x, y))
        }
    }
}

impl IntoIterator for Rectangle {
    type Item = Location;
    type IntoIter = RectangleIter;

    fn into_iter(self) -> Self::IntoIter {
        RectangleIter { rect: self, idx: 0 }
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

    #[test]
    fn area_is_width_times_height() {
        assert_eq!(
            25,
            Rectangle::new(Location::new(0, 0), Location::new(4, 4)).area()
        );
        assert_eq!(
            49,
            Rectangle::new(Location::new(3, -3), Location::new(-3, 3)).area()
        );
        assert_eq!(
            60002,
            Rectangle::new(Location::new(-10000, 0), Location::new(20000, 1))
                .area()
        );
    }

    #[test]
    fn iteration_generates_each_location_within_the_rectangle() {
        assert_eq!(
            vec![Location::new(10, 10),],
            Rectangle::new(Location::new(10, 10), Location::new(10, 10))
                .into_iter()
                .collect::<Vec<Location>>()
        );

        assert_eq!(
            vec![
                Location::new(1, 1),
                Location::new(2, 1),
                Location::new(3, 1),
                Location::new(1, 2),
                Location::new(2, 2),
                Location::new(3, 2)
            ],
            Rectangle::new(Location::new(1, 2), Location::new(3, 1))
                .into_iter()
                .collect::<Vec<Location>>()
        );
    }
}
