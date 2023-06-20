use bevy::{prelude::*, math::ivec2, utils::HashMap};
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::*;

use crate::player::player::Player;

use super::{WorldStorage, storage::ChunkData, positions::*};

#[derive(Resource, Debug, Clone, Default)]
pub struct RenderedChunks(HashMap<IVec2, Entity>);

#[allow(dead_code)]
impl RenderedChunks {
    pub fn get_chunk(&self, chunk_pos: IVec2) -> Option<&Entity> {
        self.hashmap().get(&chunk_pos)
    }
    
    pub fn add_chunk(&mut self, chunk_pos: IVec2, chunk_entity: Entity) {
        if is_out_of_bounds(chunk_pos) {
            warn!("added chunk {chunk_pos} that is out of bounds");
        }
        self.0.insert(chunk_pos, chunk_entity);
    }

    pub fn remove_all_chunks(&mut self) {
        self.0.clear();
    }

    pub fn hashmap(&self) -> &HashMap<IVec2, Entity> {
        &self.0
    }
}

#[derive(Resource)]
pub struct LightingTimer(pub Timer);

#[derive(Component)]
pub struct Collidable;

pub const CHUNK_SIZE: u32 = 64;
pub const CHUNK_SIZE_I: i32 = CHUNK_SIZE as i32;
pub const TILE_SIZE: u32 = 8;
// pub(super) const RENDER_CHUNK_SIZE: u32 = CHUNK_SIZE * 2;
pub const WORLD_SIZE: IVec2 = ivec2(32, 32);

pub fn spawn_chunks_near_player(
    mut commands: Commands,
    tilesets: Tilesets,
    mut rendered_chunks: ResMut<RenderedChunks>,
    world_storage: Res<WorldStorage>,
    player_query: Query<&ChunkPos, (With<Player>, Changed<ChunkPos>)>
) {
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let Ok(player_chunk_pos) = player_query.get_single() else { return };

    despawn_all_chunks(&mut commands, &mut rendered_chunks);
    for y in -1..=1 {
        for x in -1..=1 {
            let chunk_pos = player_chunk_pos + ivec2(x, y);
            let Some(chunk_data) = world_storage.get_chunk_data(chunk_pos) else { continue; }; // skip out of bounds chunks
            let chunk_entity = spawn_chunk(&mut commands, tileset, chunk_data, chunk_pos).unwrap(); // should be able to unwrap since chunk_data exists
            rendered_chunks.add_chunk(chunk_pos, chunk_entity);
        }
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    tileset: &Tileset,
    chunk_data: &ChunkData,
    chunk_pos: IVec2
) -> Option<Entity> {
    if is_out_of_bounds(chunk_pos) { return None; };

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
                    let tile = chunk_data.get_tile(x as i32, y as i32).unwrap();
                    
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
            ChunkPos(chunk_pos)
        ))
        .id();
    Some(chunk_entity)
}

pub fn make_collidable_near_player(
    mut commands: Commands,
    player_query: Query<&BlockPos, With<Player>>,
    rendered_chunks: Res<RenderedChunks>,
    mut chunk_query: Query<&mut TileStorage>,
) {
    let block_pos = player_query.single();
    // let chunk_pos = block_pos.0.div(CHUNK_SIZE_I);
    let chunk_pos = ChunkPos::from_block_ivec(block_pos.0);
    // info!("making coll at {} -> {}", block_pos.0, chunk_pos.0);
    let Some(chunk_entity) = rendered_chunks.get_chunk(chunk_pos.0) else {
        // warn!("couldnt make collidable in unrendered chunk: {} -> {}", block_pos.0, chunk_pos.0);
        return;
    };
    // i REALLY need a helper method for these fucking positions
    let tile_storage = chunk_query.get_mut(*chunk_entity).unwrap();
    let chunk_rel_pos = ivec2(block_pos.0.x - chunk_pos.0.x * CHUNK_SIZE_I, block_pos.0.y - chunk_pos.0.y * CHUNK_SIZE_I);
    let tile_pos = TilePos { x: chunk_rel_pos.x as u32, y: chunk_rel_pos.y as u32 };
    let Some(tile) = tile_storage.get(&tile_pos) else {
        // warn!("couldnt get tile entity lol idk y");
        return;
    };
    commands.entity(tile).insert(Collidable);
}

pub fn unmake_all_collidables(
    mut commands: Commands,
    q: Query<Entity, With<Collidable>>
) {
    // info!("unmaking {} colls", q.iter().count());
    for entity in q.iter() {
        commands.entity(entity).remove::<Collidable>();
    }
}

fn despawn_all_chunks(
    commands: &mut Commands,
    rendered_chunks: &mut ResMut<RenderedChunks>
) {
    for (_chunk_pos, chunk_entity) in rendered_chunks.hashmap().iter() {
        // info!("despawning chunk at {chunk_pos}");
        commands.entity(*chunk_entity).despawn_recursive();
    }
    rendered_chunks.remove_all_chunks();
}

fn is_out_of_bounds(chunk_pos: IVec2) -> bool {
    chunk_pos.x < 0 || chunk_pos.y < 0 || chunk_pos.x >= WORLD_SIZE.x || chunk_pos.y >= WORLD_SIZE.y
}

pub fn announce_chunks(
    rendered_chunks: Res<RenderedChunks>
) {
    if !rendered_chunks.is_changed() { return }
    // info!("{} loaded chunks", rendered_chunks.loaded.keys().len());
}