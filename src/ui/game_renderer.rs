use crate::conf;
use crate::ui::renderer::Renderer;
use tcod::console::{self, Console, Root};

pub struct GameRenderer {}

impl Renderer for GameRenderer {
    fn blit(&mut self, root: &mut Root) {
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
