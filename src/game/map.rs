use crate::data::{Location, Rectangle};

const MAP_WIDTH: i32 = 64;
const MAP_HEIGHT: i32 = 64;

pub fn map_boundaries() -> Rectangle {
    Rectangle::centered_around(Location::origin(), MAP_WIDTH, MAP_HEIGHT)
}
