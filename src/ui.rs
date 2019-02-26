//! User interface.

use crate::asset;
use crate::conf;
use crate::stage::Stage;
use crate::ui::game_renderer::GameRenderer;
use crate::ui::main_menu_renderer::MainMenuRenderer;
use crate::ui::render::Render;
use log::debug;
use tcod::console::{self, Console};
use tcod::system::get_fps;

mod constants;
mod game_renderer;
mod main_menu_renderer;
mod render;

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

        if self.is_stage_changed(stage) {
            self.reset_renderer(stage);
        }

        // TODO: This part is still too messy but it works somewhat.
        let root: &mut console::Root = &mut self.root_console;
        match (&stage, &mut self.renderer) {
            (Stage::Game(g), Some(Renderer::Game(ref mut renderer))) => {
                renderer.update(g);
                Self::blit(root, renderer);
            }
            (
                Stage::MainMenu(m),
                Some(Renderer::MainMenu(ref mut renderer)),
            ) => {
                renderer.update(m);
                Self::blit(root, renderer);
            }
            (s, Some(p)) => {
                panic!("Mismatched renderer {:?} for stage {:?}", p, s)
            }
            (_, None) => unreachable!(),
        };
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
        let renderer = match stage {
            Stage::Game(_) => Renderer::Game(GameRenderer::new(width, height)),
            Stage::MainMenu(_) => {
                Renderer::MainMenu(MainMenuRenderer::new(width, height))
            }
        };
        debug!("Updating renderer as {:?}.", &renderer);
        self.renderer = Some(renderer);
    }

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

/// Since [Stage](crate::stage::Stage) is an `enum` and dependency is from
/// [ui](crate::ui) to [stage](crate::stage) we have Renderer as an enum to
/// match its structure.
#[derive(Debug)]
enum Renderer {
    Game(GameRenderer),
    MainMenu(MainMenuRenderer),
}
