use crate::stage::Stage;
use crate::ui::game_renderer::GameRenderer;
use crate::ui::main_menu_renderer::MainMenuRenderer;
use crate::ui::render::Render;
use tcod::console::Offscreen;

/// Since [Stage](crate::stage::Stage) is an `enum` and dependency is from
/// [ui](crate::ui) to [stage](crate::stage) we have Renderer as an enum to
/// match its structure.
#[derive(Debug)]
pub enum Renderer {
    Game(GameRenderer),
    MainMenu(MainMenuRenderer),
}

impl Renderer {
    pub fn new(stage: &Stage, width: u32, height: u32) -> Self {
        match stage {
            Stage::Game(_) => Renderer::Game(GameRenderer::new(width, height)),
            Stage::MainMenu(_) => {
                Renderer::MainMenu(MainMenuRenderer::new(width, height))
            }
        }
    }

    pub fn borrow_root(&self) -> &Offscreen {
        match self {
            Renderer::Game(r) => r.borrow_root(),
            Renderer::MainMenu(r) => r.borrow_root(),
        }
    }

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

    pub fn is_stage_compatible(&self, stage: &Stage) -> bool {
        match (stage, &self) {
            (Stage::MainMenu(_), Renderer::MainMenu(_)) => true,
            (Stage::Game(_), Renderer::Game(_)) => true,
            _ => false,
        }
    }
}
