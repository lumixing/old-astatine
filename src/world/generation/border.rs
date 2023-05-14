use crate::world::WorldStorage;

pub fn generate(
    world: &mut WorldStorage,
) {
    // border at world edges
    for x in 0..world.get_width() {
        world.set_tile_usize(x, 0, 1);
        world.set_tile_usize(x, world.get_height() - 1, 1);
    }

    for y in 0..world.get_height() {
        world.set_tile_usize(0, y, 1);
        world.set_tile_usize(world.get_width() - 1, 0, 1);
    }
}