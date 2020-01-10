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

use crate::data::VisibleObject;
use tcod::colors::{self, Color};
use tcod::console::{BackgroundFlag, Console};

pub const CURSOR: AnimatedTile = AnimatedTile {
    frames: &[
        (250, Some(CURSOR_1)),
        (250, None),
        (250, Some(CURSOR_1)),
        (250, None),
        (250, Some(CURSOR_2)),
        (250, None),
        (250, Some(CURSOR_2)),
        (250, None),
    ],
};
const CURSOR_1: StaticTile = StaticTile {
    glyph: '\u{c5}',
    foreground: colors::LIGHTER_CYAN,
    background: colors::SKY,
    background_flag: BackgroundFlag::Multiply,
};
const CURSOR_2: StaticTile = StaticTile {
    glyph: '\u{ce}',
    foreground: colors::LIGHTER_CYAN,
    background: colors::SKY,
    background_flag: BackgroundFlag::Multiply,
};

pub trait Tile {
    fn put<T: Console>(self, console: &mut T, x: i32, y: i32, t: u64);
}

pub struct AnimatedTile {
    frames: &'static [(u64, Option<StaticTile>)],
}

impl Tile for AnimatedTile {
    fn put<T: Console>(self, console: &mut T, x: i32, y: i32, t: u64) {
        let sum: u64 = self.frames.iter().map(|(p, _)| p).sum();
        let mut k = t % sum;
        for (p, maybe_tile) in self.frames {
            if k < *p {
                maybe_tile.iter().for_each(|tile| {
                    tile.put(console, x, y, t);
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
        VisibleObject::Rock => StaticTile {
            glyph: '\u{2e}',
            foreground: colors::BRASS,
            background: colors::BLACK,
            background_flag: BackgroundFlag::Set,
        },
        VisibleObject::Soil => StaticTile {
            glyph: '\u{2e}',
            foreground: colors::DARK_SEPIA,
            background: colors::LIGHT_SEPIA,
            background_flag: BackgroundFlag::Set,
        },
        VisibleObject::TreeTrunk => StaticTile {
            glyph: '\u{2e}',
            foreground: colors::LIGHT_SEPIA,
            background: colors::DARKER_BLUE,
            background_flag: BackgroundFlag::Set,
        },
        VisibleObject::TreeFoilage => StaticTile {
            glyph: '\u{2f}',
            foreground: colors::DARK_SEPIA,
            background: colors::DARKER_BLUE,
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
