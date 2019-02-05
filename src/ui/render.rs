use tcod::console::Offscreen;

pub trait Render: Sized {
    type SceneType;

    fn borrow_root(&self) -> &Offscreen;
    fn update(&mut self, scene: &Self::SceneType);
}
