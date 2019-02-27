use crate::stage::Stage;
use crate::ui::game_renderer::GameRenderer;
use crate::ui::main_menu_renderer::MainMenuRenderer;
use crate::ui::render::Render;

/// Since [Stage](crate::stage::Stage) is an `enum` and dependency is from
/// [ui](crate::ui) to [stage](crate::stage) we have Renderer as an enum to
/// match its structure.
#[derive(Debug)]
pub enum Renderer {
    Game(GameRenderer),
    MainMenu(MainMenuRenderer),
}

impl Renderer {
    pub fn update(&mut self, stage: &Stage) {
        match (&stage, self) {
            (Stage::Game(g), Renderer::Game(renderer)) => {
                renderer.update(g);
            }
            (Stage::MainMenu(m), Renderer::MainMenu(renderer)) => {
                renderer.update(m);
            }
            (s, p) => panic!("Mismatched renderer {:?} for stage {:?}", p, s),
        };
    }
}
