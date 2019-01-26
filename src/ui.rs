use crate::asset;
use crate::conf;
use crate::game::Game;
use crate::stage::Stage;
use crate::ui::game_renderer::GameRenderer;
use crate::ui::main_menu_renderer::MainMenuRenderer;
use crate::ui::renderer::Renderer;
use tcod::console::{self, Root};
use tcod::system::get_fps;

mod game_renderer;
mod main_menu_renderer;
mod renderer;

/// User interface related data
pub struct UI {
    pub root_console: Root,
    pub fps: u32,
}

impl UI {
    pub fn stage_changed(&self, _new_stage: &Stage) {
        unimplemented!();
    }
}

/// Render UI based on the current stage.
pub fn draw(game: &mut Game) {
    game.ui.fps = get_fps() as u32;

    match &game.stage {
        Stage::MainMenu(m) => {
            let root: &mut Root = &mut game.ui.root_console;
            // TODO: Don't instantiate MainMenuRenderer at every draw.
            let mut renderer = MainMenuRenderer::new();
            renderer.update(m);
            renderer.blit(root);
            root.flush();
        }
        Stage::Game { .. } => {
            let root: &mut Root = &mut game.ui.root_console;
            // TODO: Don't instantiate GameRenderer at every draw.
            let mut renderer = GameRenderer {};
            renderer.blit(root);
        }
    }
}

pub fn initialize() -> UI {
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
    }
}
