mod storage;
pub(crate) mod chunks;
mod generation;
pub(crate) mod blocks;

// pub use chunks::LoadPoint;
pub use storage::WorldStorage;

use bevy::{prelude::*, math::uvec2};
use bevy_asset_loader::prelude::{AssetCollection, LoadingStateAppExt};
use bevy_ecs_tilemap::prelude::TilemapRenderSettings;
use bevy_tileset::prelude::Tileset;

use crate::states::GameState;

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
        // app.insert_resource(TilemapRenderSettings {
        //     render_chunk_size: uvec2(chunks::RENDER_CHUNK_SIZE, chunks::RENDER_CHUNK_SIZE),
        //     ..default()
        // });

        app.add_plugin(bevy_ecs_tilemap::TilemapPlugin);
        app.add_plugin(bevy_tileset::prelude::TilesetPlugin::default());
        app.add_collection_to_loading_state::<_, TileTextures>(GameState::AssetLoading);
        app.init_resource::<chunks::RenderedChunks>();        
        app.add_systems((
            chunks::spawn_all_chunks,
        ).in_schedule(OnEnter(GameState::InGame)));
        app.add_systems((
            // chunks::spawn_chunks_near_player,
            chunks::announce_chunks,
        ).in_set(OnUpdate(GameState::InGame)));
        app.add_plugin(generation::WorldGenerationPlugin);
    }
}