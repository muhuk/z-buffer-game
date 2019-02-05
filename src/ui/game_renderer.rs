use tcod::console::{self, Console, Offscreen, Root};

pub struct GameRenderer {
    console: Offscreen,
}

impl GameRenderer {
    pub fn new(width: u32, height: u32) -> GameRenderer {
        let console = Offscreen::new(width as i32, height as i32);
        GameRenderer { console }
    }

    pub fn blit(&mut self, root: &mut Root) {
        root.clear();
        root.set_alignment(console::TextAlignment::Center);
        root.print_rect(
            self.console.width() / 2,
            self.console.height() / 2 + 2,
            self.console.width(),
            1,
            "Game Stage",
        );
        root.flush();
    }
}
