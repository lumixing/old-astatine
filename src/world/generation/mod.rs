mod uni;

use bevy::prelude::*;
use bevy_tileset::prelude::Tilesets;

use crate::states::GameState;

use super::WorldStorage;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generate.in_schedule(OnEnter(GameState::WorldGeneration)));
    }
}

fn generate(mut commands: Commands, tilesets: Tilesets) {
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let mut world = WorldStorage::from_dimensions(1024, 128);
    let mut rng = rand::thread_rng();

    uni::generate(&mut rng, &mut world, &tileset);

    commands.insert_resource(world);
    commands.insert_resource(NextState(Some(GameState::InGame)));
}