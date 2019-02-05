//! User interface.

use crate::asset;
use crate::conf;
use crate::stage::Stage;
use crate::ui::game_renderer::GameRenderer;
use crate::ui::main_menu_renderer::MainMenuRenderer;
use log::debug;
use std::mem::{discriminant, Discriminant};
use tcod::console;
use tcod::system::get_fps;

mod game_renderer;
mod main_menu_renderer;

/// User interface related data
pub struct UI {
    fps: u32,
    root_console: console::Root,
    renderer: Option<(Discriminant<Stage>, Renderer)>,
}

impl UI {
    pub fn new() -> UI {
        let font_file = asset::Assets::FontTerminal16x16GsRo.extract().unwrap();

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
            let renderer = match stage {
                Stage::Game(_) => Renderer::Game,
                Stage::MainMenu(_) => Renderer::MainMenu(MainMenuRenderer::new()),
            };
            debug!("Updating renderer as {:?}.", &renderer);
            self.renderer = Some((discriminant(stage), renderer));
        }

        // TODO: Instead of passing root into the renderer, get offscreen console and
        // blit it onto the root within UI.
        match &stage {
            Stage::Game(_) => {
                let width: u32 = conf::screen_width_char();
                let height: u32 = conf::screen_height_char();

                let renderer = GameRenderer::new(width, height);
                let root: &mut console::Root = &mut self.root_console;
                console::blit(
                    renderer.borrow_root(),
                    (0, 0),
                    (width as i32, height as i32),
                    root,
                    (0, 0),
                    1.0,
                    1.0,
                );
                root.flush();
            }
            Stage::MainMenu(m) => {
                let width: u32 = conf::screen_width_char();
                let height: u32 = conf::screen_height_char();

                let renderer: &mut MainMenuRenderer = match self.renderer {
                    Some((_, Renderer::MainMenu(ref mut r))) => r,
                    _ => unreachable!(),
                };
                renderer.update(m);
                let root: &mut console::Root = &mut self.root_console;
                console::blit(
                    renderer.borrow_root(),
                    (0, 0),
                    (width as i32, height as i32),
                    root,
                    (0, 0),
                    1.0,
                    1.0,
                );
                root.flush();
            }
        };
    }

    pub fn is_running(&self) -> bool {
        !self.root_console.window_closed()
    }

    #[inline]
    fn is_stage_changed(&self, stage: &Stage) -> bool {
        self.renderer
            .as_ref()
            .map_or(true, |(d, _)| *d != discriminant(stage))
    }
}

/// Since [Stage](crate::stage::Stage) is an `enum` and dependency is from
/// [ui](crate::ui) to [stage](crate::stage) we have Renderer as an enum to
/// match its structure.
// TODO: Consider using a trait for the wrapped values in the variants.
#[derive(Debug)]
enum Renderer {
    Game,
    MainMenu(MainMenuRenderer),
}
