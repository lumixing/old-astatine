use std::ops::Div;

use bevy::{prelude::*, math::{ivec2, Vec3Swizzles}, utils::HashMap};
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::*;

use crate::player::player::Player;

pub const CHUNK_SIZE: u32 = 64;
pub const TILE_SIZE: u32 = 8;
pub(super) const RENDER_CHUNK_SIZE: u32 = CHUNK_SIZE * 2;

#[derive(Component, Debug)]
pub struct ChunkPos(pub IVec2);

#[derive(Component, Clone, Copy, Debug)]
pub struct LoadPoint {
    radius: u32,
}

impl LoadPoint {
    pub fn new(radius: u32) -> Self {
        Self { radius }
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct RenderedChunks {
    loaded: HashMap<IVec2, Entity>,
}

pub fn init(
    mut commands: Commands,
    tilesets: Tilesets,
    mut rendered_chunks: ResMut<RenderedChunks>
) { 
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    for y in 0..4 {
        for x in 0..4 {
            let chunk_pos = ivec2(x, y);
            let chunk_entity = spawn_chunk(&mut commands, chunk_pos, tileset);
            rendered_chunks.loaded.insert(chunk_pos, chunk_entity);
        }
    }
}

pub fn player_moved_chunk(
    mut commands: Commands,
    tilesets: Tilesets,
    player_query: Query<&ChunkPos, (With<Player>, Changed<ChunkPos>)>,
    mut rendered_chunks: ResMut<RenderedChunks>
) {
    let Ok(player_chunk_pos) = player_query.get_single() else { return };
    info!("PLAYER MOVED REEEEE ACTIVATING CHAOS I REPEAT ACTIVATING CHAOS.");
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    // despawn
    for (chunk_pos, chunk_entity) in rendered_chunks.loaded.iter() {
        info!("deleting chunk {chunk_pos}");
        commands.entity(*chunk_entity).despawn_recursive();
    }
    rendered_chunks.loaded.clear();
    // spawn
    rendered_chunks.loaded.insert(player_chunk_pos.0, spawn_chunk(&mut commands, player_chunk_pos.0, tileset));
}

fn spawn_chunk(
    commands: &mut Commands,
    chunk_pos: IVec2,
    tileset: &Tileset
) -> Entity {
    info!("spawning chunk at {chunk_pos}");

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TilemapSize { x: CHUNK_SIZE, y: CHUNK_SIZE });
    let chunk_transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE as f32 * TILE_SIZE as f32,
        chunk_pos.y as f32 * CHUNK_SIZE as f32 * TILE_SIZE as f32,
        0.0
    ));

    let tileset_handle = tileset.texture();

    commands.entity(tilemap_entity)
        .with_children(|builder| {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let tile_pos = TilePos { x, y };
                    let tile_entity = builder.spawn(TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex((chunk_pos.y as u32 * 4 + chunk_pos.x as u32 + chunk_pos.y as u32 % 2) % 2 + 1),
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
        .id()
}

pub fn announce_chunks(
    rendered_chunks: Res<RenderedChunks>
) {
    if !rendered_chunks.is_changed() { return }
    info!("{} loaded chunks", rendered_chunks.loaded.keys().len());
}