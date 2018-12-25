use crate::asset;
use crate::game::Game;
use crate::menu::Menu;
use crate::stage::Stage;
use tcod::colors;
use tcod::console::{self, Console, Root};
use tcod::system::get_fps;

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
            // let dt = game.dt();
            // let time = game.time();
            let root: &mut Root = &mut game.ui.root_console;
            root.clear();
            // root.set_char(SCREEN_WIDTH_CHAR / 2, SCREEN_HEIGHT_CHAR / 2 - 2, '');
            // root.set_alignment(console::TextAlignment::Center);

            let no_of_items: i32 = m.iter().count() as i32;

            for (idx, selection) in m.iter().enumerate() {
                if m.is_selected(&selection) {
                    root.set_default_background(colors::WHITE);
                    root.set_default_foreground(colors::DARKER_GREY);
                } else {
                    root.set_default_background(colors::BLACK);
                    root.set_default_foreground(colors::WHITE);
                }
                root.print_rect_ex(
                    SCREEN_WIDTH_CHAR / 2,
                    SCREEN_HEIGHT_CHAR / 2 + (idx as i32) - (no_of_items / 2),
                    SCREEN_WIDTH_CHAR,
                    1,
                    console::BackgroundFlag::Set,
                    console::TextAlignment::Center,
                    format!("{}", &selection),
                );
            }
            root.set_default_background(colors::BLACK);
            root.set_default_foreground(colors::WHITE);

            root.flush();
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
