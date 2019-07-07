use crate::data::{Location, Rectangle, VisibleObject};
use tcod::noise::{Noise, NoiseInitializer, NoiseType};
use tcod::random::{Algo, Rng};

const SCALE: f32 = 9.18325;

pub fn generate_map<F>(boundaries: Rectangle, mut f: F)
where
    F: FnMut(Location, VisibleObject),
{
    // TODO: Get seed from the caller.
    let seed: u32 = 987654;
    let rng = Rng::new_with_seed(Algo::MT, seed);

    let dimensions: u32 = 2;
    let noise = Noise::init_with_dimensions(dimensions)
        .random(rng)
        .noise_type(NoiseType::Simplex)
        .init();

    for loc in boundaries.into_iter() {
        let noise_value = noise.get(location_to_noise_coordinate(loc));
        let obj = if noise_value > 0.5 {
            VisibleObject::Soil
        } else {
            VisibleObject::Grass
        };
        f(loc, obj);
    }
}

fn location_to_noise_coordinate(location: Location) -> [f32; 2] {
    let x: f32 = (location.x as f32) / SCALE;
    let y: f32 = (location.y as f32) / SCALE;
    [x, y]
}
