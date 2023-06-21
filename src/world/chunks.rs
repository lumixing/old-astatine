use bevy::{prelude::*, math::ivec2, utils::HashMap};
use bevy_ecs_tilemap::tiles::TileStorage;
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::{Tilesets, Tileset};

use crate::player::player::Player;

use super::{position::{ChunkPos, CHUNK_SIZE, TILE_SIZE, BlockPos}, storage::{WorldStorage, ChunkData}};

pub const WORLD_SIZE: IVec2 = ivec2(8, 8);

#[derive(Resource, Default)]
pub struct LoadedChunks(HashMap<ChunkPos, Entity>);

#[allow(dead_code)]
impl LoadedChunks {
    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Option<&Entity> {
        self.0.get(&chunk_pos)
    }

    pub fn add_chunk(&mut self, chunk_pos: ChunkPos, chunk_entity: Entity) {
        if is_out_of_bounds(chunk_pos) {
            warn!("could not load chunk that is out of bounds: {:?}", chunk_pos);
            return;
        }
        self.0.insert(chunk_pos, chunk_entity);
    }

    pub fn remove_all_chunks(&mut self) {
        self.0.clear();
    }
}

// pub fn spawn_all_chunks(
//     mut commands: Commands,
//     mut loaded_chunks: ResMut<LoadedChunks>,
//     world_storage: Res<WorldStorage>,
//     tilesets: Tilesets,
// ) {
//     let tileset = tilesets.get_by_name("world_tiles").unwrap();
//     for y in 0..WORLD_SIZE.y {
//         for x in 0..WORLD_SIZE.x {
//             let chunk_pos = ChunkPos::new(x as u32, y as u32); 
//             let chunk_data = world_storage.get_chunk_data(chunk_pos).unwrap();
//             let chunk_entity = spawn_chunk(&mut commands, tileset, chunk_pos, chunk_data).unwrap();
//             loaded_chunks.add_chunk(chunk_pos, chunk_entity);
//         }
//     }
// }

pub fn spawn_chunks_near_player(
    mut commands: Commands,
    tilesets: Tilesets,
    mut loaded_chunks: ResMut<LoadedChunks>,
    world_storage: Res<WorldStorage>,
    player_query: Query<&ChunkPos, (With<Player>, Changed<ChunkPos>)>
) {
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let Ok(player_chunk_pos) = player_query.get_single() else { return };

    despawn_all_chunks(&mut commands, &mut loaded_chunks);
    for y in -1..=1 {
        for x in -1..=1 {
            let chunk_pos_raw = ivec2(x + player_chunk_pos.x() as i32, y + player_chunk_pos.y() as i32);
            if is_raw_out_of_bounds(chunk_pos_raw) { continue; };
            let chunk_pos = ChunkPos::new(chunk_pos_raw.x as u32, chunk_pos_raw.y as u32);
            let chunk_data = world_storage.get_chunk_data(chunk_pos).unwrap(); // else this if error
            let chunk_entity = spawn_chunk(&mut commands, tileset, chunk_pos, chunk_data).unwrap();
            loaded_chunks.add_chunk(chunk_pos, chunk_entity);
        }
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    tileset: &Tileset,
    chunk_pos: ChunkPos,
    chunk_data: &ChunkData
) -> Option<Entity> {
    if is_out_of_bounds(chunk_pos) { return None; };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TilemapSize { x: CHUNK_SIZE, y: CHUNK_SIZE });
    let tileset_handle = tileset.texture();
    let chunk_transform = Transform::from_translation(Vec3::new(
        (chunk_pos.x() * CHUNK_SIZE * TILE_SIZE) as f32,
        (chunk_pos.y() * CHUNK_SIZE * TILE_SIZE) as f32,
        0.0
    ));

    let chunk_entity = commands.entity(tilemap_entity)
        .with_children(|builder| {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let tile = chunk_data.get_tile(BlockPos::new(x, y)).unwrap();
                    
                    let tile_pos = TilePos { x, y };
                    let tile_entity = builder.spawn(TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(tile as u32),
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
            chunk_pos
        ))
        .id();
    Some(chunk_entity)
}

fn despawn_all_chunks(
    commands: &mut Commands,
    loaded_chunks: &mut ResMut<LoadedChunks>
) {
    for (_, chunk_entity) in loaded_chunks.0.iter() {
        commands.entity(*chunk_entity).despawn_recursive();
    }
    loaded_chunks.0.clear();
}

fn is_out_of_bounds(chunk_pos: ChunkPos) -> bool {
    chunk_pos.x() >= WORLD_SIZE.x as u32 || chunk_pos.y() >= WORLD_SIZE.y as u32
}

fn is_raw_out_of_bounds(chunk_pos_raw: IVec2) -> bool {
    chunk_pos_raw.x < 0 || chunk_pos_raw.y < 0 || chunk_pos_raw.x >= WORLD_SIZE.x || chunk_pos_raw.y >= WORLD_SIZE.y
}