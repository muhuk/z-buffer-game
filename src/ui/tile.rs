use crate::data::VisibleObject;
use tcod::colors::{self, Color};
use tcod::console::{BackgroundFlag, Console};

pub const CURSOR: StaticTile = StaticTile {
    glyph: '\u{c5}', // Alternate '\u{ce}'
    foreground: colors::LIGHTER_CYAN,
    background: colors::SKY,
    background_flag: BackgroundFlag::Multiply,
};

pub trait Tile {
    fn put<T: Console>(self, console: &mut T, x: i32, y: i32, t: u64);
}

#[derive(Clone, Copy, Debug)]
pub struct StaticTile {
    glyph: char,
    foreground: Color,
    background: Color,
    background_flag: BackgroundFlag,
}

impl Tile for StaticTile {
    fn put<T: Console>(self, console: &mut T, x: i32, y: i32, _: u64) {
        console.set_char_foreground(x, y, self.foreground);
        console.set_char_background(
            x,
            y,
            self.background,
            self.background_flag,
        );
        console.set_char(x, y, self.glyph);
    }
}

pub fn from_visible_object(v: VisibleObject) -> impl Tile {
    match v {
        VisibleObject::Grass => StaticTile {
            glyph: '\u{af}',
            foreground: colors::DESATURATED_GREEN,
            background: colors::DARKEST_GREEN,
            background_flag: BackgroundFlag::Set,
        },
        VisibleObject::Soil => StaticTile {
            glyph: '\u{2e}',
            foreground: colors::DARK_SEPIA,
            background: colors::LIGHT_SEPIA,
            background_flag: BackgroundFlag::Set,
        },
    }
}
