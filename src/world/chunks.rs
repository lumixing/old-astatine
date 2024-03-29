use bevy::{prelude::*, utils::HashMap, math::{Vec3Swizzles, Vec2Swizzles, ivec2, vec2}};
use bevy_ecs_tilemap::{tiles::*, prelude::{TilemapId, TilemapTexture}, TilemapBundle};
use bevy_tileset::prelude::*;

use super::{storage::WorldStorage, blocks::Blocks};

const CHUNK_SIZE: UVec2 = UVec2 { x: 64, y: 64 };
const I_CHUNK_SIZE: IVec2 = IVec2 {
    x: CHUNK_SIZE.x as i32,
    y: CHUNK_SIZE.y as i32,
};
pub(super) const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

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

#[derive(Component)]
pub struct Dirty;

#[derive(Component)]
pub struct ChunkPos(IVec2);

#[derive(Component)]
pub struct Collidable;

#[derive(Component)]
pub struct GlobalTilePos(pub IVec2);

pub fn despawn_chunks(
    mut commands: Commands,
    tilesets: Tilesets,
    camera_query: Query<(&Transform, &LoadPoint), With<Camera>>,
    chunks_query: Query<(Entity, &Transform), With<TileStorage>>,
    mut rendered_chunks: ResMut<RenderedChunks>
) {
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let (camera_transform, load_point) = camera_query.single();
    for (chunk_entity, chunk_transform) in chunks_query.iter() {
        if camera_transform.translation.xy().distance(chunk_transform.translation.xy()) > (load_point.radius * CHUNK_SIZE.x) as f32 * tileset.tile_size().x * 2.0 {
            let chunk_pos = camera_pos_to_chunk_pos(chunk_transform.translation.xy(), tileset.tile_size());
            rendered_chunks.loaded.remove(&chunk_pos);
            commands.entity(chunk_entity).despawn_recursive();
        }
    }
}

pub fn spawn_chunks(
    mut commands: Commands,
    tilesets: Tilesets,
    world_storage: Res<WorldStorage>,
    camera_query: Query<(&Transform, &LoadPoint), With<Camera>>,
    mut rendered_chunks: ResMut<RenderedChunks>
) {
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let (transform, load_point) = camera_query.single();
    let camera_chunk_pos = camera_pos_to_chunk_pos(transform.translation.xy(), tileset.tile_size());

    // loop through camera visible chunks (?)
    for y in (camera_chunk_pos.y - load_point.radius as i32)..(camera_chunk_pos.y + load_point.radius as i32) {
        for x in (camera_chunk_pos.x - load_point.radius as i32)..(camera_chunk_pos.x + load_point.radius as i32) {
            let chunk_pos = IVec2::new(x, y);
            if rendered_chunks.loaded.contains_key(&chunk_pos) { continue; }
            let chunk = spawn_chunk(
                &mut commands,
                |x, y| world_storage.in_bounds(x, y),
                |x, y| world_storage.get_tile(x, y) as u32,
                tileset,
                chunk_pos,
                1.0,
            );
            rendered_chunks.loaded.insert(chunk_pos, chunk);
        }
    }
}

fn spawn_chunk<F, V>(
    commands: &mut Commands,
    in_bounds: F,
    get_content: V,
    tileset: &Tileset,
    chunk_pos: IVec2,
    chunk_z: f32,
) -> Entity
where
    F: Fn(i32, i32) -> bool,
    V: Fn(i32, i32) -> u32,
{
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    let tile_size = tileset.tile_size();
    let chunk_transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * tile_size.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * tile_size.y,
        chunk_z,
    ));

    let tileset_handle = tileset.texture();

    commands
        .entity(tilemap_entity)
        .with_children(|builder| {
            for x in 0..CHUNK_SIZE.x {
                for y in 0..CHUNK_SIZE.y {
                    let tile_pos = TilePos { x, y };
                    let tile_pos_x = chunk_pos.x * CHUNK_SIZE.x as i32 + tile_pos.x as i32;
                    let tile_pos_y = chunk_pos.y * CHUNK_SIZE.y as i32 + tile_pos.y as i32;

                    let tile_index = if !in_bounds(tile_pos_x, tile_pos_y) {
                        0
                    } else {
                        get_content(tile_pos_x, tile_pos_y)
                    };

                    let tile_entity = builder
                        .spawn((
                            TileBundle {
                                position: tile_pos,
                                texture_index: TileTextureIndex(tile_index),
                                tilemap_id: TilemapId(builder.parent_entity()),
                                ..default()
                            },
                            GlobalTilePos(ivec2(tile_pos_x, tile_pos_y))
                        ))
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .insert(TilemapBundle {
            grid_size: tile_size.into(),
            size: CHUNK_SIZE.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(tileset_handle.clone()),
            tile_size: tile_size.into(),
            transform: chunk_transform,
            ..default()
        })
        .insert(ChunkPos(chunk_pos))
        .id()
}

pub fn camera_pos_to_chunk_pos(camera_pos: Vec2, tile_size: Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let tile_size = tile_size.as_ivec2();
    camera_pos / (I_CHUNK_SIZE * tile_size)
}

pub fn dirty_rendered_chunk(
    mut commands: Commands,
    chunk_pos: &IVec2, 
    rendered_chunks: ResMut<RenderedChunks>
) {
    if !rendered_chunks.loaded.contains_key(chunk_pos) { return; }
    let entity = rendered_chunks.loaded.get(chunk_pos).unwrap();
    commands.entity(*entity).insert(Dirty);
}

pub fn despawn_dirty_chunks(
    mut commands: Commands,
    dirty_query: Query<(Entity, &ChunkPos), With<Dirty>>,
    mut rendered_chunks: ResMut<RenderedChunks>
) {
    for (entity, chunk_pos) in dirty_query.iter() {
        rendered_chunks.loaded.remove(&chunk_pos.0);
        commands.entity(entity).despawn_recursive();
    }
}

pub fn make_chunk_collidable(
    mut commands: Commands,
    q: Query<(&ChunkPos, &TileStorage)>,
    rendered_chunks: Res<RenderedChunks>,
    world_storage: Res<WorldStorage>,
) {
    for (chunk_pos, tile_storage) in q.iter() {
        if chunk_pos.0.xy() != ivec2(0, 0) { continue; }
        if !rendered_chunks.loaded.contains_key(&chunk_pos.0) {
            info!("uh oh! unloaded chunk :3");
            break;
        }
        for y in 0..32 {
            for x in 0..32 {
                if world_storage.get_tile_usize(x, y) == Blocks::Air { continue; }
                let entity = tile_storage.get(&TilePos { x: x as u32, y: y as u32 }).unwrap();
                commands.entity(entity).insert(Collidable);
            }
        }
        break;
    }
}

pub fn make_coll(
    position: Vec2,
    mut commands: Commands,
    rendered_chunks: Res<RenderedChunks>,
    world_storage: Res<WorldStorage>,
) {
    let chunk_pos = camera_pos_to_chunk_pos(position, vec2(8.0, 8.0));

    if !rendered_chunks.loaded.contains_key(&chunk_pos) {
        info!("tried making collisions for unrendered chunk!");
        return;
    }

    let tile_storage = rendered_chunks.loaded.get(&chunk_pos);

    for y in 0..32 {
        for x in 0..32 {
            if world_storage.get_tile_usize(x, y) == Blocks::Air { continue; }
            let entity = tile_storage.get(&TilePos { x: x as u32, y: y as u32 }).unwrap();
            commands.entity(entity).insert(Collidable);
        }
    }
}