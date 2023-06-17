use bevy::{prelude::*, math::{ivec2, uvec2}, utils::HashMap};
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::*;

use crate::player::player::Player;

use super::{WorldStorage, storage::ChunkData};

#[derive(Resource, Debug, Clone, Default)]
pub struct RenderedChunks {
    loaded: HashMap<IVec2, Entity>
}

#[derive(Component, Debug)]
pub struct ChunkPos(pub IVec2);

pub const CHUNK_SIZE: u32 = 64;
pub const TILE_SIZE: u32 = 8;
pub(super) const RENDER_CHUNK_SIZE: u32 = CHUNK_SIZE * 2;
pub const WORLD_SIZE: IVec2 = ivec2(4, 4);

pub fn init_world_storage() {

}

pub fn spawn_all_chunks(
    mut commands: Commands,
    tilesets: Tilesets,
    mut rendered_chunks: ResMut<RenderedChunks>,
    world_storage: Res<WorldStorage>,
) {
    let tileset = tilesets.get_by_name("world_tiles").unwrap();

    for y in 0..WORLD_SIZE.y {
        for x in 0..WORLD_SIZE.x {
            let chunk_pos = ivec2(x, y);
            let chunk_entity = spawn_chunk(&mut commands, tileset, &world_storage.0, chunk_pos).unwrap();
            rendered_chunks.loaded.insert(chunk_pos, chunk_entity);
        }
    }
}

// pub fn spawn_chunks_near_player(
//     mut commands: Commands,
//     tilesets: Tilesets,
//     mut rendered_chunks: ResMut<RenderedChunks>,
//     world_storage: Res<WorldStorage>,
//     player_query: Query<&ChunkPos, (With<Player>, Changed<ChunkPos>)>
// ) {
//     let tileset = tilesets.get_by_name("world_tiles").unwrap();
//     let Ok(player_chunk_pos) = player_query.get_single() else { return };

//     despawn_all_chunks(&mut commands, &mut rendered_chunks);
//     for y in -1..=1 {
//         for x in -1..=1 {
//             let chunk_pos = player_chunk_pos.0 + ivec2(x, y);
//             if let Some(chunk_entity) = spawn_chunk(&mut commands, tileset, world_storage, chunk_pos) {
//                 rendered_chunks.loaded.insert(chunk_pos, chunk_entity);
//             }
//         }
//     }
// }

/// spawns and returns chunk entity if in bounds
fn spawn_chunk(
    commands: &mut Commands,
    tileset: &Tileset,
    world_storage: &HashMap<IVec2, ChunkData>,
    chunk_pos: IVec2
) -> Option<Entity> {
    if is_out_of_bounds(chunk_pos) { 
        info!("couldnt spawn chunk at {chunk_pos} since its out of bounds!");
        return None
    }

    info!("spawning chunk at {chunk_pos}");

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TilemapSize { x: CHUNK_SIZE, y: CHUNK_SIZE });
    let tileset_handle = tileset.texture();
    let chunk_transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE as f32 * TILE_SIZE as f32,
        chunk_pos.y as f32 * CHUNK_SIZE as f32 * TILE_SIZE as f32,
        0.0
    ));

    let chunk_entity = commands.entity(tilemap_entity)
        .with_children(|builder| {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let tile = world_storage.get(&chunk_pos).unwrap().get_tile(x as i32, y as i32).unwrap();
                    let tile_pos = TilePos { x, y };
                    let tile_entity = builder.spawn(TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(tile),
                        tilemap_id: TilemapId(builder.parent_entity()),
                        ..default()
                    }).id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .insert((
            TilemapBundle {
                transform: chunk_transform,
                storage: tile_storage,
                size: TilemapSize { x: CHUNK_SIZE, y: CHUNK_SIZE },
                grid_size: TilemapGridSize { x: TILE_SIZE as f32, y: TILE_SIZE as f32 },
                tile_size: TilemapTileSize { x: TILE_SIZE as f32, y: TILE_SIZE as f32 },
                texture: TilemapTexture::Single(tileset_handle.clone()),
                ..default()
            },
            ChunkPos(chunk_pos)
        ))
        .id();
    Some(chunk_entity)
}

fn despawn_all_chunks(
    commands: &mut Commands,
    rendered_chunks: &mut ResMut<RenderedChunks>
) {
    for (chunk_pos, chunk_entity) in rendered_chunks.loaded.iter() {
        info!("despawning chunk at {chunk_pos}");
        commands.entity(*chunk_entity).despawn_recursive();
    }
    rendered_chunks.loaded.clear();
}

fn is_out_of_bounds(chunk_pos: IVec2) -> bool {
    chunk_pos.x < 0 || chunk_pos.y < 0 || chunk_pos.x >= WORLD_SIZE.x || chunk_pos.y >= WORLD_SIZE.y
}

pub fn announce_chunks(
    rendered_chunks: Res<RenderedChunks>
) {
    if !rendered_chunks.is_changed() { return }
    info!("{} loaded chunks", rendered_chunks.loaded.keys().len());
}