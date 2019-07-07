use crate::data::{Location, Rectangle, VisibleObject};
use std::convert::TryFrom;
use std::f32::consts::PI;
use std::i32::MAX as I32_MAX;
use std::iter;
use tcod::noise::{Noise, NoiseInitializer, NoiseType};
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
    const TREE_COUNT: u16 = 5;
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
    let poisson_rng = Rng::new_with_seed(NOISE_ALGO, poisson_seed);

    // Initialize grid
    let w: u16 = boundaries.width();
    let h: u16 = boundaries.height();
    let mut grid: Vec<Vec<u16>> =
        iter::repeat_with(|| iter::repeat(0).take(usize::from(w)).collect())
            .take(usize::from(h))
            .collect();

    // Initialize active list and add the first point
    let mut active: Vec<Location> = vec![Location::new(
        poisson_rng.get_int(0, i32::from(w) - 1),
        poisson_rng.get_int(0, i32::from(h) - 1),
    )];

    // Populate the grid
    //
    // TODO: Generate `count` many trees, give up
    //       after pre-determinet number of tries.
    generate_tree(&poisson_rng, &mut grid, &mut active, radius * 2);
    generate_tree(&poisson_rng, &mut grid, &mut active, radius * 2);
    generate_tree(&poisson_rng, &mut grid, &mut active, radius * 2);

    // TODO: Filter trees by mask noise threshold.

    // Convert grid to final result
    let mut result: Vec<(Location, u16)> = vec![];
    let offset: Location = boundaries.into_iter().next().unwrap();
    for (y, row) in grid.iter().enumerate() {
        for (x, &r) in row.iter().enumerate() {
            if r > 0 {
                let loc: Location = offset.move_by(
                    i32::try_from(x).unwrap(),
                    i32::try_from(y).unwrap(),
                );
                result.push((loc, r));
            }
        }
    }
    result
}

fn generate_tree(
    rng: &Rng,
    grid: &mut Vec<Vec<u16>>,
    active: &mut Vec<Location>,
    distance: u16,
) -> bool {
    let d = f32::from(distance);
    if active.is_empty() {
        false
    } else {
        let idx: usize = usize::try_from(
            rng.get_int(0, i32::try_from(active.len()).unwrap() - 1),
        )
        .unwrap();
        let p: Location = active.remove(idx);
        let angle: f32 = rng.get_float(0.0, 2.0 * PI);
        let dx: f32 = (angle.cos() * d).floor();
        let dy: f32 = (angle.sin() * d).floor();

        let x = usize::try_from(p.x + dx as i32).unwrap();
        let y = usize::try_from(p.y + dy as i32).unwrap();

        // TODO: Reject point if there are other trees nearby.
        if let Some(Some(&v)) = grid.get(y).map(|row| row.get(x)) {
            if v > 0 {
                false
            } else {
                grid[y][x] = 1;
                active.push(Location::new(
                    i32::try_from(x).unwrap(),
                    i32::try_from(y).unwrap(),
                ));
                true
            }
        } else {
            unreachable!();
        }
    }
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
