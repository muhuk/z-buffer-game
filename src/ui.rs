//! User interface.

use crate::asset;
use crate::conf;
use crate::stage::Stage;
use crate::ui::render::Render;
use crate::ui::renderer::Renderer;
use log::debug;
use tcod::console::{self, Console};
use tcod::system::get_fps;

mod constants;
mod game_renderer;
mod main_menu_renderer;
mod render;
mod renderer;

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

        // Create a new renderer if necessary.
        // Use the existing one if the stage is not changed.
        if self.is_stage_changed(stage) {
            self.reset_renderer(stage);
        }

        // Update the renderer.
        match &mut self.renderer {
            Some(r) => r.update(stage),
            None => unreachable!(),
        }

        // Blit whatever is in the renderer's root onto the root console.
        {
            let root: &mut console::Root = &mut self.root_console;
            match &mut self.renderer {
                Some(Renderer::Game(renderer)) => Self::blit(root, renderer),
                Some(Renderer::MainMenu(renderer)) => {
                    Self::blit(root, renderer)
                }
                None => unreachable!(),
            }
        }
    }

    pub fn is_running(&self) -> bool {
        !self.root_console.window_closed()
    }

    #[inline]
    fn is_stage_changed(&self, stage: &Stage) -> bool {
        match (stage, &self.renderer) {
            (Stage::MainMenu(_), Some(Renderer::MainMenu(_))) => false,
            (Stage::Game(_), Some(Renderer::Game(_))) => false,
            _ => true,
        }
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
    fn blit<T: Render>(root: &mut console::Root, renderer: &mut T) {
        let source = renderer.borrow_root();
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
