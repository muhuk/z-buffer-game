use crate::data::VisibleObject;
use tcod::colors::{self, Color};
use tcod::console::{BackgroundFlag, Console};

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    glyph: char,
    foreground: Color,
    background: Color,
    background_flag: BackgroundFlag,
}

impl Tile {
    pub fn from_visible_object(v: VisibleObject) -> Tile {
        match v {
            VisibleObject::Grass => Tile {
                glyph: '\u{af}',
                foreground: colors::DESATURATED_GREEN,
                background: colors::DARKEST_GREEN,
                background_flag: BackgroundFlag::Set,
            },
            VisibleObject::Soil => Tile {
                glyph: '\u{2e}',
                foreground: colors::DARK_SEPIA,
                background: colors::LIGHT_SEPIA,
                background_flag: BackgroundFlag::Set,
            },
        }
    }

    pub fn put<T: Console>(self, console: &mut T, x: i32, y: i32) {
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
