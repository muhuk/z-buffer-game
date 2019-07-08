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
    let poisson_rng = Rng::new_with_seed(NOISE_ALGO, poisson_seed);

    // Initialize grid
    let w: u16 = boundaries.width();
    let h: u16 = boundaries.height();
    let mut grid: Vec<Vec<u16>> =
        iter::repeat_with(|| iter::repeat(0).take(usize::from(w)).collect())
            .take(usize::from(h))
            .collect();

    // Initialize active list and add the first point
    let mut active: Vec<(usize, usize)> = vec![(
        usize::try_from(poisson_rng.get_int(0, i32::from(w) - 1)).unwrap(),
        usize::try_from(poisson_rng.get_int(0, i32::from(h) - 1)).unwrap(),
    )];

    {
        let mut attempts: u16 = 10;
        let mut found: u16 = 0;
        while found < count && attempts > 0 {
            let attempt2: u16 = 10;
            if generate_tree(
                &poisson_rng,
                &mut grid,
                &mut active,
                radius,
                attempt2,
            ) {
                found += 1
            } else {
                attempts -= 1;
            }
        }
        println!("Added {} trees", found);
    }
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
    active: &mut Vec<(usize, usize)>,
    radius: u16,
    mut attempts: u16,
) -> bool {
    let distance = f32::from(radius * 2);
    if active.is_empty() {
        false
    } else {
        let mut found = false;
        while found == false && attempts > 0 {
            let idx: usize = usize::try_from(
                rng.get_int(0, i32::try_from(active.len()).unwrap() - 1),
            )
            .unwrap();
            let (origin_x, origin_y) = active.remove(idx);
            if let Some((x, y)) = choose_point_at_a_distance(
                &rng,
                grid.len(),
                grid[0].len(),
                origin_x,
                origin_y,
                distance,
            ) {
                // TODO: Reject point if there are other trees nearby.
                if grid[y][x] > 0 {
                    found = false;
                    attempts -= 1;
                } else {
                    grid[y][x] = 1;
                    active.push((x, y));
                    found = true;
                }
            } else {
                found = false;
                attempts -= 1;
            }
        }
        found
    }
}

fn choose_point_at_a_distance(
    rng: &Rng,
    width: usize,
    height: usize,
    origin_x: usize,
    origin_y: usize,
    distance: f32,
) -> Option<(usize, usize)> {
    let angle: f32 = rng.get_float(0.0, 2.0 * PI);
    let dx: f32 = (angle.cos() * distance).floor();
    let dy: f32 = (angle.sin() * distance).floor();
    // We need to do signed arithmetic below:
    let x = (origin_x as f32 + dx) as usize;
    let y = (origin_y as f32 + dy) as usize;
    if x < width && y < height {
        Some((x, y))
    } else {
        None
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
