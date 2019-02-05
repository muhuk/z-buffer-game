use crate::conf;
use tcod::console::{self, Console, Root};

pub struct GameRenderer {}

impl GameRenderer {
    pub fn new() -> GameRenderer {
        GameRenderer {}
    }

    pub fn blit(&mut self, root: &mut Root) {
        root.clear();
        root.set_alignment(console::TextAlignment::Center);
        root.print_rect(
            (conf::screen_width_char() / 2) as i32,
            (conf::screen_height_char() / 2 + 2) as i32,
            conf::screen_width_char() as i32,
            1,
            "Game Stage",
        );
        root.flush();
    }
}
