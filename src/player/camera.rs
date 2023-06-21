use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

use super::player::Player;

#[derive(Component)]
pub struct PlayerCamera;

pub fn spawn(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;
    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::rgb(71./255., 209./255., 1.));

    commands.spawn((
        camera_bundle,
        PlayerCamera
    ));
}

pub fn follow_player(
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = player_transform.translation;
}