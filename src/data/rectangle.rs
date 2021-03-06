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

use crate::data::Location;
use std::cmp::{max, min};
use std::convert::TryFrom;
use std::iter::{IntoIterator, Iterator};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rectangle {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl Rectangle {
    pub fn centered_around(
        center: Location,
        width: u16,
        height: u16,
    ) -> Rectangle {
        assert!(width != 0 && height != 0);
        let half_width = i32::from(width / 2);
        let width_correction = 1 - i32::from(width % 2);
        let half_height = i32::from(height / 2);
        let height_correction = 1 - i32::from(height % 2);
        Rectangle::new(
            center.move_by(
                -half_width + width_correction,
                -half_height + height_correction,
            ),
            center.move_by(half_width, half_height),
        )
    }

    pub fn new(a: Location, b: Location) -> Rectangle {
        Rectangle {
            min_x: min(a.x, b.x),
            min_y: min(a.y, b.y),
            max_x: max(a.x, b.x),
            max_y: max(a.y, b.y),
        }
    }

    pub fn contains(self, location: Location) -> bool {
        (self.min_x <= location.x && location.x <= self.max_x)
            && (self.min_y <= location.y && location.y <= self.max_y)
    }

    pub fn height(self) -> u16 {
        u16::try_from(self.max_y - self.min_y + 1)
            .expect("Rectangle height does not fit into u16")
    }

    pub fn intersect(self, other: Rectangle) -> Option<Rectangle> {
        let min_x = max(self.min_x, other.min_x);
        let min_y = max(self.min_y, other.min_y);
        let max_x = min(self.max_x, other.max_x);
        let max_y = min(self.max_y, other.max_y);
        if min_x <= max_x && min_y <= max_y {
            Some(Rectangle::new(
                Location::new(min_x, min_y),
                Location::new(max_x, max_y),
            ))
        } else {
            None
        }
    }

    pub fn width(self) -> u16 {
        u16::try_from(self.max_x - self.min_x + 1)
            .expect("Rectangle height does not fit into u16")
    }

    fn area(self) -> u32 {
        u32::from(self.width()) * u32::from(self.height())
    }

    fn center(self) -> Location {
        let width_correction = i32::from(self.width()) % 2 - 1;
        let height_correction = i32::from(self.height()) % 2 - 1;
        Location::new(
            self.min_x + i32::from(self.width()) / 2 + width_correction,
            self.min_y + i32::from(self.height()) / 2 + height_correction,
        )
    }
}

pub struct RectangleIter {
    rect: Rectangle,
    idx: u32,
}

impl Iterator for RectangleIter {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.rect.area() {
            None
        } else {
            let width = i32::from(self.rect.width());
            let idx = i32::try_from(self.idx)
                .expect("Rectangle area does not fit into i32");
            let x: i32 = self.rect.min_x + idx % width;
            let y: i32 = self.rect.min_y + idx / width;
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

    #[test]
    #[should_panic]
    fn centered_around_requires_non_zero_dimensions() {
        Rectangle::centered_around(Location::origin(), 0, 0);
    }

    #[test]
    fn centering_around_with_odd_dimensions_place_the_center_in_the_middle() {
        assert_eq!(
            Rectangle::new(Location::new(-1, -1), Location::new(1, 1)),
            Rectangle::centered_around(Location::origin(), 3, 3)
        );
        assert_eq!(
            Location::origin(),
            Rectangle::centered_around(Location::origin(), 11, 11).center()
        );
        assert_eq!(
            Location::origin(),
            Rectangle::centered_around(Location::origin(), 7, 4).center()
        );
        assert_eq!(
            Location::origin(),
            Rectangle::centered_around(Location::origin(), 4, 5).center()
        );
    }

    #[test]
    fn centering_around_with_even_dimensions_place_the_center_on_top_left_coordinate(
    ) {
        assert_eq!(
            Rectangle::new(Location::new(-1, -1), Location::new(2, 2)),
            Rectangle::centered_around(Location::origin(), 4, 4)
        );
        assert_eq!(
            Location::origin(),
            Rectangle::centered_around(Location::origin(), 4, 4).center()
        );
        assert_eq!(
            Rectangle::new(Location::new(-1, -2), Location::new(4, 3)),
            Rectangle::centered_around(Location::new(1, 0), 6, 6)
        );
        assert_eq!(
            Location::new(1, 0),
            Rectangle::centered_around(Location::new(1, 0), 6, 6).center()
        );

        assert_eq!(
            Rectangle::new(Location::new(-2, -1), Location::new(3, 4)),
            Rectangle::centered_around(Location::new(0, 1), 6, 6)
        );
        assert_eq!(
            Location::new(0, 1),
            Rectangle::centered_around(Location::new(0, 1), 6, 6).center()
        );
    }

    #[test]
    fn centering_around_produces_rectangle_with_given_dimensions() {
        let origin_odd_odd =
            Rectangle::centered_around(Location::origin(), 3, 5);
        assert_eq!(3, origin_odd_odd.width());
        assert_eq!(5, origin_odd_odd.height());

        let origin_even_even =
            Rectangle::centered_around(Location::origin(), 6, 8);
        assert_eq!(6, origin_even_even.width());
        assert_eq!(8, origin_even_even.height());

        let other_odd_odd =
            Rectangle::centered_around(Location::new(1, 1), 9, 7);
        assert_eq!(9, other_odd_odd.width());
        assert_eq!(7, other_odd_odd.height());

        let other_even_even =
            Rectangle::centered_around(Location::new(1, 1), 16, 2);
        assert_eq!(16, other_even_even.width());
        assert_eq!(2, other_even_even.height());
    }

    #[test]
    fn interrection_of_non_overlapping_rectangles_is_none() {
        let a = Rectangle::new(Location::new(-10, -10), Location::new(-5, -5));
        let b = Rectangle::new(Location::new(10, 10), Location::new(5, 5));
        assert_eq!(None, a.intersect(b));
    }

    #[test]
    fn intersection_of_a_rectangle_that_contains_another_is_the_smaller_one() {
        let a = Rectangle::new(Location::new(-10, -10), Location::new(10, 10));
        let b = Rectangle::new(Location::new(-5, -5), Location::new(5, 5));
        assert_eq!(Some(b), a.intersect(b));
    }

    #[test]
    fn intersection_of_overlapping_rectangles_is_the_overlapping_part() {
        let a = Rectangle::new(Location::new(2, 1), Location::new(8, 5));
        let b = Rectangle::new(Location::new(5, 3), Location::new(15, 13));
        let c = Rectangle::new(Location::new(5, 3), Location::new(8, 5));
        assert_eq!(Some(c), a.intersect(b));
    }

    #[test]
    fn contains_returns_true_if_the_location_is_inside_the_rectangle() {
        let rect = Rectangle::centered_around(Location::origin(), 5, 5);
        assert!(rect.contains(Location::origin()));
        assert!(rect.contains(Location::new(2, 2)));
        assert!(!rect.contains(Location::new(10, 10)));
        assert!(!rect.contains(Location::new(10, -10)));
        assert!(!rect.contains(Location::new(-10, 10)));
        assert!(!rect.contains(Location::new(-10, -10)));
    }
}
