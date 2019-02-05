use std::fmt;
use tcod::console::{self, Console, Offscreen};

pub struct GameRenderer {
    console: Offscreen,
}

impl GameRenderer {
    pub fn new(width: u32, height: u32) -> GameRenderer {
        let mut console = Offscreen::new(width as i32, height as i32);
        console.set_alignment(console::TextAlignment::Center);
        console.print_rect(
            console.width() / 2,
            console.height() / 2 + 2,
            console.width(),
            1,
            "Game Stage",
        );
        GameRenderer { console }
    }

    pub fn borrow_root(&self) -> &Offscreen {
        &self.console
    }
}

impl fmt::Debug for GameRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameRenderer")
    }
}
