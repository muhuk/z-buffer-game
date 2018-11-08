use tcod::console::{self, Console, Root};

const SCREEN_WIDTH_CHAR: i32 = 80;
const SCREEN_HEIGHT_CHAR: i32 = 50;
const FONT_FILE: &str = "assets/terminal16x16_gs_ro.png";
const FPS: i32 = 30;

pub struct UI {
    pub root_console: Root,
}

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
    let root = console::Root::initializer()
        .title("z-buffer")
        .size(SCREEN_WIDTH_CHAR, SCREEN_HEIGHT_CHAR)
        .font(FONT_FILE, console::FontLayout::AsciiInRow)
        .init();

    tcod::system::set_fps(FPS);

    UI { root_console: root }
}
