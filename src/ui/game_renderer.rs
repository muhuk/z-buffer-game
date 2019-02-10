use crate::stage::game::Game;
use crate::ui::constants::{
    BOTTOM_PANEL_HEIGHT, MAP_MIN_SIZE, SIDE_PANEL_WIDTH,
};
use crate::ui::render::Render;
use std::fmt;
use tcod::console::{blit, BackgroundFlag, Console, Offscreen, TextAlignment};

pub struct GameRenderer {
    root: Offscreen,
    map: Offscreen,
}

impl GameRenderer {
    pub fn new(width: u32, height: u32) -> GameRenderer {
        let root = Offscreen::new(width as i32, height as i32);
        let (map_w, map_h) =
            Self::calculate_map_viewport((width, height)).unwrap();
        let map = Offscreen::new(map_w as i32, map_h as i32);
        GameRenderer { root, map }
    }

    fn blit(&mut self) {
        let w = self.map.width();
        let h = self.map.height();
        blit(&self.map, (0, 0), (w, h), &mut self.root, (0, 0), 1.0, 1.0);
    }

    pub(self) fn calculate_map_viewport(
        requested_size: (u32, u32),
    ) -> Option<(u32, u32)> {
        let (req_w, req_h) = requested_size;
        let (map_min_w, map_min_h) = MAP_MIN_SIZE;
        if (req_w >= map_min_w + SIDE_PANEL_WIDTH)
            && (req_h >= map_min_h + BOTTOM_PANEL_HEIGHT)
        {
            Some((req_w - SIDE_PANEL_WIDTH, req_h - BOTTOM_PANEL_HEIGHT))
        } else {
            None
        }
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
            map.height() / 2,
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

    // 4K display & 8 pixel glyphs, times 2
    const MAX_X: u32 = 3840 / 8 * 2;
    const MAX_Y: u32 = 2160 / 8 * 2;

    #[test]
    fn viewport_calculation_is_never_less_than_minimum_requirements() {
        let (map_min_width, map_min_height) = MAP_MIN_SIZE;
        let min_width = map_min_width + SIDE_PANEL_WIDTH;
        let min_height = map_min_height + BOTTOM_PANEL_HEIGHT;

        for b in 0..MAX_Y {
            for a in 0..MAX_X {
                match GameRenderer::calculate_map_viewport((a, b)) {
                    Some((w, h)) => {
                        assert!(
                            w >= map_min_width,
                            "Map width {} is calculated as less than minimum required!",
                            w
                        );
                        assert_eq!(a, w + SIDE_PANEL_WIDTH);
                        assert!(
                            h >= map_min_height,
                            "Map height {} is calculated as less than minimum required!",
                            h
                        );
                        assert_eq!(b, h + BOTTOM_PANEL_HEIGHT);
                    }
                    None => assert!(a < min_width || b < min_height),
                }
            }
        }
    }
}
