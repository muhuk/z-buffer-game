use crate::data::{Location, Rectangle, VisibleObject};

pub fn generate_map<F>(boundaries: Rectangle, mut f: F)
where
    F: FnMut(Location, VisibleObject),
{
    for loc in boundaries.into_iter() {
        let obj = if loc.x % 8 == 0 && loc.y % 8 == 0 {
            VisibleObject::Soil
        } else {
            VisibleObject::Grass
        };
        f(loc, obj);
    }
}
