use crate::world::WorldStorage;
use crate::world::blocks::Blocks;

#[allow(dead_code)]
pub fn generate(
    world: &mut WorldStorage,
) {
    // border at world edges
    for x in 0..world.get_width() {
        world.set_tile_usize(x, 0, Blocks::Border);
        world.set_tile_usize(x, world.get_height() - 1, Blocks::Border);
    }

    for y in 0..world.get_height() {
        world.set_tile_usize(0, y, Blocks::Border);
        world.set_tile_usize(world.get_width() - 1, 0, Blocks::Border);
    }
}