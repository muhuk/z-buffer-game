use crate::menu::Menu;
use crate::stage::main_menu::{Choice, MainMenu};
use crate::ui::render::Render;
use std::fmt;
use tcod::colors;
use tcod::console::{blit, BackgroundFlag, Console, Offscreen};

pub struct MainMenuRenderer {
    root: Offscreen,
    console: Offscreen,
}

impl MainMenuRenderer {
    pub fn new(window_width: u32, window_height: u32) -> MainMenuRenderer {
        let root = Offscreen::new(window_width as i32, window_height as i32);
        let (width, height) = Self::calculate_size();
        let console = Self::make_menu_console(width, height);
        MainMenuRenderer { root, console }
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
}

impl fmt::Debug for MainMenuRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MainMenuRenderer")
    }
}

impl Render for MainMenuRenderer {
    type SceneType = MainMenu;

    fn borrow_root(&self) -> &Offscreen {
        &self.root
    }

    fn update(&mut self, menu: &MainMenu) {
        let bg_flag: BackgroundFlag = BackgroundFlag::Set;
        let width: i32 = self.console.width();
        for (idx, choice) in menu.iter().enumerate() {
            let y: i32 = idx as i32;
            let (fg_color, bg_color) = if menu.is_selected(choice) {
                (colors::WHITE, colors::RED)
            } else {
                (colors::WHITE, colors::BLACK)
            };
            for x in 0..width {
                self.console.set_char_foreground(x, y, fg_color);
                self.console.set_char_background(x, y, bg_color, bg_flag);
            }
        }
        self.blit();
    }
}
