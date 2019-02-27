use crate::stage::StageData;
use tcod::console::Offscreen;

pub trait Render {
    type StageType: StageData;

    fn borrow_root(&self) -> &Offscreen;
    fn update(&mut self, scene: &Self::StageType);
}
