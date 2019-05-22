use crate::data::{Location, Rectangle};

const DEFAULT_MAP_WIDTH: i32 = 64;
const DEFAULT_MAP_HEIGHT: i32 = 64;

#[derive(Debug)]
pub struct Map {
    width: i32,
    height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn boundaries(&self) -> Rectangle {
        Rectangle::centered_around(Location::origin(), self.width, self.height)
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            width: DEFAULT_MAP_WIDTH,
            height: DEFAULT_MAP_HEIGHT,
        }
    }
}
