use crate::asset;
use crate::game::Game;
use crate::stage::Stage;
use crate::ui::main_menu_renderer::MainMenuRenderer;
use tcod::console::{self, Root};
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
            let root: &mut Root = &mut game.ui.root_console;
            // TODO: Don't instantiate MainMenuRendered at every draw.
            let mut renderer = MainMenuRenderer::new();
            renderer.update(m);
            renderer.blit(root);
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

mod main_menu_renderer {
    use crate::menu::Menu;
    use crate::stage::main_menu::{Choice, MainMenu};
    use std::ops::{Deref, DerefMut};
    use tcod::colors::{self, Color};
    use tcod::console::{blit, BackgroundFlag, Console, Offscreen, Root};

    pub struct MainMenuRenderer {
        console: Offscreen,
    }

    impl MainMenuRenderer {
        pub fn new() -> MainMenuRenderer {
            let (width, height) = Self::calculate_size();
            let mut console = Offscreen::new(width, height);
            for (idx, choice) in Choice::ALL.iter().enumerate() {
                console.print(0, idx as i32, format!("{}", choice));
            }
            MainMenuRenderer { console }
        }

        pub fn blit(&mut self, root: &mut Root) {
            let w: i32 = self.console.width();
            let h: i32 = self.console.height();
            let x: i32 = (root.width() - w) / 2;
            let y: i32 = (root.height() - h) / 2;
            blit(&**self, (0, 0), (w, h), root, (x, y), 1.0, 1.0);
        }

        pub fn update(&mut self, menu: &MainMenu) {
            for (idx, choice) in menu.iter().enumerate() {
                let y: i32 = idx as i32;
                if menu.is_selected(choice) {
                    self.paint_row(y, colors::WHITE, colors::RED);
                } else {
                    self.paint_row(y, colors::WHITE, colors::BLACK);
                }
            }
        }

        fn calculate_size() -> (i32, i32) {
            let width = Choice::ALL
                .iter()
                .map(|c| format!("{}", c).len())
                .max()
                .unwrap();
            let height = Choice::ALL.len();
            (width as i32, height as i32)
        }

        fn paint_row(&mut self, y: i32, fg_color: Color, bg_color: Color) {
            let bg_flag: BackgroundFlag = BackgroundFlag::Set;
            let width: i32 = self.console.width();
            for x in 0..width {
                self.console.set_char_foreground(x, y, fg_color);
                self.console.set_char_background(x, y, bg_color, bg_flag);
            }
        }
    }

    impl Deref for MainMenuRenderer {
        type Target = Offscreen;

        fn deref(&self) -> &Offscreen {
            &self.console
        }
    }

    impl DerefMut for MainMenuRenderer {
        fn deref_mut(&mut self) -> &mut Offscreen {
            &mut self.console
        }
    }
}
