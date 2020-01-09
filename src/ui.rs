// © Copyright 2019-2020, Atamert Ölçgen
//
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

//! User interface.

use crate::asset;
use crate::conf;
use crate::stage::Stage;
use crate::ui::renderer::Renderer;
use log::debug;
use tcod::console::Offscreen;
use tcod::console::{self, Console};
use tcod::system::{
    force_fullscreen_resolution, get_current_resolution, get_fps,
};

mod constants;
mod game_renderer;
mod main_menu_renderer;
mod render;
mod renderer;
mod tile;

/// User interface related data
pub struct UI {
    fps: u32,
    root_console: console::Root,
    renderer: Option<Renderer>,
}

impl UI {
    pub fn new() -> UI {
        let font_file =
            asset::Assets::FontTerminal16x16GsRo.extract().unwrap();

        let root = console::Root::initializer()
            .title(conf::window_title())
            .size(
                conf::screen_width_char() as i32,
                conf::screen_height_char() as i32,
            )
            .font(font_file, console::FontLayout::AsciiInRow)
            .init();

        tcod::system::set_fps(conf::max_fps() as i32);

        UI {
            root_console: root,
            fps: 0,
            renderer: None,
        }
    }

    pub fn toggle_fullscreen(&mut self) {
        if self.root_console.is_fullscreen() {
            debug!("Turning full-screen off.");
            self.root_console.set_fullscreen(false);
        } else {
            let resolution: (i32, i32) = get_current_resolution();
            debug!("Screen resolution is {}x{}", resolution.0, resolution.1);
            force_fullscreen_resolution(resolution.0, resolution.1);
            debug!("Turning full-screen on.");
            self.root_console.set_fullscreen(true);
        }
    }

    /// Render UI based on the current stage.
    pub fn draw(&mut self, stage: &Stage) {
        self.fps = get_fps() as u32;

        // Create a new renderer if the existing one does not match the stage,
        // or if none exists.
        if self
            .renderer
            .as_ref()
            .map_or(true, |r| !r.is_stage_compatible(stage))
        {
            self.reset_renderer(stage);
        }

        let root: &mut console::Root = &mut self.root_console;
        self.renderer
            .as_mut()
            .map(|r| {
                // Update the renderer.
                r.update(stage);

                // Blit whatever is in the renderer's root onto the root console.
                Self::blit(root, r.borrow_root());
            })
            .unwrap();
    }

    pub fn is_running(&self) -> bool {
        !self.root_console.window_closed()
    }

    fn reset_renderer(&mut self, stage: &Stage) {
        let width: u32 = self.root_console.width() as u32;
        let height: u32 = self.root_console.height() as u32;
        debug!("Window dimensions are {}x{}", width, height);
        let renderer = Renderer::new(stage, width, height);
        debug!("Updating renderer as {:?}.", &renderer);
        self.renderer = Some(renderer);
    }

    #[inline]
    fn blit(root: &mut console::Root, source: &Offscreen) {
        console::blit(
            source,
            (0, 0),
            (source.width(), source.height()),
            root,
            (0, 0),
            1.0,
            1.0,
        );
        root.flush();
    }
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        self.root_console.set_fullscreen(false);
    }
}
