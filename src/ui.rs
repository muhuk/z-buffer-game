//! User interface.

use crate::asset;
use crate::conf;
use crate::stage::Stage;
use crate::ui::renderer::Renderer;
use log::debug;
use tcod::console::Offscreen;
use tcod::console::{self, Console};
use tcod::system::get_fps;

mod constants;
mod game_renderer;
mod main_menu_renderer;
mod render;
mod renderer;
mod tiles;

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
