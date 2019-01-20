use super::renderer::Renderer;
use super::{SCREEN_HEIGHT_CHAR, SCREEN_WIDTH_CHAR};
use tcod::console::{self, Console, Root};

pub struct GameRenderer {}

impl Renderer for GameRenderer {
    fn blit(&mut self, root: &mut Root) {
        root.clear();
        root.set_alignment(console::TextAlignment::Center);
        root.print_rect(
            SCREEN_WIDTH_CHAR / 2,
            SCREEN_HEIGHT_CHAR / 2 + 2,
            SCREEN_WIDTH_CHAR,
            1,
            "Game Stage",
        );
        root.flush();
    }
}
