use rand::{rngs::ThreadRng, Rng};

use crate::world::WorldStorage;
use crate::world::blocks::Blocks;
use crate::world::chunks::{WORLD_SIZE, CHUNK_SIZE};

pub fn generate(
    world: &mut WorldStorage,
    rng: &mut ThreadRng
) {
    // fill whole world with dirt
    for y in 0..5 as i32 {
        for x in 0..WORLD_SIZE.x*CHUNK_SIZE as i32 {
            world.set_tile(x, y, if rng.gen() {Blocks::Dirt} else {Blocks::Air});
        }
    }
}