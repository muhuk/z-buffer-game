use crate::data::{Location, Rectangle, VisibleObject};
use crate::noise::poisson::poisson_discrete;
use std::convert::TryFrom;
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
    let trees_poisson_seed: u32 = rng.get_int(0, I32_MAX) as u32;

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

    const TREE_RADIUS: u16 = 6;
    const TREE_COUNT: u16 = 30;
    let trees: Vec<(Location, u16)> = generate_trees(
        trees_mask_seed,
        trees_poisson_seed,
        boundaries,
        TREE_RADIUS,
        TREE_COUNT,
    );
    for (loc, radius) in trees {
        // TODO: Render foilage too.
        f(loc, VisibleObject::TreeTrunk);
    }
}

fn generate_trees(
    mask_seed: u32,
    poisson_seed: u32,
    boundaries: Rectangle,
    radius: u16,
    count: u16,
) -> Vec<(Location, u16)> {
    // TODO: Try simplifiying types, miniming type conversion.
    let poisson_rng = Rng::new_with_seed(NOISE_ALGO, poisson_seed);

    let max_retries: u32 = 100;
    let samples: Vec<(u32, u32)> = poisson_discrete(
        |l| {
            u32::try_from(
                poisson_rng.get_int(0, i32::try_from(l).unwrap() - 1),
            )
            .unwrap()
        },
        u32::from(boundaries.width()),
        u32::from(boundaries.height()),
        u32::from(count),
        u32::from(radius),
        max_retries,
    );
    // TODO: Filter trees by mask noise threshold.
    // TODO: Use variable tree radius.
    samples
        .iter()
        .map(|(x, y)| {
            (
                Location::new(
                    boundaries.min_x + i32::try_from(*x).unwrap(),
                    boundaries.min_y + i32::try_from(*y).unwrap(),
                ),
                radius,
            )
        })
        .collect()
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
