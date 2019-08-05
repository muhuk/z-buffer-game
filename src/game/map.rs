use crate::data::{Location, Rectangle, VisibleObject};
use crate::game::{components, Cursor};
use bluenoisers::blue_noise_iter;
use log::debug;
use specs::prelude::*;
use std::i32;
use std::mem::transmute;
use tcod::noise::{Noise, NoiseType};
use tcod::random::{Algo, Rng};

const MAP_WIDTH: u16 = 64;
const MAP_HEIGHT: u16 = 64;

const NOISE_ALGO: Algo = Algo::MT;
const NOISE_SCALE: f32 = 9.18325;

const TREE_DISTANCE: u16 = 3;
// There is no science to the threshold value
// noise seems to have a non-uniform histogram.
const TREE_MASK_THRESHOLD: f32 = 0.55;
const TREE_SIZE_MIN: i32 = 2;
const TREE_SIZE_MAX: i32 = 5;

#[derive(Debug, PartialEq)]
pub enum MapStatus {
    Unitialized,
    Initialized,
}

pub struct MapSystem {
    status: MapStatus,
}

impl MapSystem {
    pub fn new() -> Self {
        // Initialize the map.
        Self {
            status: MapStatus::Unitialized,
        }
    }

    fn generate_map<F, G>(
        seed: u32,
        boundaries: Rectangle,
        add_tile: F,
        add_tree: G,
    ) where
        F: FnMut(Location, VisibleObject),
        G: FnMut(Location, u16),
    {
        debug!("Generating new map with seed {}", seed);

        let rng = Rng::new_with_seed(NOISE_ALGO, seed);
        let ground_noise_seed: u32 = generate_seed(&rng);
        let trees_mask_seed: u32 = generate_seed(&rng);
        let trees_size_seed: u32 = generate_seed(&rng);

        generate_map_tiles(ground_noise_seed, boundaries, add_tile);
        generate_trees(
            trees_mask_seed,
            TREE_MASK_THRESHOLD,
            trees_size_seed,
            TREE_DISTANCE,
            TREE_SIZE_MIN,
            TREE_SIZE_MAX,
            boundaries,
            add_tree,
        );
    }
}

impl<'a> System<'a> for MapSystem {
    type SystemData = (Write<'a, Cursor>, Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, sys_data: Self::SystemData) {
        let (mut cursor, entities, lazy_update) = sys_data;

        if self.status == MapStatus::Unitialized {
            let seed: u32 = 987654;
            let boundaries = Rectangle::centered_around(
                Location::origin(),
                MAP_WIDTH,
                MAP_HEIGHT,
            );

            Self::generate_map(
                seed,
                boundaries,
                |loc, obj| {
                    lazy_update
                        .create_entity(&entities)
                        .with(components::Location::new(loc))
                        .with(components::Renderable::new(obj, 0))
                        .build();
                },
                |loc, r| {
                    lazy_update
                        .create_entity(&entities)
                        .with(components::Tree::new(r))
                        .with(components::Location::new(loc))
                        .with(components::Renderable::new(
                            VisibleObject::TreeTrunk,
                            1,
                        ))
                        .build();
                },
            );
            cursor.set_boundaries(boundaries).unwrap();
            self.status = MapStatus::Initialized
        }
    }
}

fn generate_map_tiles<F>(seed: u32, boundaries: Rectangle, mut add_tile: F)
where
    F: FnMut(Location, VisibleObject),
{
    let ground_noise = make_2d_noise(seed, NoiseType::Simplex);
    for loc in boundaries.into_iter() {
        let noise_value = ground_noise.get(location_to_noise_coordinate(loc));
        let obj = if noise_value > 0.5 {
            VisibleObject::Soil
        } else {
            VisibleObject::Grass
        };
        add_tile(loc, obj);
    }
}

fn generate_seed(rng: &Rng) -> u32 {
    unsafe {
        // Seed 0 causes problems during
        // transmute, hence i32::MIN+1.
        transmute::<i32, u32>(rng.get_int(i32::MIN + 1, i32::MAX))
    }
}

fn generate_trees<F>(
    mask_seed: u32,
    mask_threshold: f32,
    size_seed: u32,
    distance: u16,
    size_min: i32,
    size_max: i32,
    boundaries: Rectangle,
    mut add_tree: F,
) where
    F: FnMut(Location, u16),
{
    let size_rng = Rng::new_with_seed(NOISE_ALGO, size_seed);
    let mask: Noise = make_2d_noise(mask_seed, NoiseType::Perlin);
    let k_abort: usize = 30;
    let w: i32 = i32::from(boundaries.width());
    let h: i32 = i32::from(boundaries.height());
    for pos in blue_noise_iter(
        vec![f64::from(w), f64::from(h)],
        f64::from(distance),
        k_abort,
    ) {
        debug_assert!(pos.len() == 2, "Blue noise dimensions is not 2");
        let location = {
            let x = pos[0] as i32 + boundaries.min_x;
            let y = pos[1] as i32 + boundaries.min_y;
            Location::new(x, y)
        };
        if (mask.get(location_to_noise_coordinate(location)) + 1.0) * 0.5
            > mask_threshold
        {
            let radius: u16 = size_rng.get_int(size_min, size_max) as u16;
            debug!("Planting tree of size {} at {:?}", radius, location);
            add_tree(location, radius);
        } else {
            debug!("Passing up {:?} as a candidate tree location.", location);
        }
    }
}

fn location_to_noise_coordinate(location: Location) -> [f32; 2] {
    let x: f32 = (location.x as f32) / NOISE_SCALE;
    let y: f32 = (location.y as f32) / NOISE_SCALE;
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
