// © Copyright 2019-2020, Atamert Ölçgen
//
// This file is part of z-buffer-game.
//
// z-buffer-game is free software: you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// z-buffer-game is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with z-buffer-game.  If not, see <https://www.gnu.org/licenses/>.

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
