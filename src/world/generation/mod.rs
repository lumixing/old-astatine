mod uni;

use bevy::prelude::*;

use crate::states::GameState;

use super::WorldStorage;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generate.in_schedule(OnEnter(GameState::WorldGeneration)));
    }
}

// if something breaks down, re-add &Tilesets and pass to gen fns 
fn generate(mut commands: Commands) {
    let mut world = WorldStorage::from_dimensions(1024, 128);
    let mut rng = rand::thread_rng();

    uni::generate(&mut rng, &mut world);

    commands.insert_resource(world);
    commands.insert_resource(NextState(Some(GameState::InGame)));
}