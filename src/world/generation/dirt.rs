use rand::{rngs::ThreadRng, Rng};

use crate::world::WorldStorage;
use crate::world::blocks::Blocks;

pub fn generate(
    world: &mut WorldStorage,
    rng: &mut ThreadRng
) {
    // fill whole world with dirt
    for y in 0..4*64 {
        for x in 0..4*64 {
            world.set(x, y, if rng.gen() {Blocks::Dirt as u32} else {Blocks::Stone as u32});
        }
    }
}