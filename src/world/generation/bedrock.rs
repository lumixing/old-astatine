use rand::Rng;
use rand::rngs::ThreadRng;

use crate::world::WorldStorage;
use crate::world::blocks::Blocks;

pub fn generate(
    rng: &mut ThreadRng,
    world: &mut WorldStorage,
) {
    for x in 0..world.get_width() {
        // add and change to Blocks::Bedrock
        world.set_tile_usize(x, 0, Blocks::Dirt);

        if rng.gen_bool(0.8) {
            world.set_tile_usize(x, 1, Blocks::Dirt);
        } if rng.gen_bool(0.6) {
            world.set_tile_usize(x, 2, Blocks::Dirt);
        } if rng.gen_bool(0.4) {
            world.set_tile_usize(x, 3, Blocks::Dirt);
        } if rng.gen_bool(0.2) {
            world.set_tile_usize(x, 4, Blocks::Dirt);
        }
    }
}