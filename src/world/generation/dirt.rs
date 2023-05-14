use crate::world::WorldStorage;

pub fn generate(
    world: &mut WorldStorage,
) {
    // fill whole world with dirt
    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            world.set_tile_usize(x, y, 2);
        }
    }
}