use crate::data::VisibleObject;
use tcod::colors::{self, Color};
use tcod::console::{BackgroundFlag, Console};

pub const CURSOR: Cursor = Cursor {};

pub trait Tile {
    fn put<T: Console>(self, console: &mut T, x: i32, y: i32, t: u64);
}

pub struct Cursor {}

impl Cursor {
    const GLYPH: char = '\u{c5}'; // Alternate '\u{ce}'
    const FOREGROUND: Color = colors::LIGHTER_CYAN;
    const BACKGROUND: Color = colors::SKY;
    const BACKGROUND_FLAG: BackgroundFlag = BackgroundFlag::Multiply;
}

impl Tile for Cursor {
    fn put<T: Console>(self, console: &mut T, x: i32, y: i32, t: u64) {
        if t % 500 < 250 {
            put(
                console,
                x,
                y,
                Self::GLYPH,
                Self::FOREGROUND,
                Self::BACKGROUND,
                Self::BACKGROUND_FLAG,
            );
        }
    }
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
        put(
            console,
            x,
            y,
            self.glyph,
            self.foreground,
            self.background,
            self.background_flag,
        );
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

#[inline]
fn put<T: Console>(
    console: &mut T,
    x: i32,
    y: i32,
    glyph: char,
    fg: Color,
    bg: Color,
    bg_flag: BackgroundFlag,
) {
    console.set_char_foreground(x, y, fg);
    console.set_char_background(x, y, bg, bg_flag);
    console.set_char(x, y, glyph);
}
