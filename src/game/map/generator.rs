use crate::data::{Location, Rectangle, VisibleObject};
use tcod::noise::{Noise, NoiseInitializer, NoiseType};
use tcod::random::{Algo, Rng};

const SCALE: f32 = 9.18325;

pub fn generate_map<F>(seed: u32, boundaries: Rectangle, mut f: F)
where
    F: FnMut(Location, VisibleObject),
{
    let rng = Rng::new_with_seed(Algo::MT, seed);

    let ground_noise = make_2d_noise(rng, NoiseType::Simplex);

    for loc in boundaries.into_iter() {
        let noise_value = ground_noise.get(location_to_noise_coordinate(loc));
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

fn make_2d_noise(rng: Rng, noise_type: NoiseType) -> Noise {
    const DIMENSIONS: u32 = 2;
    Noise::init_with_dimensions(DIMENSIONS)
        .random(rng)
        .noise_type(noise_type)
        .init()
}
