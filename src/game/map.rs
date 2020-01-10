// © Copyright 2019-2020, Atamert Ölçgen
//
// This file is part of z-buffer-game.
//
// z-buffer-game is free software: you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// z-buffer-game is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with z-buffer-game.  If not, see <https://www.gnu.org/licenses/>.

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

enum ObjectChoice {
    Rock,
    Tree(u16),
}

impl ObjectChoice {
    fn choose(x: f64) -> Option<ObjectChoice> {
        assert!(x >= 0.0 && x <= 1.0, "x must be between 0.0 and 1.0");
        if x < 0.2 {
            Some(ObjectChoice::Rock)
        } else if x < 0.35 {
            Some(ObjectChoice::Tree(2))
        } else {
            None
        }
    }
}

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

    fn generate_map<F, G, H>(
        seed: u32,
        boundaries: Rectangle,
        mut add_ground_tile: F,
        mut add_tree: G,
        mut add_rock: H,
    ) where
        F: FnMut(Location, VisibleObject),
        G: FnMut(Location, u16),
        H: FnMut(Location),
    {
        debug!("Generating new map with seed {}", seed);

        let rng = Rng::new_with_seed(NOISE_ALGO, seed);
        let ground_noise_seed: u32 = generate_seed(&rng);
        let trees_mask_seed: u32 = generate_seed(&rng);
        let trees_size_seed: u32 = generate_seed(&rng);
        let object_rng_seed: u32 = generate_seed(&rng);

        {
            let threshold = 0.5;
            let ground_noise =
                make_2d_noise(ground_noise_seed, NoiseType::Simplex);
            for loc in boundaries.into_iter() {
                let noise_value =
                    ground_noise.get(location_to_noise_coordinate(loc));
                let obj = if noise_value > threshold {
                    VisibleObject::Soil
                } else {
                    VisibleObject::Grass
                };
                add_ground_tile(loc, obj);
            }
        }

        {
            let object_rng = Rng::new_with_seed(NOISE_ALGO, object_rng_seed);
            let k_abort = 30;
            for pos in blue_noise_iter(
                vec![
                    f64::from(boundaries.width()),
                    f64::from(boundaries.height()),
                ],
                f64::from(TREE_DISTANCE),
                k_abort,
            ) {
                debug_assert!(
                    pos.len() == 2,
                    "Blue noise is not 2-dimensional"
                );
                let location = {
                    let x = pos[0] as i32 + boundaries.min_x;
                    let y = pos[1] as i32 + boundaries.min_y;
                    Location::new(x, y)
                };
                let object_choice =
                    ObjectChoice::choose(object_rng.get_double(0.0, 1.0));
                match object_choice {
                    Some(ObjectChoice::Rock) => add_rock(location),
                    Some(ObjectChoice::Tree(radius)) => {
                        add_tree(location, radius)
                    }
                    None => (),
                }
            }
        }
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
                |loc| {
                    lazy_update
                        .create_entity(&entities)
                        .with(components::Location::new(loc))
                        .with(components::Renderable::new(
                            VisibleObject::Rock,
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

fn generate_seed(rng: &Rng) -> u32 {
    unsafe {
        // i31::MIN doesn't work so we're using i32::MIN+1.
        transmute::<i32, u32>(rng.get_int(i32::MIN + 1, i32::MAX))
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
