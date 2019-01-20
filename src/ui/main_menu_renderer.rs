use super::renderer::Renderer;
use crate::menu::Menu;
use crate::stage::main_menu::{Choice, MainMenu};
use std::ops::{Deref, DerefMut};
use tcod::colors::{self, Color};
use tcod::console::{blit, BackgroundFlag, Console, Offscreen, Root};

pub struct MainMenuRenderer {
    console: Offscreen,
}

impl MainMenuRenderer {
    pub fn new() -> MainMenuRenderer {
        let (width, height) = Self::calculate_size();
        let mut console = Offscreen::new(width, height);
        for (idx, choice) in Choice::ALL.iter().enumerate() {
            console.print(0, idx as i32, format!("{}", choice));
        }
        MainMenuRenderer { console }
    }

    pub fn update(&mut self, menu: &MainMenu) {
        for (idx, choice) in menu.iter().enumerate() {
            let y: i32 = idx as i32;
            if menu.is_selected(choice) {
                self.paint_row(y, colors::WHITE, colors::RED);
            } else {
                self.paint_row(y, colors::WHITE, colors::BLACK);
            }
        }
    }

    fn calculate_size() -> (i32, i32) {
        let width = Choice::ALL
            .iter()
            .map(|c| format!("{}", c).len())
            .max()
            .unwrap();
        let height = Choice::ALL.len();
        (width as i32, height as i32)
    }

    fn paint_row(&mut self, y: i32, fg_color: Color, bg_color: Color) {
        let bg_flag: BackgroundFlag = BackgroundFlag::Set;
        let width: i32 = self.console.width();
        for x in 0..width {
            self.console.set_char_foreground(x, y, fg_color);
            self.console.set_char_background(x, y, bg_color, bg_flag);
        }
    }
}

impl Renderer for MainMenuRenderer {
    fn blit(&mut self, root: &mut Root) {
        let w: i32 = self.console.width();
        let h: i32 = self.console.height();
        let x: i32 = (root.width() - w) / 2;
        let y: i32 = (root.height() - h) / 2;
        blit(&**self, (0, 0), (w, h), root, (x, y), 1.0, 1.0);
    }
}

impl Deref for MainMenuRenderer {
    type Target = Offscreen;

    fn deref(&self) -> &Offscreen {
        &self.console
    }
}

impl DerefMut for MainMenuRenderer {
    fn deref_mut(&mut self) -> &mut Offscreen {
        &mut self.console
    }
}
