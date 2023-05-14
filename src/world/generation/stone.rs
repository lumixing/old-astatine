use rand::{rngs::ThreadRng, Rng};

use crate::world::WorldStorage;
use crate::world::blocks::Blocks;

const STONE_LENGTH: f32 = 0.4;
const STONE_HEIGHT: f32 = 1.6;
const STONE_OFFSET: f32 = 50.0;
const STONE_THRESHOLD: usize = 5;

pub fn generate(
    rng: &mut ThreadRng,
    world: &mut WorldStorage,
) {
    for x in 0..world.get_width() {
        let val = ((x as f32 * STONE_LENGTH).sin() * STONE_HEIGHT + world.get_height() as f32 - STONE_OFFSET) as usize;

        for y in (0..val).rev() {
            if y < val - STONE_THRESHOLD {
                world.set_tile_usize(x, y, Blocks::Stone);
                continue;
            }

            let block = if rng.gen_bool(0.5) { Blocks::Dirt } else { Blocks::Stone };
            world.set_tile_usize(x, y, block);
        }
    }
}