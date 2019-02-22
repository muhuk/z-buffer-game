use crate::data::Location;
use crate::stage::game::Game;
use crate::ui::constants::{
    BOTTOM_PANEL_HEIGHT, MAP_MIN_SIZE, SIDE_PANEL_WIDTH,
};
use crate::ui::render::Render;
use std::fmt;
use tcod::console::{blit, BackgroundFlag, Console, Offscreen, TextAlignment};

pub struct GameRenderer {
    bottom_panel: Offscreen,
    root: Offscreen,
    map: Offscreen,
}

impl GameRenderer {
    pub fn new(width: u32, height: u32) -> GameRenderer {
        let root = Offscreen::new(width as i32, height as i32);
        let (map_w, map_h) =
            Self::calculate_map_viewport_size((width, height)).unwrap();
        let map = Offscreen::new(map_w as i32, map_h as i32);
        let bottom_panel =
            Offscreen::new(width as i32, (height - map_h) as i32);
        GameRenderer {
            bottom_panel,
            root,
            map,
        }
    }

    pub(self) fn calculate_map_viewport_size(
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

    fn blit(&mut self) {
        let w = self.root.width();
        let h = self.root.height();
        let mw = self.map.width();
        let mh = self.map.height();
        blit(
            &self.map,
            (0, 0),
            (mw, mh),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );
        blit(
            &self.bottom_panel,
            (0, 0),
            (w, h - mh),
            &mut self.root,
            (0, mh),
            1.0,
            1.0,
        );
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

    fn update(&mut self, stage: &Game) {
        {
            let mut map = &self.map;
            let w = map.width();
            let h = map.height();

            // Fill the map with some glyph.
            for y in 0..h {
                for x in 0..w {
                    map.put_char(x, y, '\u{f7}', BackgroundFlag::None);
                }
            }

            let Location { x, y } =
                stage.scene_data().upgrade().unwrap().cursor_location.get();

            map.set_alignment(TextAlignment::Center);
            let (mid_x, mid_y) = (map.width() / 2, map.height() / 2);
            let s: String = format!(" Player location {}:{} ", x, y);
            let e: String = " ".repeat(s.len());
            map.print_rect(mid_x, mid_y - 1, w, 1, &e);
            map.print_rect(mid_x, mid_y, w, 1, s);
            map.print_rect(mid_x, mid_y + 1, w, 1, &e);
        }

        {
            let mut bottom_panel = &self.bottom_panel;
            let w = bottom_panel.width();
            let h = bottom_panel.height();

            // Fill the map with some glyph.
            for y in 0..h {
                for x in 0..w {
                    bottom_panel.put_char(
                        x,
                        y,
                        '\u{b0}',
                        BackgroundFlag::None,
                    );
                }
            }

            use crate::data::SceneData;
            use std::rc::Rc;
            let scene_data: Rc<SceneData> =
                stage.scene_data().upgrade().unwrap();

            for (idx, msg) in
                scene_data.messages.borrow().iter().enumerate().take(5)
            {
                bottom_panel.print_rect(0, idx as i32, w, 1, msg);
            }
        }

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
    fn map_dimensions_are_never_less_than_minimum_requirements() {
        let (map_min_width, map_min_height) = MAP_MIN_SIZE;
        let min_width = map_min_width + SIDE_PANEL_WIDTH;
        let min_height = map_min_height + BOTTOM_PANEL_HEIGHT;

        for b in 0..MAX_Y {
            for a in 0..MAX_X {
                match GameRenderer::calculate_map_viewport_size((a, b)) {
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
