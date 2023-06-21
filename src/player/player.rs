use bevy::{prelude::*, math::vec3};

use crate::world::position::{ChunkPos, BlockPos};

#[derive(Component)]
pub struct Player;

pub fn spawn(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                ..default()
            },
            transform: Transform {
                translation: vec3(100.0, 100.0, 20.0),
                scale: vec3(8.0, 16.0, 8.0),
                ..default()
            },
            ..default()
        },
        Player,
        BlockPos::new(0, 0),
        ChunkPos::new(0, 0)
    ));
}

pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    // translate camera
    let mut transform = player_query.single_mut();

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
}

pub fn update_positions(
    mut player_query: Query<(&Transform, &mut BlockPos, &mut ChunkPos), With<Player>>
) {
    let (transform, mut block_pos, mut chunk_pos) = player_query.single_mut();
    let translation = transform.translation;

    let new_block_pos = BlockPos::from_world_pos(translation.x, translation.y);
    if *block_pos != new_block_pos {
        *block_pos = new_block_pos;
    }

    let new_chunk_pos = ChunkPos::from_world_pos(translation.x, translation.y);
    if *chunk_pos != new_chunk_pos {
        *chunk_pos = new_chunk_pos;
    }
}