use bevy_tileset::prelude::{Tileset};
use noise::{NoiseFn, Perlin, Fbm};
use rand::{rngs::ThreadRng, Rng};

use crate::world::WorldStorage;

pub fn generate(
    rng: &mut ThreadRng,
    world: &mut WorldStorage,
    tileset: &Tileset,
) {
    let fbm = Fbm::<Perlin>::new(0);

    // dirt
    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            let idx = world.linearize(x, y);
            world.set_tile_idx(idx, 2);
        }
    }

    // surface n grass
    for x in 0..world.get_width() {
        let val = fbm.get([x as f64 / 24.0, 0.0, 0.0]) * 32.0 + world.get_height() as f64 - 30.0;
        world.set_tile(x as i32, val as i32, 3);

        for y in (val as usize + 1)..world.get_height() {
            world.set_tile(x as i32, y as i32, 0);
        }
    }

    // stone
    for x in 0..world.get_width() {
        let val = (x as f32 * 0.4).sin() * 1.6 + world.get_height() as f32 - 50.0;

        for y in (0..(val as i32)).rev() {
            if y < val as i32 - 5 {
                world.set_tile(x as i32, y as i32, 4);
                continue;
            }

            let block = if rng.gen_bool(0.5) { 2 } else { 4 };
            world.set_tile(x as i32, y as i32, block);
        }
    }

    // caves
    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            if world.get_tile(x as i32, y as i32) != 4 { continue; }
            let val = fbm.get([x as f64 / 10.0, y as f64 / 10.0, 0.0]);
            if val < -0.1 {
                world.set_tile(x as i32, y as i32, 0);
            }
        }
    }
}