use std::ops::{Div, Add};

use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, input::mouse::MouseWheel, math::ivec2};
use bevy_ecs_tilemap::tiles::{TileStorage, TilePos, TileTextureIndex};
// use bevy_hanabi::*;
// use bevy_tileset::prelude::Tilesets;
// use bevy_tileset::prelude::Tilesets;

// use crate::world::{WorldStorage};
// use crate::world::blocks::Blocks;
// use crate::world::chunks::{self};

use crate::world::{WorldStorage, chunks::{RenderedChunks, CHUNK_SIZE_I}, blocks::Blocks};

use super::player::Player;

#[derive(Component)]
pub struct PlayerCamera;

const CAMERA_MOVE_SPEED: f32 = 5.0;

pub fn spawn(
    mut commands: Commands,
    // tilesets: Tilesets,
    // world: Res<WorldStorage>
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;
    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::rgb(71./255., 209./255., 1.));
    // let tileset = tilesets.get_by_name("world_tiles").unwrap();
    // let tile_size = tileset.tile_size();
    // let spawn_point = world.get_spawn_point();
    // camera_bundle.transform.translation.x = spawn_point.x as f32 * tile_size.x;
    // camera_bundle.transform.translation.y = spawn_point.y as f32 * tile_size.y;

    commands.spawn((
        camera_bundle,
        PlayerCamera
        // LoadPoint::new(4),
    ));
}

#[allow(dead_code)]
pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut q: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    // translate camera
    let (mut transform, mut projection) = q.single_mut();

    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= CAMERA_MOVE_SPEED * projection.scale;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += CAMERA_MOVE_SPEED * projection.scale;
    }
    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += CAMERA_MOVE_SPEED * projection.scale;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= CAMERA_MOVE_SPEED * projection.scale;
    }
    if keyboard_input.pressed(KeyCode::E) {
        projection.scale = 0.5;
    }

    // scroll zoom camera
    for ev in scroll_evr.iter() {
        projection.scale -= ev.y * 0.1;
    }
}

pub fn mine(
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    // tilesets: Tilesets,
    mut world_storage: ResMut<WorldStorage>,
    // mut commands: Commands,
    rendered_chunks: Res<RenderedChunks>,
    mut chunk_query: Query<&mut TileStorage>,
    mut tile_query: Query<&mut TileTextureIndex>,
) {
    let window = windows.single();
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let (camera, camera_global_transform) = camera_query.single();

    if mouse_input.pressed(MouseButton::Left) {
        let Some(world_cursor_pos) = camera.viewport_to_world_2d(camera_global_transform, cursor_pos) else { return; };
        if world_cursor_pos.x < 0.0 || world_cursor_pos.y < 0.0 { return; };
        let tile_cursor_pos = world_cursor_pos.as_ivec2().add(4).div(8); // change this if something breaks
        world_storage.set_tile(tile_cursor_pos.x, tile_cursor_pos.y, Blocks::Air);

        let chunk_pos = tile_cursor_pos.div(64);
        let Some(chunk_entity) = rendered_chunks.get_chunk(chunk_pos) else { return; };
        let tile_storage = chunk_query.get_mut(*chunk_entity).unwrap();
        let chunk_rel_pos = ivec2(tile_cursor_pos.x - chunk_pos.x * CHUNK_SIZE_I, tile_cursor_pos.y - chunk_pos.y * CHUNK_SIZE_I);
        let tile_pos = TilePos { x: chunk_rel_pos.x as u32, y: chunk_rel_pos.y as u32 };
        let Some(tile) = tile_storage.get(&tile_pos) else { return; };
        let mut tile_texture = tile_query.get_mut(tile).unwrap();
        tile_texture.0 = Blocks::Air as u32;
    } else if mouse_input.pressed(MouseButton::Right) {
        let Some(world_cursor_pos) = camera.viewport_to_world_2d(camera_global_transform, cursor_pos) else { return; };
        if world_cursor_pos.x < 0.0 || world_cursor_pos.y < 0.0 { return; };
        let tile_cursor_pos = world_cursor_pos.as_ivec2().add(4).div(8); // change this if something breaks
        world_storage.set_tile(tile_cursor_pos.x, tile_cursor_pos.y, Blocks::Dirt);

        let chunk_pos = tile_cursor_pos.div(64);
        let Some(chunk_entity) = rendered_chunks.get_chunk(chunk_pos) else { return; };
        let tile_storage = chunk_query.get_mut(*chunk_entity).unwrap();
        let chunk_rel_pos = ivec2(tile_cursor_pos.x - chunk_pos.x * CHUNK_SIZE_I, tile_cursor_pos.y - chunk_pos.y * CHUNK_SIZE_I);
        let tile_pos = TilePos { x: chunk_rel_pos.x as u32, y: chunk_rel_pos.y as u32 };
        let Some(tile) = tile_storage.get(&tile_pos) else { return; };
        let mut tile_texture = tile_query.get_mut(tile).unwrap();
        tile_texture.0 = Blocks::Dirt as u32;
    }
}

pub fn follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = player_transform.translation;
}