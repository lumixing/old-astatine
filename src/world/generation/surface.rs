use noise::{NoiseFn, Perlin, Fbm};
use rand::{rngs::ThreadRng, Rng};

use crate::world::WorldStorage;

const SURFACE_LENGTH: f64 = 24.0;
const SURFACE_HEIGHT: f64 = 24.0;
const SURFACE_OFFSET: f64 = 30.0;

pub fn generate(
    rng: &mut ThreadRng,
    world: &mut WorldStorage,
) {
    let fbm = Fbm::<Perlin>::new(rng.gen());

    for x in 0..world.get_width() {
        let val = (fbm.get([x as f64 / SURFACE_LENGTH, 0.0, 0.0]) * SURFACE_HEIGHT + world.get_height() as f64 - SURFACE_OFFSET) as usize;
        world.set_tile_usize(x, val, 3);

        for y in (val + 1)..world.get_height() {
            world.set_tile_usize(x, y, 0);
        }
    }
}