use crate::menu::Menu;
use crate::stage::main_menu::{Choice, MainMenu};
use std::fmt;
use tcod::colors::{self, Color};
use tcod::console::{blit, BackgroundFlag, Console, Offscreen};

pub struct MainMenuRenderer {
    root: Offscreen,
    console: Offscreen,
}

impl MainMenuRenderer {
    pub fn new(window_width: u32, window_height: u32) -> MainMenuRenderer {
        let (width, height) = Self::calculate_size();
        let root = Offscreen::new(window_width as i32, window_height as i32);
        let console = Self::make_menu_console(width, height);
        MainMenuRenderer { root, console }
    }

    pub fn borrow_root(&self) -> &Offscreen {
        &self.root
    }

    pub fn update(&mut self, menu: &MainMenu) {
        // TODO: Refactor paint_row
        for (idx, choice) in menu.iter().enumerate() {
            let y: i32 = idx as i32;
            if menu.is_selected(choice) {
                self.paint_row(y, colors::WHITE, colors::RED);
            } else {
                self.paint_row(y, colors::WHITE, colors::BLACK);
            }
        }
        self.blit();
    }

    fn calculate_size() -> (u32, u32) {
        let width = Choice::ALL
            .iter()
            .map(|c| format!("{}", c).len())
            .max()
            .unwrap();
        let height = Choice::ALL.len();
        (width as u32, height as u32)
    }

    fn make_menu_console(width: u32, height: u32) -> Offscreen {
        let mut console = Offscreen::new(width as i32, height as i32);
        for (idx, choice) in Choice::ALL.iter().enumerate() {
            console.print(0, idx as i32, format!("{}", choice));
        }
        console
    }

    /// Blit menu items onto the root console for this renderer.
    fn blit(&mut self) {
        let w = self.console.width();
        let h = self.console.height();
        let sw = self.root.width();
        let sh = self.root.height();
        blit(
            &self.console,
            (0, 0),
            (w, h),
            &mut self.root,
            ((sw - w) / 2, (sh - h) / 2),
            1.0,
            1.0,
        );
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

impl fmt::Debug for MainMenuRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MainMenuRenderer")
    }
}
