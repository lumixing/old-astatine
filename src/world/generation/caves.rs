use noise::{NoiseFn, Perlin, Fbm};
use rand::{rngs::ThreadRng, Rng};

use crate::world::WorldStorage;
use crate::world::blocks::Blocks;

const CAVES_SCALE: f64 = 10.0;
const CAVES_TRESHOLD: f64 = -0.1;

pub fn generate(
    rng: &mut ThreadRng,
    world: &mut WorldStorage,
) {
    let fbm = Fbm::<Perlin>::new(rng.gen());

    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            if world.get_tile_usize(x, y) != Blocks::Stone { continue; }

            let val = fbm.get([x as f64 / CAVES_SCALE, y as f64 / CAVES_SCALE, 0.0]);
            if val < CAVES_TRESHOLD {
                world.set_tile_usize(x, y, Blocks::Air);
            }
        }
    }
}