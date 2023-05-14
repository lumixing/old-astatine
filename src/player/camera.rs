use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, input::mouse::MouseWheel};
use bevy_tileset::prelude::Tilesets;
use crate::world::{LoadPoint, WorldStorage};

#[derive(Component)]
pub struct PlayerCamera;

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

pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut q: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    // translate camera
    let (mut transform, mut projection) = q.single_mut();

    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= 5.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += 5.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += 5.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= 5.0;
    }
    if keyboard_input.pressed(KeyCode::E) {
        projection.scale = 0.5;
    }

    // scroll zoom camera
    for ev in scroll_evr.iter() {
        projection.scale -= ev.y * 0.1;
    }
}