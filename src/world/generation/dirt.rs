use rand::{rngs::ThreadRng, Rng};

use crate::world::WorldStorage;
use crate::world::blocks::Blocks;
use crate::world::chunks::{WORLD_SIZE, CHUNK_SIZE};

pub fn generate(
    world: &mut WorldStorage,
    rng: &mut ThreadRng
) {
    // fill whole world with dirt
    for y in 0..WORLD_SIZE.y*CHUNK_SIZE as i32 {
        for x in 0..WORLD_SIZE.x*CHUNK_SIZE as i32 {
            world.set(x, y, if rng.gen() {Blocks::Dirt as u32} else {Blocks::Stone as u32});
        }
    }
}