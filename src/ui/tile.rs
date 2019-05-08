use crate::data::VisibleObject;
use tcod::colors::{self, Color};
use tcod::console::{BackgroundFlag, Console};

pub const CURSOR: AnimatedTile = AnimatedTile {
    glyphs: &[
        (250, Some('\u{ce}')),
        (250, None),
        (250, Some('\u{ce}')),
        (250, None),
        (250, Some('\u{c5}')),
        (250, None),
        (250, Some('\u{c5}')),
        (250, None),
    ],
    foreground: colors::LIGHTER_CYAN,
    background: colors::SKY,
    background_flag: BackgroundFlag::Multiply,
};

pub trait Tile {
    fn put<T: Console>(self, console: &mut T, x: i32, y: i32, t: u64);
}

pub struct AnimatedTile {
    glyphs: &'static [(u64, Option<char>)],
    foreground: Color,
    background: Color,
    background_flag: BackgroundFlag,
}

impl Tile for AnimatedTile {
    fn put<T: Console>(self, console: &mut T, x: i32, y: i32, t: u64) {
        let sum: u64 = self.glyphs.iter().map(|(p, _)| p).sum();
        let mut k = t % sum;
        for (p, maybe_glyph) in self.glyphs {
            if k < *p {
                maybe_glyph.iter().for_each(|glyph| {
                    put(
                        console,
                        x,
                        y,
                        *glyph,
                        self.foreground,
                        self.background,
                        self.background_flag,
                    );
                });
                break;
            } else {
                k -= p
            }
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
