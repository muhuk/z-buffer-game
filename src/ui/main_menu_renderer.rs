// © Copyright 2019-2020, Atamert Ölçgen
//
// This file is part of z-buffer-game.
//
// z-buffer-game is free software: you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// z-buffer-game is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with z-buffer-game.  If not, see <https://www.gnu.org/licenses/>.

use crate::menu::Menu;
use crate::stage::main_menu::{Choice, MainMenu};
use crate::ui::render::Render;
use std::fmt;
use tcod::colors;
use tcod::console::{blit, BackgroundFlag, Console, Offscreen, TextAlignment};

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
        // TODO: Put this in an offscreen or make it part of the background.
        self.root.print_rect_ex(
            sw / 2,
            sh - 2,
            0,
            1,
            BackgroundFlag::Set,
            TextAlignment::Center,
            "Press <alt> + <enter> to toggle full-screen.",
        );
    }
}

impl fmt::Debug for MainMenuRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MainMenuRenderer")
    }
}

impl Render for MainMenuRenderer {
    type StageType = MainMenu;

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
