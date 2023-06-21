use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingStateAppExt};
use bevy_tileset::prelude::Tileset;

use crate::states::GameState;

pub mod position;
mod storage;
mod chunks;
mod generation;

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub struct TileTextures {
    #[asset(path = "world_tiles.ron")]
    tileset: Handle<Tileset>,
    #[asset(path = "world_walls.ron")]
    wallset: Handle<Tileset>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_ecs_tilemap::TilemapPlugin);
        app.add_plugin(bevy_tileset::prelude::TilesetPlugin::default());

        app.add_collection_to_loading_state::<_, TileTextures>(GameState::AssetLoading);
        app.init_resource::<chunks::LoadedChunks>(); 

        app.add_systems((
            chunks::spawn_chunks_near_player,
        ).in_set(OnUpdate(GameState::InGame)));

        app.add_plugin(generation::WorldGenerationPlugin);
    }
}