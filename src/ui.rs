use asset;
use stage::Stage;
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

/// Render UI based on the current stage.
pub fn draw(stage: &Stage, root: &mut Root) {
    match stage {
        Stage::Menu => {
            root.clear();
            root.set_char(SCREEN_WIDTH_CHAR / 2, SCREEN_HEIGHT_CHAR / 2 - 2, '');
            root.set_alignment(console::TextAlignment::Center);
            root.print_rect(
                SCREEN_WIDTH_CHAR / 2,
                SCREEN_HEIGHT_CHAR / 2 + 2,
                SCREEN_WIDTH_CHAR,
                1,
                "Hello, World!",
            );
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

    tcod::system::set_fps(FPS);

    UI {
        root_console: root,
        screen_width_char: SCREEN_WIDTH_CHAR,
        screen_height_char: SCREEN_HEIGHT_CHAR,
    }
}
