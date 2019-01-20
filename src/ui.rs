use crate::asset;
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

const MAX_FPS: u32 = 30;
const SCREEN_WIDTH_CHAR: i32 = 80;
const SCREEN_HEIGHT_CHAR: i32 = 50;
const TITLE: &str = "z-buffer";

/// User interface related data
pub struct UI {
    pub root_console: Root,
    pub screen_width_char: i32,
    pub screen_height_char: i32,
    pub fps: u32,
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
        .title(TITLE)
        .size(SCREEN_WIDTH_CHAR, SCREEN_HEIGHT_CHAR)
        .font(font_file, console::FontLayout::AsciiInRow)
        .init();

    tcod::system::set_fps(MAX_FPS as i32);

    UI {
        root_console: root,
        screen_width_char: SCREEN_WIDTH_CHAR,
        screen_height_char: SCREEN_HEIGHT_CHAR,
        fps: 0,
    }
}
