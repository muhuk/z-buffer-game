use crate::stage::game::Game;
use crate::ui::constants::MAP_MIN_SIZE;
use crate::ui::render::Render;
use std::cmp::max;
use std::fmt;
use tcod::console::{blit, BackgroundFlag, Console, Offscreen, TextAlignment};

pub struct GameRenderer {
    root: Offscreen,
    map: Offscreen,
}

impl GameRenderer {
    pub fn new(width: u32, height: u32) -> GameRenderer {
        let root = Offscreen::new(width as i32, height as i32);
        let (map_w, map_h) = Self::calculate_map_viewport((width, height));
        let map = Offscreen::new(map_w as i32, map_h as i32);
        GameRenderer { root, map }
    }

    fn blit(&mut self) {
        let w = self.map.width();
        let h = self.map.height();
        blit(&self.map, (0, 0), (w, h), &mut self.root, (0, 0), 1.0, 1.0);
    }

    pub fn calculate_map_viewport(requested_size: (u32, u32)) -> (u32, u32) {
        let (req_w, req_h) = requested_size;
        let (min_w, min_h) = MAP_MIN_SIZE;
        (max(req_w, min_w), max(req_h, min_h))
    }
}

impl fmt::Debug for GameRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameRenderer")
    }
}

impl Render for GameRenderer {
    type SceneType = Game;

    fn borrow_root(&self) -> &Offscreen {
        &self.root
    }

    fn update(&mut self, _stage: &Game) {
        // TODO: Remove notes below after implementation.
        //
        // 1. Figure out the camera location.  It comes from the stage.
        // 2. Based on map size, figure out the viewport in world coordinates.
        // 3. Query the map in stage for tile types.
        // 4. Convert tiles to glyphs and render them on map.

        let mut map = &self.map;
        let w = map.width();
        let h = map.height();

        // Fill the map with soma glyph.
        for y in 0..h {
            for x in 0..w {
                map.put_char(x, y, '\u{f7}', BackgroundFlag::None);
            }
        }
        map.set_alignment(TextAlignment::Center);
        map.print_rect(
            map.width() / 2,
            map.height() / 2 + 2,
            map.width(),
            1,
            "Game Stage",
        );
        self.blit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_X: u32 = MAP_MIN_SIZE.0 * 2;
    const MAX_Y: u32 = MAP_MIN_SIZE.1 * 2;

    #[test]
    fn map_takes_at_least_the_minimums_defined_in_constants() {
        let (min_width, min_height) = MAP_MIN_SIZE;
        for b in 0..MAX_Y {
            for a in 0..MAX_X {
                let (w, h) = GameRenderer::calculate_map_viewport((a, b));
                assert!(
                    w >= min_width,
                    "Calculated witdth is {}, but the minimum is set as {}.",
                    w,
                    min_width
                );
                assert_eq!(max(a, min_width), w);
                assert!(
                    h >= min_height,
                    "Calculated height is {}, but the minimum is set as {}.",
                    h,
                    min_height
                );
                assert_eq!(max(b, min_height), h);
            }
        }
    }
}
