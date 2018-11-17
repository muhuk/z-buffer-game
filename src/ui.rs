use asset;
use tcod::console::{self, Console, Root};

const FPS: i32 = 30;
const SCREEN_WIDTH_CHAR: i32 = 80;
const SCREEN_HEIGHT_CHAR: i32 = 50;
const TITLE: &str = "z-buffer";

/// User interface related data
pub struct UI {
    pub root_console: Root,
    pub screen_width_char: i32,
    pub screen_height_char: i32,
}

/// Render a smiley & write "Hello, World!" on the center.
///
/// Remove this after proper stages are implemented.
pub fn draw_hello_world<T: Console>(con: &mut T) {
    con.clear();
    con.set_char(SCREEN_WIDTH_CHAR / 2, SCREEN_HEIGHT_CHAR / 2 - 2, '');
    con.set_alignment(console::TextAlignment::Center);
    con.print_rect(
        SCREEN_WIDTH_CHAR / 2,
        SCREEN_HEIGHT_CHAR / 2 + 2,
        SCREEN_WIDTH_CHAR,
        1,
        "Hello, World!",
    );
}

pub fn initialize() -> UI {
    let font_file = asset::Assets::FontTerminal16x16GsRo.extract().unwrap();

    let root = console::Root::initializer()
        .title(TITLE)
        .size(SCREEN_WIDTH_CHAR, SCREEN_HEIGHT_CHAR)
        .font(font_file, console::FontLayout::AsciiInRow)
        .init();

    tcod::system::set_fps(FPS);

    UI {
        root_console: root,
        screen_width_char: SCREEN_WIDTH_CHAR,
        screen_height_char: SCREEN_HEIGHT_CHAR,
    }
}
