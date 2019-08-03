use crate::data::{Location, Rectangle, VisibleObject};
use bluenoisers::blue_noise_iter;
use std::i32::MAX as I32_MAX;
use tcod::noise::{Noise, NoiseType};
use tcod::random::{Algo, Rng};

const SCALE: f32 = 9.18325;
const NOISE_ALGO: Algo = Algo::MT;

pub fn generate_map<F>(seed: u32, boundaries: Rectangle, mut f: F)
where
    F: FnMut(Location, VisibleObject),
{
    let rng = Rng::new_with_seed(NOISE_ALGO, seed);
    let ground_noise_seed: u32 = rng.get_int(0, I32_MAX) as u32;
    let trees_mask_seed: u32 = rng.get_int(0, I32_MAX) as u32;
    let trees_radius_seed: u32 = rng.get_int(0, I32_MAX) as u32;

    let ground_noise = make_2d_noise(ground_noise_seed, NoiseType::Simplex);
    for loc in boundaries.into_iter() {
        let noise_value = ground_noise.get(location_to_noise_coordinate(loc));
        let obj = if noise_value > 0.5 {
            VisibleObject::Soil
        } else {
            VisibleObject::Grass
        };
        f(loc, obj);
    }

    const TREE_RADIUS: u16 = 2;
    let trees: Vec<(Location, u16)> = generate_trees(
        trees_mask_seed,
        trees_radius_seed,
        boundaries,
        TREE_RADIUS,
    );
    for (loc, radius) in trees {
        // TODO: Render foilage too.
        f(loc, VisibleObject::TreeTrunk);
    }
}

fn generate_trees(
    mask_seed: u32,
    radius_seed: u32,
    boundaries: Rectangle,
    radius: u16,
) -> Vec<(Location, u16)> {
    let radius_rng = Rng::new_with_seed(NOISE_ALGO, radius_seed);
    let k_abort: usize = 30;
    let w: i32 = i32::from(boundaries.width());
    let h: i32 = i32::from(boundaries.height());
    let mut result: Vec<(Location, u16)> = Vec::new();
    for pos in blue_noise_iter(
        vec![f64::from(w), f64::from(h)],
        f64::from(radius * 2 + 1),
        k_abort,
    ) {
        debug_assert!(pos.len() == 2, "Blue noise dimensions is not 2");
        let x = pos[0] as i32 + boundaries.min_x;
        let y = pos[1] as i32 + boundaries.min_y;
        // TODO: Vary radius
        // TODO: Apply the mask
        result.push((
            Location::new(x, y),
            radius_rng.get_int(1, i32::from(radius)) as u16,
        ));
    }
    result
}

fn location_to_noise_coordinate(location: Location) -> [f32; 2] {
    let x: f32 = (location.x as f32) / SCALE;
    let y: f32 = (location.y as f32) / SCALE;
    [x, y]
}

fn make_2d_noise(seed: u32, noise_type: NoiseType) -> Noise {
    const DIMENSIONS: u32 = 2;
    let rng = Rng::new_with_seed(NOISE_ALGO, seed);
    Noise::init_with_dimensions(DIMENSIONS)
        .random(rng)
        .noise_type(noise_type)
        .init()
}
