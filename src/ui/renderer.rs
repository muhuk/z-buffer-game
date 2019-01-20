use tcod::console::Root;

pub trait Renderer {
    fn blit(&mut self, root: &mut Root);
}
