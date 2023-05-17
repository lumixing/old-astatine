use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, input::mouse::MouseWheel, window::PrimaryWindow};
use bevy_tileset::prelude::Tilesets;

use crate::world::{LoadPoint, WorldStorage};
use crate::world::blocks::Blocks;
use crate::world::chunks::{self, RenderedChunks};

use super::player::Player;

#[derive(Component)]
pub struct PlayerCamera;

const CAMERA_MOVE_SPEED: f32 = 5.0;

pub fn spawn(
    mut commands: Commands,
    tilesets: Tilesets,
    world: Res<WorldStorage>
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;
    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::rgb(71./255., 209./255., 1.));
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let tile_size = tileset.tile_size();
    let spawn_point = world.get_spawn_point();
    camera_bundle.transform.translation.x = spawn_point.x as f32 * tile_size.x;
    camera_bundle.transform.translation.y = spawn_point.y as f32 * tile_size.y;

    commands.spawn((
        camera_bundle,
        LoadPoint::new(4),
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

pub fn mouse(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    mut world: ResMut<WorldStorage>,
    tilesets: Tilesets,
    rendered_chunks: ResMut<RenderedChunks>
) {
    if !buttons.just_pressed(MouseButton::Left) { return }

    let (camera, camera_transform) = camera_query.single();
    let Ok(primary) = window_query.get_single() else { return };

    if let Some(world_position) = primary
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let tileset = tilesets.get_by_name("world_tiles").unwrap();
        let tile_pos = (world_position.as_ivec2() + 4) / 8;
        if !world.in_bounds(tile_pos.x, tile_pos.y) { return; }

        
        world.set_tile(tile_pos.x, tile_pos.y, Blocks::Air);
        let chunk_pos = chunks::camera_pos_to_chunk_pos(world_position, tileset.tile_size());
        info!("clicking at tile:{tile_pos}, chunk:{chunk_pos}");
        chunks::dirty_rendered_chunk(commands, &chunk_pos, rendered_chunks);
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